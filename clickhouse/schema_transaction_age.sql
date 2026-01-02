-- this tracks transaction age to identify validators including old transactions

USE solana;

CREATE TABLE IF NOT EXISTS transaction_age_analysis
(
    slot UInt64,
    block_time UInt64,
    validator_key String,
    max_transaction_age_slots UInt64,  -- Oldest transaction age in slots
    avg_transaction_age_slots Float64,
    old_transaction_count UInt32,  -- Transactions >150 slots old
    total_transactions UInt32,
    created_at DateTime DEFAULT now()
)
ENGINE = MergeTree()
ORDER BY (slot, validator_key)
PARTITION BY toYYYYMM(toDateTime(block_time));

