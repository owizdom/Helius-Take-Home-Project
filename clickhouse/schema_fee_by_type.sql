-- this compares fee patterns across different transaction types (vote, system, SPL token, DEX, etc.)

USE solana;

CREATE TABLE IF NOT EXISTS fee_by_transaction_type
(
    slot UInt64,
    block_time UInt64,
    transaction_type String,           -- the type of transaction (vote, system, spl_token, jupiter, raydium, orca, other)
    transaction_count UInt32,
    total_fee UInt64,
    created_at DateTime DEFAULT now()
)
ENGINE = MergeTree()
ORDER BY (slot, transaction_type)
PARTITION BY toYYYYMM(toDateTime(block_time));

