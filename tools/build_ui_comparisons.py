#!/usr/bin/env python3
import argparse
import math
import shutil
from pathlib import Path

from PIL import Image, ImageChops, ImageOps

CLIENT_CROP = {
    "left": 132 / 2460,
    "right": 128 / 2460,
    "top": 180 / 1840,
    "bottom": 160 / 1840,
}

TOOLBOX_ORDER = [
    ("tool-move-selected-pixels", 0, 0),
    ("tool-move-selection", 0, 1),
    ("tool-zoom", 1, 0),
    ("tool-pan", 1, 1),
    ("tool-rectangle-select", 2, 0),
    ("tool-ellipse-select", 2, 1),
    ("tool-lasso-select", 3, 0),
    ("tool-magic-wand-select", 3, 1),
    ("tool-paintbrush", 4, 0),
    ("tool-pencil", 4, 1),
    ("tool-eraser", 5, 0),
    ("tool-paint-bucket", 5, 1),
    ("tool-gradient", 6, 0),
    ("tool-color-picker", 6, 1),
    ("tool-text", 7, 0),
    ("tool-line-curve", 7, 1),
    ("tool-rectangle", 8, 0),
    ("tool-rounded-rectangle", 8, 1),
    ("tool-ellipse", 9, 0),
    ("tool-freeform-shape", 9, 1),
    ("tool-clone-stamp", 10, 0),
    ("tool-recolor", 10, 1),
]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--workspace-root", required=True)
    parser.add_argument("--upstream-window", required=True)
    parser.add_argument("--mock-window", required=True)
    parser.add_argument("--upstream-session-dir", required=True)
    parser.add_argument("--mock-session-dir", required=True)
    return parser.parse_args()


def compare_images(left_path: Path, right_path: Path, side_by_side_path: Path, diff_path: Path) -> float:
    left = flatten_rgba(Image.open(left_path).convert("RGBA"))
    right = flatten_rgba(Image.open(right_path).convert("RGBA"))

    if right.size != left.size:
        right = right.resize(left.size, Image.Resampling.LANCZOS)

    left_rgb = left.convert("RGB")
    right_rgb = right.convert("RGB")

    comparison = Image.new("RGBA", (left.width * 2, left.height))
    comparison.paste(left, (0, 0))
    comparison.paste(right, (left.width, 0))
    comparison.save(side_by_side_path)

    diff = ImageOps.autocontrast(ImageChops.difference(left_rgb, right_rgb))
    diff.save(diff_path)

    bands = diff.split()[:3]
    sq_error = 0.0
    for band in bands:
        hist = band.histogram()
        sq_error += sum(count * (value ** 2) for value, count in enumerate(hist))

    return math.sqrt(sq_error / (left.width * left.height * max(len(bands), 1)))


def flatten_rgba(image: Image.Image) -> Image.Image:
    if image.mode != "RGBA":
        image = image.convert("RGBA")

    background = Image.new("RGBA", image.size, (255, 255, 255, 255))
    return Image.alpha_composite(background, image)


def crop_window_chrome(image: Image.Image) -> Image.Image:
    left = int(round(image.width * CLIENT_CROP["left"]))
    right = int(round(image.width * CLIENT_CROP["right"]))
    top = int(round(image.height * CLIENT_CROP["top"]))
    bottom = int(round(image.height * CLIENT_CROP["bottom"]))

    if left + right >= image.width or top + bottom >= image.height:
        return image

    return image.crop((left, top, image.width - right, image.height - bottom))


def parse_bounds(path: Path) -> dict[str, int] | None:
    if not path.exists():
        return None

    values: dict[str, int] = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        if "=" not in line:
            continue

        key, value = line.split("=", 1)
        try:
            values[key] = int(value)
        except ValueError:
            return None

    required = {"x", "y", "width", "height", "window-width", "window-height"}
    if not required.issubset(values):
        return None

    return values


def export_bounds_crop(
    window_path: Path,
    bounds_path: Path,
    output_path: Path,
    *,
    trim_window: bool,
) -> bool:
    bounds = parse_bounds(bounds_path)
    if bounds is None or not window_path.exists():
        return False

    image = Image.open(window_path).convert("RGBA")
    if trim_window:
        image = crop_window_chrome(image)

    scale_x = image.width / bounds["window-width"]
    scale_y = image.height / bounds["window-height"]

    left = int(round(bounds["x"] * scale_x))
    top = int(round(bounds["y"] * scale_y))
    right = int(round((bounds["x"] + bounds["width"]) * scale_x))
    bottom = int(round((bounds["y"] + bounds["height"]) * scale_y))

    left = max(0, min(left, image.width))
    top = max(0, min(top, image.height))
    right = max(left, min(right, image.width))
    bottom = max(top, min(bottom, image.height))

    if left == right or top == bottom:
        return False

    output_path.parent.mkdir(parents=True, exist_ok=True)
    image.crop((left, top, right, bottom)).save(output_path)
    return True


def latest_match(directory: Path, pattern: str) -> Path | None:
    matches = sorted(directory.glob(pattern))
    return matches[-1] if matches else None


def stable_copy(source: Path | None, target: Path) -> bool:
    if source is None or not source.exists():
        return False
    target.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(source, target)
    return True


def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def export_named_capture(session_dir: Path, pattern: str, output_path: Path) -> bool:
    return stable_copy(latest_match(session_dir, pattern), output_path)


def main() -> int:
    args = parse_args()

    workspace_root = Path(args.workspace_root)
    upstream_window = Path(args.upstream_window)
    mock_window = Path(args.mock_window)
    upstream_session_dir = Path(args.upstream_session_dir)
    mock_session_dir = Path(args.mock_session_dir)

    control_root = workspace_root / "ui-control-comparisons"
    upstream_dir = control_root / "upstream"
    mock_dir = control_root / "mock"
    compare_dir = control_root / "compare"

    for directory in (upstream_dir, mock_dir, compare_dir):
        if directory.exists():
            shutil.rmtree(directory)
        directory.mkdir(parents=True, exist_ok=True)

    summary_lines: list[str] = []

    window_side = workspace_root / "pinta-window-side-by-side.png"
    window_diff = workspace_root / "pinta-window-diff.png"
    window_rmse = compare_images(upstream_window, mock_window, window_side, window_diff)
    write_text(workspace_root / "pinta-window-metric.txt", f"RMSE={window_rmse:.4f}\n")
    summary_lines.append(f"window\t{window_rmse:.4f}")

    major_patterns = {
        "tool-toolbar": "capture-*-tool-toolbar-crop.png",
        "workspace-layout": "capture-*-workspace-layout-crop.png",
        "toolbox": "capture-*-toolbox-crop.png",
        "canvas": "capture-*-canvas-crop.png",
        "layers-list": "capture-*-layers-list-crop.png",
        "history-list": "capture-*-history-list-crop.png",
        "statusbar": "capture-*-statusbar-crop.png",
    }

    for name, pattern in major_patterns.items():
        upstream_exported = export_bounds_crop(
            upstream_window,
            upstream_session_dir / f"bounds-{name}.txt",
            upstream_dir / f"{name}.png",
            trim_window=True,
        )
        if not upstream_exported:
            export_named_capture(upstream_session_dir, pattern, upstream_dir / f"{name}.png")

        mock_exported = export_bounds_crop(
            mock_window,
            mock_session_dir / f"bounds-{name}.txt",
            mock_dir / f"{name}.png",
            trim_window=False,
        )
        if not mock_exported:
            export_named_capture(mock_session_dir, pattern, mock_dir / f"{name}.png")

    for name, row, column in TOOLBOX_ORDER:
        export_named_capture(upstream_session_dir, f"capture-*-{name}-crop.png", upstream_dir / f"{name}.png")
        export_named_capture(mock_session_dir, f"capture-*-{name}-crop.png", mock_dir / f"{name}.png")

    compare_names = sorted({
        path.stem for path in upstream_dir.glob("*.png")
    } & {
        path.stem for path in mock_dir.glob("*.png")
    })

    for name in compare_names:
        rmse = compare_images(
            upstream_dir / f"{name}.png",
            mock_dir / f"{name}.png",
            compare_dir / f"{name}-side-by-side.png",
            compare_dir / f"{name}-diff.png",
        )
        write_text(compare_dir / f"{name}-metric.txt", f"RMSE={rmse:.4f}\n")
        summary_lines.append(f"{name}\t{rmse:.4f}")

    write_text(control_root / "summary.tsv", "name\trmse\n" + "\n".join(summary_lines) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())