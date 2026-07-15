#!/usr/bin/env bash
# Fail-closed: @sylphx/webgpu production GPU authority must be native Rust NAPI, not pure TS.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
LEDGER="$ROOT/docs/specs/migration-ledger.json"

if [[ ! -f "$LEDGER" ]]; then
  echo "FAIL: missing migration ledger" >&2
  exit 1
fi

node - "$LEDGER" <<'NODE'
const ledger = JSON.parse(require("fs").readFileSync(process.argv[2], "utf8"));
const bad = ledger.capabilities.filter((c) => c.state !== "ts_deleted");
if (bad.length) {
  console.error("FAIL: non-ts_deleted capabilities:", bad.map((c) => c.id + "=" + c.state));
  process.exit(1);
}
console.log("OK ledger: all", ledger.capabilities.length, "capabilities ts_deleted");
NODE

# Prefer host-local NAPI binary; fall back to platform package trees under npm/.
native_found=0
shopt -s nullglob
for candidate in "$ROOT"/index.*.node "$ROOT"/npm/*/*.node; do
  if [[ -f "$candidate" && -s "$candidate" ]]; then
    echo "OK native artifact: ${candidate#$ROOT/} ($(wc -c <"$candidate") bytes)"
    native_found=1
    break
  fi
done
shopt -u nullglob

if [[ "$native_found" -ne 1 ]]; then
  echo "FAIL: no native .node artifact present" >&2
  exit 1
fi

if [[ ! -f "$ROOT/index.js" ]]; then
  echo "FAIL: missing index.js entry" >&2
  exit 1
fi

if ! grep -qE '\.node|createRequire|native' "$ROOT/index.js"; then
  echo "FAIL: index.js does not load a native binding" >&2
  exit 1
fi

echo "PASS: native Rust NAPI is sole WebGPU authority"
