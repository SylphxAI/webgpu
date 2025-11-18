#!/bin/bash

echo "🧪 Testing WebGPU with Bun..."
echo ""

# Test all examples with Bun
examples=(
  "compute.js"
  "triangle.js"
  "render-bundle.js"
  "msaa.js"
)

passed=0
failed=0

for example in "${examples[@]}"; do
  echo "Testing: $example"
  if bun "examples/$example" > /dev/null 2>&1; then
    echo "  ✅ PASS"
    ((passed++))
  else
    echo "  ❌ FAIL"
    ((failed++))
  fi
done

echo ""
echo "Results: $passed passed, $failed failed"

if [ $failed -eq 0 ]; then
  echo "✅ All tests passed with Bun!"
  exit 0
else
  echo "❌ Some tests failed"
  exit 1
fi
