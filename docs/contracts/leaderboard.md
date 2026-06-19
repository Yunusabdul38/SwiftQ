# Leaderboard Contract Specification

## Overview
The `leaderboard` contract stores persistent, global player rankings
across all quiz rounds. It is updated by the admin/oracle after each round closes.

## Storage Layout

| Key | Type | Storage Kind | Description |
|---|---|---|---|
| `Admin` | `Address` | Instance | Authorized caller (quiz-pool contract or oracle) |
| `Stats(player)` | `PlayerStats` | Persistent | Lifetime stats per player address |

## Data Types

### `PlayerStats`
```
total_wins: u32           — Total rounds won
total_rounds: u32         — Total rounds participated in
total_earnings: i128      — Cumulative prize earnings (in token stroops)
highest_score: u32        — Best score in a single round
last_played_at: u64       — Ledger timestamp of most recent round
```

## Public Functions

| Function | Caller | Description |
|---|---|---|
| `initialize(admin)` | Admin | One-time setup |
| `record_result(caller, player, score, earnings, won)` | Admin/Oracle | Update player stats after a round |
| `get_stats(player) -> PlayerStats?` | Anyone | Fetch a player's lifetime stats |
