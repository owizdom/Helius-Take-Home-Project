-- this analyzes the fee landscape of a block
USE solana;

CREATE TABLE IF NOT EXISTS fee_landscape
(
    slot UInt64,
    block_time UInt64,
    fee_avg Float64,
    compute_budget_percent Float32,
    fee_ordering_correlation Float32,
    created_at DateTime DEFAULT now()
)
ENGINE = MergeTree()
ORDER BY (slot)
PARTITION BY toYYYYMM(toDateTime(block_time));


