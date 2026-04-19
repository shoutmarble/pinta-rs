use iced::widget::canvas::{path, Canvas, Fill, Frame, Geometry, Path, Program, Stroke};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Size, Theme};

#[derive(Debug, Clone, Copy)]
pub enum IconKind {
    DocumentNew,
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
    Canvas::new(IconProgram {
        kind,
        color,
        size: Size::new(width, height),
    })
    .width(Length::Fixed(width))
    .height(Length::Fixed(height))
    .into()
}

#[derive(Debug, Clone, Copy)]
struct IconProgram {
    kind: IconKind,
    color: Color,
    size: Size,
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
        draw_icon(&mut frame, self.kind, self.color, self.size);
        vec![frame.into_geometry()]
    }
}

fn draw_icon(frame: &mut Frame, kind: IconKind, color: Color, size: Size) {
    let point = |x: f32, y: f32| Point::new(size.width * x / 24.0, size.height * y / 24.0);
    let stroke = Stroke::default().with_width((size.width / 12.0).max(1.2)).with_color(color);
    let thin = Stroke::default().with_width((size.width / 18.0).max(1.0)).with_color(color);

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
        IconKind::MovePixels => {
            frame.stroke(&Path::line(point(12.0, 4.0), point(12.0, 20.0)), thin);
            frame.stroke(&Path::line(point(4.0, 12.0), point(20.0, 12.0)), thin);
            triangle(frame, point(12.0, 2.0), point(10.0, 6.0), point(14.0, 6.0), color);
            triangle(frame, point(12.0, 22.0), point(10.0, 18.0), point(14.0, 18.0), color);
            triangle(frame, point(2.0, 12.0), point(6.0, 10.0), point(6.0, 14.0), color);
            triangle(frame, point(22.0, 12.0), point(18.0, 10.0), point(18.0, 14.0), color);
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
            frame.stroke(&Path::line(point(12.0, 4.0), point(12.0, 19.0)), stroke);
            frame.stroke(&Path::line(point(7.0, 9.0), point(12.0, 4.0)), stroke);
            frame.stroke(&Path::line(point(17.0, 9.0), point(12.0, 4.0)), stroke);
            frame.stroke(&Path::line(point(8.0, 19.0), point(16.0, 19.0)), stroke);
        }
        IconKind::RectSelect => rect_outline(frame, point(5.0, 5.0), size_scale(size, 14.0, 14.0), stroke),
        IconKind::EllipseSelect => frame.stroke(&Path::circle(point(12.0, 12.0), size.width * 0.24), stroke),
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
            frame.stroke(&Path::line(point(8.0, 18.0), point(15.0, 11.0)), stroke);
            sparkle(frame, point(16.5, 7.5), color, size.width * 0.18);
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
            triangle(frame, point(18.0, 6.0), point(20.5, 3.5), point(20.5, 8.5), color);
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
                b.move_to(point(7.0, 9.0));
                b.line_to(point(13.0, 4.0));
                b.line_to(point(18.0, 10.0));
                b.line_to(point(12.0, 15.0));
                b.close();
            });
            frame.fill(&bucket, color);
            frame.fill(&Path::circle(point(18.0, 17.0), size.width * 0.08), color);
        }
        IconKind::Gradient => {
            frame.fill(&Path::rectangle(point(5.0, 7.0), size_scale(size, 14.0, 10.0)), Fill::from(Color::from_rgba(color.r, color.g, color.b, 0.2)));
            frame.fill(&Path::rectangle(point(11.0, 7.0), size_scale(size, 8.0, 10.0)), color);
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
        IconKind::Rectangle => rect_outline(frame, point(5.0, 7.0), size_scale(size, 14.0, 10.0), stroke),
        IconKind::RoundedRectangle => rounded_rect(frame, point(5.0, 7.0), size_scale(size, 14.0, 10.0), size.width * 0.12, thin),
        IconKind::Ellipse => frame.stroke(&Path::circle(point(12.0, 12.0), size.width * 0.22), stroke),
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
            rect_outline(frame, point(7.0, 10.0), size_scale(size, 10.0, 8.0), stroke);
            frame.stroke(&Path::line(point(12.0, 10.0), point(12.0, 5.0)), stroke);
            frame.stroke(&Path::line(point(9.0, 5.0), point(15.0, 5.0)), stroke);
        }
        IconKind::Recolor => {
            let drop = Path::new(|b| {
                b.move_to(point(12.0, 4.0));
                b.bezier_curve_to(point(16.0, 8.0), point(18.0, 12.0), point(12.0, 20.0));
                b.bezier_curve_to(point(6.0, 12.0), point(8.0, 8.0), point(12.0, 4.0));
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
            frame.fill(&Path::rectangle(point(2.0, 2.0), size_scale(size, 20.0, 20.0)), Color::WHITE);
            frame.stroke(&Path::rectangle(point(2.0, 2.0), size_scale(size, 20.0, 20.0)), thin);
            frame.fill(&Path::circle(point(8.0, 9.0), size.width * 0.10), Color::from_rgb8(0xE0, 0x48, 0x3D));
            frame.fill(&Path::rectangle(point(11.0, 6.0), size_scale(size, 7.0, 5.0)), Color::from_rgb8(0x5A, 0x8D, 0x4B));
            frame.stroke(&Path::line(point(5.0, 18.0), point(19.0, 15.0)), Stroke::default().with_width((size.width / 16.0).max(1.0)).with_color(Color::from_rgb8(0x21, 0x21, 0x23)));
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
        IconKind::MoveUp => triangle(frame, point(12.0, 7.0), point(7.0, 14.0), point(17.0, 14.0), color),
        IconKind::MoveDown => triangle(frame, point(12.0, 17.0), point(7.0, 10.0), point(17.0, 10.0), color),
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
            frame.fill(&Path::rectangle(point(8.0, 6.0), size_scale(size, 8.0, 4.0)), color);
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
            rounded_rect(frame, point(9.0, 4.0), size_scale(size, 6.0, 4.0), size.width * 0.10, thin);
        }
        IconKind::ImageLandscape => {
            rect_outline(frame, point(4.0, 6.0), size_scale(size, 16.0, 12.0), thin);
            frame.fill(&Path::circle(point(8.0, 10.0), size.width * 0.07), Color::from_rgb8(0x70, 0xC6, 0x55));
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
            frame.fill(&Path::new(|b| {
                b.move_to(point(12.0, 4.0));
                b.arc(path::Arc {
                    center: point(12.0, 12.0),
                    radius: size.width * 0.16,
                    start_angle: iced::Radians(-std::f32::consts::FRAC_PI_2),
                    end_angle: iced::Radians(std::f32::consts::FRAC_PI_2),
                });
                b.line_to(point(12.0, 20.0));
                b.close();
            }), color);
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
        IconKind::WindowClose => {
            frame.stroke(&Path::line(point(7.0, 7.0), point(17.0, 17.0)), thin);
            frame.stroke(&Path::line(point(17.0, 7.0), point(7.0, 17.0)), thin);
        }
        IconKind::Undo => curved_arrow(frame, point(8.0, 11.0), point(19.0, 10.0), true, color, size.width),
        IconKind::Redo => curved_arrow(frame, point(16.0, 11.0), point(5.0, 10.0), false, color, size.width),
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
        b.quadratic_curve_to(Point::new(origin.x + size.width, origin.y), Point::new(origin.x + size.width, origin.y + radius));
        b.line_to(Point::new(origin.x + size.width, origin.y + size.height - radius));
        b.quadratic_curve_to(Point::new(origin.x + size.width, origin.y + size.height), Point::new(origin.x + size.width - radius, origin.y + size.height));
        b.line_to(Point::new(origin.x + radius, origin.y + size.height));
        b.quadratic_curve_to(Point::new(origin.x, origin.y + size.height), Point::new(origin.x, origin.y + size.height - radius));
        b.line_to(Point::new(origin.x, origin.y + radius));
        b.quadratic_curve_to(Point::new(origin.x, origin.y), Point::new(origin.x + radius, origin.y));
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
    frame.stroke(&Path::line(Point::new(center.x - arm, center.y), Point::new(center.x + arm, center.y)), stroke);
    frame.stroke(&Path::line(Point::new(center.x, center.y - arm), Point::new(center.x, center.y + arm)), stroke);
}

fn sparkle(frame: &mut Frame, center: Point, color: Color, arm: f32) {
    let stroke = Stroke::default().with_width(1.2).with_color(color);
    frame.stroke(&Path::line(Point::new(center.x - arm, center.y), Point::new(center.x + arm, center.y)), stroke);
    frame.stroke(&Path::line(Point::new(center.x, center.y - arm), Point::new(center.x, center.y + arm)), stroke);
}

fn curved_arrow(frame: &mut Frame, start: Point, end: Point, left_head: bool, color: Color, width: f32) {
    let path = Path::new(|b| {
        b.move_to(start);
        b.bezier_curve_to(
            Point::new((start.x + end.x) / 2.0, start.y - width * 0.22),
            Point::new((start.x + end.x) / 2.0, end.y - width * 0.22),
            end,
        );
    });
    let stroke = Stroke::default().with_width((width / 15.0).max(1.0)).with_color(color);
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
