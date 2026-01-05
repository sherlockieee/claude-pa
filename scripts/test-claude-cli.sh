#!/bin/bash
# Test script to verify Claude Code CLI integration works correctly

echo "=== Testing Claude Code CLI Integration ==="
echo ""

# Test 1: Check if claude is installed
echo "1. Checking if Claude Code is installed..."
if command -v claude &> /dev/null; then
    echo "   ✓ Claude Code CLI found"
    claude --version
else
    echo "   ✗ Claude Code CLI not found"
    exit 1
fi

echo ""

# Test 2: Test basic text output
echo "2. Testing basic text output..."
RESULT=$(claude -p "Say 'test passed' exactly" --output-format text 2>&1)
if echo "$RESULT" | grep -qi "test passed"; then
    echo "   ✓ Text output works"
else
    echo "   ✗ Text output failed: $RESULT"
fi

echo ""

# Test 3: Test JSON output format
echo "3. Testing JSON output format..."
RESULT=$(claude -p "Say hello" --output-format json 2>&1)
if echo "$RESULT" | grep -q '"result"'; then
    echo "   ✓ JSON output works"
else
    echo "   ✗ JSON output failed: $RESULT"
fi

echo ""

# Test 4: Test stream-json (requires --verbose)
echo "4. Testing stream-json output format..."
RESULT=$(claude -p "Say hello" --output-format stream-json --verbose 2>&1)
if [ $? -eq 0 ]; then
    echo "   ✓ Stream-JSON output works (with --verbose)"
else
    echo "   ⚠ Stream-JSON requires --verbose flag"
    echo "   Result: $RESULT"
fi

echo ""

# Test 5: Test session continuity
echo "5. Testing session continuity..."
RESULT1=$(claude -p "Remember this number: 42" --output-format json 2>&1)
SESSION_ID=$(echo "$RESULT1" | grep -o '"session_id":"[^"]*"' | cut -d'"' -f4)
if [ -n "$SESSION_ID" ]; then
    RESULT2=$(claude --resume "$SESSION_ID" -p "What number did I ask you to remember?" --output-format json 2>&1)
    if echo "$RESULT2" | grep -q "42"; then
        echo "   ✓ Session continuity works"
    else
        echo "   ⚠ Session resumed but didn't remember (may need longer context)"
    fi
else
    echo "   ✗ No session_id returned"
fi

echo ""
echo "=== Tests Complete ==="
