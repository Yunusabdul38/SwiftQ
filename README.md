# SwiftQ 🎯

> **On-Chain Trivia / Quiz-to-Earn on Stellar** — Open Source

SwiftQ is a fully open-source, Passkey-native, gasless quiz-to-earn platform built on the Stellar network. Players sign in with FaceID or fingerprint, answer daily trivia questions, and win real USDC — all with sub-second finality and zero seed phrases.

---

## ✨ Why SwiftQ?

| Feature | How it works |
|---|---|
| **No seed phrases** | Passkeys (secp256r1) create a smart wallet tied to your device biometrics |
| **Zero gas for players** | Fee-bump relayer (Rust/Axum) sponsors every transaction |
| **Instant payouts** | Stellar's sub-second finality — USDC hits your account the moment the round ends |
| **Anti-cheat** | Soroban contract enforces per-question time windows |
| **Real off-ramp** | MoneyGram Access + USDC means winnings are real money |

---

## 📁 Repository Structure

```
swiftq/
├── apps/
│   ├── web/                 # Next.js 15 — Player-facing app
│   └── admin/               # Next.js 15 — Quiz management & analytics
├── packages/
│   ├── contracts/
│   │   ├── quiz-pool/       # Soroban: Escrow, commit-reveal, prize distribution
│   │   └── leaderboard/     # Soroban: Persistent on-chain leaderboard
│   ├── relayer/             # Rust/Axum: Fee-bump transaction relayer
│   ├── sdk/                 # TypeScript: Passkey Kit + contract client
│   └── ui/                  # Shared React component library
├── docs/                    # Architecture, contributing guide
└── scripts/                 # Deploy & testnet seed scripts
```

---

## 🚀 Quick Start (Contributors)

### Prerequisites
- [Rust](https://rustup.rs/) `>= 1.78`
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup) `>= 21.0`
- [Node.js](https://nodejs.org/) `>= 20`
- [pnpm](https://pnpm.io/) `>= 9`

### Setup

```bash
git clone https://github.com/your-org/swiftq
cd swiftq

# Install JS dependencies
pnpm install

# Build Rust workspace
cargo build --workspace

# Run the web app (testnet)
pnpm dev
```

### Running Tests

```bash
# Soroban contract tests
cargo test --workspace

# TypeScript / UI tests
pnpm test
```

---

## 🛠 Tech Stack

- **Smart Contracts** — [Soroban](https://soroban.stellar.org/) (Rust)
- **Fee Relayer** — [Axum](https://github.com/tokio-rs/axum) (Rust)
- **Frontend** — [Next.js 15](https://nextjs.org/) with App Router
- **Passkeys** — [Stellar Passkey Kit](https://github.com/kalepail/passkey-kit)
- **Monorepo** — [pnpm](https://pnpm.io/) + [Turborepo](https://turbo.build/)
- **Stablecoin** — USDC on Stellar (Testnet & Mainnet)

---

## 🤝 Contributing

We welcome all contributors! Please read [CONTRIBUTING.md](./CONTRIBUTING.md) and check the open issues for `good first issue` labels.

---

## 📄 License

MIT — see [LICENSE](./LICENSE)
