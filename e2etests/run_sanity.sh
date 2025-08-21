#!/bin/bash

# Run sanity - runs all sanity tests
# Usage: ./run_sanity.sh [path_to_q_binary]

Q_BINARY="q"

if [ $# -gt 0 ]; then
    Q_BINARY="$1"
    export Q_CLI_PATH="$Q_BINARY"
fi

echo "🚀 Running All Sanity Tests"
echo "============================="
echo ""

# Running only sanity tests
echo "🔄 Running all sanity tests..."
cargo test --tests --features "session_mgmt" --features "core_session"  --features "sanity" -- --nocapture --test-threads=1

exit_code=$?

echo ""
if [ $exit_code -eq 0 ]; then
    echo "🎉 All Sanity tests passed!"
else
    echo "💥 Some Sanity tests failed!"
fi

exit $exit_code
