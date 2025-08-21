#!/bin/bash

# Compact test runner - runs only compact test files for sanity tests.
# Usage: ./run_compact_sanity.sh [path_to_q_binary]

Q_BINARY="q"

if [ $# -gt 0 ]; then
    Q_BINARY="$1"
    export Q_CLI_PATH="$Q_BINARY"
fi

echo "🚀 Running Compact Commands for Sanity Tests"
echo "============================="
echo ""

# Run only the specific MCP test files
echo "🔄 Running /compact for sanity tests..."
cargo test --tests --features "compact" --features "sanity"  -- --nocapture --test-threads=1

exit_code=$?

echo ""
if [ $exit_code -eq 0 ]; then
    echo "🎉 All Compact tests passed!"
else
    echo "💥 Some Compact tests failed!"
fi

exit $exit_code
