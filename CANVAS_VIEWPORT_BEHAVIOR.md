# Canvas Viewport Behavior

This note defines the intended UI/UX contract for the Pinta canvas area in `pinta-rs`.

## Terms

- Application window: the full top-level window, including title bar, toolbar, toolbox, side panels, canvas workspace, and status bar.
- Workspace: the center region between the left toolbox and right sidebar where the canvas viewport is shown.
- Viewport: the visible area available for drawing the image canvas inside the workspace.
- Canvas: the image surface itself. For the current mock document, this is `800x600` logical pixels.
- Zoom: the user-facing zoom value shown in the status bar.
- Scale: the actual display multiplier applied to the `800x600` canvas inside the viewport.

## Window And Viewport Rules

- The workspace viewport is derived from the application window after subtracting the fixed shell chrome: top bar, tool options bar, left toolbox, right sidebar, and footer.
- The application window should stop shrinking once the shell chrome and the minimum usable canvas viewport would no longer present correctly.
- The canvas should remain centered inside the viewport.
- At `100%` zoom, the full `800x600` canvas must always be visible.

## Zoom And Scale Rules

- `100%` zoom means: display the canvas at `1:1` when the viewport is large enough.
- If the viewport is smaller than `800x600`, `100%` zoom should scale the canvas down just enough to fit fully inside the viewport while preserving aspect ratio.
- `100%` zoom must never scale the canvas larger than `1:1`.
- Zoom values above `100%` may scale the canvas larger than the viewport and allow natural cropping by the viewport.
- Zoom values below `100%` scale the canvas down proportionally.

In shorthand:

- `100%` = `min(1.0, fit-to-viewport)`
- `>100%` = `fit baseline * zoom multiplier`, with overflow allowed
- `<100%` = `fit baseline * zoom multiplier`

## Margin Rules

- When the canvas is smaller than the viewport, the empty space around it should be visually centered.
- Left and right margins should be equal.
- Top and bottom margins should be equal.
- Margin symmetry should not be biased by whole-pixel rounding.
- Any visible outer margin belongs to the viewport/workspace, not to the image canvas.

## Border Indicator Rules

- The canvas border is the visual indicator of the `800x600` image bounds.
- At true `1:1`, the border indicates the exact image extent.
- When the canvas is scaled down to fit below `1:1`, the border still indicates the scaled image extent.
- When zoom is above `1:1` and the canvas overflows the viewport, the border may be partially clipped by the viewport.
- The region outside the border but inside the workspace should act as the workspace margin area.

## Aspect Ratio Rules

- The canvas must preserve the source aspect ratio of `800:600`.
- Resizing the application window must not distort the image.
- Any fit behavior should be uniform in both axes and derived from a single aspect-preserving scale factor.

## Parity Goal

- Upstream-like behavior is:
  - narrow border/margin treatment around the image,
  - centered canvas in the workspace,
  - `100%` behaving as `1:1 unless the viewport is smaller`,
  - full image visibility at `100%`,
  - scaling down on smaller windows without distortion.

## Recommended Control Split

- Keep a single shared `pinta-rs` theme for colors, spacing, border widths, and sizing tokens.
- Do not combine theme, workspace layout, viewport behavior, and canvas rendering into one monolithic control.

The recommended split is:

- Workspace layout control:
  owns shell geometry such as top bars, left toolbox, right sidebar, footer, and the center workspace allocation.
- Viewport control:
  owns fit-to-viewport math, `100%` semantics, aspect-ratio preservation, centering, margins, border visibility, and clipping behavior.
- Canvas/document renderer:
  draws the image, layers, and tool overlays inside the rectangle chosen by the viewport control.
- Theme tokens:
  define only presentation values, not viewport behavior.

In practical terms:

- the workspace decides how much room is available,
- the viewport decides how the `800x600` canvas sits in that room,
- the renderer draws into the canvas rectangle it is given,
- the theme supplies styling for all three.

This split keeps parity work local:

- shell parity changes stay in workspace layout,
- zoom/fit parity changes stay in the viewport,
- drawing parity changes stay in the renderer,
- color and spacing changes stay in the theme.