DROP FUNCTION IF EXISTS order_considerations_amount(TEXT,order_start_end_amount_sum_selector,TEXT);
DROP FUNCTION IF EXISTS order_offers_amount(TEXT,order_start_end_amount_sum_selector,TEXT);
DROP FUNCTION IF EXISTS get_orders_lite(TEXT,TEXT,TEXT[],TEXT[],BOOLEAN,order_lite_ordering_value_selector,order_lite_ordering_order_selector,INT,INT);

DROP TYPE IF EXISTS order_lite_ordering_value_selector;
DROP TYPE IF EXISTS order_lite_ordering_order_selector;
DROP TYPE IF EXISTS order_start_end_amount_sum_selector;

CREATE TYPE order_lite_ordering_value_selector AS ENUM (
    'OFFERS_AMOUNT',
    'CONSIDERATIONS_AMOUNT',
    'START_TIME',
    'END_TIME',
    'LISTING_TIME',
    'OFFERER',
    'HASH'
);
CREATE TYPE order_lite_ordering_order_selector AS ENUM ('ASC', 'DESC');
CREATE TYPE order_start_end_amount_sum_selector AS ENUM ('START', 'END');

CREATE OR REPLACE FUNCTION order_considerations_amount(order_hash TEXT, type order_start_end_amount_sum_selector, target_token TEXT)
    RETURNS NUMERIC AS
$$
    SELECT
        CASE WHEN type = 'START'::order_start_end_amount_sum_selector
            THEN SUM(C.start_amount)
            ELSE SUM(C.end_amount)
        END
    FROM considerations C
    WHERE ((C."order" = order_hash) OR order_hash = '') AND (C."token" = target_token)
$$ LANGUAGE sql STABLE;

CREATE OR REPLACE FUNCTION order_offers_amount(order_hash TEXT, type order_start_end_amount_sum_selector, target_token TEXT)
    RETURNS NUMERIC AS
$$
    SELECT
        CASE WHEN type = 'START'::order_start_end_amount_sum_selector
            THEN SUM(OF.start_amount)
            ELSE SUM(OF.end_amount)
        END
    FROM offers OF
    WHERE ((OF."order" = order_hash) OR order_hash = '') AND (OF."token" = target_token)
$$ LANGUAGE sql STABLE;

CREATE OR REPLACE FUNCTION get_orders_lite(
    asset_contract_address TEXT,
    currency_token_address TEXT,
    token_ids TEXT[],
    offerers TEXT[],
    active BOOLEAN,
    ordering order_lite_ordering_value_selector DEFAULT 'HASH'::order_lite_ordering_value_selector,
    ordering_order order_lite_ordering_order_selector DEFAULT 'ASC'::order_lite_ordering_order_selector,
    "limit" INT DEFAULT 10,
    "offset" INT DEFAULT 0
)
    RETURNS SETOF order_lite AS
$$
    WITH
        -- UOFIOC -> Unique OFfers Identifier Or Criteria
        -- UCIOC -> Unique Considerations Identifier Or Criteria
        -- UUIOC -> Unique Union Identifier Or Criteria
        -- GUIOCO -> Grouped Unique Identifier Or Consideration Orders
        -- LTOO -> Listing Time Ordered Orders
        -- LLO -> Limited Last Orders
        -- SO -> Switchable Orders
        UOFIOC AS (
            SELECT DISTINCT OF."identifier_or_criteria"
                FROM offers OF
                WHERE OF.token = asset_contract_address
        ),
        UCIOC AS (
            SELECT DISTINCT C."identifier_or_criteria"
                FROM considerations C
                WHERE C.token = asset_contract_address
        ),
        UUIOC AS (SELECT DISTINCT * FROM ((SELECT * FROM UOFIOC) UNION (SELECT * FROM UCIOC)) u),
        GUIOCO AS (
            SELECT
                UUIOC.identifier_or_criteria,
                array(
                    SELECT DISTINCT O.hash
                    FROM orders O
                        INNER JOIN considerations OC ON O.hash = OC.order
                        INNER JOIN offers OOF ON O.hash = OOF.order
                    WHERE 
                        OC.identifier_or_criteria = UUIOC.identifier_or_criteria
                     OR
                        OOF.identifier_or_criteria = UUIOC.identifier_or_criteria
                ) as container_orders
            FROM UUIOC
            GROUP BY UUIOC.identifier_or_criteria
        ),
        LTOO AS (
            SELECT *
            FROM orders O
                INNER JOIN GUIOCO ON O.hash = ANY(GUIOCO.container_orders)
            ORDER BY listing_time
        ),
        LLO AS (
            SELECT
                O."hash",
                O."offerer",
                O."zone",
                O."zone_hash",
                O."start_time",
                O."end_time",
                O."order_type",
                O."total_original_consideration_items",
                O."salt",
                O."conduit_key",
                O."signature",
                O."cancelled",
                O."finalized",
                O."marked_invalid",
                O."listing_time",
                O."counter"
            FROM GUIOCO G
                INNER JOIN (
                    SELECT DISTINCT ON (identifier_or_criteria) *
                        FROM LTOO
                        ORDER BY identifier_or_criteria, listing_time DESC
                ) O ON O.hash = ANY(G.container_orders)
        ),
        SO AS (
            SELECT *
            FROM LLO
            WHERE active IS TRUE
            UNION ALL
            SELECT *
            FROM orders
            WHERE active IS FALSE
        ),
        -- To be able to correctly do the dynamic ordering we need the actual data as a sub-query
        OSQ AS (
            SELECT
                O.signature AS "signature!",
                O.hash AS "hash!",
                O.start_time AS "start_time!",
                O.end_time AS "end_time!",
                O.order_type AS "order_type!",
                O.offerer AS "offerer!",
                O.listing_time AS "listing_time!",

                -- We sum all of the values in considerations and offers
                -- This is a nice and simple way to get the price of a listing for example
                order_considerations_amount(O.hash, 'START'::order_start_end_amount_sum_selector, currency_token_address) AS "considerations_total",
                order_offers_amount(O.hash, 'START'::order_start_end_amount_sum_selector, currency_token_address) AS "offers_total",

                array_agg(DISTINCT (
                    OOF.position,
                    OOF.item_type,
                    OOF.token,
                    OOF.identifier_or_criteria,
                    OOF.start_amount,
                    OOF.end_amount
                )::order_lite_item) AS "offers!: Vec<DBOrderItemSimple>",
                array_agg(DISTINCT (
                    OC.position,
                    OC.item_type,
                    OC.token,
                    OC.identifier_or_criteria,
                    OC.start_amount,
                    OC.end_amount
                )::order_lite_item) AS "considerations!: Vec<DBOrderItemSimple>",

                O.cancelled AS "cancelled!",
                O.finalized AS "finalized!",
                O.marked_invalid AS "marked_invalid!"
            FROM SO O
                INNER JOIN considerations OC ON O.hash = OC.order
                INNER JOIN offers OOF ON O.hash = OOF.order
            -- Filtering works as follows (equal indents should be as AND)
            -- 1) The offer OR consideration needs to have a token matching the passed "asset_contract_address" AND
            --    1.1) The "token id" (property "identifier_or_criteria") is present in the passed token_ids array OR the token_ids array is empty
            -- 2) The order offerer is present in the passed offerers array OR the offerers array is empty
            -- 3) If we decide to exclude invalid orders then we need to compare the 3 states (cancelled, finalized, and marked_invalid) as well as the start and end times
            WHERE
                (
                    O.hash IN (
                        SELECT OF.order FROM offers OF
                            WHERE
                                OF.token = asset_contract_address
                            AND
                                -- We check if the passed "token_ids" array contains the identifier or if the array is empty
                                -- If it's empty then we never actually intended to filter using it
                                (OF.identifier_or_criteria = ANY(token_ids) OR cardinality(token_ids) = 0)
                    )
                OR
                    O.hash IN (
                        SELECT C.order FROM considerations C 
                            WHERE 
                                C.token = asset_contract_address
                            AND
                                -- We check if the passed "token_ids" array contains the identifier or if the array is empty
                                -- If it's empty then we never actually intended to filter using it
                                (C.identifier_or_criteria = ANY(token_ids) OR cardinality(token_ids) = 0)
                    )
                )
                -- We check if the passed "offerers" array contains the offerer set in the order or if the array is empty
                -- If it's empty then we never actually intended to filter using it
            AND (O.offerer = ANY(offerers) OR cardinality(offerers) = 0)
            AND
                -- We invert "exclude_invalid" so that if's true then we trigger the 2nd part of the check and compare order validity
                ((NOT active) OR (
                        NOT O.cancelled
                    AND NOT O.finalized
                    AND NOT O.marked_invalid
                    AND     O.start_time <= extract(epoch from now())
                    AND     O.end_time >= extract(epoch from now())
                ))
            GROUP BY O.signature, O.hash, O.start_time, O.end_time, O.order_type, O.offerer, O.listing_time, O.cancelled, O.finalized, O.marked_invalid
        )
    SELECT *
    FROM OSQ
    ORDER BY
        ((CASE
            WHEN ordering = 'OFFERS_AMOUNT'::order_lite_ordering_value_selector THEN 8          -- "offers_total"
            WHEN ordering = 'CONSIDERATIONS_AMOUNT'::order_lite_ordering_value_selector THEN 7  -- "considerations_total"
            WHEN ordering = 'START_TIME'::order_lite_ordering_value_selector THEN 2             -- "start_time!"
            WHEN ordering = 'END_TIME'::order_lite_ordering_value_selector THEN 3               -- "end_time!"
            WHEN ordering = 'LISTING_TIME'::order_lite_ordering_value_selector THEN 6           -- "listing_time!"
            WHEN ordering = 'OFFERER'::order_lite_ordering_value_selector THEN 5                -- "offerer!"
            WHEN ordering = 'HASH'::order_lite_ordering_value_selector THEN 1                   -- "hash!"
            ELSE 1                                                                              -- "hash!"
                -- By default we're always in ASC (represented by 1) so then to switch to DESC (-1) or ASC (1, technically changes nothing)
                -- we switch multiple to switch over to DESC by representing the SELECTed column sequence as a negative number
        END) * (CASE WHEN ordering_order = 'ASC'::order_lite_ordering_order_selector THEN 1 ELSE -1 END)) ASC
    LIMIT "limit"
    OFFSET "offset";
$$ LANGUAGE SQL STABLE;
