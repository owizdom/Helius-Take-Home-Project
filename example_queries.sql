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

-- 2. Which Swap Landings Cost Users the Most
SELECT 
    CASE 
        WHEN ba.landing_service != '' THEN ba.landing_service
        ELSE 'Unknown'
    END as landing_service,
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
GROUP BY landing_service, ftt.transaction_type
ORDER BY swap_fees_lamports DESC;

-- ============================================================================
-- 3. Pump.fun Overpayment Analysis (Jito Middlemanning Signal)

WITH pump_fun_stats AS (
    SELECT 
        slot,
        block_time,
        total_fee,
        transaction_count,
        total_fee / NULLIF(transaction_count, 0) as avg_fee_per_tx
    FROM solana.program_fee_analysis
    WHERE program_name = '6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P'
),
overall_avg AS (
    SELECT 
        AVG(total_fee / NULLIF(transaction_count, 0)) as overall_avg_fee_per_tx
    FROM solana.program_fee_analysis
    WHERE program_name != '6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P'
)
SELECT 
    'Pump.fun' as program,
    COUNT(DISTINCT slot) as blocks_analyzed,
    SUM(transaction_count) as total_transactions,
    SUM(total_fee) as total_fees_lamports,
    ROUND(AVG(avg_fee_per_tx), 0) as pump_avg_fee_per_tx,
    ROUND((SELECT overall_avg_fee_per_tx FROM overall_avg), 0) as overall_avg_fee_per_tx,
    ROUND(AVG(avg_fee_per_tx) - (SELECT overall_avg_fee_per_tx FROM overall_avg), 0) as overpayment_per_tx,
    ROUND((AVG(avg_fee_per_tx) - (SELECT overall_avg_fee_per_tx FROM overall_avg)) * SUM(transaction_count), 0) as total_overpayment_lamports,
    ROUND((AVG(avg_fee_per_tx) / NULLIF((SELECT overall_avg_fee_per_tx FROM overall_avg), 0) - 1) * 100, 2) as overpayment_percentage
FROM pump_fun_stats;

-- ============================================================================
-- 4. Jito Bundle Statistics

SELECT 
    -- Bundle detection summary
    COUNT(DISTINCT CASE WHEN landing_service = 'Jito' AND largest_bundle_size >= 2 THEN slot END) as blocks_with_jito_bundles,
    COUNT(DISTINCT CASE WHEN landing_service = 'Jito' THEN slot END) as blocks_with_jito_transactions,
    
    -- Bundle size statistics
    ROUND(AVG(CASE WHEN landing_service = 'Jito' AND largest_bundle_size >= 2 THEN largest_bundle_size END), 2) as avg_bundle_size,
    ROUND(quantile(0.5)(CASE WHEN landing_service = 'Jito' AND largest_bundle_size >= 2 THEN largest_bundle_size END), 0) as p50_bundle_size,
    ROUND(quantile(0.9)(CASE WHEN landing_service = 'Jito' AND largest_bundle_size >= 2 THEN largest_bundle_size END), 0) as p90_bundle_size,
    MAX(CASE WHEN landing_service = 'Jito' THEN largest_bundle_size END) as max_bundle_size,
    
    -- Transaction counts
    SUM(CASE WHEN landing_service = 'Jito' AND largest_bundle_size >= 2 THEN landing_service_count ELSE 0 END) as total_jito_bundle_transactions,
    SUM(CASE WHEN landing_service = 'Jito' THEN landing_service_count ELSE 0 END) as total_jito_transactions,
    
    -- Bundle count estimation (rough estimate: divide transactions by average bundle size)
    ROUND(SUM(CASE WHEN landing_service = 'Jito' AND largest_bundle_size >= 2 THEN landing_service_count ELSE 0 END) / 
          NULLIF(AVG(CASE WHEN landing_service = 'Jito' AND largest_bundle_size >= 2 THEN largest_bundle_size END), 0), 0) as estimated_bundle_count,
    
    -- Percentage of Jito transactions that are in bundles
    ROUND(SUM(CASE WHEN landing_service = 'Jito' AND largest_bundle_size >= 2 THEN landing_service_count ELSE 0 END) * 100.0 / 
          NULLIF(SUM(CASE WHEN landing_service = 'Jito' THEN landing_service_count ELSE 0 END), 0), 2) as pct_jito_txs_in_bundles

FROM solana.bundling_analysis;

-- ============================================================================