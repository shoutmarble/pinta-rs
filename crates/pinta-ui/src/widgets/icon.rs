use iced::widget::{
    canvas::{Canvas, Fill, Frame, Geometry, Path, Program, Stroke, path},
    svg,
};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Size, Theme};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy)]
pub enum IconKind {
    DocumentNew,
    CursorArrow,
    MovePixels,
    MoveSelection,
    Zoom,
    Pan,
    RectSelect,
    EllipseSelect,
    LassoSelect,
    MagicWand,
    Paintbrush,
    Pencil,
    Eraser,
    PaintBucket,
    Gradient,
    ColorPicker,
    Text,
    LineCurve,
    Rectangle,
    RoundedRectangle,
    Ellipse,
    Freeform,
    CloneStamp,
    Recolor,
    Eye,
    ThumbnailSample,
    Add,
    Duplicate,
    Delete,
    Merge,
    MoveUp,
    MoveDown,
    More,
    OpenImage,
    Save,
    Scissors,
    Clipboard,
    ImageLandscape,
    ImageGeneric,
    Adjustments,
    Effects,
    Menu,
    ValueDecrease,
    ValueIncrease,
    ViewReveal,
    ChevronDown,
    ChevronUp,
    WindowMinimize,
    WindowMaximize,
    WindowClose,
    Undo,
    Redo,
    ColorSwap,
    ColorReset,
    HistoryList,
}

pub fn view<'a, Message: 'a>(
    kind: IconKind,
    width: f32,
    height: f32,
    color: Color,
) -> Element<'a, Message> {
    if let Some(handle) = svg_handle(kind, color) {
        return svg(handle)
            .width(Length::Fixed(width))
            .height(Length::Fixed(height))
            .into();
    }

    Canvas::new(IconProgram { kind, color })
        .width(Length::Fixed(width))
        .height(Length::Fixed(height))
        .into()
}

fn svg_handle(kind: IconKind, color: Color) -> Option<svg::Handle> {
    if let Some(markup) = upstream_svg_markup(kind, color) {
        return Some(svg::Handle::from_memory(markup.into_bytes()));
    }

    if let Some(markup) = standard_svg_markup(kind, color) {
        return Some(svg::Handle::from_memory(markup.into_bytes()));
    }

    let stroke = svg_color(color);
    let markup = match kind {
        IconKind::ColorReset => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='1.8' stroke-linecap='round' stroke-linejoin='round'><path d='M6.5 6.5h5.5v5.5H6.5z'/><path d='M12 12h5.5v5.5H12z'/></svg>"#
        ),
        IconKind::ChevronDown => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2.2' stroke-linecap='round' stroke-linejoin='round'><path d='m7 9 5 6 5-6'/></svg>"#
        ),
        _ => return None,
    };

    Some(svg::Handle::from_memory(markup.into_bytes()))
}

fn upstream_svg_markup(kind: IconKind, color: Color) -> Option<String> {
    let source = match kind {
        IconKind::CursorArrow => include_str!("../../assets/upstream-icons/actions/ui-cursor-location-symbolic.svg"),
        IconKind::MovePixels => include_str!("../../assets/upstream-icons/actions/tool-move-cursor-symbolic.svg"),
        IconKind::MoveSelection => {
            include_str!("../../assets/upstream-icons/actions/tool-move-selection-symbolic.svg")
        }
        IconKind::Zoom => include_str!("../../assets/upstream-icons/actions/tool-zoom-symbolic.svg"),
        IconKind::Pan => include_str!("../../assets/upstream-icons/actions/tool-pan-symbolic.svg"),
        IconKind::RectSelect => {
            include_str!("../../assets/upstream-icons/actions/tool-select-rectangle-symbolic.svg")
        }
        IconKind::EllipseSelect => {
            include_str!("../../assets/upstream-icons/actions/tool-select-ellipse-symbolic.svg")
        }
        IconKind::LassoSelect => {
            include_str!("../../assets/upstream-icons/actions/tool-select-lasso-symbolic.svg")
        }
        IconKind::MagicWand => {
            include_str!("../../assets/upstream-icons/actions/tool-select-magicwand-symbolic.svg")
        }
        IconKind::Paintbrush => {
            include_str!("../../assets/upstream-icons/actions/tool-paintbrush-symbolic.svg")
        }
        IconKind::Pencil => include_str!("../../assets/upstream-icons/actions/tool-pencil-symbolic.svg"),
        IconKind::Eraser => include_str!("../../assets/upstream-icons/actions/tool-eraser-symbolic.svg"),
        IconKind::PaintBucket => {
            include_str!("../../assets/upstream-icons/actions/tool-paintbucket-symbolic.svg")
        }
        IconKind::Gradient => {
            include_str!("../../assets/upstream-icons/actions/tool-gradient-symbolic.svg")
        }
        IconKind::ColorPicker => {
            include_str!("../../assets/upstream-icons/actions/tool-colorpicker-symbolic.svg")
        }
        IconKind::Text => include_str!("../../assets/upstream-icons/actions/tool-text-symbolic.svg"),
        IconKind::LineCurve => include_str!("../../assets/upstream-icons/actions/tool-line-symbolic.svg"),
        IconKind::Rectangle => {
            include_str!("../../assets/upstream-icons/actions/tool-rectangle-symbolic.svg")
        }
        IconKind::RoundedRectangle => {
            include_str!("../../assets/upstream-icons/actions/tool-rectangle-rounded-symbolic.svg")
        }
        IconKind::Ellipse => include_str!("../../assets/upstream-icons/actions/tool-ellipse-symbolic.svg"),
        IconKind::Freeform => {
            include_str!("../../assets/upstream-icons/actions/tool-freeformshape-symbolic.svg")
        }
        IconKind::CloneStamp => {
            include_str!("../../assets/upstream-icons/actions/tool-clonestamp-symbolic.svg")
        }
        IconKind::Recolor => include_str!("../../assets/upstream-icons/actions/tool-recolor-symbolic.svg"),
        IconKind::Add => include_str!("../../assets/upstream-icons/actions/layers-add-layer-symbolic.svg"),
        IconKind::Duplicate => {
            include_str!("../../assets/upstream-icons/actions/layers-duplicate-layer-symbolic.svg")
        }
        IconKind::Delete => {
            include_str!("../../assets/upstream-icons/actions/layers-remove-layer-symbolic.svg")
        }
        IconKind::Merge => {
            include_str!("../../assets/upstream-icons/actions/layers-merge-down-symbolic.svg")
        }
        IconKind::Adjustments => {
            include_str!("../../assets/upstream-icons/actions/adjustments-default-symbolic.svg")
        }
        IconKind::Effects => {
            include_str!("../../assets/upstream-icons/actions/effects-default-symbolic.svg")
        }
        IconKind::ImageLandscape => {
            include_str!("../../assets/upstream-icons/actions/image-orientation-landscape-symbolic.svg")
        }
        IconKind::ColorSwap => {
            include_str!("../../assets/upstream-icons/actions/edit-swap-vert-symbolic.svg")
        }
        IconKind::HistoryList => {
            include_str!("../../assets/upstream-icons/actions/ui-historylist-symbolic.svg")
        }
        _ => return None,
    };

    Some(recolor_symbolic_svg(source, color))
}

fn standard_svg_markup(kind: IconKind, color: Color) -> Option<String> {
    let name = match kind {
        IconKind::DocumentNew => "document-new-symbolic.svg",
        IconKind::OpenImage => "document-open-symbolic.svg",
        IconKind::Save => "document-save-symbolic.svg",
        IconKind::Undo => "edit-undo-symbolic.svg",
        IconKind::Redo => "edit-redo-symbolic.svg",
        IconKind::Scissors => "edit-cut-symbolic.svg",
        IconKind::Clipboard => "edit-paste-symbolic.svg",
        IconKind::Menu => "open-menu-symbolic.svg",
        IconKind::WindowClose => "window-close-symbolic.svg",
        IconKind::WindowMaximize => "window-maximize-symbolic.svg",
        IconKind::WindowMinimize => "window-minimize-symbolic.svg",
        IconKind::ValueDecrease => "value-decrease-symbolic.svg",
        IconKind::ValueIncrease => "value-increase-symbolic.svg",
        IconKind::MoveUp => "pan-up-symbolic.svg",
        IconKind::MoveDown => "pan-down-symbolic.svg",
        IconKind::ViewReveal => "view-reveal-symbolic.svg",
        IconKind::ImageGeneric => "image-x-generic-symbolic.svg",
        _ => return None,
    };

    let source = standard_icon_source(name).or_else(|| vendored_standard_icon_source(name))?;
    Some(recolor_symbolic_svg(source, color))
}

fn standard_icon_source(name: &'static str) -> Option<&'static str> {
    static SOURCES: OnceLock<HashMap<&'static str, Option<String>>> = OnceLock::new();

    SOURCES
        .get_or_init(|| {
            STANDARD_ICON_FILENAMES
                .iter()
                .copied()
                .map(|file_name| {
                    (
                        file_name,
                        find_standard_icon_file(file_name)
                            .and_then(|path| fs::read_to_string(path).ok()),
                    )
                })
                .collect()
        })
        .get(name)
        .and_then(|source| source.as_deref())
}

fn vendored_standard_icon_source(name: &'static str) -> Option<&'static str> {
    match name {
        "document-new-symbolic.svg" => Some(include_str!("../../assets/standard-icons/actions/document-new-symbolic.svg")),
        "document-open-symbolic.svg" => Some(include_str!("../../assets/standard-icons/actions/document-open-symbolic.svg")),
        "document-save-symbolic.svg" => Some(include_str!("../../assets/standard-icons/actions/document-save-symbolic.svg")),
        "edit-undo-symbolic.svg" => Some(include_str!("../../assets/standard-icons/actions/edit-undo-symbolic.svg")),
        "edit-redo-symbolic.svg" => Some(include_str!("../../assets/standard-icons/actions/edit-redo-symbolic.svg")),
        "edit-cut-symbolic.svg" => Some(include_str!("../../assets/standard-icons/actions/edit-cut-symbolic.svg")),
        "edit-paste-symbolic.svg" => Some(include_str!("../../assets/standard-icons/actions/edit-paste-symbolic.svg")),
        "open-menu-symbolic.svg" => Some(include_str!("../../assets/standard-icons/actions/open-menu-symbolic.svg")),
        "window-close-symbolic.svg" => Some(include_str!("../../assets/standard-icons/actions/window-close-symbolic.svg")),
        "window-maximize-symbolic.svg" => Some(include_str!("../../assets/standard-icons/actions/window-maximize-symbolic.svg")),
        "window-minimize-symbolic.svg" => Some(include_str!("../../assets/standard-icons/actions/window-minimize-symbolic.svg")),
        "value-decrease-symbolic.svg" => Some(include_str!("../../assets/standard-icons/actions/value-decrease-symbolic.svg")),
        "value-increase-symbolic.svg" => Some(include_str!("../../assets/standard-icons/actions/value-increase-symbolic.svg")),
        "pan-up-symbolic.svg" => Some(include_str!("../../assets/standard-icons/actions/pan-up-symbolic.svg")),
        "pan-down-symbolic.svg" => Some(include_str!("../../assets/standard-icons/actions/pan-down-symbolic.svg")),
        _ => None,
    }
}

const STANDARD_ICON_FILENAMES: &[&str] = &[
    "document-new-symbolic.svg",
    "document-open-symbolic.svg",
    "document-save-symbolic.svg",
    "edit-undo-symbolic.svg",
    "edit-redo-symbolic.svg",
    "edit-cut-symbolic.svg",
    "edit-paste-symbolic.svg",
    "open-menu-symbolic.svg",
    "window-close-symbolic.svg",
    "window-maximize-symbolic.svg",
    "window-minimize-symbolic.svg",
    "value-decrease-symbolic.svg",
    "value-increase-symbolic.svg",
    "view-reveal-symbolic.svg",
    "image-x-generic-symbolic.svg",
    "pan-up-symbolic.svg",
    "pan-down-symbolic.svg",
];

fn find_standard_icon_file(name: &str) -> Option<PathBuf> {
    let themes = ["Adwaita", "breeze-dark", "breeze", "hicolor", "HighContrast"];
    let subdirs = [
        "actions/24",
        "actions/22",
        "actions/16",
        "symbolic/actions",
        "scalable/actions",
        "mimetypes/24",
        "symbolic/mimetypes",
        "scalable/mimetypes",
        "status/24",
        "symbolic/status",
        "scalable/status",
    ];

    for root in icon_roots() {
        for theme in themes {
            for subdir in subdirs {
                let candidate = root.join(theme).join(subdir).join(name);
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }
    }

    None
}

fn icon_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();

    if let Ok(home) = env::var("HOME") {
        roots.push(PathBuf::from(home).join(".local/share/icons"));
    }

    if let Ok(data_home) = env::var("XDG_DATA_HOME") {
        roots.push(PathBuf::from(data_home).join("icons"));
    }

    if let Ok(data_dirs) = env::var("XDG_DATA_DIRS") {
        roots.extend(
            data_dirs
                .split(':')
                .filter(|part| !part.is_empty())
                .map(|part| PathBuf::from(part).join("icons")),
        );
    }

    roots.push(PathBuf::from("/usr/local/share/icons"));
    roots.push(PathBuf::from("/usr/share/icons"));
    roots
}

fn recolor_symbolic_svg(svg: &str, color: Color) -> String {
    let rgba = svg_color(color);

    svg.replace("url(#gpa:foreground) rgb(0,0,0)", &rgba)
        .replace("url(#gpa:foreground) rgb(0, 0, 0)", &rgba)
        .replace("#bebebe", &rgba)
        .replace("#BEBEBE", &rgba)
        .replace("currentColor", &rgba)
        .replace("#000000", &rgba)
        .replace("rgb(0,0,0)", &rgba)
        .replace("rgb(0, 0, 0)", &rgba)
}

fn svg_color(color: Color) -> String {
    format!(
        "rgba({:.0},{:.0},{:.0},{:.3})",
        color.r * 255.0,
        color.g * 255.0,
        color.b * 255.0,
        color.a,
    )
}

#[derive(Debug, Clone, Copy)]
struct IconProgram {
    kind: IconKind,
    color: Color,
}

impl<Message> Program<Message> for IconProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        draw_icon(&mut frame, self.kind, self.color, bounds.size());
        vec![frame.into_geometry()]
    }
}

fn draw_icon(frame: &mut Frame, kind: IconKind, color: Color, size: Size) {
    let point = |x: f32, y: f32| Point::new(size.width * x / 24.0, size.height * y / 24.0);
    let stroke = Stroke::default()
        .with_width((size.width / 12.0).max(1.2))
        .with_color(color);
    let thin = Stroke::default()
        .with_width((size.width / 18.0).max(1.0))
        .with_color(color);

    match kind {
        IconKind::DocumentNew => {
            let doc = Path::new(|b| {
                b.move_to(point(7.0, 4.0));
                b.line_to(point(15.0, 4.0));
                b.line_to(point(19.0, 8.0));
                b.line_to(point(19.0, 20.0));
                b.line_to(point(7.0, 20.0));
                b.close();
            });
            frame.stroke(&doc, thin);
            frame.stroke(&Path::line(point(15.0, 4.0), point(15.0, 8.0)), thin);
            frame.stroke(&Path::line(point(15.0, 8.0), point(19.0, 8.0)), thin);
            plus(frame, point(10.5, 13.0), size.width * 0.16, thin);
        }
        IconKind::CursorArrow => {
            let arrow = Path::new(|b| {
                b.move_to(point(5.0, 4.0));
                b.line_to(point(5.0, 18.0));
                b.line_to(point(8.8, 14.8));
                b.line_to(point(11.8, 20.0));
                b.line_to(point(13.6, 18.9));
                b.line_to(point(10.6, 13.8));
                b.line_to(point(16.0, 13.8));
                b.close();
            });
            frame.fill(&arrow, Fill::from(color));
        }
        IconKind::MovePixels => {
            frame.stroke(&Path::line(point(12.0, 4.0), point(12.0, 20.0)), thin);
            frame.stroke(&Path::line(point(4.0, 12.0), point(20.0, 12.0)), thin);
            triangle(
                frame,
                point(12.0, 2.0),
                point(10.0, 6.0),
                point(14.0, 6.0),
                color,
            );
            triangle(
                frame,
                point(12.0, 22.0),
                point(10.0, 18.0),
                point(14.0, 18.0),
                color,
            );
            triangle(
                frame,
                point(2.0, 12.0),
                point(6.0, 10.0),
                point(6.0, 14.0),
                color,
            );
            triangle(
                frame,
                point(22.0, 12.0),
                point(18.0, 10.0),
                point(18.0, 14.0),
                color,
            );
        }
        IconKind::MoveSelection => {
            rect_outline(frame, point(5.0, 5.0), size_scale(size, 14.0, 14.0), thin);
            frame.stroke(&Path::line(point(12.0, 2.0), point(12.0, 6.0)), thin);
            frame.stroke(&Path::line(point(2.0, 12.0), point(6.0, 12.0)), thin);
            frame.stroke(&Path::line(point(18.0, 12.0), point(22.0, 12.0)), thin);
            frame.stroke(&Path::line(point(12.0, 18.0), point(12.0, 22.0)), thin);
        }
        IconKind::Zoom => {
            frame.stroke(&Path::circle(point(10.0, 10.0), size.width * 0.22), stroke);
            frame.stroke(&Path::line(point(15.0, 15.0), point(21.0, 21.0)), stroke);
        }
        IconKind::Pan => {
            let hand = Path::new(|b| {
                b.move_to(point(8.0, 19.0));
                b.line_to(point(8.0, 11.0));
                b.line_to(point(9.8, 10.0));
                b.line_to(point(10.8, 14.8));
                b.line_to(point(10.9, 7.4));
                b.line_to(point(12.8, 7.0));
                b.line_to(point(13.8, 14.4));
                b.line_to(point(14.0, 8.1));
                b.line_to(point(15.9, 8.4));
                b.line_to(point(16.7, 15.1));
                b.line_to(point(17.0, 10.5));
                b.line_to(point(19.0, 10.9));
                b.line_to(point(18.3, 17.0));
                b.line_to(point(15.8, 20.0));
                b.line_to(point(10.8, 20.0));
                b.close();
            });
            frame.stroke(&hand, thin);
        }
        IconKind::RectSelect => {
            rect_outline(frame, point(5.0, 5.0), size_scale(size, 14.0, 14.0), stroke)
        }
        IconKind::EllipseSelect => {
            frame.stroke(&Path::circle(point(12.0, 12.0), size.width * 0.24), stroke)
        }
        IconKind::LassoSelect => {
            let path = Path::new(|b| {
                b.move_to(point(6.0, 8.0));
                b.bezier_curve_to(point(8.0, 4.0), point(17.0, 4.0), point(18.0, 10.0));
                b.bezier_curve_to(point(19.0, 15.0), point(13.0, 18.0), point(9.0, 17.0));
                b.bezier_curve_to(point(5.0, 16.0), point(4.0, 11.0), point(6.0, 8.0));
            });
            frame.stroke(&path, stroke);
        }
        IconKind::MagicWand => {
            frame.stroke(&Path::line(point(7.0, 18.0), point(14.0, 11.0)), stroke);
            frame.stroke(&Path::line(point(6.0, 19.0), point(9.0, 22.0)), thin);
            sparkle(frame, point(16.5, 7.5), color, size.width * 0.20);
            sparkle(frame, point(11.0, 9.2), color, size.width * 0.08);
        }
        IconKind::Paintbrush => {
            frame.stroke(&Path::line(point(7.0, 18.0), point(16.0, 9.0)), stroke);
            let brush = Path::new(|b| {
                b.move_to(point(15.0, 8.0));
                b.line_to(point(19.0, 4.0));
                b.line_to(point(21.0, 6.0));
                b.line_to(point(17.0, 10.0));
                b.close();
            });
            frame.fill(&brush, color);
        }
        IconKind::Pencil => {
            frame.stroke(&Path::line(point(6.0, 18.0), point(17.0, 7.0)), stroke);
            triangle(
                frame,
                point(18.0, 6.0),
                point(20.5, 3.5),
                point(20.5, 8.5),
                color,
            );
        }
        IconKind::Eraser => {
            let eraser = Path::new(|b| {
                b.move_to(point(6.0, 16.0));
                b.line_to(point(11.0, 7.0));
                b.line_to(point(18.0, 11.0));
                b.line_to(point(13.0, 20.0));
                b.close();
            });
            frame.fill(&eraser, color);
        }
        IconKind::PaintBucket => {
            let bucket = Path::new(|b| {
                b.move_to(point(6.0, 10.0));
                b.line_to(point(12.0, 4.0));
                b.line_to(point(18.0, 10.0));
                b.line_to(point(12.0, 16.0));
                b.close();
            });
            frame.fill(&bucket, color);
            frame.fill(&Path::circle(point(18.0, 17.0), size.width * 0.09), color);
            frame.stroke(&Path::line(point(5.0, 20.0), point(16.0, 20.0)), thin);
        }
        IconKind::Gradient => {
            frame.fill(
                &Path::rectangle(point(5.0, 7.0), size_scale(size, 14.0, 10.0)),
                Fill::from(Color::from_rgba(color.r, color.g, color.b, 0.18)),
            );
            frame.fill(
                &Path::new(|b| {
                    b.move_to(point(9.0, 17.0));
                    b.line_to(point(19.0, 7.0));
                    b.line_to(point(19.0, 17.0));
                    b.close();
                }),
                color,
            );
            frame.stroke(
                &Path::rectangle(point(5.0, 7.0), size_scale(size, 14.0, 10.0)),
                thin,
            );
        }
        IconKind::ColorPicker => {
            frame.stroke(&Path::line(point(7.0, 17.0), point(16.0, 8.0)), stroke);
            frame.fill(&Path::circle(point(18.0, 6.0), size.width * 0.08), color);
        }
        IconKind::Text => {
            frame.stroke(&Path::line(point(7.0, 6.0), point(17.0, 6.0)), stroke);
            frame.stroke(&Path::line(point(12.0, 6.0), point(12.0, 18.0)), stroke);
        }
        IconKind::LineCurve => {
            frame.stroke(&Path::line(point(5.0, 18.0), point(10.0, 14.0)), thin);
            let curve = Path::new(|b| {
                b.move_to(point(10.0, 14.0));
                b.bezier_curve_to(point(13.0, 8.0), point(17.0, 8.0), point(20.0, 15.0));
            });
            frame.stroke(&curve, stroke);
        }
        IconKind::Rectangle => {
            rect_outline(frame, point(5.0, 7.0), size_scale(size, 14.0, 10.0), stroke)
        }
        IconKind::RoundedRectangle => rounded_rect(
            frame,
            point(5.0, 7.0),
            size_scale(size, 14.0, 10.0),
            size.width * 0.12,
            thin,
        ),
        IconKind::Ellipse => {
            frame.stroke(&Path::circle(point(12.0, 12.0), size.width * 0.22), stroke)
        }
        IconKind::Freeform => {
            let blob = Path::new(|b| {
                b.move_to(point(6.0, 14.0));
                b.bezier_curve_to(point(5.0, 7.0), point(13.0, 4.0), point(18.0, 7.0));
                b.bezier_curve_to(point(21.0, 11.0), point(18.0, 18.0), point(11.0, 19.0));
                b.bezier_curve_to(point(8.0, 19.0), point(6.0, 17.0), point(6.0, 14.0));
            });
            frame.stroke(&blob, stroke);
        }
        IconKind::CloneStamp => {
            rounded_rect(
                frame,
                point(7.0, 11.0),
                size_scale(size, 10.0, 7.0),
                size.width * 0.08,
                thin,
            );
            frame.stroke(&Path::line(point(12.0, 11.0), point(12.0, 5.0)), thin);
            frame.stroke(&Path::line(point(9.0, 5.0), point(15.0, 5.0)), thin);
            frame.stroke(&Path::line(point(9.6, 8.0), point(14.4, 8.0)), thin);
        }
        IconKind::Recolor => {
            frame.stroke(&Path::line(point(7.0, 17.0), point(15.0, 9.0)), stroke);
            frame.stroke(&Path::circle(point(16.4, 7.6), size.width * 0.09), thin);
            let drop = Path::new(|b| {
                b.move_to(point(18.0, 13.0));
                b.bezier_curve_to(point(20.0, 15.0), point(20.0, 17.0), point(18.0, 20.0));
                b.bezier_curve_to(point(16.0, 17.0), point(16.0, 15.0), point(18.0, 13.0));
            });
            frame.fill(&drop, color);
        }
        IconKind::Eye => {
            let eye = Path::new(|b| {
                b.move_to(point(4.0, 12.0));
                b.bezier_curve_to(point(8.0, 6.0), point(16.0, 6.0), point(20.0, 12.0));
                b.bezier_curve_to(point(16.0, 18.0), point(8.0, 18.0), point(4.0, 12.0));
            });
            frame.stroke(&eye, thin);
            frame.fill(&Path::circle(point(12.0, 12.0), size.width * 0.10), color);
        }
        IconKind::ViewReveal => {
            let eye = Path::new(|b| {
                b.move_to(point(4.0, 12.0));
                b.bezier_curve_to(point(8.0, 6.0), point(16.0, 6.0), point(20.0, 12.0));
                b.bezier_curve_to(point(16.0, 18.0), point(8.0, 18.0), point(4.0, 12.0));
            });
            frame.stroke(&eye, thin);
            frame.fill(&Path::circle(point(12.0, 12.0), size.width * 0.10), color);
        }
        IconKind::ThumbnailSample => {
            frame.fill(
                &Path::rectangle(point(2.0, 2.0), size_scale(size, 20.0, 20.0)),
                Color::WHITE,
            );
            frame.stroke(
                &Path::rectangle(point(2.0, 2.0), size_scale(size, 20.0, 20.0)),
                thin,
            );
            frame.fill(
                &Path::circle(point(8.0, 9.0), size.width * 0.10),
                Color::from_rgb8(0xE0, 0x48, 0x3D),
            );
            frame.fill(
                &Path::rectangle(point(11.0, 6.0), size_scale(size, 7.0, 5.0)),
                Color::from_rgb8(0x5A, 0x8D, 0x4B),
            );
            frame.stroke(
                &Path::line(point(5.0, 18.0), point(19.0, 15.0)),
                Stroke::default()
                    .with_width((size.width / 16.0).max(1.0))
                    .with_color(Color::from_rgb8(0x21, 0x21, 0x23)),
            );
        }
        IconKind::Add => plus(frame, point(12.0, 12.0), size.width * 0.26, stroke),
        IconKind::Duplicate => {
            rect_outline(frame, point(7.0, 7.0), size_scale(size, 9.0, 9.0), thin);
            rect_outline(frame, point(10.0, 10.0), size_scale(size, 9.0, 9.0), thin);
        }
        IconKind::Delete => {
            frame.stroke(&Path::line(point(7.0, 7.0), point(17.0, 17.0)), stroke);
            frame.stroke(&Path::line(point(17.0, 7.0), point(7.0, 17.0)), stroke);
        }
        IconKind::Merge => {
            frame.stroke(&Path::line(point(8.0, 8.0), point(12.0, 12.0)), thin);
            frame.stroke(&Path::line(point(16.0, 8.0), point(12.0, 12.0)), thin);
            frame.stroke(&Path::line(point(12.0, 12.0), point(12.0, 18.0)), thin);
        }
        IconKind::MoveUp => triangle(
            frame,
            point(12.0, 7.0),
            point(7.0, 14.0),
            point(17.0, 14.0),
            color,
        ),
        IconKind::MoveDown => triangle(
            frame,
            point(12.0, 17.0),
            point(7.0, 10.0),
            point(17.0, 10.0),
            color,
        ),
        IconKind::More => {
            frame.fill(&Path::circle(point(8.0, 12.0), size.width * 0.06), color);
            frame.fill(&Path::circle(point(12.0, 12.0), size.width * 0.06), color);
            frame.fill(&Path::circle(point(16.0, 12.0), size.width * 0.06), color);
        }
        IconKind::OpenImage => {
            rect_outline(frame, point(4.0, 7.0), size_scale(size, 16.0, 11.0), thin);
            frame.stroke(&Path::line(point(8.0, 11.0), point(12.0, 8.0)), thin);
            frame.stroke(&Path::line(point(12.0, 8.0), point(16.0, 13.0)), thin);
        }
        IconKind::Save => {
            rect_outline(frame, point(5.0, 5.0), size_scale(size, 14.0, 14.0), thin);
            frame.fill(
                &Path::rectangle(point(8.0, 6.0), size_scale(size, 8.0, 4.0)),
                color,
            );
            rect_outline(frame, point(8.0, 13.0), size_scale(size, 8.0, 5.0), thin);
        }
        IconKind::Scissors => {
            frame.stroke(&Path::line(point(8.0, 8.0), point(16.0, 16.0)), thin);
            frame.stroke(&Path::line(point(16.0, 8.0), point(8.0, 16.0)), thin);
            frame.stroke(&Path::circle(point(7.0, 7.0), size.width * 0.08), thin);
            frame.stroke(&Path::circle(point(17.0, 7.0), size.width * 0.08), thin);
        }
        IconKind::Clipboard => {
            rect_outline(frame, point(7.0, 7.0), size_scale(size, 10.0, 12.0), thin);
            rounded_rect(
                frame,
                point(9.0, 4.0),
                size_scale(size, 6.0, 4.0),
                size.width * 0.10,
                thin,
            );
        }
        IconKind::ImageLandscape => {
            rect_outline(frame, point(4.0, 6.0), size_scale(size, 16.0, 12.0), thin);
            frame.fill(
                &Path::circle(point(8.0, 10.0), size.width * 0.07),
                Color::from_rgb8(0x70, 0xC6, 0x55),
            );
            frame.stroke(&Path::line(point(7.0, 15.0), point(11.0, 11.0)), thin);
            frame.stroke(&Path::line(point(11.0, 11.0), point(15.0, 15.0)), thin);
            frame.stroke(&Path::line(point(15.0, 15.0), point(19.0, 9.0)), thin);
        }
        IconKind::ImageGeneric => {
            rect_outline(frame, point(4.0, 6.0), size_scale(size, 16.0, 12.0), thin);
            frame.fill(
                &Path::circle(point(8.0, 10.0), size.width * 0.07),
                Color::from_rgb8(0x70, 0xC6, 0x55),
            );
            frame.stroke(&Path::line(point(7.0, 15.0), point(11.0, 11.0)), thin);
            frame.stroke(&Path::line(point(11.0, 11.0), point(15.0, 15.0)), thin);
            frame.stroke(&Path::line(point(15.0, 15.0), point(19.0, 9.0)), thin);
        }
        IconKind::Adjustments => {
            let left = Path::new(|b| {
                b.move_to(point(12.0, 4.0));
                b.line_to(point(12.0, 20.0));
                b.arc(path::Arc {
                    center: point(12.0, 12.0),
                    radius: size.width * 0.16,
                    start_angle: iced::Radians(-std::f32::consts::FRAC_PI_2),
                    end_angle: iced::Radians(std::f32::consts::FRAC_PI_2),
                });
            });
            frame.stroke(&left, thin);
            frame.fill(
                &Path::new(|b| {
                    b.move_to(point(12.0, 4.0));
                    b.arc(path::Arc {
                        center: point(12.0, 12.0),
                        radius: size.width * 0.16,
                        start_angle: iced::Radians(-std::f32::consts::FRAC_PI_2),
                        end_angle: iced::Radians(std::f32::consts::FRAC_PI_2),
                    });
                    b.line_to(point(12.0, 20.0));
                    b.close();
                }),
                color,
            );
        }
        IconKind::Effects => {
            sparkle(frame, point(8.0, 11.0), color, size.width * 0.13);
            frame.stroke(&Path::line(point(12.5, 16.0), point(18.5, 8.0)), thin);
            frame.stroke(&Path::line(point(14.0, 16.0), point(17.2, 16.0)), thin);
        }
        IconKind::Menu => {
            frame.stroke(&Path::line(point(6.0, 8.0), point(18.0, 8.0)), thin);
            frame.stroke(&Path::line(point(6.0, 12.0), point(18.0, 12.0)), thin);
            frame.stroke(&Path::line(point(6.0, 16.0), point(18.0, 16.0)), thin);
        }
        IconKind::ValueDecrease => {
            frame.stroke(&Path::line(point(7.0, 12.0), point(17.0, 12.0)), thin);
        }
        IconKind::ValueIncrease => {
            frame.stroke(&Path::line(point(7.0, 12.0), point(17.0, 12.0)), thin);
            frame.stroke(&Path::line(point(12.0, 7.0), point(12.0, 17.0)), thin);
        }
        IconKind::ChevronDown => {
            frame.stroke(&Path::line(point(7.0, 9.0), point(12.0, 15.0)), thin);
            frame.stroke(&Path::line(point(12.0, 15.0), point(17.0, 9.0)), thin);
        }
        IconKind::ChevronUp => {
            frame.stroke(&Path::line(point(7.0, 15.0), point(12.0, 9.0)), thin);
            frame.stroke(&Path::line(point(12.0, 9.0), point(17.0, 15.0)), thin);
        }
        IconKind::WindowMinimize => {
            frame.stroke(&Path::line(point(7.0, 15.0), point(17.0, 15.0)), thin);
        }
        IconKind::WindowMaximize => {
            rect_outline(frame, point(7.0, 7.0), size_scale(size, 10.0, 10.0), thin);
        }
        IconKind::WindowClose => {
            frame.stroke(&Path::line(point(7.0, 7.0), point(17.0, 17.0)), thin);
            frame.stroke(&Path::line(point(17.0, 7.0), point(7.0, 17.0)), thin);
        }
        IconKind::Undo => curved_arrow(
            frame,
            point(8.0, 11.0),
            point(19.0, 10.0),
            true,
            color,
            size.width,
        ),
        IconKind::Redo => curved_arrow(
            frame,
            point(16.0, 11.0),
            point(5.0, 10.0),
            false,
            color,
            size.width,
        ),
        IconKind::ColorSwap => {
            frame.stroke(&Path::line(point(6.0, 7.0), point(16.0, 7.0)), thin);
            frame.stroke(&Path::line(point(18.0, 17.0), point(8.0, 17.0)), thin);
            triangle(frame, point(16.0, 4.5), point(20.0, 7.0), point(16.0, 9.5), color);
            triangle(frame, point(8.0, 14.5), point(4.0, 17.0), point(8.0, 19.5), color);
        }
        IconKind::ColorReset => {
            rect_outline(frame, point(6.0, 6.0), size_scale(size, 6.5, 6.5), thin);
            rect_outline(frame, point(11.5, 11.5), size_scale(size, 6.5, 6.5), thin);
        }
        IconKind::HistoryList => {
            frame.fill(&Path::circle(point(5.0, 8.0), size.width * 0.06), color);
            frame.fill(&Path::circle(point(5.0, 12.0), size.width * 0.06), color);
            frame.fill(&Path::circle(point(5.0, 16.0), size.width * 0.06), color);
            frame.stroke(&Path::line(point(8.0, 8.0), point(18.0, 8.0)), thin);
            frame.stroke(&Path::line(point(8.0, 12.0), point(18.0, 12.0)), thin);
            frame.stroke(&Path::line(point(8.0, 16.0), point(18.0, 16.0)), thin);
        }
    }
}

fn size_scale(size: Size, width: f32, height: f32) -> Size {
    Size::new(size.width * width / 24.0, size.height * height / 24.0)
}

fn rect_outline(frame: &mut Frame, origin: Point, size: Size, stroke: Stroke<'_>) {
    frame.stroke(&Path::rectangle(origin, size), stroke);
}

fn rounded_rect(frame: &mut Frame, origin: Point, size: Size, radius: f32, stroke: Stroke<'_>) {
    let path = Path::new(|b| {
        b.move_to(Point::new(origin.x + radius, origin.y));
        b.line_to(Point::new(origin.x + size.width - radius, origin.y));
        b.quadratic_curve_to(
            Point::new(origin.x + size.width, origin.y),
            Point::new(origin.x + size.width, origin.y + radius),
        );
        b.line_to(Point::new(
            origin.x + size.width,
            origin.y + size.height - radius,
        ));
        b.quadratic_curve_to(
            Point::new(origin.x + size.width, origin.y + size.height),
            Point::new(origin.x + size.width - radius, origin.y + size.height),
        );
        b.line_to(Point::new(origin.x + radius, origin.y + size.height));
        b.quadratic_curve_to(
            Point::new(origin.x, origin.y + size.height),
            Point::new(origin.x, origin.y + size.height - radius),
        );
        b.line_to(Point::new(origin.x, origin.y + radius));
        b.quadratic_curve_to(
            Point::new(origin.x, origin.y),
            Point::new(origin.x + radius, origin.y),
        );
    });
    frame.stroke(&path, stroke);
}

fn triangle(frame: &mut Frame, a: Point, b: Point, c: Point, color: Color) {
    let triangle = Path::new(|builder| {
        builder.move_to(a);
        builder.line_to(b);
        builder.line_to(c);
        builder.close();
    });
    frame.fill(&triangle, color);
}

fn plus(frame: &mut Frame, center: Point, arm: f32, stroke: Stroke<'_>) {
    frame.stroke(
        &Path::line(
            Point::new(center.x - arm, center.y),
            Point::new(center.x + arm, center.y),
        ),
        stroke,
    );
    frame.stroke(
        &Path::line(
            Point::new(center.x, center.y - arm),
            Point::new(center.x, center.y + arm),
        ),
        stroke,
    );
}

fn sparkle(frame: &mut Frame, center: Point, color: Color, arm: f32) {
    let stroke = Stroke::default().with_width(1.2).with_color(color);
    frame.stroke(
        &Path::line(
            Point::new(center.x - arm, center.y),
            Point::new(center.x + arm, center.y),
        ),
        stroke,
    );
    frame.stroke(
        &Path::line(
            Point::new(center.x, center.y - arm),
            Point::new(center.x, center.y + arm),
        ),
        stroke,
    );
}

fn curved_arrow(
    frame: &mut Frame,
    start: Point,
    end: Point,
    left_head: bool,
    color: Color,
    width: f32,
) {
    let path = Path::new(|b| {
        b.move_to(start);
        b.bezier_curve_to(
            Point::new((start.x + end.x) / 2.0, start.y - width * 0.22),
            Point::new((start.x + end.x) / 2.0, end.y - width * 0.22),
            end,
        );
    });
    let stroke = Stroke::default()
        .with_width((width / 15.0).max(1.0))
        .with_color(color);
    frame.stroke(&path, stroke);
    if left_head {
        triangle(
            frame,
            Point::new(start.x - width * 0.12, start.y),
            Point::new(start.x + width * 0.05, start.y - width * 0.10),
            Point::new(start.x + width * 0.05, start.y + width * 0.10),
            color,
        );
    } else {
        triangle(
            frame,
            Point::new(start.x + width * 0.12, start.y),
            Point::new(start.x - width * 0.05, start.y - width * 0.10),
            Point::new(start.x - width * 0.05, start.y + width * 0.10),
            color,
        );
    }
}
