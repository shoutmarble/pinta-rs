use glam::DVec2;
use iced::mouse;
use iced::widget::Action;
use iced::widget::canvas::{Canvas, Frame, Geometry, Path, Program, Stroke};
use iced::{Element, Event, Length, Point, Rectangle, Renderer, Size, Theme};
use pinta_theme::PintaTheme;

use crate::widgets::icon::IconKind;

const SURFACE_INSET_X: f32 = 12.0;
const SURFACE_INSET_Y_TOP: f32 = 18.0;
const SURFACE_INSET_Y_BOTTOM: f32 = 12.0;

#[derive(Debug, Clone)]
pub struct ViewportState {
    pub viewport_size: (u32, u32),
    pub zoom: f32,
    pub pan: DVec2,
    pub hovered_image_pos: Option<DVec2>,
    pub checker_size: f32,
}

impl Default for ViewportState {
    fn default() -> Self {
        Self {
            viewport_size: (800, 600),
            zoom: 0.86,
            pan: DVec2::ZERO,
            hovered_image_pos: None,
            checker_size: 12.0,
        }
    }
}

impl ViewportState {
    pub fn surface_size(&self) -> Size {
        Size::new(
            self.viewport_size.0 as f32 * self.zoom,
            self.viewport_size.1 as f32 * self.zoom,
        )
    }

    pub fn image_to_screen(&self, image: DVec2) -> DVec2 {
        image * self.zoom as f64
    }

    pub fn screen_to_image(&self, screen: DVec2) -> DVec2 {
        screen / self.zoom as f64
    }

    pub fn zoom_about_screen_point(&mut self, _cursor: DVec2, next_zoom: f32) {
        self.zoom = next_zoom.max(0.05);
        self.pan = DVec2::ZERO;
    }
}

#[derive(Debug, Clone)]
pub enum CanvasAction {
    CursorMoved(DVec2),
    Pressed(DVec2),
    Released(DVec2),
    Scrolled { delta_lines: f32, cursor: DVec2 },
}

#[derive(Debug, Clone)]
pub struct CanvasViewport {
    pub theme: PintaTheme,
    pub state: ViewportState,
    pub active_tool: IconKind,
    pub scripted_effect: bool,
}

impl CanvasViewport {
    pub fn new(theme: PintaTheme, state: ViewportState, active_tool: IconKind, scripted_effect: bool) -> Self {
        Self { theme, state, active_tool, scripted_effect }
    }

    pub fn view(self) -> Element<'static, CanvasAction> {
        Canvas::new(ViewportProgram {
            theme: self.theme,
            state: self.state,
            active_tool: self.active_tool,
            scripted_effect: self.scripted_effect,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

#[derive(Debug, Clone)]
struct ViewportProgram {
    theme: PintaTheme,
    state: ViewportState,
    active_tool: IconKind,
    scripted_effect: bool,
}

impl Program<CanvasAction> for ViewportProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let surface_bounds = anchored_surface_rect(bounds.size(), self.state.surface_size());
        let page = Path::rectangle(surface_bounds.position(), surface_bounds.size());
        frame.fill(&page, self.theme.colors.canvas_page_bg);
        frame.stroke(
            &page,
            Stroke::default()
                .with_width(1.0)
                .with_color(self.theme.colors.border_strong),
        );

        let px = |x: f32| {
            surface_bounds.x + surface_bounds.width * (x / self.state.viewport_size.0 as f32)
        };
        let py = |y: f32| {
            surface_bounds.y + surface_bounds.height * (y / self.state.viewport_size.1 as f32)
        };
        let scale = surface_bounds.width / self.state.viewport_size.0 as f32;

        let red_circle = Path::circle(Point::new(px(188.0), py(176.0)), 72.0 * scale);
        frame.fill(&red_circle, iced::Color::from_rgb8(0xE0, 0x48, 0x3D));

        let green_rect = Path::rectangle(
            Point::new(px(318.0), py(108.0)),
            Size::new(
                surface_bounds.width * (274.0 / self.state.viewport_size.0 as f32),
                surface_bounds.height * (122.0 / self.state.viewport_size.1 as f32),
            ),
        );
        frame.fill(&green_rect, iced::Color::from_rgb8(0x5A, 0x8D, 0x4B));

        let black_line = Path::line(
            Point::new(px(96.0), py(492.0)),
            Point::new(px(692.0), py(364.0)),
        );
        frame.stroke(
            &black_line,
            Stroke::default()
                .with_width((8.0 * scale).max(3.0))
                .with_color(iced::Color::from_rgb8(0x21, 0x21, 0x23)),
        );

        let wave = Path::new(|builder| {
            builder.move_to(Point::new(px(110.0), py(318.0)));
            builder.bezier_curve_to(
                Point::new(px(218.0), py(178.0)),
                Point::new(px(398.0), py(184.0)),
                Point::new(px(503.0), py(324.0)),
            );
            builder.bezier_curve_to(
                Point::new(px(614.0), py(444.0)),
                Point::new(px(700.0), py(448.0)),
                Point::new(px(785.0), py(210.0)),
            );
        });
        frame.stroke(
            &wave,
            Stroke::default()
                .with_width((8.0 * scale).max(3.0))
                .with_color(iced::Color::from_rgb8(0x3F, 0x66, 0xB6)),
        );

        if self.scripted_effect {
            draw_scripted_effect(&mut frame, self.active_tool, &self.state, surface_bounds);
        }

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Action<CanvasAction>> {
        let surface_bounds = anchored_surface_rect(bounds.size(), self.state.surface_size());

        match event {
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                let local = DVec2::new(
                    position.x as f64 - surface_bounds.x as f64,
                    position.y as f64 - surface_bounds.y as f64,
                );
                if surface_bounds.contains(Point::new(position.x, position.y)) {
                    Some(Action::publish(CanvasAction::CursorMoved(local)).and_capture())
                } else {
                    None
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(position) = cursor.position_in(bounds) {
                    if surface_bounds.contains(position) {
                        let local = DVec2::new(
                            position.x as f64 - surface_bounds.x as f64,
                            position.y as f64 - surface_bounds.y as f64,
                        );
                        Some(Action::publish(CanvasAction::Pressed(local)).and_capture())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if let Some(position) = cursor.position_in(bounds) {
                    if surface_bounds.contains(position) {
                        let local = DVec2::new(
                            position.x as f64 - surface_bounds.x as f64,
                            position.y as f64 - surface_bounds.y as f64,
                        );
                        Some(Action::publish(CanvasAction::Released(local)).and_capture())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                let lines = match delta {
                    mouse::ScrollDelta::Lines { y, .. } => *y,
                    mouse::ScrollDelta::Pixels { y, .. } => *y / 40.0,
                };

                if let Some(position) = cursor.position_in(bounds) {
                    if surface_bounds.contains(position) {
                        let local = DVec2::new(
                            position.x as f64 - surface_bounds.x as f64,
                            position.y as f64 - surface_bounds.y as f64,
                        );
                        Some(
                            Action::publish(CanvasAction::Scrolled {
                                delta_lines: lines,
                                cursor: local,
                            })
                            .and_capture(),
                        )
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

fn draw_scripted_effect(frame: &mut Frame, active_tool: IconKind, state: &ViewportState, surface_bounds: Rectangle) {
    let px = |x: f32| surface_bounds.x + surface_bounds.width * (x / state.viewport_size.0 as f32);
    let py = |y: f32| surface_bounds.y + surface_bounds.height * (y / state.viewport_size.1 as f32);
    let scale = surface_bounds.width / state.viewport_size.0 as f32;

    match active_tool {
        IconKind::MovePixels => {
            let moved = Path::rectangle(Point::new(px(148.0), py(104.0)), Size::new(170.0 * scale, 120.0 * scale));
            frame.fill(&moved, iced::Color::from_rgba8(0xE0, 0x48, 0x3D, 0.35));
            let border = Path::rectangle(Point::new(px(148.0), py(104.0)), Size::new(170.0 * scale, 120.0 * scale));
            frame.stroke(&border, Stroke::default().with_width(2.0).with_color(iced::Color::from_rgb8(0x20, 0x20, 0x20)));
        }
        IconKind::MoveSelection | IconKind::RectSelect | IconKind::EllipseSelect | IconKind::LassoSelect | IconKind::MagicWand => {
            let selection = Path::rectangle(Point::new(px(120.0), py(90.0)), Size::new(240.0 * scale, 170.0 * scale));
            frame.stroke(&selection, Stroke::default().with_width(2.0).with_color(iced::Color::from_rgb8(0x22, 0x22, 0x22)));
        }
        IconKind::Zoom => {
            let zoom_focus = Path::rectangle(Point::new(px(165.0), py(125.0)), Size::new(330.0 * scale, 240.0 * scale));
            frame.stroke(&zoom_focus, Stroke::default().with_width(3.0).with_color(iced::Color::from_rgb8(0x6B, 0x85, 0xD6)));
        }
        IconKind::Pan => {
            let moved = Path::rectangle(Point::new(px(35.0), py(25.0)), Size::new(surface_bounds.width - 40.0 * scale, surface_bounds.height - 40.0 * scale));
            frame.stroke(&moved, Stroke::default().with_width(1.5).with_color(iced::Color::from_rgb8(0xB9, 0xB9, 0xBF)));
        }
        IconKind::Paintbrush | IconKind::Pencil => {
            let stroke = Path::new(|builder| {
                builder.move_to(Point::new(px(92.0), py(478.0)));
                builder.bezier_curve_to(Point::new(px(210.0), py(360.0)), Point::new(px(290.0), py(410.0)), Point::new(px(402.0), py(448.0)));
                builder.bezier_curve_to(Point::new(px(520.0), py(490.0)), Point::new(px(560.0), py(428.0)), Point::new(px(652.0), py(372.0)));
            });
            frame.stroke(&stroke, Stroke::default().with_width((10.0 * scale).max(4.0)).with_color(iced::Color::from_rgb8(0xA4, 0x2B, 0x22)));
        }
        IconKind::Eraser => {
            let erase = Path::rectangle(Point::new(px(300.0), py(170.0)), Size::new(180.0 * scale, 70.0 * scale));
            frame.fill(&erase, iced::Color::from_rgb8(0xFF, 0xFF, 0xFF));
        }
        IconKind::PaintBucket => {
            let fill = Path::rectangle(Point::new(px(16.0), py(16.0)), Size::new(250.0 * scale, 190.0 * scale));
            frame.fill(&fill, iced::Color::from_rgba8(0xF7, 0xD2, 0x3A, 0.75));
        }
        IconKind::Gradient => {
            let gradient = Path::rectangle(Point::new(px(80.0), py(90.0)), Size::new(560.0 * scale, 220.0 * scale));
            frame.fill(&gradient, iced::Color::from_rgba8(0x5A, 0x8D, 0x4B, 0.4));
        }
        IconKind::ColorPicker => {
            let marker = Path::circle(Point::new(px(185.0), py(175.0)), 10.0 * scale);
            frame.stroke(&marker, Stroke::default().with_width(3.0).with_color(iced::Color::from_rgb8(0x22, 0x22, 0x22)));
        }
        IconKind::Text => {
            let baseline = Path::line(Point::new(px(150.0), py(430.0)), Point::new(px(300.0), py(430.0)));
            frame.stroke(&baseline, Stroke::default().with_width(1.5).with_color(iced::Color::from_rgb8(0x44, 0x44, 0x44)));
            let glyph = Path::rectangle(Point::new(px(170.0), py(360.0)), Size::new(90.0 * scale, 50.0 * scale));
            frame.stroke(&glyph, Stroke::default().with_width(2.0).with_color(iced::Color::from_rgb8(0x22, 0x22, 0x22)));
        }
        IconKind::LineCurve => {
            let line = Path::line(Point::new(px(120.0), py(460.0)), Point::new(px(580.0), py(240.0)));
            frame.stroke(&line, Stroke::default().with_width((6.0 * scale).max(3.0)).with_color(iced::Color::from_rgb8(0x28, 0x28, 0x2A)));
        }
        IconKind::Rectangle | IconKind::RoundedRectangle => {
            let shape = Path::rectangle(Point::new(px(120.0), py(110.0)), Size::new(270.0 * scale, 170.0 * scale));
            frame.stroke(&shape, Stroke::default().with_width((5.0 * scale).max(2.0)).with_color(iced::Color::from_rgb8(0x95, 0x42, 0x2A)));
        }
        IconKind::Ellipse => {
            let ellipse = Path::new(|builder| {
                let center_x = px(290.0);
                let center_y = py(230.0);
                let radius_x = 150.0 * scale;
                let radius_y = 100.0 * scale;

                builder.move_to(Point::new(center_x + radius_x, center_y));
                builder.bezier_curve_to(
                    Point::new(center_x + radius_x, center_y - radius_y * 0.5523),
                    Point::new(center_x + radius_x * 0.5523, center_y - radius_y),
                    Point::new(center_x, center_y - radius_y),
                );
                builder.bezier_curve_to(
                    Point::new(center_x - radius_x * 0.5523, center_y - radius_y),
                    Point::new(center_x - radius_x, center_y - radius_y * 0.5523),
                    Point::new(center_x - radius_x, center_y),
                );
                builder.bezier_curve_to(
                    Point::new(center_x - radius_x, center_y + radius_y * 0.5523),
                    Point::new(center_x - radius_x * 0.5523, center_y + radius_y),
                    Point::new(center_x, center_y + radius_y),
                );
                builder.bezier_curve_to(
                    Point::new(center_x + radius_x * 0.5523, center_y + radius_y),
                    Point::new(center_x + radius_x, center_y + radius_y * 0.5523),
                    Point::new(center_x + radius_x, center_y),
                );
                builder.close();
            });
            frame.stroke(&ellipse, Stroke::default().with_width((5.0 * scale).max(2.0)).with_color(iced::Color::from_rgb8(0x95, 0x42, 0x2A)));
        }
        IconKind::Freeform => {
            let freeform = Path::new(|builder| {
                builder.move_to(Point::new(px(130.0), py(320.0)));
                builder.bezier_curve_to(Point::new(px(220.0), py(180.0)), Point::new(px(360.0), py(190.0)), Point::new(px(420.0), py(250.0)));
                builder.bezier_curve_to(Point::new(px(470.0), py(310.0)), Point::new(px(430.0), py(420.0)), Point::new(px(250.0), py(430.0)));
                builder.close();
            });
            frame.stroke(&freeform, Stroke::default().with_width((5.0 * scale).max(2.0)).with_color(iced::Color::from_rgb8(0x95, 0x42, 0x2A)));
        }
        IconKind::CloneStamp => {
            let patch = Path::rectangle(Point::new(px(470.0), py(130.0)), Size::new(110.0 * scale, 80.0 * scale));
            frame.fill(&patch, iced::Color::from_rgba8(0xE0, 0x48, 0x3D, 0.45));
        }
        IconKind::Recolor => {
            let recolor = Path::rectangle(Point::new(px(320.0), py(108.0)), Size::new(274.0 * scale, 122.0 * scale));
            frame.fill(&recolor, iced::Color::from_rgba8(0x7C, 0x61, 0xD9, 0.5));
        }
        _ => {}
    }
}

fn anchored_surface_rect(bounds: Size, surface: Size) -> Rectangle {
    let safe_width = (bounds.width - SURFACE_INSET_X * 2.0).max(1.0);
    let safe_height = (bounds.height - SURFACE_INSET_Y_TOP - SURFACE_INSET_Y_BOTTOM).max(1.0);

    let width = surface.width.min(safe_width);
    let height = surface.height.min(safe_height);

    let extra_width = (safe_width - width).max(0.0);
    let extra_height = (safe_height - height).max(0.0);

    let x = SURFACE_INSET_X + extra_width.min(8.0);
    let y = SURFACE_INSET_Y_TOP + extra_height.min(64.0);

    Rectangle {
        x,
        y,
        width,
        height,
    }
}

#[cfg(test)]
mod tests {
    use super::anchored_surface_rect;
    use iced::Size;

    #[test]
    fn anchored_surface_keeps_left_gutter_when_width_is_tight() {
        let rect = anchored_surface_rect(Size::new(180.0, 300.0), Size::new(688.0, 516.0));

        assert!(rect.x >= 12.0);
        assert!(rect.width <= 156.0);
    }

    #[test]
    fn anchored_surface_respects_top_and_bottom_insets() {
        let rect = anchored_surface_rect(Size::new(400.0, 180.0), Size::new(300.0, 516.0));

        assert!(rect.y >= 18.0);
        assert!(rect.height <= 150.0);
    }
}
