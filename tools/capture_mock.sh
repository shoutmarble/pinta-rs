#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_DIR="${1:-$ROOT_DIR/captures}"
STAMP="$(date +%Y%m%d-%H%M%S)"
OUTPUT_PATH="$OUTPUT_DIR/pinta-rs-$STAMP.png"
LOG_PATH="$OUTPUT_DIR/pinta-rs-$STAMP.log"

mkdir -p "$OUTPUT_DIR"

pushd "$ROOT_DIR" >/dev/null
PINTA_MOCK_CAPTURE_PATH="$OUTPUT_PATH" cargo run >"$LOG_PATH" 2>&1 &
APP_PID=$!

cleanup() {
	kill "$APP_PID" 2>/dev/null || true
	wait "$APP_PID" 2>/dev/null || true
}
trap cleanup EXIT

for _ in $(seq 1 80); do
	if [[ -f "$OUTPUT_PATH" ]]; then
		echo "$OUTPUT_PATH"
		exit 0
	fi
	python3 - <<'PY'
import time
time.sleep(0.2)
PY
done

echo "failed to capture mock screenshot" >&2
exit 1