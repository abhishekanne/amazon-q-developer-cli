#!/bin/bash

# Usage  test runner - runs only usage test files for regresion tests.
# Usage: ./run_usage_regression.sh [path_to_q_binary]

Q_BINARY="q"

if [ $# -gt 0 ]; then
    Q_BINARY="$1"
    export Q_CLI_PATH="$Q_BINARY"
fi

echo "🚀 Running Usage Commands for Regression Tests"
echo "============================="
echo ""

# Run only the specific MCP test files
echo "🔄 Running /usage for regression tests..."
cargo test --tests --features "usage"  --features "regression" -- --nocapture --test-threads=1

exit_code=$?

echo ""
if [ $exit_code -eq 0 ]; then
    echo "🎉 All Usage tests passed!"
else
    echo "💥 Some Usage tests failed!"
fi

exit $exit_code
