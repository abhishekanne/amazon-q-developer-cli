#!/bin/bash

# Clean model-only test runner - runs only model test files
# Usage: ./run_model_clean.sh [path_to_q_binary]

Q_BINARY="q"

if [ $# -gt 0 ]; then
    Q_BINARY="$1"
    export Q_CLI_PATH="$Q_BINARY"
fi

echo "🚀 Running Model Commands Tests"
echo "============================="
echo ""

# Run only the specific MCP test files
echo "🔄 Running /model tests..."
cargo test --tests --features "model" -- --nocapture --test-threads=1

exit_code=$?

echo ""
if [ $exit_code -eq 0 ]; then
    echo "🎉 All Model tests passed!"
else
    echo "💥 Some Model tests failed!"
fi

exit $exit_code
