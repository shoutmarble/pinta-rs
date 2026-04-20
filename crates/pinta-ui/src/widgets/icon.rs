use iced::widget::{
    canvas::{Canvas, Fill, Frame, Geometry, Path, Program, Stroke, path},
    svg,
};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Size, Theme};

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
    Adjustments,
    Effects,
    Menu,
    ChevronDown,
    ChevronUp,
    WindowMinimize,
    WindowMaximize,
    WindowClose,
    Undo,
    Redo,
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
    let stroke = svg_color(color);
    let fill = svg_color(color);
    let markup = match kind {
        IconKind::CursorArrow => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='1.9' stroke-linecap='round' stroke-linejoin='round'><path fill='{fill}' stroke='none' d='M5 4.5 5 18l3.8-3 2.7 4.5 2.2-1.2-2.8-4.6H16z'/></svg>"#
        ),
        IconKind::MovePixels => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2.1' stroke-linecap='round' stroke-linejoin='round'><path d='M12 4v16M4 12h16'/><path fill='{fill}' stroke='none' d='M12 2l2.5 4h-5zM12 22l-2.5-4h5zM2 12l4-2.5v5zM22 12l-4 2.5v-5z'/></svg>"#
        ),
        IconKind::MoveSelection => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><rect x='6' y='6' width='12' height='12' rx='1'/><path d='M12 2v3M12 19v3M2 12h3M19 12h3'/></svg>"#
        ),
        IconKind::Zoom => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2.2' stroke-linecap='round' stroke-linejoin='round'><circle cx='10' cy='10' r='5.5'/><path d='M14.5 14.5 20 20'/></svg>"#
        ),
        IconKind::Pan => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2.1' stroke-linecap='round' stroke-linejoin='round'><path fill='#ffffff' d='M7.2 18.3v-7.2c0-.9.7-1.6 1.5-1.6.9 0 1.5.7 1.5 1.6V7.4c0-.9.7-1.6 1.6-1.6s1.6.7 1.6 1.6v4.1V6.2c0-.9.7-1.6 1.6-1.6S17 5.3 17 6.2v5.3V7.5c0-.9.7-1.6 1.5-1.6.9 0 1.5.7 1.5 1.6v6.1c0 1.6-.6 3.1-1.7 4.2l-1.2 1.2c-.8.8-1.8 1.2-2.9 1.2h-2.7c-1.2 0-2.3-.5-3.2-1.3l-3.1-3c-.7-.7-1.4-.5-1.4.5v.3'/><path d='M7 16.2 4.4 13.7'/></svg>"#
        ),
        IconKind::RectSelect => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2' stroke-linecap='square' stroke-linejoin='miter'><rect x='5' y='5' width='14' height='14' stroke-dasharray='3 3'/></svg>"#
        ),
        IconKind::EllipseSelect => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2' stroke-dasharray='3 3'><circle cx='12' cy='12' r='7'/></svg>"#
        ),
        IconKind::LassoSelect => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><path d='M6 8c1.5-2.7 8.8-3.7 11.3-.9 2.2 2.4 1.4 6.2-1.6 8-2.3 1.4-5.8 1.8-8 .8-3.6-1.7-3.5-5.3-1.7-7.9Z'/></svg>"#
        ),
        IconKind::MagicWand => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><path d='M6 18 14 10'/><path d='M4.5 19.5 7 22'/><path d='M15 4v4M13 6h4M17.5 8.5v2M16.5 9.5h2'/></svg>"#
        ),
        IconKind::Paintbrush => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><path d='M6 18c1.6 0 2.6-1 2.6-2.5 0-.7.3-1.3.8-1.8l7.9-7.9 2.9 2.9-7.9 7.9c-.5.5-1.1.8-1.8.8H9.9c-.9 0-1.8.4-2.4 1.1L6 20'/><path fill='{fill}' stroke='none' d='M17.2 4.8 19.3 2.7a1 1 0 0 1 1.4 0l.6.6a1 1 0 0 1 0 1.4l-2.1 2.1z'/></svg>"#
        ),
        IconKind::Pencil => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><path d='M5 19 17.5 6.5l2 2L7 21H5z'/><path fill='{fill}' stroke='none' d='M18.2 5.8 20.3 3.7a1 1 0 0 1 1.4 0l.6.6a1 1 0 0 1 0 1.4l-2.1 2.1z'/></svg>"#
        ),
        IconKind::Eraser => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='1.9' stroke-linecap='round' stroke-linejoin='round'><path fill='{fill}' d='M10.2 19.1 19 10.3c.9-.9.9-2.4 0-3.3l-2.1-2.1c-.9-.9-2.4-.9-3.3 0l-8.4 8.4 4.9 5.8Z'/><path fill='#ffffff' stroke='#ffffff' d='M6.7 14.3 10.1 17.7 14.8 13l-3.4-3.4Z'/><path d='M14.2 6.7 18.4 10.9'/><path d='M9.6 19.4H20'/></svg>"#
        ),
        IconKind::PaintBucket => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><path fill='{fill}' d='M5.7 11.2 12.3 4.6l5.2 5.2-6.6 6.6Z'/><path d='M7.9 13.4h8.1'/><path fill='{fill}' stroke='none' d='M18.7 10.9c1.3 1.6 1.3 2.9 0 4.5-1.1-1.6-1.1-2.9 0-4.5Z'/><path d='M4.1 19.2h15.8' stroke-width='2.3'/></svg>"#
        ),
        IconKind::Gradient => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none'><rect x='4.4' y='4.4' width='15.2' height='15.2' rx='0.4' stroke='{stroke}' stroke-width='2.2'/><rect x='7.2' y='9.6' width='2.8' height='2.8' fill='{fill}'/><rect x='10.4' y='9.6' width='2.8' height='2.8' fill='#ffffff'/><rect x='13.6' y='9.6' width='2.8' height='2.8' fill='{fill}'/><rect x='7.2' y='12.8' width='2.8' height='2.8' fill='#ffffff'/><rect x='10.4' y='12.8' width='2.8' height='2.8' fill='{fill}'/><rect x='13.6' y='12.8' width='2.8' height='2.8' fill='#ffffff'/></svg>"#
        ),
        IconKind::ColorPicker => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><path d='M7 17 16 8'/><circle cx='18' cy='6' r='1.7' fill='{fill}' stroke='none'/></svg>"#
        ),
        IconKind::Text => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2.2' stroke-linecap='round'><path d='M7 6h10M12 6v12'/></svg>"#
        ),
        IconKind::LineCurve => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><path d='M5 18 10 14'/><path d='M10 14c2-5 6-5 9 1'/></svg>"#
        ),
        IconKind::Rectangle => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2'><rect x='5' y='7' width='14' height='10'/></svg>"#
        ),
        IconKind::RoundedRectangle => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2'><rect x='5' y='7' width='14' height='10' rx='3'/></svg>"#
        ),
        IconKind::Ellipse => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2'><circle cx='12' cy='12' r='7'/></svg>"#
        ),
        IconKind::Freeform => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><path d='M6 14c-.8-5 5-8 10-6 3 1.2 4 5 1.8 8-1.8 2.4-6.3 3.4-9 2.1C7.1 17 6.2 15.7 6 14Z'/></svg>"#
        ),
        IconKind::CloneStamp => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><rect x='8' y='11' width='8' height='6' rx='1.5'/><path d='M12 11V5M9 5h6M9.5 8h5'/></svg>"#
        ),
        IconKind::Recolor => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><path d='M7 17 15 9'/><circle cx='16.4' cy='7.6' r='1.4'/><path fill='{fill}' stroke='none' d='M18 13c1.4 1.7 1.4 3.2 0 5-1.4-1.8-1.4-3.3 0-5Z'/></svg>"#
        ),
        IconKind::ChevronDown => format!(
            r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='{stroke}' stroke-width='2.2' stroke-linecap='round' stroke-linejoin='round'><path d='m7 9 5 6 5-6'/></svg>"#
        ),
        _ => return None,
    };

    Some(svg::Handle::from_memory(markup.into_bytes()))
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
