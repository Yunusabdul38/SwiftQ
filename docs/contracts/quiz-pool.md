# Quiz Pool Contract Specification

## Overview
The `quiz-pool` contract manages the full lifecycle of a quiz round:
entry fee collection, answer commitment, and prize distribution.

## Storage Layout

| Key | Type | Storage Kind | Description |
|---|---|---|---|
| `Admin` | `Address` | Instance | Contract administrator |
| `Config` | `Config` | Instance | Global platform configuration |
| `RoundId` | `u64` | Instance | Current active round counter |
| `Pool(round_id)` | `i128` | Temporary | Accumulated prize pool for a round |
| `Session(player, round_id)` | `PlayerSession` | Temporary | Per-player round state |
| `Commitment(player, round_id, q_idx)` | `Commitment` | Temporary | Hashed answer for commit-reveal |

## Data Types

### `Config`
```
token: Address            — USDC token contract
entry_fee: i128           — Fee per round (e.g. 500_000 = $0.05 USDC)
platform_fee_bps: u32     — Platform cut in basis points (e.g. 1000 = 10%)
treasury: Address         — Receives platform fee
max_secs_per_question: u64 — Anti-bot time window per question
questions_per_round: u32  — Number of questions per round
```

### `PlayerSession`
```
started_at: u64           — Ledger timestamp of round join
questions_answered: u32   — How many questions submitted
score: u32                — Correct answer count
completed: bool           — Whether round is closed for this player
```

### `Commitment`
```
answer_hash: BytesN<32>   — SHA-256(answer_bytes ++ nonce_bytes)
committed_at: u64         — Ledger timestamp of commitment
```

## Public Functions

| Function | Caller | Description |
|---|---|---|
| `initialize(admin, config)` | Admin | One-time setup |
| `update_config(config)` | Admin | Update platform config |
| `join_round(player) -> round_id` | Player | Pay entry fee, start session |
| `commit_answer(player, round_id, q_idx, hash)` | Player | Submit hashed answer |
| `reveal_answer(player, round_id, q_idx, ...)` | Oracle + Player | Verify & score answer |
| `distribute_prizes(round_id, winners)` | Admin | Pay out prize pool |
| `get_config() -> Config` | Anyone | Read config |
| `get_round_id() -> u64` | Anyone | Read active round |
| `get_pool(round_id) -> i128` | Anyone | Read prize pool balance |
| `get_session(player, round_id) -> PlayerSession?` | Anyone | Read player session |

## Prize Distribution Split (Default)
- 1st place: 60% of distributable pool
- 2nd place: 30% of distributable pool
- 3rd place: 10% of distributable pool
- Platform: `platform_fee_bps` of total pool (taken first)
