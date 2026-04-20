#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PINTA_RS_DIR="$ROOT_DIR/pinta-rs"
PINTA_UPSTREAM_DIR="$ROOT_DIR/pinta-upstream"

UPSTREAM_WINDOW="$ROOT_DIR/pinta-upstream-window.png"
MOCK_WINDOW="$ROOT_DIR/pinta-rs-window.png"
WINDOW_SIDE_BY_SIDE="$ROOT_DIR/pinta-window-side-by-side.png"
WINDOW_DIFF="$ROOT_DIR/pinta-window-diff.png"
WINDOW_METRIC="$ROOT_DIR/pinta-window-metric.txt"
REFLECTION_DIR="$ROOT_DIR/pinta-upstream-reflection"
MOCK_DIAGNOSTICS_ROOT="$ROOT_DIR/pinta-rs-diagnostics-output"
DIAGNOSTICS_ROOT="$ROOT_DIR/upstream-diagnostics-output"
SAMPLE_INPUT="$ROOT_DIR/sample-input.png"

rm -f \
	"$ROOT_DIR/pinta-upstream-session.log" \
	"$ROOT_DIR/pinta-upstream-widget-tree.txt" \
	"$ROOT_DIR/pinta-upstream-capture.log" \
	"$ROOT_DIR/pinta-rs-capture.log"

mkdir -p "$DIAGNOSTICS_ROOT"
mkdir -p "$MOCK_DIAGNOSTICS_ROOT"

if [[ ! -f "$SAMPLE_INPUT" ]]; then
	python3 - <<'PY'
from PIL import Image, ImageDraw

image = Image.new('RGBA', (800, 600), (255, 255, 255, 255))
draw = ImageDraw.Draw(image)
draw.ellipse((80, 70, 240, 230), fill=(231, 74, 59, 255))
draw.rectangle((290, 70, 560, 210), fill=(96, 148, 76, 255))
draw.line((70, 440, 660, 320), fill=(36, 36, 40, 255), width=8)
draw.line((80, 300, 160, 220, 260, 190, 360, 220, 470, 360, 580, 390, 700, 230), fill=(67, 109, 195, 255), width=8)
image.save('/home/terry/Documents/GIT/varcop/sample-input.png')
PY
fi

capture_upstream() {
	local before session_dir main_window snapshot upstream_log
	before="$(find "$DIAGNOSTICS_ROOT" -mindepth 1 -maxdepth 1 -type d | sort)"
	upstream_log="$(mktemp)"

	pushd "$PINTA_UPSTREAM_DIR" >/dev/null
	PINTA_DIAGNOSTICS_DIR="$DIAGNOSTICS_ROOT" dotnet run --project Pinta -- --debug "$SAMPLE_INPUT" >"$upstream_log" 2>&1 &
	local upstream_pid=$!
	popd >/dev/null

	cleanup_upstream() {
		kill "$upstream_pid" 2>/dev/null || true
		wait "$upstream_pid" 2>/dev/null || true
		rm -f "$upstream_log"
	}

	trap cleanup_upstream RETURN

	for _ in $(seq 1 180); do
		session_dir="$(find "$DIAGNOSTICS_ROOT" -mindepth 1 -maxdepth 1 -type d | sort | tail -n1)"
		if [[ -n "$session_dir" && -d "$session_dir" && "$(printf '%s' "$before" | rg -F --line-regexp "$session_dir" -c || true)" == "0" ]]; then
			main_window="$(find "$session_dir" -maxdepth 1 -name 'capture-*-main-window-spectacle.png' | sort | tail -n1)"
			snapshot="$session_dir/snapshot-003-post-activate.txt"
			if [[ -f "$main_window" && -f "$snapshot" ]]; then
				local crop_count
				crop_count="$(find "$session_dir" -maxdepth 1 -name 'capture-*-*-crop.png' | wc -l | tr -d ' ')"
				if [[ "$crop_count" -ge 28 ]]; then
					break
				fi
			fi
		fi

		python3 - <<'PY'
import time
time.sleep(0.2)
PY
	done

	if [[ -z "${session_dir:-}" || ! -d "$session_dir" ]]; then
		echo "failed to locate upstream diagnostics session" >&2
		return 1
	fi

	main_window="$(find "$session_dir" -maxdepth 1 -name 'capture-*-main-window-spectacle.png' | sort | tail -n1)"
	if [[ -z "$main_window" || ! -f "$main_window" ]]; then
		echo "failed to capture upstream main window" >&2
		return 1
	fi

	cp "$main_window" "$UPSTREAM_WINDOW"
	rm -rf "$REFLECTION_DIR"
	cp -a "$session_dir" "$REFLECTION_DIR"

	printf '%s\n' "$session_dir"

	trap - RETURN
	cleanup_upstream
}

capture_mock() {
	local upstream_session_dir="$1"
	local before session_dir mock_log ready_path
	before="$(find "$MOCK_DIAGNOSTICS_ROOT" -mindepth 1 -maxdepth 1 -type d | sort)"
	mock_log="$(mktemp)"
	ready_path="$MOCK_WINDOW.ready"

	pushd "$PINTA_RS_DIR" >/dev/null
	PINTA_MOCK_CAPTURE_PATH="$MOCK_WINDOW" \
	PINTA_MOCK_DIAGNOSTICS_DIR="$MOCK_DIAGNOSTICS_ROOT" \
	PINTA_MOCK_UPSTREAM_SESSION_DIR="$upstream_session_dir" \
	cargo run >"$mock_log" 2>&1 &
	local mock_pid=$!
	popd >/dev/null

	cleanup_mock() {
		kill "$mock_pid" 2>/dev/null || true
		wait "$mock_pid" 2>/dev/null || true
		rm -f "$mock_log"
	}

	trap cleanup_mock RETURN

	rm -f "$MOCK_WINDOW"
	rm -f "$ready_path"

	for _ in $(seq 1 180); do
		if [[ -f "$MOCK_WINDOW" && -f "$ready_path" ]]; then
			rm -f "$ready_path"
			break
		fi

		python3 - <<'PY'
import time
time.sleep(0.2)
PY
	done

	if [[ ! -f "$MOCK_WINDOW" ]]; then
		echo "failed to capture pinta-rs full window" >&2
		return 1
	fi

	for _ in $(seq 1 180); do
		session_dir="$(find "$MOCK_DIAGNOSTICS_ROOT" -mindepth 1 -maxdepth 1 -type d | sort | tail -n1)"
		if [[ -n "$session_dir" && -d "$session_dir" && "$(printf '%s' "$before" | rg -F --line-regexp "$session_dir" -c || true)" == "0" ]]; then
			local crop_count
			crop_count="$(find "$session_dir" -maxdepth 1 -name 'capture-*-*-crop.png' | wc -l | tr -d ' ')"
			if [[ -f "$session_dir/snapshot-003-post-activate.txt" && "$crop_count" -ge 28 ]]; then
				break
			fi
		fi

		python3 - <<'PY'
import time
time.sleep(0.2)
PY
	done

	if [[ -z "${session_dir:-}" || ! -d "$session_dir" ]]; then
		echo "failed to locate mock diagnostics session" >&2
		return 1
	fi

	trap - RETURN
	cleanup_mock
	printf '%s\n' "$session_dir"
}

UPSTREAM_SESSION_DIR="$(capture_upstream)"
MOCK_SESSION_DIR="$(capture_mock "$UPSTREAM_SESSION_DIR")"

python3 "$PINTA_RS_DIR/tools/build_ui_comparisons.py" \
	--workspace-root "$ROOT_DIR" \
	--upstream-window "$UPSTREAM_WINDOW" \
	--mock-window "$MOCK_WINDOW" \
	--upstream-session-dir "$UPSTREAM_SESSION_DIR" \
	--mock-session-dir "$MOCK_SESSION_DIR"

printf 'upstream_window=%s\n' "$UPSTREAM_WINDOW"
printf 'mock_window=%s\n' "$MOCK_WINDOW"
printf 'window_side_by_side=%s\n' "$WINDOW_SIDE_BY_SIDE"
printf 'window_diff=%s\n' "$WINDOW_DIFF"
printf 'window_metric=%s\n' "$WINDOW_METRIC"
printf 'reflection_dir=%s\n' "$REFLECTION_DIR"
printf 'controls_dir=%s\n' "$ROOT_DIR/ui-control-comparisons"
printf 'mock_diagnostics_dir=%s\n' "$MOCK_SESSION_DIR"