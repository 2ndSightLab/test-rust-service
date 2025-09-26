#!/bin/bash

set -e

echo "Running Test Rust Service Test Suite"
echo "===================================="

echo "Running all tests..."
cargo test

# Extract test results
TEST_OUTPUT=$(cargo test 2>&1)
SECURITY_TESTS=$(echo "$TEST_OUTPUT" | grep -A20 "Running tests/security_checks.rs" | grep -E "test result: ok\." | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+" | head -1)
UNIT_TESTS=$(echo "$TEST_OUTPUT" | grep -A20 "Running tests/unit_tests.rs" | grep -E "test result: ok\." | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+" | head -1)

# Set defaults if empty
SECURITY_TESTS=${SECURITY_TESTS:-0}
UNIT_TESTS=${UNIT_TESTS:-0}
TOTAL_TESTS=$((SECURITY_TESTS + UNIT_TESTS))

# Check for failures
if echo "$TEST_OUTPUT" | grep -q "FAILED"; then
    FAILED_COUNT=$(echo "$TEST_OUTPUT" | grep -E "test result: FAILED\." | tail -1 | grep -oE "[0-9]+ failed" | grep -oE "[0-9]+")
    echo ""
    echo "Test Results Summary:"
    echo "===================="
    echo "❌ Tests: FAILED ($TOTAL_TESTS passed, $FAILED_COUNT failed)"
    echo ""
    echo "💥 Test suite failed!"
    exit 1
else
    echo ""
    echo "Test Results Summary:"
    echo "===================="
    echo "✅ Security Checks: PASSED ($SECURITY_TESTS passed)"
    echo "✅ Unit Tests: PASSED ($UNIT_TESTS passed)"
    echo "✅ All Tests: PASSED ($TOTAL_TESTS passed, 0 failed)"
    echo ""
    echo "🎉 All tests completed successfully!"
fi
