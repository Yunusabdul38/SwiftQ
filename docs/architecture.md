# SwiftQ — Architecture Overview

## System Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│                        Player's Device                              │
│                                                                     │
│   ┌──────────────┐    Passkey Sign     ┌──────────────────────┐    │
│   │  Web App     │ ──────────────────► │  Hardware Credential  │   │
│   │ (Next.js 15) │                     │  (FaceID / TouchID)   │   │
│   └──────┬───────┘                     └──────────────────────┘    │
└──────────┼──────────────────────────────────────────────────────────┘
           │ Signed XDR (inner tx)
           ▼
┌──────────────────────────────┐
│    Fee-Bump Relayer           │
│    (Rust / Axum)              │
│                               │
│  1. Validate inner tx XDR     │
│  2. Wrap in fee-bump tx       │
│  3. Sign with sponsor key     │
│  4. Submit to Stellar RPC     │
└──────────────┬────────────────┘
               │ Fee-bump tx
               ▼
┌──────────────────────────────┐
│    Stellar Network            │
│                               │
│  ┌────────────────────────┐   │
│  │  quiz-pool Contract    │   │
│  │  (Soroban / Rust)      │   │
│  │  - Entry fee escrow    │   │
│  │  - Commit-reveal       │   │
│  │  - Prize distribution  │   │
│  └────────────────────────┘   │
│                               │
│  ┌────────────────────────┐   │
│  │  leaderboard Contract  │   │
│  │  (Soroban / Rust)      │   │
│  │  - Persistent stats    │   │
│  │  - Rankings            │   │
│  └────────────────────────┘   │
└──────────────────────────────┘
```

## Component Responsibilities

| Component | Responsibility |
|---|---|
| `apps/web` | Player-facing UI — onboarding, quiz play, results, leaderboard |
| `apps/admin` | Internal dashboard — question management, round control, analytics |
| `packages/contracts/quiz-pool` | Escrow, commit-reveal anti-cheat, prize distribution |
| `packages/contracts/leaderboard` | Persistent on-chain player stats and rankings |
| `packages/relayer` | Fee-bump sponsorship — abstracts gas from players |
| `packages/sdk` | TypeScript bindings — Passkey Kit, contract client, relayer client |
| `packages/ui` | Shared React components used by both apps |

## Key Design Decisions

### 1. Passkeys Over Seed Phrases
Players never see a private key. Their device generates a `secp256r1` credential via WebAuthn. The Stellar Passkey Kit wraps this into a smart contract wallet.

### 2. Gasless UX via Fee-Bump
Every transaction is wrapped in a Stellar fee-bump with the platform's sponsor account as the outer signer. Players pay $0 in gas.

### 3. Commit-Reveal Anti-Cheat
Players submit `SHA-256(answer + nonce)` before revealing. The contract enforces a per-question time window to prevent bot scripting.

### 4. Temporary vs Persistent Storage
- **Temporary**: Active sessions, commitments (auto-expiring, minimal rent)
- **Persistent/Instance**: Config, leaderboard stats, round ID
