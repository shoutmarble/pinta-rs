# VIBE Notes

This file exists to resume the VIBE-coded Rust port mock of Pinta without needing to reconstruct the whole session from git history.

## Repository Roles

- `pinta-rs/`: Rust + Iced parity workspace.
- `../pinta-upstream/`: local upstream Pinta checkout used as the behavior and screenshot reference.
- `../sample-input.png`: sample image used to drive upstream captures.

## Current Baseline

- Workspace version: `0.1.2`
- Branch: `main`
- Upstream reference capture session: `../pinta-upstream/diagnostics/20260419-230608/`
- Main upstream screenshot: `../pinta-upstream/diagnostics/20260419-230608/capture-004-main-window-spectacle.png`
- Latest retained mock screenshot before this note: `captures/pinta-rs-20260419-200348.png`
- Latest compare bundle before this note: `compares/20260419-200350/`
- Latest measured RMSE before this note: `49.5944`
- Best earlier RMSE mentioned in-session: `49.2716`

## Architecture Snapshot

- `crates/pinta-app`: top-level mock application, shell composition, and view state.
- `crates/pinta-theme`: shared sizing, spacing, colors, radii, and typography tokens.
- `crates/pinta-ui`: reusable custom widgets including toolbox, pads, status bar, icons, and canvas viewport.
- `tools/capture_mock.sh`: launches the app and saves a screenshot.
- `tools/compare_with_upstream.sh`: normalizes mock and upstream captures, then produces side-by-side, diff, and RMSE artifacts.

## Local Environment Notes

- On this machine, `cargo run` is stable with the workspace-local `WGPU_BACKEND=gl` configuration.
- `.NET 10` is installed and `dotnet build Pinta.sln` succeeds in `../pinta-upstream`.
- Upstream diagnostics use `spectacle` plus Python/Pillow for external window capture and canvas cropping.
- Release artifacts are staged locally under `releases/` only for publishing and are ignored by git; canonical downloads live in GitHub Releases.

## Where The Upstream Hooks Live

- `../pinta-upstream/Pinta/Main.cs`: diagnostics initialization and scheduling.
- `../pinta-upstream/Pinta/Diagnostics.cs`: session logs, widget tree dumps, window screenshots, bounds capture, and canvas crop generation.
- `../pinta-upstream/Pinta.Core/Extensions/Cairo/CairoExtensions.Samples.cs`: pixbuf-based fallback to avoid the local missing GDK symbol path.

## Current Visual Tuning Direction

- The stronger baseline is the earlier pad/canvas/shell balance associated with the lower RMSE values.
- Recent passes kept the improved toolbox glyph work from `crates/pinta-ui/src/widgets/icon.rs` while restoring more of the earlier shell geometry.
- The latest retained pass adjusted:
  - toolbox width and button size,
  - toolbox icon scale and row spacing,
  - unselected toolbox button chrome,
  - toolbar icon sizing and muted opacity,
  - toolbar helper centering so header icons no longer expand incorrectly.

## Next Resume Steps

1. Freeze the current shell except for toolbox vertical spacing and icon scale.
2. Re-run `./tools/capture_mock.sh` and `./tools/compare_with_upstream.sh` against the upstream reference screenshot.
3. If RMSE improves further, refresh `docs/readme/pinta-rs-current.png` and consider a follow-up tag later.
4. If RMSE regresses, keep the helper-level toolbar fix and back out only broader header grouping experiments.
5. After visual tuning, move from parity mock work toward real editor behaviors one surface at a time.

## Quick Commands

```bash
cargo check
cargo run
./tools/capture_mock.sh
./tools/compare_with_upstream.sh \
  ../pinta-upstream/diagnostics/20260419-230608/capture-004-main-window-spectacle.png \
  captures/pinta-rs-20260419-200348.png
```