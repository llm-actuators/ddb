#!/usr/bin/env bash
# Layer-1 source guards for ddb. Pure grep checks — no compilation, no device.
# Add to pre-commit / CI before cargo test so regressions are caught at the
# source-text level before tests run.
#
# Each guard is a (pattern, scope, reason) tuple. A non-empty match exits 1.
#
# Run from the ddb crate root:  ./scripts/check-guards.sh

set -euo pipefail

cd "$(dirname "$0")/.."

fail=0

# Heredoc rows: pattern \t paths (space-separated) \t reason
#
# Notes:
#   - "src/cmd/crawl.rs" only on patterns specifically about the crawl
#   - paths support globs; expanded by find when needed
#   - patterns are extended regex (grep -E)
#
guards=$(cat <<'EOF'
^fn (curl_get|curl_post|adb_shell|agent_base_url)	src/cmd/crawl.rs	crawl must not re-introduce private HTTP/adb helpers — use crate::adb + cmd::test_element
parse_elements\s*\(	src/agent_yaml.rs	serde_yaml::parse_elements removed; only split_elements + chunk_top_field + chunk_bounds belong here
\.unwrap\(\)\s*$	src/cmd/crawl.rs	crawl must not unwrap — surface errors so device runs fail loud
fn wait_idle\s*\(.*\)\s*\{[^}]*sleep\(.*15.*\)	src/cmd/test.rs	wait_idle hard-coded 15s discarded — must respect timeout arg
EOF
)

# App-specific identifiers are read from an external, gitignored config so this
# public toolchain ships no app/client names. Copy scripts/guard-identifiers.txt.example
# to scripts/guard-identifiers.txt and list your identifiers (one grep -E alternation
# token per line; blank lines and '#' comments ignored). Override the path with
# DDB_GUARD_IDENTIFIERS. Falls back to a generic default when the file is absent.
identifiers_file="${DDB_GUARD_IDENTIFIERS:-scripts/guard-identifiers.txt}"
if [ -f "$identifiers_file" ]; then
  app_ids=$(grep -vE '^[[:space:]]*#|^[[:space:]]*$' "$identifiers_file" | paste -sd'|' - || true)
else
  # No local identifier file → the app-name guard is opt-in and stays dormant
  # (the four static guards above still run). Copy the .example to enable it.
  app_ids=""
fi
if [ -n "$app_ids" ]; then
  guards="$guards
$app_ids	src/	app-specific identifiers must not appear in the toolchain — keep names in env vars / catalogue"
fi

while IFS=$'\t' read -r pattern scope reason; do
  [ -z "${pattern:-}" ] && continue
  # shellcheck disable=SC2086
  hits=$(grep -RnE "$pattern" $scope 2>/dev/null || true)
  if [ -n "$hits" ]; then
    echo "[guard FAIL] $reason"
    echo "  pattern: $pattern"
    echo "  scope:   $scope"
    echo "$hits" | sed 's/^/  /'
    echo
    fail=1
  fi
done <<<"$guards"

if [ "$fail" -ne 0 ]; then
  echo "ddb source guards: FAIL"
  exit 1
fi

echo "ddb source guards: OK"
