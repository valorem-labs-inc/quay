-- Create addresses table
CREATE TABLE addresses
(
    address TEXT PRIMARY KEY
);

CREATE TABLE users
(
    "user"    TEXT REFERENCES addresses (address) PRIMARY KEY,
    userName  TEXT,
    email     TEXT,
    picture   TEXT,
    bio       TEXT,
    twitter   TEXT,
    instagram TEXT,
    webLink   TEXT,
    banner    TEXT
);

CREATE TABLE networks
(
    network       INTEGER PRIMARY KEY,
    indexed_block BIGINT NOT NULL
);

CREATE TABLE orders
(
    hash TEXT PRIMARY KEY,
    offerer TEXT REFERENCES addresses(address) NOT NULL,
    zone TEXT REFERENCES addresses(address) NOT NULL,
    zone_hash TEXT NOT NULL,
    start_time BIGINT NOT NULL,
    end_time BIGINT NOT NULL,
    order_type INT NOT NULL,
    total_original_consideration_items INT NOT NULL,
    salt TEXT NOT NULL,
    conduit_key TEXT NOT NULL,
    signature TEXT NOT NULL
);

CREATE TABLE offers
(
    position INT NOT NULL,
    "order" TEXT REFERENCES orders(hash) NOT NULL,
    PRIMARY KEY(position, "order"),
    item_type INT NOT NULL,
    token TEXT NOT NULL,
    identifier_or_criteria TEXT NOT NULL,
    start_amount TEXT NOT NULL,
    end_amount TEXT NOT NULL
);

CREATE TABLE considerations
(
    position INT NOT NULL,
    "order" TEXT REFERENCES orders(hash) NOT NULL,
    PRIMARY KEY(position, "order"),
    item_type INT NOT NULL,
    token TEXT NOT NULL,
    identifier_or_criteria TEXT NOT NULL,
    start_amount TEXT NOT NULL,
    end_amount TEXT NOT NULL,
    recipient TEXT NOT NULL
);