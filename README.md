# Solana Block Analysis System

A lightweight Rust application that ingests Solana block data and extracts meaningful signals about transaction landing, scheduling, and fees, among other things, for research purposes.

I recorded and uploaded a 35-minute demo video that walks through my take-home project. In the video, I explain the overall schema system design, break down three sample queries, and run the system live while detailing the extraction logic behind each result and defending my findings.**

**I do naturally stutter at times, so I appreciate your patience while watching. Thanks very much for taking the time to review it.**

Here's the Official Demo video for my take-home project: 

[![Official Demo](https://github.com/user-attachments/assets/0c00590e-9415-417a-823a-01b1f30703f9)](https://youtu.be/1rToDHcUTK8)

**This image is clickable and links to the full demo.**

## Interesting Findings

During my project, I uncovered several particularly interesting queries spanning blocks 390,327,103 to 390,327,587, including the following:

    1. The Solana programs Pump.fun (6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P) and FLASHX8DrLbgeR8FcfNV1F5krxYcYMUdBkrP1EPBtxB9 are currently overcharging users in these blocks by over 15x. At scale, they are likely to become among the top programs imposing the highest fees on users.
    2. The landing service that handled the most transactions in the analyzed Solana block is Jito Tip1 (T1pyyaTNZsKv2WcRAB8oVnk93mLJw2XzjtVYqCsaHqt), along with other tip services such as JitoTip5.
    3. In the analyzed block, system transaction types on Solana generated the most revenue, despite representing a smaller share (13%) of the overall transaction volume.
    4. Inter-block transaction latency was around 0.02 seconds, and intra-block latency averaged between 0.33 and 0.44 seconds.

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
**Purpose**:Transaction types in Solana make it easier for users to understand and read what is happening on-chain, despite the Solana data being complex. Categorizing transactions in this way provides a high-level view of fee distribution across different transaction types, helping analysts and users grasp how the network operates.

**Key Fields**:
- `transaction_type`: Category (vote, system, spl_token, jupiter, raydium, orca, other)
- `transaction_count`, `total_fee`: Aggregated metrics


### Design Decisions I Made.

1. All tables use `PARTITION BY toYYYYMM(toDateTime(block_time))` for efficient time-based queries and data retention
2. I set the primary key on `slot` for fast slot-range queries
3. I pre-computed aggregations (totals, counts) for faster analytical queries
4. The program IDs and landing address are mapped manually to names for better interpretability
5. The application is centred around a CLI-based approach, focusing on precision in data delivery, and it uses the default ClickHouse UI to query and view the data.

## The Signals  I decided to Extract ( I did 3)

### 1. **The Solana Program that Overcharges Users**

While analyzing Solana fees, I realized something subtle but important: total fees alone don’t tell the real story. High fees can simply mean high usage. What actually matters is how expensive it is for a user to interact with a program each time.

This signal is built around that idea.

My signal looks at per-transaction cost, zooming in on p90 and p99 fee behavior and normalizing it against transaction volume. The result is a clearer view of programs where users consistently pay more than they should, even when demand doesn’t justify it. These aren’t one-off spikes or congestion artifacts.

For teams, this signal highlights where UX and fee mechanics are leaking value and silently taxing users. For analysts, it separates true demand-driven fees from inefficiencies, poor instruction design, or unnecessary compute usage.

### 2. **The Most Fee-Efficient Things to Do on Solana**

While breaking down Solana fee behavior, one pattern kept standing out: some activities turn usage into fees far more efficiently than others. This signal is designed to capture exactly that. Instead of treating fees or transaction volume in isolation, it normalizes fees by transaction activity, revealing which programs and transaction types extract the most value per unit of usage. In other words, it shows where demand actually translates into revenue, not just noise.

This matters because fees are behavioral.

Users, bots, and market makers respond to cost structures. When an activity becomes fee-efficient, it attracts automation. When it isn’t, volume dries up or routes change. Tracking this signal over time surfaces where bots are likely to cluster next, where fee pressure will compound, and which programs are quietly becoming structural contributors to validator revenue.

For builders, it highlights which interactions are economically tight versus wasteful. For analysts, it provides an early signal of shifting incentives before they show up in raw volume or headline fees.


### 3. **Which Swap Landings Cost Users the Most**

While analyzing swap fees on Solana, it became clear that not all swap landings are equal. Beyond the DEX or route itself, where a transaction lands can materially change what users end up paying. This analysis isolates the specific landing services and addresses responsible for higher-than-average swap fees. Some landings consistently add extra cost through tips, prioritization fees, or routing choices designed to secure faster inclusion costs that are often invisible to users.

Understanding this matters because these fees aren’t always driven by market conditions. In many cases, they’re structural overhead introduced by the landing path, not by the swap itself. By identifying which landing services cost users the most, this signal helps:

    - optimize routing to avoid unnecessary fee leakage

    - distinguish speed premiums from inefficiencies

    - surface landings that quietly tax users at scale

For builders, it’s a lever to improve swap UX and reduce hidden costs. For analysts, it exposes where fee pressure originates after the routing decision is made.


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

6. **The Rust implementation is not yet heavily optimized. This reflects the fact that I am still progressing through intermediate and advanced Rust development.**

7. **Inline comments are currently minimal. This is a known limitation and a deliberate short-term trade-off made to prioritize rapid research iteration**


## What I'd Build Next

If I had more time, this is what I would build:

- **MEV Opportunity Detection:** Look for blocks with unusual bundle sizes, fee patterns, or program combinations to find potential MEV extraction.  
- **Validator Behavior Classification:** Track how validators include transactions and bundles to see which ones follow the rules and which might be prioritizing private orderflow.
- **Frontend Interface:** Build a simple web dashboard to visualize, making the data easier to explore and analyze.
- **Build a model to continuously learn new fee and routing patterns over time, catching emerging strategies before they show up in aggregate metrics.**
