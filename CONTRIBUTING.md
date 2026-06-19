# Contributing to SwiftQ

Thank you for your interest in contributing to SwiftQ! 🎉

## Ways to Contribute

- 🐛 **Bug Reports** — Open an issue with the `bug` label
- 💡 **Feature Ideas** — Open an issue with the `enhancement` label
- 📝 **Documentation** — Improve docs, add architecture diagrams
- 🔧 **Code** — Pick up a `good first issue` and submit a PR

## Development Setup

See the [README](./README.md#quick-start-contributors) for environment setup.

## Project Areas

| Area | Path | Language |
|---|---|---|
| Quiz Pool Contract | `packages/contracts/quiz-pool` | Rust |
| Leaderboard Contract | `packages/contracts/leaderboard` | Rust |
| Fee Relayer | `packages/relayer` | Rust |
| TypeScript SDK | `packages/sdk` | TypeScript |
| Player Web App | `apps/web` | TypeScript / React |
| Admin Dashboard | `apps/admin` | TypeScript / React |

## Pull Request Process

1. Fork the repo and create a feature branch: `feat/your-feature-name`
2. Write tests for new Soroban contract logic
3. Ensure `cargo test --workspace` and `pnpm test` pass
4. Keep PRs focused — one feature per PR
5. Fill out the PR template

## Code Style

- **Rust**: `cargo fmt` + `cargo clippy --all-targets -- -D warnings`
- **TypeScript**: ESLint + Prettier (run `pnpm format`)

## Commit Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(contracts): add commit-reveal anti-cheat logic
fix(relayer): handle fee-bump timeout gracefully
docs: update architecture diagram
```

## Community

Questions? Open a Discussion on GitHub or join our community channel.
