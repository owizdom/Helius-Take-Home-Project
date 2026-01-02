# Solana Block Analysis System

A lightweight Rust application that ingests Solana block data and extracts meaningful signals about transaction landing, scheduling, and fees, among other things, for research purposes.

## Interesting Findings

During my analysis, I identified several particularly interesting patterns across 358 blocks (390,697,210 to 390,697,568), including the following:

1. The Solana programs Pump.fun (6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P) and FLASHX (FLASHX8DrLbgeR8FcfNV1F5krxYcYMUdBkrP1EPBtxB9) consistently cause users to pay more in fees compared to comparable interactions in the same blocks.

    In this context, overcharging means that transactions involving these programs exhibit abnormally high fees on a per-transaction basis, even after normalizing for congestion and transaction volume. At scale, this behavior would place them among the highest fee-consuming programs on Solana.

Furthermore, I delved deeper to find how much these programs are overpaying which could further help in investigating how much Jito is middlemanning.

    a. Pump.fun appeared in 307 blocks and processed 1,261 transactions, with an average fee of 183,030 lamports per transaction. Based on my analysis, the program overcharged users by 145.45%, resulting in a total excess cost of 136,760,625 lamports paid by users.
    b. Flash appeared in 317 blocks and processed 1,207 transactions with an average fee of 710,410 lamports per transaction. Based on my analysis, the program overcharged users by 1000+%, resulting in a total excess cost of 779,661,341 lamports paid by users.

2.  In the swap-focused scope of this analysis, the Jito landing service (ADuUkR4vqLUMWXxW9gh6D6L8pMSawimctcNZ5pGwDcEt) landed the highest number of transactions. Other tip-related accounts, such as JitoTip5 or JitoTip3, also appear prominently, but these are merely distinct tip wallets used to spread load and reduce contention, they all ultimately route through the same Jito landing service rather than represent separate providers.

    Furthermore, the leading landing service(87wyLh2iDzszjYTPi5tnDhRx5GGrxzWsRAUbBboVm743) accounted for 22% of all blocks built during the analyzed window, while Jito collectively landed over 40% of the blocks spread across the top 10 services, underscoring its dominant role in landing blocks.

    This observation aligns with the kind of fee routing dynamics discussed in Benedict’s PFOF on Solana article, where swap routing, priority tips, and landing service incentives can materially impact how user fees are allocated and monetized.
     

3. One interesting validator-level signal that emerged is how unevenly transaction load and backlog age are distributed across block producers. Despite producing a similar number of blocks, some validators consistently processed far more transactions than others. 

    For example, Jupiter `JupmVLmA8RoyTUbTMMuTtoPWHEiNQobxgTeGTrPNkzT` produced 11 blocks yet handled nearly 12,000 transactions, including 8 older transactions, while other validators producing only four blocks processed closer to 4,000–5,000 transactions, including an average of 5 or more transactions in blocks being built.
    
    This raises a natural question: why are some validators repeatedly including older transactions?
The analysis indicates that this behavior cannot be explained by higher fees, suggesting that factors other than fee maximization such as landing service behaviour are driving transaction inclusion.

    At the same time, all validators included transactions with a maximum age of up to 151 slots, showing that under congestion, significantly older transactions can still make it into blocks. This underscores how validator-level differences in block production and transaction intake can meaningfully influence latency and fairness on the network. 

    This highlights meaningful differences in how validators absorb transaction volume and backlog, which can materially affect latency, fairness, and user experience depending on which leader ultimately produces the block.


## How to Run

### Prerequisites
- Docker and Docker Compose
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
- `unique_blockhashes`: Number of unique blockhashes in the block
- `largest_blockhash_group`: Size of the largest blockhash cluster (indicates bundle size; larger groups suggest coordinated transaction groups)
- `largest_blockhash`: The blockhash that appears most frequently in the block (identifies the dominant bundle or transaction group)
- `validator_key`: Validator who built the block (enables validator-level behaviour analysis)
- `landing_service`: Most common landing service identified in the block (The current implementation still shows empty due to it being unknown)
- `landing_service_count`: Number of transactions using the identified landing service (quantifies how much blockspace is routed through specific infrastructure)

### 2. `fee_landscape` 
**Why I Chose this**: I believe that despite Solana being known for low transaction costs due to its architecture, fees are often overlooked, so I decided to highlight their importance in my project. In an article I wrote earlier this year, I explained why Solana’s fee market and compute–unit dynamics are economically important, and that reasoning is exactly why I chose this schema. By analysing fee-ordering correlation, we can observe whether validators are ordering transactions by fees, which helps reveal potential MEV opportunities and clustered spam behaviour, while tracking compute-budget usage (Compute Units per transaction and per block) provides a clear signal of network growth, showing how rising computational demand reshapes validator load and fee dynamics over time.

Link to article: **Economic Implications of SIMD-253** — Parallel Research (wisdom), March 18, 2025  
[Economic Implications of SIMD-253 — Parallel Research](https://parallelresearch.substack.com/p/economic-implications-of-simd-253)

**Key Fields**:
- `fee_avg`: Average fee per transaction in block
- `compute_budget_percent`: Percentage of transactions using compute budget instructions
- `fee_ordering_correlation`: Correlation between transaction position and fee (1.0 = perfect fee-based ordering)


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

### 5. `transaction_age_analysis`
**Why I Chose This**: Understanding how validators handle transaction scheduling is critical for detecting potential manipulation or unfair behavior. This schema tracks transaction age by looking up when blockhashes were first created, allowing us to identify validators that include very old transactions in their blocks. This is important because validators playing games with transaction scheduling might prioritize their own old transactions or manipulate inclusion order, which can affect network fairness and user experience.

**Key Fields**:
- `validator_key`: Validator who built the block
- `max_transaction_age_slots`: Oldest transaction age in slots (identifies how stale transactions can get)
- `avg_transaction_age_slots`: Average transaction age in slots (shows typical backlog age)
- `old_transaction_count`: Number of transactions older than 150 slots (identifies validators including very old transactions)
- `total_transactions`: Total number of transactions in the block


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

### 3. **Validator Transaction Scheduling Behavior**

One interesting validator-level signal that emerged is how unevenly transaction load and backlog age are distributed across block producers. This signal tracks how validators handle transaction scheduling, including whether they're playing games with transaction ordering or including very old transactions.

This highlights meaningful differences in how validators absorb transaction volume and backlog, which can materially affect latency, fairness, and user experience depending on which leader ultimately produces the block.


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
