-- 1. The Solana Program that Overcharges Users
SELECT 
    program_name,
    program_type,
    
    -- Transaction volume
    SUM(transaction_count) as total_transactions,
    COUNT(DISTINCT slot) as blocks_appeared_in,
    
    -- Fee statistics (high fees = potential overcharging)
    ROUND(quantile(0.5)(min_fee), 0) as p50_fee_lamports,
    ROUND(quantile(0.9)(max_fee), 0) as p90_fee_lamports,
    ROUND(quantile(0.99)(max_fee), 0) as p99_fee_lamports,
    
    -- Average fees
    ROUND(AVG(total_fee / transaction_count), 0) as avg_fee_per_tx_lamports,
    ROUND(AVG(max_fee), 0) as avg_max_fee_lamports,
    
    -- Total fees
    SUM(total_fee) as total_fees_lamports,
    
    -- Fee efficiency (lower = more overcharging relative to volume)
    ROUND(
        (AVG(total_fee / transaction_count)) / NULLIF(AVG(transaction_count), 0), 
        2
    ) as fee_efficiency_score,
    
    -- Time range
    MIN(toDateTime(block_time)) as first_seen,
    MAX(toDateTime(block_time)) as last_seen
    
FROM solana.program_fee_analysis
GROUP BY program_name, program_type
HAVING total_transactions > 1000
    AND quantile(0.9)(max_fee) > 100000  -- High p90 fees (potential overcharging)
ORDER BY p99_fee_lamports DESC
LIMIT 30;

-- ============================================================================

-- 2. The Most Fee-Efficient Things to Do on Solana
SELECT 
    transaction_type,
    -- X-axis: Market share
    ROUND(
        SUM(transaction_count) * 100.0 / (SELECT SUM(transaction_count) FROM solana.fee_by_transaction_type), 
        2
    ) as market_share_percent,
    -- Y-axis: Average fee
    ROUND(
        SUM(total_fee) / NULLIF(SUM(transaction_count), 0), 
        2
    ) as avg_fee_per_transaction,
    -- Size: Total revenue (for bubble chart)
    SUM(total_fee) as total_revenue,
    -- Color: Transaction count (for heat map)
    SUM(transaction_count) as total_transactions
FROM solana.fee_by_transaction_type
GROUP BY transaction_type
ORDER BY market_share_percent DESC;

-- ============================================================================

-- 3. Which Swap Landings Cost Users the Most
SELECT 
    CASE 
        WHEN ba.landing_service != '' THEN ba.landing_service
        ELSE 'Unknown'
    END as landing_method,
    ftt.transaction_type,
    
    -- Swap-specific analysis
    SUM(ftt.transaction_count) as swap_transactions,
    SUM(ftt.total_fee) as swap_fees_lamports,
    ROUND(SUM(ftt.total_fee) / SUM(ftt.transaction_count), 0) as avg_fee_per_swap_lamports,
    
    -- Fee percentiles for swaps
    ROUND(quantile(0.5)(ftt.total_fee / ftt.transaction_count), 0) as p50_swap_fee_lamports,
    ROUND(quantile(0.9)(ftt.total_fee / ftt.transaction_count), 0) as p90_swap_fee_lamports,
    ROUND(quantile(0.99)(ftt.total_fee / ftt.transaction_count), 0) as p99_swap_fee_lamports
    
FROM solana.bundling_analysis ba
JOIN solana.fee_by_transaction_type ftt ON ba.slot = ftt.slot
WHERE ftt.transaction_type IN ('jupiter', 'raydium', 'orca')  -- DEX swaps
GROUP BY landing_method, ftt.transaction_type
ORDER BY swap_fees_lamports DESC;

-- ============================================================================
-- 4. The Latency of Transaction Execution

SELECT 
    ftt.transaction_type,
    
    -- Sample size
    COUNT(DISTINCT ba.slot) as blocks_analyzed,
    
    -- Latency statistics
    ROUND(AVG(exec_block.block_time - hash_block.block_time), 2) as avg_latency_seconds,
    ROUND(quantile(0.5)(exec_block.block_time - hash_block.block_time), 2) as median_latency_seconds,
    ROUND(quantile(0.95)(exec_block.block_time - hash_block.block_time), 2) as p95_latency_seconds,
    ROUND(MIN(exec_block.block_time - hash_block.block_time), 2) as min_latency_seconds,
    ROUND(MAX(exec_block.block_time - hash_block.block_time), 2) as max_latency_seconds,
    
    -- Transaction volume for context
    SUM(ftt.transaction_count) as total_transactions,
    SUM(ftt.total_fee) as total_fees
    
FROM solana.bundling_analysis ba
JOIN solana.blocks exec_block ON ba.slot = exec_block.slot
JOIN solana.blocks hash_block ON ba.largest_blockhash = hash_block.blockhash
JOIN solana.fee_by_transaction_type ftt ON ba.slot = ftt.slot
WHERE hash_block.block_time > 0 AND exec_block.block_time > 0
GROUP BY ftt.transaction_type
ORDER BY avg_latency_seconds DESC;