-- Create a type for 256 bit unsigned integers
CREATE DOMAIN uint_256 AS NUMERIC NOT NULL
    CHECK (VALUE >= 0 AND VALUE < 2 ^ 256)
    CHECK (SCALE(VALUE) = 0);

-- TODO(Should accounts be represented as a uint160 rather than a hex string)
-- TODO(Should bytes32 be uint256 rather than a hex string)
-- TODO(Will it be easier to store uint256 as a CHAR(64) hex?)
-- TODO(Do timestamps really need to support uint256?)
-- TODO(Is there a natural key for offers and considerations, like a hash?)
-- TODO(Is there a natural key for order_components)
-- TODO(Is there a natural key for orders)

-- Create addresses table
CREATE TABLE addresses
(
    address CHAR(40) PRIMARY KEY
);

-- Create fees table
CREATE TABLE fees
(
    id           BIGSERIAL PRIMARY KEY,
    address      CHAR(40) REFERENCES addresses,
    -- Basis points can never exceed 327%, but they shouldn't need to.
    basis_points SMALLINT
);

-- Create considerations table
CREATE TABLE considerations
(
    id                     BIGSERIAL PRIMARY KEY,
    -- This is the ItemType enum
    item_type              SMALLINT,
    token                  CHAR(40) REFERENCES addresses,
    identifier_or_criteria uint_256,
    start_amount           uint_256,
    end_amount             uint_256,
    recipient              CHAR(40) REFERENCES addresses
);

-- Create offers table
CREATE TABLE offers
(
    id                     BIGSERIAL PRIMARY KEY,
    item_type              SMALLINT,
    token                  CHAR(40) REFERENCES addresses,
    identifier_or_criteria uint_256,
    start_amount           uint_256,
    end_amount             uint_256
);

-- Create order components table
CREATE TABLE order_components
(
    id            BIGSERIAL PRIMARY KEY,
    offerer       CHAR(40) REFERENCES addresses,
    zone          CHAR(40) REFERENCES addresses,
    offer         BIGSERIAL REFERENCES offers,
    consideration BIGSERIAL REFERENCES considerations,
    start_time    uint_256,
    end_time      uint_256,
    zone_hash     CHAR(64),
    salt          uint_256,
    conduit_key   CHAR(64),
    counter       uint_256
);

CREATE TABLE orders
(
    id               BIGSERIAL PRIMARY KEY,
    created_date     uint_256,
    closing_date     uint_256                      NULL,
    listing_time     uint_256,
    expiration_time  uint_256,
    order_hash       CHAR(64)                      NULL,
    protocol_data    BIGSERIAL REFERENCES order_components,
    protocol_address CHAR(40) REFERENCES addresses NULL,
    maker            CHAR(40) REFERENCES addresses,
    taker            CHAR(40) REFERENCES addresses NULL,
    -- TODO(Nail down the unit here)
    current_price    TEXT,
    maker_fees       BIGSERIAL REFERENCES fees,
    taker_fees       BIGSERIAL REFERENCES fees,
    side             SMALLINT,
    order_type       SMALLINT,
    canceled         BOOLEAN,
    finalized        BOOLEAN,
    marked_invalid   BOOLEAN,
    client_signature CHAR(64)                      NULL
);