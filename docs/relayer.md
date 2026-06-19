# Fee-Bump Relayer Specification

## Overview
The relayer is a lightweight Rust/Axum HTTP server.
Its sole job is to wrap player-signed Stellar transactions in a
fee-bump envelope using the platform's sponsor account, then submit
them to the Stellar network — making gas completely invisible to players.

## Endpoints

### `GET /health`
Returns server status and version.

**Response**
```json
{ "status": "ok", "version": "0.1.0" }
```

---

### `POST /relay`
Accepts a player-signed transaction XDR, wraps it in a fee-bump, and submits it.

**Request Body**
```json
{
  "signed_xdr": "<base64-encoded XDR of the inner signed transaction>",
  "fee_stroops": 1000000
}
```

**Success Response (200)**
```json
{ "hash": "<transaction hash>", "status": "SUCCESS" }
```

**Error Response (4xx / 5xx)**
```json
{ "error": "<human readable message>" }
```

---

### `GET /round/active`
Returns the current active round ID by reading the quiz-pool contract state.

**Response**
```json
{ "round_id": 42 }
```

## Fee-Bump Flow

```
Player Device
  └─ Signs inner tx with passkey credential
       │
       ▼
Relayer (POST /relay)
  1. Validate inner XDR is well-formed
  2. Build fee-bump envelope:
       fee_source = SPONSOR_ACCOUNT
       base_fee   = req.fee_stroops (default: 1,000,000 stroops)
       inner_tx   = decoded player-signed tx
  3. Sign fee-bump with RELAYER_SPONSOR_SECRET_KEY
  4. Submit to Horizon POST /transactions
  5. Return { hash, status }
```

## Environment Variables

| Variable | Required | Description |
|---|---|---|
| `RELAYER_PORT` | No (default: 3001) | HTTP port |
| `STELLAR_RPC_URL` | No (default: testnet) | Soroban RPC endpoint |
| `STELLAR_HORIZON_URL` | No (default: testnet) | Horizon REST API |
| `STELLAR_NETWORK_PASSPHRASE` | No (default: testnet) | Network passphrase |
| `RELAYER_SPONSOR_SECRET_KEY` | **Yes** | Sponsor account secret key |
| `QUIZ_POOL_CONTRACT_ID` | **Yes** | Deployed quiz-pool contract ID |
| `LEADERBOARD_CONTRACT_ID` | **Yes** | Deployed leaderboard contract ID |

## Source Modules

| File | Responsibility |
|---|---|
| `src/main.rs` | Server startup, router setup |
| `src/config.rs` | Env var loading and validation |
| `src/handlers.rs` | HTTP route handler functions |
| `src/stellar.rs` | Fee-bump construction + Horizon submission |
| `src/error.rs` | Typed error enum → HTTP response mapping |
