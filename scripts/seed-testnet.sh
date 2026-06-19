#!/usr/bin/env bash
# seed-testnet.sh
#
# Seeds the Stellar testnet with:
#   - A funded sponsor account (for the relayer)
#   - A funded admin/oracle account
#   - Initializes both contracts with a default Config
#
# Usage: ./scripts/seed-testnet.sh
#
# TODO: Implement seeding steps:
#   1. soroban keys generate sponsor --network testnet
#   2. soroban keys fund sponsor --network testnet  (friendbot)
#   3. soroban contract invoke quiz-pool -- initialize ...
#   4. soroban contract invoke leaderboard -- initialize ...

set -euo pipefail

echo "Seeding SwiftQ on Stellar Testnet..."
echo "TODO: implement this script — see comments above"
