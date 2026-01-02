# Solana Block Analysis System

A lightweight Rust application that ingests Solana block data and extracts meaningful signals about transaction landing, scheduling, and fees, among other things, for research purposes.

# Interesting Findings

**Note: These findings were obtained through analysis using the queries defined in `example_queries.sql`.**

During my analysis, I identified several particularly interesting patterns across 1345 blocks (390,804,664 to 390,806,009), including the following:

## 1. Solana Program That Overcharges

<img width="1714" height="778" alt="Screenshot 2026-01-02 at 13 13 43" src="https://github.com/user-attachments/assets/bb914275-fcdf-4c94-a0bf-1f1f20f70c8f" />

The Solana programs Pump.fun (6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P) and FLASHX (FLASHX8DrLbgeR8FcfNV1F5krxYcYMUdBkrP1EPBtxB9) consistently cause users to pay more in fees compared to comparable interactions in the same blocks, amongst others like King...cb7T and term...ZzN3.

In this context, overcharging means that transactions involving these programs exhibit abnormally high fees on a per-transaction basis, even after normalizing for congestion and transaction volume. At scale, this behavior would place them among the highest fee-consuming programs on Solana.

Furthermore, I delved deeper to find how much these programs are overpaying which could further help in investigating how much Jito is middlemanning. The query for this analysis is in the `example_queries.sql`

    a. Pump.fun (program ID 6EF8r...F6P): appeared in 1177 blocks, 3,715 txns, avg fee 188,031 lamports. Calculated to be 313.33% overcharge above normal, costing users an extra ~529,531,554 lamports in total.
    b. FlashX (FLASHX8D...txB9): appeared in 1091 blocks, 2,886 txns, avg fee 541,454 lamports. Overcharged by ~1263%+, totaling ~144,012,706 lamports extra paid.

## 2. Jito Landing Service Dominance in Swap Transactions

<img width="1709" height="751" alt="Screenshot 2026-01-02 at 13 30 31" src="https://github.com/user-attachments/assets/817944b5-4348-4be4-a9f5-a2839970d900" />

In the swap-focused scope of this analysis, the Jito landing service (ADuUkR4vqLUMWXxW9gh6D6L8pMSawimctcNZ5pGwDcEt) landed the highest number of transactions. Other tip-related accounts, such as JitoTip5 or JitoTip3, also appear prominently, but these are merely distinct tip wallets used to spread load and reduce contention, they all ultimately route through the same Jito landing service rather than represent separate providers.

Jito collectively landed over 54% of the blocks spread across the top 10 services, underscoring its dominant role in landing blocks.

This observation aligns with the kind of fee routing dynamics discussed in Benedict’s PFOF on Solana article, where swap routing, priority tips, and landing service incentives can materially impact how user fees are allocated and monetized.

Note: I was inspired to write this query by reading Sir Benedict work on PFOF

## 3. Detecting Jito Bundles

<img width="1714" height="726" alt="Screenshot 2026-01-02 at 13 22 55" src="https://github.com/user-attachments/assets/5cb3e1a9-547b-46f1-9fb0-ab50ce163f3b" />

Around 92% of Jito transactions appear in bundles, with most bundles containing 3–4 sequential transactions. This demonstrates a deliberate strategy of grouping transactions, which can influence transaction ordering, fee distribution, and block dynamics. The high consistency of these bundles suggests a structured approach rather than random sequencing, pointing toward optimization for both execution efficiency and potential MEV capture.

## How to Run

### Prerequisites
- Ensure Docker Desktop is open before running the `docker-compose up -d` command
- Rust (latest stable version)
- Solana RPC access (the code uses a GetBlock RPC endpoint and Alchemy Solana RPC endpoint as a Fallback mechanism)

### Configuration (in your `docker-compose.yml` file)

- **ClickHouse password**: Set via `CLICKHOUSE_PASSWORD` environment variable (default: `solana123`)

### Setup

1. **Start ClickHouse database:**
   ```bash
   docker-compose up -d
   ```

2. **Run the application:**
   ```bash
   cargo run --bin main
   ```

3. **Interactive Setup Menu**

    When you run the application, you'll see:
    
    - **ClickHouse Configuration Display** showing:
      - Username: `default`
      - Password: (your configured password) -> defaults to `solana123` if not set
      - Database: `solana`
      - Dashboard URL: `http://localhost:8123/play`
    
    After entering your API keys, the application will start streaming and analyzing blocks. You can access the ClickHouse dashboard using the provided URL above to view your data and run queries.

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --test bundling_test
cargo test --test program_name_test
cargo test --test fee_ordering_test
cargo test --test primary_program_test
```

## Schema Design

The system uses **ClickHouse** for data storage, which is used for further analysis. These schemas form the backbone of my signal queries, giving every analysis a solid, reliable foundation and making it possible to extract insights cleanly and efficiently:

### 1. `bundling_analysis`
**Why I Chose This**: After studying more on Solana blocks, this was the first part that made me think. Being able to figure out in what manner a block is landed—whether through Jito bundles, direct leader inclusion, or other MEV infrastructure—is critical for understanding validator behaviour and their role in the Solana network. This analysis is essential for MEV research on Solana, as it enables the detection of spam patterns, opportunity clusters, and the distribution of blockspace across different execution paths.

**Key Fields**:
- `largest_bundle_size`: Size of largest detected bundle (up to 5 transactions with same tip recipient). Bundles are detected by identifying sequential transactions that tip the same landing service account.
- `validator_key`: Validator who built the block (enables validator-level behaviour analysis)
- `landing_service`: Most common landing service identified in the block (e.g., Jito)
- `landing_service_count`: Number of transactions using the identified landing service (quantifies how much blockspace is routed through specific infrastructure)

### 2. `fee_landscape` 
**Why I Chose this**: I believe that despite Solana being known for low transaction costs due to its architecture, fees are often overlooked, so I decided to highlight their importance in my project. In an article I wrote earlier this year, I explained why Solana's fee market and compute–unit dynamics are economically important, and that reasoning is exactly why I chose this schema. Tracking compute-budget usage (Compute Units per transaction and per block) provides a clear signal of network growth, showing how rising computational demand reshapes validator load and fee dynamics over time.

Link to article: **Economic Implications of SIMD-253** — Parallel Research (wisdom), March 18, 2025  
[Economic Implications of SIMD-253 — Parallel Research](https://parallelresearch.substack.com/p/economic-implications-of-simd-253)

**Key Fields**:
- `fee_avg`: Average fee per transaction in block
- `compute_budget_percent`: Percentage of transactions using compute budget instructions


### 3. `program_fee_analysis`
**Why I Chose This**: Solana programs are closely comparable to Ethereum smart contracts and the Bitcoin UTXO account model, and their economics are a fundamental part of how the network functions. They allow analysts to understand what is happening on-chain, which programs are being used, the program types, total fees, and overall activity. In the same way that Ethereum contract economics and Bitcoin transaction economics are critical for understanding value flow on those networks, understanding program economics on Solana helps reveal revenue flows and how effectively programs use fees.

**Key Fields**:
- `program_name`: Human-readable program name (e.g., "Jupiter", "Raydium CLMM")
- `program_type`: Category (DEX, SPL Token, System, etc.)
- `transaction_count`: Number of transactions for this program
- `total_fee`, `min_fee`, `max_fee`: Fee statistics

### 4. `fee_by_transaction_type`
**Why I Chose This**:Transaction types in Solana make it easier for users to understand and read what is happening on-chain, despite the Solana data being complex. Categorizing transactions in this way provides a high-level view of fee distribution across different transaction types, helping analysts and users grasp how the network operates.

**Key Fields**:
- `transaction_type`: Category (vote, system, spl_token, jupiter, raydium, orca, other)
- `transaction_count`, `total_fee`: Aggregated metrics

### Design Decisions I Made.

1. All tables use `PARTITION BY toYYYYMM(toDateTime(block_time))` for efficient time-based queries and data retention
2. I set the primary key on `slot` for fast slot-range queries
3. I pre-computed aggregations (totals, counts) for faster analytical queries
4. The program IDs and landing address are mapped manually to names for better interpretability
5. The application is centred around a CLI-based approach, focusing on precision in data delivery, and it uses the default ClickHouse UI to query and view the data.

## The Signals I Decided to Extract

### 1. **Programs That Overcharge Users**

While analyzing Solana fees, I realized something subtle but important: total fees alone don't tell the real story. High fees can simply mean high usage. What actually matters is how expensive it is for a user to interact with a program each time.

This signal identifies programs where users consistently pay more than they should, even after normalizing for congestion and transaction volume. During my analysis, I found that Pump.fun and FLASHX consistently cause users to pay significantly more in fees compared to comparable interactions in the same blocks.

This analysis helps investigate how much services like Jito are middlemanning, as overpayment patterns can reveal the cost of routing through specific landing services.

### 2. **How Transactions Are Landing**

Understanding how transactions land—whether through Jito bundles, direct to leader, or through specific RPCs—is critical for understanding validator behavior and their role in the Solana network. This signal tracks landing services and quantifies how much blockspace is routed through specific infrastructure.

This analysis is essential for MEV research on Solana, as it enables detection of spam patterns, opportunity clusters, and the distribution of blockspace across different execution paths.


**Note:** Earlier this year, I wrote an article titled “Economic Implications of SIMD-253” exploring how a proposed improvement to Solana’s fee market could reshape network economics. In it, I break down SIMD-253, a governance proposal designed to introduce a fee controller and a target Compute Unit (CU) utilization limit to the network’s existing first-price auction fee model, a mechanism that currently forces users to guess how much to bid for inclusion, often resulting in overpayment and poor UX.

Looking back, this article was foundational for how I think about fee behavior and design signals on Solana, including the new per-transaction and fee-efficiency signals I’ve been building. It helped me understand the deeper economics that drive fee pressure, block inclusion, and user cost, and ultimately informed how I analyze where inefficiencies and overcharging occur in real usage.

**Link to article:**  
**Economic Implications of SIMD-253 — Parallel Research (wisdom), March 18, 2025**  
[Economic Implications of SIMD-253 — Parallel Research](https://parallelresearch.substack.com/p/economic-implications-of-simd-253)

## Trade-offs Made

1. **I do not store individual transaction positions within blocks in the database. This means that I cannot currently perform exact position-based ordering analysis or fine-grained reordering detection. This was a deliberate trade-off made to prioritize block-level aggregation performance and keep analytical queries fast. Position-level data can be added later if deeper ordering analysis becomes necessary, but it is not required for the current scope.**

2. **Known program IDs and landing address are mapped to human-readable names using a hardcoded mapping in the utils.rs and landing_service.rs files, respectively. Unknown programs and landing addresses, therefore, appear only raw and require periodic maintenance to keep the mapping current. I chose this approach because it provides immediate analytical value for the most economically significant programs and addresses them without introducing dependencies on external registries. The mapping can later be extended or replaced with on-chain program metadata resolution.**

4. **When the database is empty, ingestion begins from recent slots rather than performing a full historical backfill. This means the system cannot immediately analyze historical data, but it allows the platform to focus on real-time and forward-looking network behavior. Historical backfilling can be introduced later as a separate pipeline without interfering with live analysis.**

5. **The project does not include a dedicated frontend application. Instead, I operate the system through a terminal-based interface and rely on the ClickHouse UI for querying and visualization. This design choice prioritizes rapid iteration, while still allowing analysis without building and maintaining a full web frontend.**

6. **Inline comments are currently minimal. This is a known limitation and a deliberate short-term trade-off made to prioritize rapid research iteration**


## What I'd Build Next

If I had more time, this is what I would build:

- **MEV Opportunity Detection:** Look for blocks with unusual bundle sizes, fee patterns, or program combinations to find potential MEV extraction.  
- **Validator Behavior Classification:** Track how validators include transactions and bundles to see which ones follow the rules and which might be prioritizing private orderflow.
- **Frontend Interface:** Build a simple web dashboard to visualize, making the data easier to explore and analyze.
- **Build a model to continuously learn new fee and routing patterns over time, catching emerging strategies before they show up in aggregate metrics.**


This is my submission for the Product Research Engineer role at Helius. I look forward to hearing from you. Thanks

**Through my work, I shape understanding the way an artist shapes form; deliberately, patiently, and with intent.**
