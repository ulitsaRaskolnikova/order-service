CREATE TABLE delivery (
    delivery_id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255),
    phone VARCHAR(20),
    zip VARCHAR(20),
    city VARCHAR(255),
    address VARCHAR(255),
    region VARCHAR(255),
    email VARCHAR(255)
);

CREATE TABLE payment (
    transaction VARCHAR(255) PRIMARY KEY,
    request_id VARCHAR(255),
    currency VARCHAR(10),
    provider VARCHAR(50),
    amount INTEGER,
    payment_dt BIGINT,
    bank VARCHAR(255),
    delivery_cost INTEGER,
    goods_total INTEGER,
    custom_fee INTEGER
);

CREATE TABLE order_info (
    order_uid VARCHAR(255) PRIMARY KEY,
    track_number VARCHAR(255),
    entry VARCHAR(50),
    delivery_id BIGINT REFERENCES delivery ON DELETE SET NULL,
    payment_transaction VARCHAR(255) REFERENCES payment ON DELETE SET NULL,
    locale VARCHAR(10),
    internal_signature VARCHAR(255),
    customer_id VARCHAR(255),
    delivery_service VARCHAR(50),
    shardkey VARCHAR(10),
    sm_id BIGINT,
    date_created VARCHAR(50),
    oof_shard VARCHAR(10)
);

CREATE TABLE item (
    chrt_id BIGINT PRIMARY KEY,
    track_number VARCHAR(255),
    price INTEGER,
    rid VARCHAR(255),
    name VARCHAR(255),
    sale INTEGER,
    size VARCHAR(10),
    total_price INTEGER,
    nm_id BIGINT,
    brand VARCHAR(255),
    status INTEGER
);

CREATE TABLE order_item (
    order_uid VARCHAR(255) REFERENCES order_info ON DELETE CASCADE,
    item_chrt_id BIGINT REFERENCES item ON DELETE CASCADE,
    PRIMARY KEY(order_uid, item_chrt_id)
);