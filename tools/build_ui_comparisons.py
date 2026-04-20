#!/usr/bin/env python3
import argparse
import math
import shutil
from pathlib import Path

from PIL import Image, ImageChops, ImageOps

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
    left = Image.open(left_path).convert("RGBA")
    right = Image.open(right_path).convert("RGBA")

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
        export_named_capture(upstream_session_dir, pattern, upstream_dir / f"{name}.png")
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