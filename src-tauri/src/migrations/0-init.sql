CREATE TABLE IF NOT EXISTS categories (
    id    TEXT PRIMARY KEY NOT NULL,
    label TEXT NOT NULL,
    color TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS products (
    id          TEXT PRIMARY KEY NOT NULL,
    name        TEXT NOT NULL,
    price       INTEGER NOT NULL,
    category_id TEXT NOT NULL,
    available   INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

CREATE TABLE IF NOT EXISTS orders (
    id              TEXT PRIMARY KEY NOT NULL,
    created_at      TEXT NOT NULL,
    total           INTEGER NOT NULL,
    payment_method  TEXT NOT NULL CHECK (payment_method IN ('cash', 'card'))
);

CREATE TABLE IF NOT EXISTS order_items (
    id            TEXT PRIMARY KEY NOT NULL,
    order_id      TEXT NOT NULL,
    product_id    TEXT NOT NULL,
    product_name  TEXT NOT NULL,
    unit_price    INTEGER NOT NULL,
    quantity      INTEGER NOT NULL,
    total         INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_order_items_order_id ON order_items (order_id);
