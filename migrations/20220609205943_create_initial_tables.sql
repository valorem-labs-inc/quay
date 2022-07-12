-- Some food for thought:
-- https://dba.stackexchange.com/questions/62934/adding-unsigned-256-bit-integers-in-postgresql

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

CREATE TABLE orders
(
    id               BIGSERIAL PRIMARY KEY,
    created_date     BIGINT,
    closing_date     BIGINT                      NULL,
    listing_time     BIGINT,
    expiration_time  BIGINT,
    order_hash       CHAR(64)                      NULL,
    protocol_address CHAR(40) REFERENCES addresses NULL,
    maker            CHAR(40) REFERENCES addresses,
    taker            CHAR(40) REFERENCES addresses NULL,
    side             SMALLINT,
    order_type       SMALLINT,
    canceled         BOOLEAN,
    finalized        BOOLEAN,
    marked_invalid   BOOLEAN,
    client_signature CHAR(64)                      NULL
);