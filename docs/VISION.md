# SwiftQ — Project Vision & Whitepaper

> *The world's first gasless, passkey-native, on-chain Quiz-to-Earn platform built on the Stellar network.*

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [The Problem We Are Solving](#2-the-problem-we-are-solving)
3. [Our Solution — SwiftQ](#3-our-solution--swiftq)
4. [Why Stellar?](#4-why-stellar)
5. [How It Works — The Full User Journey](#5-how-it-works--the-full-user-journey)
6. [Technology Deep Dive](#6-technology-deep-dive)
7. [Tokenomics & Prize Pool Economics](#7-tokenomics--prize-pool-economics)
8. [Anti-Cheat & Fairness Guarantees](#8-anti-cheat--fairness-guarantees)
9. [The Viral Growth Loop](#9-the-viral-growth-loop)
10. [Market Opportunity](#10-market-opportunity)
11. [Roadmap](#11-roadmap)
12. [Open Source Model & Community](#12-open-source-model--community)
13. [Team & Contribution Model](#13-team--contribution-model)
14. [Why Invest / Contribute Now?](#14-why-invest--contribute-now)

---

## 1. Executive Summary

SwiftQ is an open-source, on-chain trivia and quiz-to-earn platform that puts real money in players' pockets within seconds of answering a question correctly — with **zero cryptocurrency knowledge required**.

A player opens SwiftQ, taps their phone's fingerprint or FaceID sensor, answers five trivia questions in under two minutes, and if they win, **USDC stablecoin lands in their account before they put their phone down.** No seed phrases. No gas fees. No exchange accounts. No waiting.

This is made possible by three converging technologies on the **Stellar network**:

1. **Passkeys (WebAuthn / secp256r1)** — Device biometrics replace seed phrases entirely, creating a smart contract wallet invisible to the user.
2. **Fee-Bump Transactions** — The platform sponsors every transaction fee on behalf of the player. Players never need to hold XLM.
3. **Soroban Smart Contracts** — Rust-based contracts on Stellar handle prize escrow, anti-cheat verification, and instant payouts with sub-second finality.

SwiftQ is **not a casino**. It is a skill-based, knowledge-rewarding platform — the digital equivalent of a pub quiz that pays you instantly, transparently, and globally.

---

## 2. The Problem We Are Solving

### 2.1 Web3 Has a UX Crisis

The promise of blockchain technology — open, permissionless, borderless money — has been held back almost entirely by user experience. Consider what a new user must currently do to interact with any typical crypto app:

1. Research and choose a wallet application
2. Write down a 12–24 word seed phrase and store it safely
3. Purchase cryptocurrency on an exchange (requiring ID verification, bank linkage, 3–5 day wait)
4. Transfer cryptocurrency to their wallet
5. Understand "gas fees" and why they fluctuate
6. Finally interact with the app — which may fail if gas is too low

This is not an onboarding funnel. **It is a wall.** Studies consistently show that over 95% of people who download a crypto app never complete a single on-chain transaction. The industry is building products for the 5% who already understand crypto, while ignoring the 95% who could genuinely benefit from it.

### 2.2 Trivia & Gaming Has Proven Mass-Market Demand

Mobile trivia is one of the most proven formats in the history of gaming:

- **HQ Trivia** attracted **2.38 million concurrent players** for a single live game in 2018, crashing their servers — before blockchain infrastructure could support it.
- **Ludo King**, a simple board game, achieved over **1 billion downloads** by targeting emerging markets with low-friction mobile gameplay.
- **Kahoot!** has over **350 million registered users** and is valued at over $1.5 billion — purely on the back of competitive, real-time quiz mechanics.

The demand for competitive trivia is massive and proven. What has **never existed** is a version that pays winners real money **instantly**, globally, and without requiring a bank account or an exchange.

### 2.3 The Billion People Left Behind by Traditional Finance

Over **1.4 billion adults** globally remain unbanked. In Sub-Saharan Africa, Southeast Asia, and Latin America, mobile phone penetration dramatically outpaces bank account penetration. These communities represent both the largest opportunity for crypto adoption **and** the demographic most likely to be genuinely life-changed by earning even $1–$5 from a quiz game.

Traditional quiz-to-earn models have failed these users because payouts require PayPal, bank wires, or exchange accounts — all of which require documents, addresses, and fees these users cannot easily access.

SwiftQ's payout is a Stellar stablecoin transfer. It settles in **3–5 seconds**. It can be cashed out at a **MoneyGram location** (a Stellar network partner) within minutes. A player in Lagos, Nairobi, Manila, or Bogotá can win $2 on SwiftQ and walk to their local phone shop to pick it up in cash. **That is a fundamentally new capability.**

---

## 3. Our Solution — SwiftQ

SwiftQ is designed around a single north-star principle: **the experience must feel like a regular mobile game, not a crypto app.**

Every design decision — from passkey onboarding to fee sponsorship to USDC payouts — serves this principle.

### 3.1 Core Features

| Feature | Description |
|---|---|
| **Passkey Onboarding** | Sign up in one tap using FaceID or fingerprint. No seed phrase. No email required. |
| **Gasless Play** | The platform pays all transaction fees. Players never need XLM or any other gas token. |
| **Instant Payouts** | Winnings arrive in under 5 seconds thanks to Stellar's finality. |
| **Commit-Reveal Anti-Cheat** | Cryptographic answer hashing prevents bots and cheaters at the contract level. |
| **On-Chain Leaderboard** | Permanent, tamper-proof global rankings stored on the Stellar blockchain. |
| **USDC Stablecoin Rewards** | Prizes are in USD Coin — a stable, globally recognised dollar-pegged stablecoin. |
| **Real-World Cash Out** | Via MoneyGram Access and Stellar's existing off-ramp infrastructure. |
| **Daily Rounds** | Fresh quiz rounds every day keep players returning for their next shot at the prize pool. |
| **Share-to-Earn Challenges** | Players can challenge friends via a shareable link, creating viral referral loops. |

### 3.2 What Makes SwiftQ Different

SwiftQ is not the first "play-to-earn" or "quiz-to-earn" concept. However, it is the first to combine all of the following simultaneously:

- **No wallet setup** — Passkeys mean zero onboarding friction
- **No gas** — Fee-bump sponsorship means players never think about transaction costs
- **Sub-second settlement** — Stellar's 3–5 second finality is orders of magnitude faster than Ethereum or Solana for stablecoin payouts
- **Verifiable fairness** — Every round is settled on-chain; anyone can audit the prize distribution
- **Real off-ramp** — MoneyGram integration means winnings are spendable in the real world immediately
- **Fully open source** — No black box; the contract logic, prize math, and anti-cheat rules are publicly auditable by anyone

---

## 4. Why Stellar?

The choice of Stellar is not incidental. It is the **only blockchain** that simultaneously offers everything SwiftQ requires.

### 4.1 Stellar vs. Alternatives

| Capability | Stellar | Ethereum | Solana | Polygon |
|---|---|---|---|---|
| Transaction finality | **3–5 seconds** | 12–64 seconds | ~400ms (but frequent outages) | 2+ seconds |
| Transaction cost | **$0.000001** | $0.50–$50+ | $0.001–$0.01 | $0.01–$0.10 |
| Native Passkey (secp256r1) support | **Yes (native)** | No (ERC-4337 workaround) | No | No |
| Native USDC | **Yes (Circle)** | Yes | Yes | Yes |
| MoneyGram cash-out integration | **Yes (native partnership)** | No | No | No |
| Soroban smart contracts (Rust) | **Yes** | No (Solidity) | No (Rust, but different) | No |
| Fee-bump transaction standard | **Yes (native)** | No (requires ERC-4337) | No | No |

### 4.2 Native Passkey Support Is a Game Changer

Stellar natively supports the **secp256r1 elliptic curve** — the same curve used by device TPMs (Trusted Platform Modules) for passkeys. This means a hardware-generated biometric credential can directly sign Stellar transactions **without any browser extension, wallet adapter, or bridging layer.**

On every other major blockchain, implementing passkey wallets requires complex account abstraction workarounds (e.g., ERC-4337 on Ethereum) that add latency, cost, and failure points. On Stellar, it is a first-class primitive.

### 4.3 Stellar's Real-World Integration Layer

Stellar was designed from day one to bridge the gap between blockchain and the traditional financial system. Its existing integrations include:

- **MoneyGram Access** — Cash-in/cash-out at 350,000+ locations in 200+ countries
- **Circle (USDC issuer)** — Native USDC issuance directly on Stellar, not bridged from another chain
- **Anchors** — A network of licensed financial institutions that convert Stellar tokens to local currency in 50+ countries

This means SwiftQ winnings are not "locked inside crypto." They are instantly accessible as real spending money through infrastructure that already exists and is already regulated.

---

## 5. How It Works — The Full User Journey

### 5.1 Onboarding (First Time — Under 30 Seconds)

```
User opens SwiftQ
        │
        ▼
Taps "Create Account"
        │
        ▼
Device prompts FaceID / fingerprint
        │
        ▼
Device generates a secp256r1 passkey credential
(stored securely in device hardware — never leaves the device)
        │
        ▼
Passkey Kit creates a smart contract wallet on Stellar
(sponsored by SwiftQ — user pays nothing)
        │
        ▼
User is instantly inside the app ✓
No seed phrase. No email verification. No KYC for small amounts.
```

### 5.2 Joining a Quiz Round

```
Player sees "Daily Quiz — Prize Pool: $47.50 — Entry: $0.05"
        │
        ▼
Taps "Join Round"
        │
        ▼
Passkey signs the entry fee transaction on the device
        │
        ▼
SwiftQ relayer wraps it in a fee-bump and submits to Stellar
        │
        ▼
$0.05 USDC moved from player wallet into the on-chain escrow
Player is locked in for this round ✓
```

### 5.3 Answering Questions (Commit-Reveal Flow)

```
Question appears on screen with a 15-second countdown
        │
        ▼
Player selects an answer
        │
        ▼
App computes: SHA-256(answer + random_nonce)
and submits this hash to the contract
(the plaintext answer is NEVER sent to the blockchain at this stage)
        │
        ▼
After all questions, player reveals all answers
        │
        ▼
Contract verifies:
  ✓ The hash matches the committed hash
  ✓ The timestamp is within the allowed window (anti-bot check)
  ✓ The answer is correct (verified by oracle signature)
        │
        ▼
Score recorded on-chain
```

### 5.4 Round Settlement & Payout

```
Round closes (all players submitted or time expired)
        │
        ▼
Admin/Oracle calls distribute_prizes() on the contract
        │
        ▼
Contract sends USDC directly from escrow:
  🥇 1st place → 60% of prize pool
  🥈 2nd place → 30% of prize pool
  🥉 3rd place → 10% of prize pool
  🏛️ Platform  → deducted fee (e.g. 10%)
        │
        ▼
Transaction settles in 3–5 seconds
        │
        ▼
Player sees: "🎉 You won $1.80! It's in your wallet."
        │
        ▼
Player can immediately cash out via the app's off-ramp
or hold USDC for future rounds
```

---

## 6. Technology Deep Dive

### 6.1 Smart Contract Layer (Soroban / Rust)

SwiftQ runs on two Soroban smart contracts deployed on the Stellar blockchain:

**`quiz-pool` Contract**
The core game engine. It is responsible for:
- Collecting entry fees from players (USDC)
- Storing cryptographic answer commitments (commit-reveal pattern)
- Enforcing per-question time windows (anti-bot)
- Distributing the prize pool to winners
- Maintaining round state and advancing round IDs

All active game state (sessions, commitments, prize pools) is stored in **Soroban Temporary Storage**, which auto-expires after a set ledger window and charges minimal state rent — keeping operational costs as close to zero as possible.

Global configuration and the round counter use **Instance Storage**, which persists as long as the contract is alive.

**`leaderboard` Contract**
A separate contract holding permanent player statistics. Because leaderboard data must survive indefinitely, it uses **Persistent Storage**. This contract stores:
- Total wins per player
- Total rounds played
- Cumulative USDC earnings
- Highest single-round score
- Last active timestamp

Separating the leaderboard into its own contract allows it to be updated by multiple game types in the future (not just the quiz pool), making SwiftQ extensible into a broader gaming ecosystem.

### 6.2 Fee-Bump Relayer (Rust / Axum)

The relayer is a lightweight Rust HTTP server running on the platform's infrastructure. It is the bridge between the player's device and the Stellar network.

**Why Rust?** Consistency with the smart contract layer, zero runtime overhead, and production-grade reliability for a service that must handle real-money transactions.

**Fee-Bump Mechanics:**
Stellar's fee-bump transaction standard allows a third-party account to pay the network fee for another account's transaction. The inner transaction is signed by the player's passkey. The outer fee-bump envelope is signed by SwiftQ's sponsor account. The network sees a valid, sponsored transaction and processes it without any requirement for the player to hold XLM.

**Relayer Endpoints:**

| Endpoint | Purpose |
|---|---|
| `POST /relay` | Accept player-signed XDR, wrap in fee-bump, submit to Stellar |
| `GET /health` | Liveness probe for infrastructure monitoring |
| `GET /round/active` | Return current active round ID |

### 6.3 TypeScript SDK (`@swiftq/sdk`)

A developer-friendly TypeScript package that abstracts all blockchain interactions for the frontend applications. Contributors building the web apps never call Stellar APIs directly — they call the SDK.

The SDK wraps:
- **Passkey Kit** — Stellar's official TypeScript library for secp256r1 smart wallets
- **Contract Client** — Auto-generated bindings from `soroban contract bindings typescript`
- **Relayer Client** — Typed HTTP client for the fee-bump relayer

### 6.4 Frontend Applications (Next.js 15)

**Player App (`apps/web`)** — The primary consumer-facing experience. Optimised for mobile browsers. Key routes:
- `/` — Landing + onboarding (passkey creation)
- `/play` — Active quiz round interface
- `/leaderboard` — Global on-chain rankings
- `/profile` — Player stats and earnings history
- `/results/[roundId]` — Post-round breakdown and share prompt

**Admin Dashboard (`apps/admin`)** — Internal tooling for platform operators. Manages question content, triggers round settlement, monitors prize pool balances, and views player analytics.

### 6.5 Infrastructure & CI/CD

The monorepo uses **pnpm + Turborepo** for the JavaScript workspace and **Cargo workspace** for Rust. Three GitHub Actions pipelines run automatically on every pull request:

- **Contracts CI** — `cargo fmt`, `cargo clippy`, `cargo test` for both Soroban contracts
- **Relayer CI** — `cargo fmt`, `cargo clippy`, `cargo build --release` for the Axum server
- **Frontend CI** — `pnpm lint`, `pnpm typecheck`, `pnpm build` for all JS packages

No code reaches the `main` branch without passing all three pipelines.

---

## 7. Tokenomics & Prize Pool Economics

SwiftQ uses **USDC on Stellar** as its sole currency. There is no custom platform token. This is a deliberate and important design decision:

> **Players win real dollars. Not points. Not tokens with speculative value. Real, stable, USD-equivalent money.**

This removes the risk of a platform token collapse destroying player earnings (a failure mode that destroyed numerous play-to-earn games in 2022).

### 7.1 Entry Fee & Pool Mechanics

| Parameter | Default | Configurable? |
|---|---|---|
| Entry fee per round | $0.05 USDC | Yes (by admin) |
| Platform fee | 10% of pool | Yes (by admin, capped) |
| 1st place prize share | 60% of net pool | Yes |
| 2nd place prize share | 30% of net pool | Yes |
| 3rd place prize share | 10% of net pool | Yes |
| Max questions per round | 5 | Yes |
| Max seconds per question | 15 | Yes |

### 7.2 Revenue Model

The platform earns revenue through the platform fee deducted from each prize pool. At scale:

| Metric | Conservative | Moderate | Optimistic |
|---|---|---|---|
| Daily active players | 5,000 | 50,000 | 500,000 |
| Rounds played / player / day | 2 | 3 | 5 |
| Entry fee | $0.05 | $0.05 | $0.05 |
| Platform fee (10%) | $0.005 / round | $0.005 / round | $0.005 / round |
| **Daily platform revenue** | **$50** | **$750** | **$12,500** |
| **Annual platform revenue** | **$18,250** | **$273,750** | **$4,562,500** |

Additionally, the **ecosystem sponsorship model** (other Stellar projects sponsoring quiz categories) creates a secondary revenue stream where sponsors pay to have their project featured in a quiz round, subsidising the prize pool beyond the base entry fees.

---

## 8. Anti-Cheat & Fairness Guarantees

Fairness is non-negotiable for SwiftQ. Every anti-cheat mechanism is enforced at the smart contract level — not in a centrally-controlled database that could be manipulated.

### 8.1 Commit-Reveal Pattern

Players never send their answers in plaintext. The flow is:

1. **Commit phase**: Player sends `SHA-256(answer + nonce)` to the contract
2. **Reveal phase**: Player sends the plaintext answer and nonce; the contract recomputes the hash and verifies it matches the commitment

This means a malicious actor monitoring the Stellar mempool cannot see what other players answered before deciding their own answer.

### 8.2 Time-Window Enforcement

The contract records the ledger timestamp when each answer is committed. If the elapsed time between question display and commitment exceeds `max_secs_per_question` (default: 15 seconds), the submission is rejected by the contract. Automated scripts that solve questions via API cannot answer faster than the human-readable window allows.

### 8.3 Oracle-Verified Correctness

Answer correctness is verified via a signed attestation from a trusted oracle account (the SwiftQ admin account or a dedicated oracle service). This keeps question content off-chain (preventing blockchain bots from reading the correct answer) while ensuring the contract can fairly score submissions.

### 8.4 Transparent Prize Distribution

Every prize distribution is a public Stellar transaction. Anyone can verify on a Stellar explorer (like Stellar Expert) that:
- The correct prize amounts were sent
- They went to the correct winning addresses
- The platform fee matches the configured percentage

There is no way to secretly alter prize distributions because the rules are encoded in the Soroban contract, not in a private database.

---

## 9. The Viral Growth Loop

SwiftQ is designed to grow without paid advertising. The viral mechanics are built into the product.

### 9.1 The Share Trigger

The moment a player wins a round, the app shows:

> *"You just won $1.80 answering 5 questions! 🎉 Challenge a friend — they have 24 hours to beat your score."*

Tapping "Share" generates a deep link that pre-populates a WhatsApp, Telegram, or Twitter message. When a friend clicks the link, they are taken directly to SwiftQ, prompted to create a passkey account (30 seconds), and immediately dropped into the challenge round.

**The winner's entry fee for the challenge round is sponsored by the platform for the first round of any referred user.** This removes the last remaining barrier (the $0.05 entry fee) for the first-time experience.

### 9.2 The Social Proof Loop

```
Player wins $1.80
    │
    ▼
Shares with WhatsApp group
    │
    ▼
5 friends click the link
    │
    ▼
3 create accounts (passkey — 30 seconds)
    │
    ▼
2 of them play and lose their first round
    │
    ▼
1 of them wins — gets $0.90 — shares with their own group
    │
    └────────────────────────► Viral loop repeats
```

### 9.3 Daily Retention Mechanics

- **Daily question rotation** — New questions every day mean players return daily
- **Winning streaks** — The leaderboard tracks consecutive daily wins, creating status motivation
- **Progressive prize pools** — Prize pools grow throughout the day as more players join, creating FOMO that drives early evening play spikes (when friends are active and sharing)
- **Ecosystem-sponsored special rounds** — Weekly "mega rounds" with $100+ prize pools sponsored by Stellar ecosystem projects, heavily marketed across crypto and gaming communities

---

## 10. Market Opportunity

### 10.1 Total Addressable Market

| Market | Size |
|---|---|
| Mobile gaming (global) | $98 billion (2023) |
| Online trivia / casual gaming | $12 billion (2023) |
| Play-to-earn / GameFi | $4.6 billion (2023, recovering) |
| Crypto users globally | 420+ million (2023) |
| Smartphone users globally | 6.8 billion |
| Unbanked adults globally | 1.4 billion |

SwiftQ sits at the intersection of mobile gaming, financial inclusion, and blockchain adoption. Our realistic initial target market is the **420 million existing crypto users** who already understand wallets but are frustrated by UX — combined with the **2+ billion casual mobile gamers** who have never touched crypto but play trivia games.

### 10.2 Why Now?

Three macro trends are converging in 2025–2026:

1. **Passkey adoption is accelerating.** Apple, Google, and Microsoft have all embedded passkey support natively into their operating systems. As of 2024, over 15 billion devices support WebAuthn passkeys. The infrastructure for frictionless biometric auth is already in billions of pockets.

2. **Stablecoin regulation is maturing.** The EU's MiCA framework and the US stablecoin bills mean USDC is increasingly treated as a regulated, legitimate payment instrument — not a speculative asset. This opens the door for mainstream media and app stores to feature stablecoin-paying apps without regulatory risk.

3. **Stellar's real-world integrations are live.** MoneyGram Access, Circle's native USDC issuance, and a growing network of Stellar Anchors mean the cash-out problem is already solved. SwiftQ does not need to build any off-ramp infrastructure — it plugs into an existing global network.

---

## 11. Roadmap

### Phase 1 — Foundation (Current)
- [x] Open source monorepo scaffolded
- [ ] Soroban quiz-pool contract implemented and audited
- [ ] Soroban leaderboard contract implemented
- [ ] Fee-bump relayer implemented and deployed to testnet
- [ ] TypeScript SDK with Passkey Kit integration
- [ ] Web app MVP (onboarding + play + results)

### Phase 2 — Testnet Launch
- [ ] Full end-to-end testnet demo
- [ ] Community bug bash (open contribution bounties)
- [ ] Security audit of Soroban contracts
- [ ] Admin dashboard for question management
- [ ] First set of daily quiz questions (100+ questions across 10 categories)

### Phase 3 — Mainnet Beta
- [ ] Mainnet deployment with capped prize pools ($10 max per round)
- [ ] MoneyGram cash-out integration (via Stellar Anchor)
- [ ] Share-to-earn referral links
- [ ] Daily round scheduling (3 rounds/day)
- [ ] iOS and Android Progressive Web App (PWA) packaging

### Phase 4 — Growth
- [ ] Ecosystem sponsorship program (other Stellar projects sponsor quiz categories)
- [ ] Telegram Mini App integration
- [ ] "Proof of Knowledge" soulbound achievement NFTs on Stellar
- [ ] Multi-language support (Spanish, French, Swahili, Tagalog, Portuguese)
- [ ] $1,000+ weekly mega prize pools
- [ ] SDK published for third-party developers to build custom quiz experiences

### Phase 5 — Ecosystem
- [ ] SwiftQ as a platform: any developer can deploy a custom quiz pool using the contracts
- [ ] Corporate training use case (companies pay to host knowledge-verification quizzes with prize incentives)
- [ ] University and educational institution partnerships
- [ ] Cross-chain bridge for players on other networks to participate via USDC bridging

---

## 12. Open Source Model & Community

SwiftQ is **fully open source** under the MIT license. Every line of smart contract code, every relayer module, every SDK function, and every UI component is publicly auditable and forkable.

### 12.1 Why Open Source?

**For users and investors:** Transparency is trust. Players can verify that the prize distribution logic is exactly what we say it is. There is no hidden admin backdoor, no way to secretly inflate the platform cut, no black-box scoring. The code is the law.

**For growth:** A thriving open source community builds faster than any single company. By welcoming contributors from the Stellar ecosystem, the broader Rust community, and the Web3 frontend world, SwiftQ can expand its question bank, add language support, build integrations, and improve security faster than a closed-source team ever could.

**For adoption:** Open source projects become infrastructure. When SwiftQ's contracts are deployed, audited, and battle-tested on mainnet, other Stellar developers can fork and customise them to build their own quiz experiences — each one driving more users to the Stellar network.

### 12.2 Contributor Incentives

We believe contributors should share in the success they create. The following incentive structures are being designed:

- **Bug bounties** — Paid in USDC for smart contract vulnerabilities
- **Feature bounties** — Specific roadmap items offered as paid bounties to the community
- **On-chain attribution** — Major contributors receive a permanent acknowledgement stored in the leaderboard contract's instance storage
- **Future governance** — As the platform matures, meaningful contributors will receive governance participation rights over platform parameters (entry fees, question categories, prize splits)

---

## 13. Team & Contribution Model

SwiftQ is a community-owned project. There is no venture-funded founding team gatekeeping the codebase. The project was initiated with a clear architectural vision and a fully scaffolded, well-documented repository — and is now open for contributors.

### 13.1 What We Need

The following skill sets are actively needed:

| Role | Technologies | Priority |
|---|---|---|
| Soroban Contract Developer | Rust, Soroban SDK | 🔴 Critical |
| Rust Backend Developer | Rust, Axum, Tokio | 🔴 Critical |
| Frontend Developer | TypeScript, Next.js 15, React | 🟠 High |
| TypeScript SDK Developer | TypeScript, Stellar SDK, Passkey Kit | 🟠 High |
| UI/UX Designer | Figma + React implementation | 🟡 Medium |
| Technical Writer | Markdown, Documentation | 🟡 Medium |
| Question Content Creator | Any background | 🟢 Open |
| Security Auditor | Rust, Soroban, blockchain security | 🔴 Critical |

### 13.2 How to Get Involved

1. Read the [CONTRIBUTING.md](../CONTRIBUTING.md)
2. Browse open issues labelled `good first issue`
3. Join the discussion on GitHub Discussions
4. Pick a module from the architecture and claim an issue

Every module has a specification document in `docs/` that describes exactly what needs to be built — contributors can dive straight into implementation without needing to understand the whole system first.

---

## 14. Why Invest / Contribute Now?

### For Potential Investors

SwiftQ represents a rare convergence:

- **Proven demand** — Trivia gaming is a multi-billion dollar industry with demonstrated viral potential (HQ Trivia, Kahoot!)
- **Unique technology moat** — The combination of Passkey onboarding + Stellar fee-bump + Soroban contracts creates a UX that no competitor can replicate on any other blockchain
- **Real revenue model** — USDC entry fees + platform cut = immediate, sustainable, stablecoin-denominated revenue from day one of mainnet launch
- **Global financial inclusion angle** — Access to regulated ESG/impact capital for a project that demonstrably extends financial access to the unbanked
- **Open source network effects** — Community contributions compound over time, reducing future development costs while increasing the surface area of the platform
- **Stellar ecosystem tailwinds** — Stellar's existing MoneyGram, Circle, and Anchor partnerships mean the off-ramp problem is already solved

### For Developers and Contributors

- **Rust on the frontier** — Soroban is one of the most exciting smart contract environments for Rust developers. Working on SwiftQ contracts is a direct path to becoming a recognised expert in a fast-growing niche
- **Real users, real money** — This is not a demo project. When SwiftQ launches on mainnet, your code will handle real USDC transactions for real players around the world
- **Attribution and recognition** — Open source contributions here are permanently visible and verifiable on GitHub, contributing to your public portfolio
- **Community first** — Decisions about the platform are made transparently, with the community's input shaping the roadmap

### For Stellar Ecosystem Participants

SwiftQ is designed to drive **net new adoption of the Stellar network.** Every player who creates a passkey wallet on SwiftQ is a new Stellar user. Every quiz round that settles is a batch of Stellar transactions. Every USDC payout that flows to a MoneyGram cash-out location is proof of Stellar's real-world utility.

SwiftQ is not competing with existing Stellar applications. It is building the **top of the funnel** — bringing in the casual mobile gaming audience who would never otherwise encounter Stellar — and proving that blockchain technology can disappear into the background of an experience that just feels like a great game.

> *The best crypto app is one where users don't know they're using crypto.*

That is SwiftQ.

---

## Contact & Links

- **GitHub**: [github.com/your-org/swiftq](https://github.com/your-org/swiftq)
- **Architecture Docs**: [`docs/architecture.md`](./architecture.md)
- **Contract Specs**: [`docs/contracts/`](./contracts/)
- **Contributing Guide**: [`CONTRIBUTING.md`](../CONTRIBUTING.md)
- **License**: MIT

---

*SwiftQ is an open-source project. All smart contract code is publicly auditable. Prize pool mechanics are fully on-chain and transparent. Past performance of any quiz round can be verified independently on the Stellar blockchain.*
