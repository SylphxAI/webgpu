#!/usr/bin/env bash
# WebGPU buffer-mapping differential harness (TICK026 product remediation).
# Fail-closed: requires bun + native .node for host platform — no SKIP-as-pass.
# Scope: buffer map/unmap write flush cases that historically caused prod bugs.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SCRATCH="${SCRATCH_DIR:-/tmp/webgpu-differential}"
mkdir -p "$SCRATCH"
LOG="$SCRATCH/differential.log"
ARTIFACT="$SCRATCH/verification.json"
: >"$LOG"

cd "$REPO_ROOT"

if ! command -v bun >/dev/null 2>&1; then
  echo "::error::bun required for webgpu differential — no SKIP-as-pass" | tee -a "$LOG"
  exit 1
fi

if [[ ! -f "$REPO_ROOT/index.js" ]]; then
  echo "::error::index.js missing (napi loader)" | tee -a "$LOG"
  exit 1
fi

# Host platform native binary must resolve via index.js optionalDeps / local .node
echo "=== webgpu buffer-mapping differential $(date -Iseconds) ===" | tee -a "$LOG"

set +e
bun test \
  test/buffer-unmap-bug.test.js \
  test/mapped-buffer-getmappedrange.test.js \
  test/getmappedrange-edgecases.test.js \
  test/overlapping-ranges.test.js \
  2>&1 | tee -a "$LOG"
status=${PIPESTATUS[0]}
set -e

CANDIDATE_SHA="$(git -C "$REPO_ROOT" rev-parse HEAD 2>/dev/null || echo unknown)"
CASE_COUNT=4
if [[ $status -ne 0 ]]; then
  jq -n \
    --arg verifiedAt "$(date -Iseconds)" \
    --arg candidateSha "$CANDIDATE_SHA" \
    --argjson caseCount "$CASE_COUNT" \
    '{
      schemaVersion: 1,
      slice: "native/buffer-mapping",
      status: "differential_fail",
      verifiedAt: $verifiedAt,
      lastComparedMainSha: $candidateSha,
      caseCount: $caseCount,
      note: "buffer mapping corpus failed — see log"
    }' >"$ARTIFACT"
  echo "::error::webgpu differential FAIL" | tee -a "$LOG"
  cat "$ARTIFACT" | tee -a "$LOG"
  exit 1
fi

jq -n \
  --arg verifiedAt "$(date -Iseconds)" \
  --arg candidateSha "$CANDIDATE_SHA" \
  --argjson caseCount "$CASE_COUNT" \
  '{
    schemaVersion: 1,
    slice: "native/buffer-mapping",
    status: "differential_green",
    verifiedAt: $verifiedAt,
    lastComparedMainSha: $candidateSha,
    caseCount: $caseCount,
    cases: [
      "buffer-unmap-bug",
      "mapped-buffer-getmappedrange",
      "getmappedrange-edgecases",
      "overlapping-ranges"
    ],
    note: "TS/JS wrapper oracle exercises native buffer map/unmap flush semantics on host platform binary"
  }' >"$ARTIFACT"

echo "webgpu-differential: OK (slice=native/buffer-mapping cases=$CASE_COUNT sha=$CANDIDATE_SHA)" | tee -a "$LOG"
cat "$ARTIFACT" | tee -a "$LOG"
cp "$ARTIFACT" "$REPO_ROOT/verification-webgpu-differential-local.json" 2>/dev/null || true
exit 0
