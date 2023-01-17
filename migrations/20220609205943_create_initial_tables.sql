CREATE EXTENSION IF NOT EXISTS citext;

-- Create addresses table
CREATE TABLE addresses
(
    address citext PRIMARY KEY
);

CREATE TABLE networks
(
    network       INTEGER PRIMARY KEY,
    indexed_block BIGINT NOT NULL
);

CREATE TABLE orders
(
    hash TEXT PRIMARY KEY,

    offerer citext REFERENCES addresses(address) NOT NULL,

    zone citext REFERENCES addresses(address) NOT NULL,
    zone_hash TEXT NOT NULL,

    start_time BIGINT NOT NULL,
    end_time BIGINT NOT NULL,
    
    order_type INT NOT NULL,
    total_original_consideration_items INT NOT NULL,
    salt TEXT NOT NULL,

    counter BIGINT NOT NULL,
    conduit_key TEXT NOT NULL,

    signature TEXT NOT NULL,

    cancelled BOOLEAN NOT NULL DEFAULT FALSE,
    finalized BOOLEAN NOT NULL DEFAULT FALSE,
    marked_invalid BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE offers
(
    "order" TEXT REFERENCES orders(hash) NOT NULL,
    position INT NOT NULL,
    item_type INT NOT NULL,

    token citext REFERENCES addresses(address) NOT NULL,
    identifier_or_criteria TEXT NOT NULL,

    start_amount NUMERIC NOT NULL,
    end_amount NUMERIC NOT NULL,

    PRIMARY KEY("order", position),

    CONSTRAINT offers_start_amount_within_range CHECK (start_amount >= 0 AND start_amount < 2^256),
    CONSTRAINT offers_start_amount_no_decimals CHECK (SCALE(start_amount) = 0),
    CONSTRAINT offers_end_amount_within_range CHECK (end_amount >= 0 AND end_amount < 2^256),
    CONSTRAINT offers_end_amount_no_decimals CHECK (SCALE(end_amount) = 0)
);

CREATE TABLE considerations
(
    "order" TEXT REFERENCES orders(hash) NOT NULL,
    position INT NOT NULL,
    item_type INT NOT NULL,

    token citext REFERENCES addresses(address) NOT NULL,
    identifier_or_criteria TEXT NOT NULL,

    start_amount NUMERIC NOT NULL,
    end_amount NUMERIC NOT NULL,

    recipient citext REFERENCES addresses(address) NOT NULL,

    PRIMARY KEY("order", position),

    CONSTRAINT considerations_start_amount_within_range CHECK (start_amount >= 0 AND start_amount < 2^256),
    CONSTRAINT considerations_start_amount_no_decimals CHECK (SCALE(start_amount) = 0),
    CONSTRAINT considerations_end_amount_within_range CHECK (end_amount >= 0 AND end_amount < 2^256),
    CONSTRAINT considerations_end_amount_no_decimals CHECK (SCALE(end_amount) = 0)
);

CREATE INDEX IF NOT EXISTS orders_offerer_idx on orders(offerer);
CREATE INDEX IF NOT EXISTS orders_counter_idx on orders(counter);
