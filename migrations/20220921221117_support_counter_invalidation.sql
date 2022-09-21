-- Add an index for quick lookup by offerer
CREATE INDEX IF NOT EXISTS orders_offerer_idx on orders(offerer);
CREATE INDEX IF NOT EXISTS orders_counter_idx on orders(counter);