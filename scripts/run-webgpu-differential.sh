#!/usr/bin/env bash
# WebGPU buffer-mapping differential harness (TICK026 product remediation).
# Fail-closed: requires bun + native .node for host platform — no SKIP-as-pass.
# Scope: buffer map/unmap write flush cases that historically caused prod bugs.
#
# IMPORTANT: corpus files are self-invoking scripts (not bun:test registries).
# Running `bun test` reports 0 pass / exit 0 — that is false green. Execute
# each file as a script and require non-zero case count evidence.
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

CASES=(
  "test/buffer-unmap-bug.test.js"
  "test/mapped-buffer-getmappedrange.test.js"
  "test/getmappedrange-edgecases.test.js"
  "test/overlapping-ranges.test.js"
)

echo "=== webgpu buffer-mapping differential $(date -Iseconds) ===" | tee -a "$LOG"
CANDIDATE_SHA="$(git -C "$REPO_ROOT" rev-parse HEAD 2>/dev/null || echo unknown)"
CASE_COUNT=0
FAILED=0

for case_file in "${CASES[@]}"; do
  if [[ ! -f "$REPO_ROOT/$case_file" ]]; then
    echo "::error::missing corpus file $case_file" | tee -a "$LOG"
    FAILED=1
    continue
  fi
  echo "--- RUN $case_file ---" | tee -a "$LOG"
  set +e
  # Self-invoking scripts: MUST use bun/path, not `bun test` (0 registered tests).
  bun "$REPO_ROOT/$case_file" >>"$LOG" 2>&1
  status=$?
  set -e
  if [[ $status -ne 0 ]]; then
    echo "::error::$case_file exited $status" | tee -a "$LOG"
    FAILED=1
  else
    CASE_COUNT=$((CASE_COUNT + 1))
    echo "OK $case_file" | tee -a "$LOG"
  fi
done

if [[ $FAILED -ne 0 || $CASE_COUNT -lt ${#CASES[@]} ]]; then
  jq -n \
    --arg verifiedAt "$(date -Iseconds)" \
    --arg candidateSha "$CANDIDATE_SHA" \
    --argjson caseCount "$CASE_COUNT" \
    --argjson expected "${#CASES[@]}" \
    '{
      schemaVersion: 1,
      slice: "native/buffer-mapping",
      status: "differential_fail",
      verifiedAt: $verifiedAt,
      lastComparedMainSha: $candidateSha,
      caseCount: $caseCount,
      expectedCaseCount: $expected,
      note: "buffer mapping corpus failed or incomplete — see log; bun test is forbidden (false green)"
    }' >"$ARTIFACT"
  echo "::error::webgpu differential FAIL (cases=$CASE_COUNT expected=${#CASES[@]} failed=$FAILED)" | tee -a "$LOG"
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
    note: "Self-invoking JS corpus executed via bun file.js (not bun test). Host-platform native .node required."
  }' >"$ARTIFACT"

echo "webgpu-differential: OK (slice=native/buffer-mapping cases=$CASE_COUNT sha=$CANDIDATE_SHA)" | tee -a "$LOG"
cat "$ARTIFACT" | tee -a "$LOG"
cp "$ARTIFACT" "$REPO_ROOT/verification-webgpu-differential-local.json" 2>/dev/null || true
exit 0
