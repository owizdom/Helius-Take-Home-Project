-- this detects if transactions land via Jito bundles, direct to leader, RPCs, or MEV infrastructure

USE solana;

CREATE TABLE IF NOT EXISTS bundling_analysis
(
    slot UInt64,
    blockhash String,
    block_time UInt64,

    largest_bundle_size UInt32,   -- Size of largest detected bundle (up to 5 transactions with same tip recipient)
    
    validator_key String,         -- the validator key who built the block
    landing_service String,       -- most common landing service in block (empty if unknown)
    landing_service_count UInt32, -- number of transactions using identified landing service
    
    created_at DateTime DEFAULT now()
)
ENGINE = MergeTree()
ORDER BY (slot)
PARTITION BY toYYYYMM(toDateTime(block_time));


