#!/bin/bash

# Clean subscribet-only test runner - runs only subscribet test files
# Usage: ./run_subscribet_clean.sh [path_to_q_binary]

Q_BINARY="q"

if [ $# -gt 0 ]; then
    Q_BINARY="$1"
    export Q_CLI_PATH="$Q_BINARY"
fi

echo "🚀 Running subscribet Commands Tests"
echo "============================="
echo ""

# Run only the specific MCP test files
echo "🔄 Running /subscribet tests..."
cargo test --tests --features "subscribet" --features "sanity"  -- --nocapture --test-threads=1

exit_code=$?

echo ""
if [ $exit_code -eq 0 ]; then
    echo "🎉 All Subscribet tests passed!"
else
    echo "💥 Some Subscribet tests failed!"
fi

exit $exit_code
