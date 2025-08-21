#!/bin/bash

# tools command test runner - runs only usage test files for sanity tests.
# Usage: ./run_tools_sanity.sh [path_to_q_binary]

Q_BINARY="q"

if [ $# -gt 0 ]; then
    Q_BINARY="$1"
    export Q_CLI_PATH="$Q_BINARY"
fi

echo "🚀 Running Tools Command for Sanity Tests"
echo "============================="
echo ""

# Run only the /tools command test
echo "🔄 Running /tools for sanity test..."
cargo test --test  --features "tools" --features "sanity" -- --nocapture --test-threads=1

exit_code=$?

echo ""
if [ $exit_code -eq 0 ]; then
    echo "🎉 Tools test passed!"
else
    echo "💥 Tools test failed!"
fi

exit $exit_code
