use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use image::{ImageFormat, RgbaImage, imageops};

use crate::state::AppState;
use crate::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};

#[derive(Debug, Clone)]
pub struct CaptureRequest {
    pub output_path: Option<String>,
    pub diagnostics_root: Option<String>,
    pub upstream_session_dir: Option<String>,
}

impl CaptureRequest {
    pub fn is_empty(&self) -> bool {
        self.output_path.is_none() && self.diagnostics_root.is_none()
    }
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Rect {
    fn crop(&self, image: &RgbaImage) -> RgbaImage {
        let max_width = image.width().saturating_sub(self.x);
        let max_height = image.height().saturating_sub(self.y);
        let width = self.width.min(max_width).max(1);
        let height = self.height.min(max_height).max(1);

        imageops::crop_imm(image, self.x, self.y, width, height).to_image()
    }
}

#[derive(Debug, Clone)]
struct NamedRect {
    name: &'static str,
    rect: Rect,
}

#[derive(Debug, Clone)]
struct RuntimeLayout {
    window_width: u32,
    window_height: u32,
    rects: Vec<NamedRect>,
}

pub fn save_artifacts(
    request: &CaptureRequest,
    state: &AppState,
    screenshot_rgba: &[u8],
    width: u32,
    height: u32,
) {
    let Some(raw_image) = RgbaImage::from_raw(width, height, screenshot_rgba.to_vec()) else {
        eprintln!("capture failed: screenshot bytes did not match image dimensions");
        return;
    };

    let image = normalize_capture_image(raw_image);

    if let Some(output_path) = &request.output_path {
        if let Err(error) = write_image_with_ready(&image, Path::new(output_path)) {
            eprintln!("capture failed: could not write {}: {error}", output_path);
        }
    }

    if let Some(diagnostics_root) = &request.diagnostics_root {
        if let Err(error) = export_runtime_diagnostics(
            Path::new(diagnostics_root),
            request.upstream_session_dir.as_deref().map(Path::new),
            state,
            &image,
        ) {
            eprintln!("mock diagnostics export failed: {error}");
        }
    }
}

fn normalize_capture_image(image: RgbaImage) -> RgbaImage {
    let scale_x = image.width() as f32 / DEFAULT_WINDOW_WIDTH;
    let scale_y = image.height() as f32 / DEFAULT_WINDOW_HEIGHT;
    let rounded_scale = ((scale_x + scale_y) * 0.5).round();

    if rounded_scale <= 1.0 {
        return image;
    }

    if (scale_x - rounded_scale).abs() > 0.1 || (scale_y - rounded_scale).abs() > 0.1 {
        return image;
    }

    let target_width = (image.width() as f32 / rounded_scale).round().max(1.0) as u32;
    let target_height = (image.height() as f32 / rounded_scale).round().max(1.0) as u32;

    imageops::resize(&image, target_width, target_height, imageops::FilterType::Lanczos3)
}

fn write_image_with_ready(image: &RgbaImage, output_path: &Path) -> Result<(), String> {
    let temp_path =
        output_path.with_extension(match output_path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => format!("{ext}.tmp"),
            None => "tmp".to_string(),
        });
    let ready_path =
        output_path.with_extension(match output_path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => format!("{ext}.ready"),
            None => "ready".to_string(),
        });

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }

    image
        .save_with_format(&temp_path, ImageFormat::Png)
        .map_err(|error| error.to_string())?;
    fs::rename(&temp_path, output_path).map_err(|error| error.to_string())?;
    fs::write(&ready_path, b"ok").map_err(|error| error.to_string())?;
    Ok(())
}

fn export_runtime_diagnostics(
    diagnostics_root: &Path,
    upstream_session_dir: Option<&Path>,
    state: &AppState,
    image: &RgbaImage,
) -> Result<(), String> {
    fs::create_dir_all(diagnostics_root).map_err(|error| error.to_string())?;

    let session_name = upstream_session_dir
        .and_then(|path| path.file_name())
        .and_then(|name| name.to_str())
        .map(ToOwned::to_owned)
        .unwrap_or_else(timestamp_session_name);
    let session_dir = diagnostics_root.join(&session_name);

    if session_dir.exists() {
        fs::remove_dir_all(&session_dir).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(&session_dir).map_err(|error| error.to_string())?;

    let layout = runtime_layout(state, image.width(), image.height());
    write_capture_files(&session_dir, upstream_session_dir, &layout, image)?;
    write_bounds_files(&session_dir, upstream_session_dir, &layout)?;
    write_snapshot_files(&session_dir, upstream_session_dir, &layout)?;
    write_reflection_manifest(&session_dir, upstream_session_dir, &layout)?;
    write_session_log(&session_dir, upstream_session_dir, &layout)?;
    update_latest_symlink(diagnostics_root, &session_name)?;

    Ok(())
}

fn write_capture_files(
    session_dir: &Path,
    upstream_session_dir: Option<&Path>,
    layout: &RuntimeLayout,
    image: &RgbaImage,
) -> Result<(), String> {
    for (file_name, label) in capture_file_specs(upstream_session_dir) {
        let output_path = session_dir.join(file_name);

        if label == "main-window-internal" {
            image
                .save_with_format(&output_path, ImageFormat::Png)
                .map_err(|error| error.to_string())?;
            continue;
        }

        let Some(rect) = layout
            .rects
            .iter()
            .find(|entry| entry.name == label)
            .map(|entry| entry.rect)
        else {
            continue;
        };

        rect.crop(image)
            .save_with_format(&output_path, ImageFormat::Png)
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn write_reflection_manifest(
    session_dir: &Path,
    upstream_session_dir: Option<&Path>,
    layout: &RuntimeLayout,
) -> Result<(), String> {
    let source_session = upstream_session_dir
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "none".to_string());

    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"capture_source\": \"internal-window-screenshot\",\n");
    json.push_str(&format!(
        "  \"source_upstream_session\": \"{}\",\n",
        json_escape(&source_session)
    ));
    json.push_str(&format!("  \"window_width\": {},\n", layout.window_width));
    json.push_str(&format!("  \"window_height\": {},\n", layout.window_height));
    json.push_str("  \"controls\": [\n");

    for (index, entry) in layout.rects.iter().enumerate() {
        let suffix = if index + 1 == layout.rects.len() {
            ""
        } else {
            ","
        };
        json.push_str(&format!(
            "    {{ \"name\": \"{}\", \"x\": {}, \"y\": {}, \"width\": {}, \"height\": {} }}{}\n",
            entry.name, entry.rect.x, entry.rect.y, entry.rect.width, entry.rect.height, suffix,
        ));
    }

    json.push_str("  ]\n");
    json.push_str("}\n");

    fs::write(session_dir.join("reflection-controls.json"), json)
        .map_err(|error| error.to_string())?;

    let mut text = vec![
        "capture_source=internal-window-screenshot".to_string(),
        format!("source_upstream_session={source_session}"),
        format!("window_width={}", layout.window_width),
        format!("window_height={}", layout.window_height),
    ];

    for entry in &layout.rects {
        text.push(format!(
            "{} x={} y={} width={} height={}",
            entry.name, entry.rect.x, entry.rect.y, entry.rect.width, entry.rect.height,
        ));
    }

    fs::write(
        session_dir.join("reflection-controls.txt"),
        text.join("\n") + "\n",
    )
    .map_err(|error| error.to_string())?;

    Ok(())
}

fn write_bounds_files(
    session_dir: &Path,
    upstream_session_dir: Option<&Path>,
    layout: &RuntimeLayout,
) -> Result<(), String> {
    for (file_name, label) in bounds_file_specs(upstream_session_dir) {
        let Some(rect) = layout
            .rects
            .iter()
            .find(|entry| entry.name == label)
            .map(|entry| entry.rect)
        else {
            continue;
        };

        fs::write(
            session_dir.join(file_name),
            format!(
                "x={}\ny={}\nwidth={}\nheight={}\nwindow-width={}\nwindow-height={}\n",
                rect.x, rect.y, rect.width, rect.height, layout.window_width, layout.window_height,
            ),
        )
        .map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn write_snapshot_files(
    session_dir: &Path,
    upstream_session_dir: Option<&Path>,
    layout: &RuntimeLayout,
) -> Result<(), String> {
    let mut lines = vec![
        "snapshot=mock-runtime-layout".to_string(),
        format!(
            "Window logical-size={}x{}",
            layout.window_width, layout.window_height
        ),
    ];

    for entry in &layout.rects {
        lines.push(format!(
            "{} x={} y={} width={} height={}",
            entry.name, entry.rect.x, entry.rect.y, entry.rect.width, entry.rect.height,
        ));
    }

    let text = lines.join("\n") + "\n";
    for file_name in snapshot_file_names(upstream_session_dir) {
        fs::write(session_dir.join(file_name), &text).map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn write_session_log(
    session_dir: &Path,
    upstream_session_dir: Option<&Path>,
    layout: &RuntimeLayout,
) -> Result<(), String> {
    let capture_count = session_dir
        .read_dir()
        .map_err(|error| error.to_string())?
        .filter_map(Result::ok)
        .map(|entry| entry.file_name())
        .filter(|name| name.to_string_lossy().starts_with("capture-"))
        .count();

    let source_session = upstream_session_dir
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "none".to_string());
    let timestamp = unix_timestamp();

    fs::write(
        session_dir.join("session.log"),
        format!(
            "[{timestamp}] mock diagnostics exported: session-dir={}\n[{timestamp}] source upstream session: {}\n[{timestamp}] logical window size={}x{}\n[{timestamp}] captures exported={}\n",
            session_dir.display(),
            source_session,
            layout.window_width,
            layout.window_height,
            capture_count,
        ),
    )
    .map_err(|error| error.to_string())
}

fn update_latest_symlink(diagnostics_root: &Path, session_name: &str) -> Result<(), String> {
    let latest_path = diagnostics_root.join("latest");
    if latest_path.exists() || latest_path.is_symlink() {
        fs::remove_file(&latest_path).map_err(|error| error.to_string())?;
    }

    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(session_name, &latest_path)
            .map_err(|error| error.to_string())?;
    }

    #[cfg(not(unix))]
    {
        fs::write(&latest_path, session_name).map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn capture_file_specs(upstream_session_dir: Option<&Path>) -> Vec<(String, String)> {
    if let Some(upstream_session_dir) = upstream_session_dir {
        let mut upstream_specs = upstream_session_dir
            .read_dir()
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.ends_with("main-window-spectacle.png") {
                    return Some((
                        name.replace("main-window-spectacle", "main-window-internal"),
                        "main-window-internal".to_string(),
                    ));
                }

                if !name.ends_with("-crop.png") {
                    return None;
                }

                let label = name
                    .splitn(3, '-')
                    .nth(2)
                    .map(|rest| rest.trim_end_matches("-crop.png").to_string())?;
                Some((name, label))
            })
            .collect::<Vec<_>>();
        upstream_specs.sort_by(|left, right| left.0.cmp(&right.0));
        if !upstream_specs.is_empty() {
            return upstream_specs;
        }
    }

    let mut specs = vec![(
        "capture-004-main-window-internal.png".to_string(),
        "main-window-internal".to_string(),
    )];
    specs.extend(
        default_labels()
            .into_iter()
            .enumerate()
            .map(|(index, label)| {
                (
                    format!("capture-{:03}-{label}-crop.png", index + 5),
                    label.to_string(),
                )
            }),
    );
    specs
}

fn bounds_file_specs(upstream_session_dir: Option<&Path>) -> Vec<(String, String)> {
    if let Some(upstream_session_dir) = upstream_session_dir {
        let mut upstream_specs = upstream_session_dir
            .read_dir()
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let name = entry.file_name().to_string_lossy().to_string();
                if !name.starts_with("bounds-") || !name.ends_with(".txt") {
                    return None;
                }
                Some((
                    name.clone(),
                    name.trim_start_matches("bounds-")
                        .trim_end_matches(".txt")
                        .to_string(),
                ))
            })
            .collect::<Vec<_>>();
        upstream_specs.sort_by(|left, right| left.0.cmp(&right.0));
        if !upstream_specs.is_empty() {
            return upstream_specs;
        }
    }

    default_labels()
        .into_iter()
        .map(|label| (format!("bounds-{label}.txt"), label.to_string()))
        .collect()
}

fn snapshot_file_names(upstream_session_dir: Option<&Path>) -> Vec<String> {
    if let Some(upstream_session_dir) = upstream_session_dir {
        let mut snapshot_names = upstream_session_dir
            .read_dir()
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .map(|entry| entry.file_name().to_string_lossy().to_string())
            .filter(|name| name.starts_with("snapshot-") && name.ends_with(".txt"))
            .collect::<Vec<_>>();
        snapshot_names.sort();
        if !snapshot_names.is_empty() {
            return snapshot_names;
        }
    }

    vec![
        "snapshot-000-after-create-window.txt".to_string(),
        "snapshot-001-after-main-window-activate.txt".to_string(),
        "snapshot-002-after-load-user-settings.txt".to_string(),
        "snapshot-003-post-activate.txt".to_string(),
    ]
}

fn runtime_layout(state: &AppState, window_width: u32, window_height: u32) -> RuntimeLayout {
    let sizing = &state.theme.sizing;
    let spacing = &state.theme.spacing;

    let scale_x = window_width as f32 / DEFAULT_WINDOW_WIDTH;
    let scale_y = window_height as f32 / DEFAULT_WINDOW_HEIGHT;
    let px = |logical: f32| (logical * scale_x).round() as u32;
    let py = |logical: f32| (logical * scale_y).round() as u32;

    let top_bar_height = py(f32::from(sizing.top_bar_height));
    let tool_options_height = py(f32::from(sizing.tool_options_height));
    let footer_height = py(f32::from(sizing.footer_height));
    let footer_inset_top = py(f32::from(sizing.footer_inset_top));
    let main_y = top_bar_height + tool_options_height;
    let main_height = window_height.saturating_sub(main_y + footer_height);

    let left_toolbar_width = px(f32::from(sizing.left_toolbar_width));
    let right_sidebar_width = px(f32::from(sizing.right_sidebar_width));
    let right_sidebar_x = window_width.saturating_sub(right_sidebar_width);
    let workspace_x = left_toolbar_width;
    let workspace_width = right_sidebar_x.saturating_sub(workspace_x);
    let right_sidebar_top_inset = py(f32::from(sizing.right_sidebar_top_inset));
    let right_sidebar_gap = py(f32::from(sizing.right_sidebar_gap));
    let layers_pad_height = py(f32::from(sizing.layers_pad_height));
    let history_pad_height = py(f32::from(sizing.history_pad_height));
    let viewport_width = workspace_width;
    let viewport_height = main_height;
    let width_scale = viewport_width as f32 / state.viewport.viewport_size.0 as f32;
    let height_scale = viewport_height as f32 / state.viewport.viewport_size.1 as f32;
    let scale = width_scale.min(height_scale).min(1.0) * state.viewport.zoom.max(0.05);
    let surface_width = (state.viewport.viewport_size.0 as f32 * scale).round() as u32;
    let surface_height = (state.viewport.viewport_size.1 as f32 * scale).round() as u32;
    let canvas_x = workspace_x + viewport_width.saturating_sub(surface_width) / 2;
    let canvas_y = main_y + viewport_height.saturating_sub(surface_height) / 2;

    let top_padding = py(spacing.sm);
    let button_width = px(f32::from(sizing.toolbox_button_size));
    let button_height = py(f32::from(sizing.toolbox_button_size.saturating_sub(8)));
    let button_gap_x = px(spacing.xs);
    let button_gap_y = py(spacing.xxs);
    let row_inset_x = px(spacing.xxs);
    let grid_width = button_width * 2 + button_gap_x;
    let grid_x = if left_toolbar_width > grid_width + row_inset_x * 2 {
        (left_toolbar_width - grid_width) / 2
    } else {
        row_inset_x
    };

    let layers_y = main_y + right_sidebar_top_inset;
    let history_y = layers_y + layers_pad_height + right_sidebar_gap;

    let mut rects = vec![
        NamedRect {
            name: "tool-toolbar",
            rect: Rect {
                x: 0,
                y: top_bar_height,
                width: window_width,
                height: tool_options_height,
            },
        },
        NamedRect {
            name: "workspace-layout",
            rect: Rect {
                x: workspace_x,
                y: main_y,
                width: workspace_width,
                height: main_height,
            },
        },
        NamedRect {
            name: "toolbox",
            rect: Rect {
                x: 0,
                y: main_y,
                width: left_toolbar_width,
                height: main_height,
            },
        },
        NamedRect {
            name: "canvas",
            rect: Rect {
                x: canvas_x,
                y: canvas_y,
                width: surface_width,
                height: surface_height,
            },
        },
        NamedRect {
            name: "layers-list",
            rect: Rect {
                x: right_sidebar_x,
                y: layers_y,
                width: right_sidebar_width,
                height: layers_pad_height,
            },
        },
        NamedRect {
            name: "history-list",
            rect: Rect {
                x: right_sidebar_x,
                y: history_y,
                width: right_sidebar_width,
                height: history_pad_height,
            },
        },
        NamedRect {
            name: "statusbar",
            rect: Rect {
                x: 0,
                y: window_height.saturating_sub(footer_height) + footer_inset_top,
                width: window_width,
                height: footer_height.saturating_sub(footer_inset_top),
            },
        },
    ];

    for (index, name) in tool_labels().into_iter().enumerate() {
        let row = index as u32 / 2;
        let column = index as u32 % 2;
        rects.push(NamedRect {
            name,
            rect: Rect {
                x: grid_x + column * (button_width + button_gap_x),
                y: main_y + top_padding + row * (button_height + button_gap_y),
                width: button_width,
                height: button_height,
            },
        });
    }

    RuntimeLayout {
        window_width,
        window_height,
        rects,
    }
}

fn default_labels() -> Vec<&'static str> {
    let mut labels = vec![
        "canvas",
        "tool-toolbar",
        "workspace-layout",
        "statusbar",
        "toolbox",
        "layers-list",
        "history-list",
    ];
    labels.extend(tool_labels());
    labels
}

fn tool_labels() -> Vec<&'static str> {
    vec![
        "tool-move-selected-pixels",
        "tool-move-selection",
        "tool-zoom",
        "tool-pan",
        "tool-rectangle-select",
        "tool-ellipse-select",
        "tool-lasso-select",
        "tool-magic-wand-select",
        "tool-paintbrush",
        "tool-pencil",
        "tool-eraser",
        "tool-paint-bucket",
        "tool-gradient",
        "tool-color-picker",
        "tool-text",
        "tool-line-curve",
        "tool-rectangle",
        "tool-rounded-rectangle",
        "tool-ellipse",
        "tool-freeform-shape",
        "tool-clone-stamp",
        "tool-recolor",
    ]
}

fn timestamp_session_name() -> String {
    format!("session-{}", unix_timestamp())
}

fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn json_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
