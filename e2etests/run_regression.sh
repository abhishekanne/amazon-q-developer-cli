#!/bin/bash

# Run regression - runs all regression tests
# Usage: ./run_regression.sh [path_to_q_binary]

Q_BINARY="q"

if [ $# -gt 0 ]; then
    Q_BINARY="$1"
    export Q_CLI_PATH="$Q_BINARY"
fi

echo "🚀 Running All regression Tests"
echo "============================="
echo ""

# Running only regression tests
echo "🔄 Running all regression tests..."
cargo test --tests --features "session_mgmt" --features "core_session"  --features "regression" -- --nocapture --test-threads=1

exit_code=$?

echo ""
if [ $exit_code -eq 0 ]; then
    echo "🎉 All regression tests passed!"
else
    echo "💥 Some regression tests failed!"
fi

exit $exit_code
