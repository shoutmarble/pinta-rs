#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
UPSTREAM_DEFAULT="$(find "$ROOT_DIR/../pinta-upstream/diagnostics" -path '*/capture-004-main-window-spectacle.png' | sort | tail -n1)"
MOCK_DEFAULT="$(find "$ROOT_DIR/captures" -maxdepth 1 -name 'pinta-rs-*.png' | sort | tail -n1)"

UPSTREAM_PATH="${1:-$UPSTREAM_DEFAULT}"
MOCK_PATH="${2:-$MOCK_DEFAULT}"
OUT_DIR="${3:-$ROOT_DIR/compares/$(date +%Y%m%d-%H%M%S)}"

if [[ -z "$UPSTREAM_PATH" || ! -f "$UPSTREAM_PATH" ]]; then
	echo "upstream screenshot not found" >&2
	exit 1
fi

if [[ -z "$MOCK_PATH" || ! -f "$MOCK_PATH" ]]; then
	echo "mock screenshot not found" >&2
	exit 1
fi

mkdir -p "$OUT_DIR"

UPSTREAM_NORM="$OUT_DIR/upstream-normalized.png"
MOCK_NORM="$OUT_DIR/mock-normalized.png"
SIDE_BY_SIDE="$OUT_DIR/side-by-side.png"
DIFF_IMAGE="$OUT_DIR/diff.png"
METRIC_FILE="$OUT_DIR/diff-metric.txt"

python3 - "$UPSTREAM_PATH" "$MOCK_PATH" "$UPSTREAM_NORM" "$MOCK_NORM" "$SIDE_BY_SIDE" "$DIFF_IMAGE" "$METRIC_FILE" <<'PY'
import math
import sys

from PIL import Image, ImageChops, ImageOps

upstream_path, mock_path, upstream_norm, mock_norm, side_by_side, diff_image, metric_file = sys.argv[1:]

def content_crop(image):
	rgb = image.convert("RGB")
	mask = Image.eval(ImageOps.grayscale(rgb), lambda value: 255 if value > 10 else 0)
	bbox = mask.getbbox()
	if bbox is None:
		return image

	image = image.crop(bbox)

	inset = min(6, max(1, min(image.width, image.height) // 80))
	if image.width > inset * 2 and image.height > inset * 2:
		image = image.crop((inset, inset, image.width - inset, image.height - inset))

	rgb = image.convert("RGB")
	first_row = [rgb.getpixel((x, 0)) for x in range(rgb.width)]
	means = [sum(channel) / len(first_row) for channel in zip(*first_row)]
	avg_luma = sum(means) / 3.0

	if 170.0 <= avg_luma <= 235.0:
		trim = 0
		consecutive = 0

		for y in range(1, min(rgb.height, 96)):
			row = [rgb.getpixel((x, y)) for x in range(rgb.width)]
			row_means = [sum(channel) / len(row) for channel in zip(*row)]
			distance = math.sqrt(sum((row_means[i] - means[i]) ** 2 for i in range(3)))

			if distance > 10.0:
				consecutive += 1
				if consecutive >= 6:
					trim = max(0, y - consecutive + 1)
					break
			else:
				consecutive = 0

		if trim > 18 and trim < image.height // 5:
			image = image.crop((0, trim, image.width, image.height))

	return image

upstream = content_crop(Image.open(upstream_path).convert("RGBA"))
mock = content_crop(Image.open(mock_path).convert("RGBA"))
mock = mock.resize(upstream.size, Image.Resampling.LANCZOS)
upstream_rgb = upstream.convert("RGB")
mock_rgb = mock.convert("RGB")

upstream.save(upstream_norm)
mock.save(mock_norm)

comparison = Image.new("RGBA", (upstream.width * 2, upstream.height))
comparison.paste(upstream, (0, 0))
comparison.paste(mock, (upstream.width, 0))
comparison.save(side_by_side)

diff = ImageOps.autocontrast(ImageChops.difference(upstream_rgb, mock_rgb))
diff.save(diff_image)

bands = diff.split()[:3]
sq_error = 0.0
for band in bands:
    hist = band.histogram()
    sq_error += sum(count * (value ** 2) for value, count in enumerate(hist))

rmse = math.sqrt(sq_error / (upstream.width * upstream.height * max(len(bands), 1)))
with open(metric_file, "w", encoding="utf-8") as handle:
    handle.write(f"RMSE={rmse:.4f}\n")
PY

printf '%s\n' "$OUT_DIR"