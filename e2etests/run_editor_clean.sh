#!/bin/bash

# Clean editor-only test runner - runs only editor test files
# Usage: ./run_editor_clean.sh [path_to_q_binary]

Q_BINARY="q"

if [ $# -gt 0 ]; then
    Q_BINARY="$1"
    export Q_CLI_PATH="$Q_BINARY"
fi

echo "🚀 Running Editor Commands Tests"
echo "============================="
echo ""

# Run only the specific MCP test files
echo "🔄 Running /editor tests..."
cargo test --tests --features "editor" -- --nocapture --test-threads=1

exit_code=$?

echo ""
if [ $exit_code -eq 0 ]; then
    echo "🎉 All Editor tests passed!"
else
    echo "💥 Some Editor tests failed!"
fi

exit $exit_code
