#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PINTA_RS_DIR="$ROOT_DIR/pinta-rs"
OUTPUT_ROOT="${1:-$ROOT_DIR/pinta-rs-tool-mock-tests}"
DIAGNOSTICS_ROOT="$OUTPUT_ROOT/diagnostics"
WINDOW_OUTPUT="$OUTPUT_ROOT/pinta-rs-window.png"
SUMMARY_PATH="$OUTPUT_ROOT/summary.tsv"

TOOLS=(
	"move-selected-pixels"
	"move-selection"
	"zoom"
	"pan"
	"rectangle-select"
	"ellipse-select"
	"lasso-select"
	"magic-wand-select"
	"paintbrush"
	"pencil"
	"eraser"
	"paint-bucket"
	"gradient"
	"color-picker"
	"text"
	"line-curve"
	"rectangle"
	"rounded-rectangle"
	"ellipse"
	"freeform-shape"
	"clone-stamp"
	"recolor"
)

mkdir -p "$OUTPUT_ROOT" "$DIAGNOSTICS_ROOT"

capture_session() {
	local tool_name="$1"
	local scenario_mode="${2:-0}"
	local before session_dir mock_log ready_path
	before="$(find "$DIAGNOSTICS_ROOT" -mindepth 1 -maxdepth 1 -type d 2>/dev/null | sort || true)"
	mock_log="$(mktemp)"
	ready_path="$WINDOW_OUTPUT.ready"

	pushd "$PINTA_RS_DIR" >/dev/null
	PINTA_MOCK_CAPTURE_PATH="$WINDOW_OUTPUT" \
	PINTA_MOCK_DIAGNOSTICS_DIR="$DIAGNOSTICS_ROOT" \
	PINTA_MOCK_ACTIVE_TOOL="$tool_name" \
	PINTA_MOCK_TOOL_SCENARIO="$scenario_mode" \
	cargo run >"$mock_log" 2>&1 &
	local mock_pid=$!
	popd >/dev/null

	cleanup() {
		kill "$mock_pid" 2>/dev/null || true
		wait "$mock_pid" 2>/dev/null || true
		rm -f "$mock_log"
	}
	trap cleanup RETURN

	rm -f "$WINDOW_OUTPUT" "$ready_path"

	for _ in $(seq 1 180); do
		if [[ -f "$WINDOW_OUTPUT" && -f "$ready_path" ]]; then
			rm -f "$ready_path"
			break
		fi

		python3 - <<'PY'
import time
time.sleep(0.2)
PY
	done

	for _ in $(seq 1 180); do
		session_dir="$(find "$DIAGNOSTICS_ROOT" -mindepth 1 -maxdepth 1 -type d | sort | tail -n1)"
		if [[ -n "$session_dir" && -d "$session_dir" && "$(printf '%s' "$before" | rg -F --line-regexp "$session_dir" -c || true)" == "0" ]]; then
			crop_count="$(find "$session_dir" -maxdepth 1 -name 'capture-*-*-crop.png' | wc -l | tr -d ' ')"
			if [[ -f "$session_dir"/capture-005-canvas-crop.png && "$crop_count" -ge 28 ]]; then
				break
			fi
		fi

		python3 - <<'PY'
import time
time.sleep(0.2)
PY
	done

	if [[ -z "${session_dir:-}" || ! -d "$session_dir" ]]; then
		echo "failed to locate pinta-rs diagnostics session for $tool_name" >&2
		return 1
	fi

	trap - RETURN
	cleanup
	printf '%s\n' "$session_dir"
}

printf 'tool\trmse\tbefore_session\tafter_session\n' >"$SUMMARY_PATH"

for tool_name in "${TOOLS[@]}"; do
	target_dir="$OUTPUT_ROOT/$tool_name"
	rm -rf "$target_dir"
	mkdir -p "$target_dir"

	before_session="$(capture_session "$tool_name" 0)"
	after_session="$(capture_session "$tool_name" 1)"
	cp "$before_session"/capture-005-canvas-crop.png "$target_dir/canvas-before.png"
	cp "$after_session"/capture-005-canvas-crop.png "$target_dir/canvas-after.png"
	cp "$WINDOW_OUTPUT" "$target_dir/main-window.png"
	if [[ -f "$after_session"/capture-006-tool-toolbar-crop.png ]]; then
		cp "$after_session"/capture-006-tool-toolbar-crop.png "$target_dir/tool-toolbar.png"
	fi
	if [[ -f "$after_session"/capture-009-toolbox-crop.png ]]; then
		cp "$after_session"/capture-009-toolbox-crop.png "$target_dir/toolbox.png"
	fi

	rmse="$(python3 - "$target_dir/canvas-before.png" "$target_dir/canvas-after.png" <<'PY'
import math
import sys
from PIL import Image

left = Image.open(sys.argv[1]).convert('RGB')
right = Image.open(sys.argv[2]).convert('RGB')
if right.size != left.size:
    right = right.resize(left.size, Image.Resampling.LANCZOS)
sq = 0.0
for a, b in zip(left.getdata(), right.getdata()):
    sq += sum((x - y) ** 2 for x, y in zip(a, b))
rmse = math.sqrt(sq / (left.width * left.height * 3))
print(f"{rmse:.4f}")
PY
)"

	cat >"$target_dir/manifest.json" <<EOF
{
  "tool": "$tool_name",
	"before_session": "$before_session",
	"after_session": "$after_session",
  "before_canvas": "$target_dir/canvas-before.png",
  "after_canvas": "$target_dir/canvas-after.png",
  "rmse": $rmse
}
EOF

	printf '%s\t%s\t%s\t%s\n' "$tool_name" "$rmse" "$before_session" "$after_session" >>"$SUMMARY_PATH"
done

printf 'summary=%s\n' "$SUMMARY_PATH"
printf 'output_root=%s\n' "$OUTPUT_ROOT"