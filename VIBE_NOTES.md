# VIBE Notes

This file exists to resume the VIBE-coded Rust port mock of Pinta without needing to reconstruct the whole session from git history.

## Repository Roles

- `pinta-rs/`: Rust + Iced parity workspace.
- `../pinta-upstream/`: local upstream Pinta checkout used as the behavior and screenshot reference.
- `../sample-input.png`: sample image used to drive upstream captures.

## Current Baseline

- Workspace version: `0.1.5`
- Branch: `main`
- Upstream reference capture session: `../upstream-diagnostics-output/20260420-144419/`
- Main upstream screenshot: `../pinta-upstream-window.png`
- Latest retained mock screenshot before this note: `captures/pinta-rs-20260419-203253.png`
- Latest compare bundle before this note: `compares/20260419-203259/`
- Latest measured RMSE before this note: `48.3335`
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
- Upstream diagnostics use embedded reflection-like inspection hooks plus `spectacle`/Python helpers for external window capture and control cropping.
- Release artifacts are staged locally under `releases/` only for publishing and are ignored by git; canonical downloads live in GitHub Releases.
- Mock screenshots are now captured internally via Iced `window::screenshot()` and written atomically by the app; this avoids active-window focus drift from `spectacle --activewindow`.
- The compare script now crops external window screenshots down to client area before scoring, so the new internal mock captures are comparable again.
- Canonical parity artifacts now belong in the workspace root, not inside `pinta-rs/` or `pinta-upstream/`.

## Where The Upstream Hooks Live

- `../pinta-upstream/Pinta/Main.cs`: diagnostics initialization and scheduling.
- `../pinta-upstream/Pinta/Diagnostics.cs`: session logs, widget tree dumps, window screenshots, bounds capture, and canvas crop generation.
- `../pinta-upstream/Pinta.Core/Extensions/Cairo/CairoExtensions.Samples.cs`: pixbuf-based fallback to avoid the local missing GDK symbol path.

## Reflection Inspection Workflow

The look-and-feel clone workflow is now based on embedded, reflection-like inspection in both applications rather than ad hoc repo-local captures.

1. Launch upstream with diagnostics enabled and write its session to the workspace root `../upstream-diagnostics-output/`.
2. Let the upstream inspection hooks dump widget-tree snapshots, bounds files, the main window screenshot, and one cropped PNG per UI control widget.
3. Copy the latest upstream session into `../pinta-upstream-reflection/` as the stable reflection snapshot used for parity review.
4. Launch `pinta-rs` with `PINTA_MOCK_CAPTURE_PATH`, `PINTA_MOCK_DIAGNOSTICS_DIR`, and `PINTA_MOCK_UPSTREAM_SESSION_DIR` pointing at workspace-root outputs.
5. Let the Rust mock save its main window screenshot to `../pinta-rs-window.png` and export its own per-control crops into `../pinta-rs-diagnostics-output/<session>/`.
6. Rebuild `../ui-control-comparisons/` so each control has an upstream image, a mock image, a side-by-side diff, a per-control RMSE, and a full-window layout diff.

The point of the inspection pass is to evaluate each UI control widget independently while still preserving one full application-window screenshot for layout-level diffs.

## Artifact Placement Policy

- Upstream diagnostic sessions go to `../upstream-diagnostics-output/`.
- Mock diagnostic sessions go to `../pinta-rs-diagnostics-output/`.
- Stable upstream reflection snapshots go to `../pinta-upstream-reflection/`.
- Full-window screenshots and layout diffs stay in the workspace root:
  - `../pinta-upstream-window.png`
  - `../pinta-rs-window.png`
  - `../pinta-window-side-by-side.png`
  - `../pinta-window-diff.png`
  - `../pinta-window-metric.txt`
- Per-control compare artifacts stay under `../ui-control-comparisons/`.

Do not write logs, screenshots, crops, or compare outputs under `pinta-rs/` or `pinta-upstream/` as part of the diagnostics flow. Those repositories should stay focused on source, not generated inspection artifacts.

## Tooling Status And Cleanup Rule

- Upstream already honors `PINTA_DIAGNOSTICS_DIR`, so its diagnostic session can be redirected cleanly to the workspace root.
- The parity bundle script already uses workspace-root destinations for upstream sessions, mock sessions, reflection copies, control comparisons, and full-window diffs.
- The Rust mock diagnostics exporter already writes session artifacts to the external path supplied by `PINTA_MOCK_DIAGNOSTICS_DIR` and mirrors upstream control file naming closely enough for automated compare generation.
- Keep diagnostics opt-in. Normal `cargo run` and normal upstream builds should not emit screenshots or logs unless the explicit diagnostics environment variables are set.
- If any helper starts defaulting back to repo-local output, treat that as drift and fix the pathing before trusting new compare results.

## Current Visual Tuning Direction

- The stronger baseline is the earlier pad/canvas/shell balance associated with the lower RMSE values.
- Recent passes kept the improved toolbox glyph work from `crates/pinta-ui/src/widgets/icon.rs` while restoring more of the earlier shell geometry.
- The latest retained pass adjusted:
  - tighter top, options, dock, and footer heights,
  - narrower side gutter padding around the main content,
  - slightly denser tool option controls,
  - narrower right dock width and more compact layer/history row content,
  - tighter toolbox row density and client-area compare normalization,
  - previously retained toolbox and toolbar parity fixes.

## Next Resume Steps

1. Use `./tools/capture_parity_bundle.sh` as the default parity pass so upstream reflection, mock diagnostics, control crops, and full-window diffs refresh together.
2. Freeze the current shell except for toolbox vertical spacing and icon scale.
3. Review `../ui-control-comparisons/summary.tsv` first, then inspect the worst individual control diffs before changing layout tokens.
4. If RMSE improves further, refresh `docs/readme/pinta-rs-current.png` and consider a follow-up tag later.
5. If RMSE regresses, keep the externalized diagnostics layout and back out only the visual tuning pass that caused the regression.
6. If mock capture becomes inconsistent again, debug the internal screenshot/export path before trusting compare metrics.
7. After visual tuning, move from parity mock work toward real editor behaviors one surface at a time.

## Quick Commands

```bash
cargo check
cargo run
./tools/capture_parity_bundle.sh
```