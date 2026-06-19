#!/usr/bin/env bash
# deploy-contracts.sh
#
# Builds and deploys both Soroban contracts to the target network.
# Usage: ./scripts/deploy-contracts.sh [testnet|mainnet]
#
# TODO: Implement deployment steps:
#   1. cargo build --target wasm32-unknown-unknown --release -p swiftq-quiz-pool
#   2. soroban contract deploy --wasm target/.../swiftq_quiz_pool.wasm --network $NETWORK
#   3. cargo build --target wasm32-unknown-unknown --release -p swiftq-leaderboard
#   4. soroban contract deploy --wasm target/.../swiftq_leaderboard.wasm --network $NETWORK
#   5. Write deployed contract IDs to .env.local

set -euo pipefail

NETWORK="${1:-testnet}"
echo "Deploying SwiftQ contracts to: $NETWORK"
echo "TODO: implement this script — see comments above"
