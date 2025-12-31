-- this detects if transactions land via Jito bundles, direct to leader, RPCs, or MEV infrastructure

USE solana;

CREATE TABLE IF NOT EXISTS bundling_analysis
(
    slot UInt64,
    blockhash String,
    block_time UInt64,

    unique_blockhashes UInt32,    -- the number of unique blockhashes in the block
    largest_blockhash_group UInt32, -- the size of the largest blockhash cluster
    largest_blockhash String,     -- the blockhash with most transactions
    
    validator_key String,         -- the validator key who built the block
    landing_service String,       -- most common landing service in block (empty if unknown)
    landing_service_count UInt32, -- number of transactions using identified landing service
    
    created_at DateTime DEFAULT now()
)
ENGINE = MergeTree()
ORDER BY (slot)
PARTITION BY toYYYYMM(toDateTime(block_time));


