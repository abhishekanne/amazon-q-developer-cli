#!/bin/bash

# Clean hooks-only test runner - runs only Hooks test files
# Usage: ./run_hooks_clean.sh [path_to_q_binary]

Q_BINARY="q"

if [ $# -gt 0 ]; then
    Q_BINARY="$1"
    export Q_CLI_PATH="$Q_BINARY"
fi

echo "🚀 Running Hooks Commands Tests"
echo "============================="
echo ""

# Run only the specific MCP test files
echo "🔄 Running /hooks tests..."
cargo test --tests --features "hooks" -- --nocapture --test-threads=1

exit_code=$?

echo ""
if [ $exit_code -eq 0 ]; then
    echo "🎉 All Hooks tests passed!"
else
    echo "💥 Some Hooks tests failed!"
fi

exit $exit_code
