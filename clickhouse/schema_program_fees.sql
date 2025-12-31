-- this tracks fees by program type to identify what's paying the most

USE solana;

CREATE TABLE IF NOT EXISTS program_fee_analysis
(
    slot UInt64,
    block_time UInt64,
    program_type String,
    program_name String,
    transaction_count UInt32,
    total_fee UInt64,
    min_fee UInt64,
    max_fee UInt64,
    created_at DateTime DEFAULT now()
)
ENGINE = MergeTree()
ORDER BY (slot, program_type)
PARTITION BY toYYYYMM(toDateTime(block_time));

