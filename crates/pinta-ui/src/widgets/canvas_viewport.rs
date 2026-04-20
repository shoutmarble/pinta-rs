use glam::DVec2;
use iced::mouse;
use iced::widget::Action;
use iced::widget::canvas::{Canvas, Frame, Geometry, Path, Program, Stroke};
use iced::{Element, Event, Length, Point, Rectangle, Renderer, Size, Theme};
use pinta_theme::PintaTheme;

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
            zoom: 1.0,
            pan: DVec2::ZERO,
            hovered_image_pos: None,
            checker_size: 12.0,
        }
    }
}

impl ViewportState {
    pub fn image_to_screen(&self, image: DVec2) -> DVec2 {
        image * self.zoom as f64 + self.pan
    }

    pub fn screen_to_image(&self, screen: DVec2) -> DVec2 {
        (screen - self.pan) / self.zoom as f64
    }

    pub fn zoom_about_screen_point(&mut self, cursor: DVec2, next_zoom: f32) {
        let image_before = self.screen_to_image(cursor);
        self.zoom = next_zoom.max(0.05);
        let screen_after = self.image_to_screen(image_before);
        self.pan += cursor - screen_after;
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
}

impl CanvasViewport {
    pub fn new(theme: PintaTheme, state: ViewportState) -> Self {
        Self { theme, state }
    }

    pub fn view(self) -> Element<'static, CanvasAction> {
        Canvas::new(ViewportProgram {
            theme: self.theme,
            state: self.state,
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
        frame.fill_rectangle(
            Point::ORIGIN,
            bounds.size(),
            self.theme.colors.canvas_surround_bg,
        );

        let horizontal_margin = bounds.width * 0.105;
        let vertical_margin = bounds.height * 0.105;
        let max_width = (bounds.width - horizontal_margin * 2.0).max(280.0);
        let max_height = (bounds.height - vertical_margin * 2.0).max(220.0);
        let page_width = max_width.min(max_height * (4.0 / 3.0)) * 0.90;
        let page_height = page_width * 0.75;
        let page_size = Size::new(page_width, page_height);
        let page_x = (bounds.width - page_size.width) / 2.0;
        let page_y = (bounds.height - page_size.height) / 2.0 + 10.0;
        let px = |x: f32| page_x + page_size.width * (x / 800.0);
        let py = |y: f32| page_y + page_size.height * (y / 600.0);
        let scale = page_size.width / 800.0;

        // Drop shadow (offset right and down)
        let shadow_offset = 4.0;
        let shadow_color = iced::Color::from_rgba8(0x00, 0x00, 0x00, 0.25);
        let shadow = Path::rectangle(
            Point::new(page_x + shadow_offset, page_y + shadow_offset),
            page_size,
        );
        frame.fill(&shadow, shadow_color);

        let page = Path::rectangle(Point::new(page_x, page_y), page_size);
        frame.fill(&page, self.theme.colors.canvas_page_bg);
        frame.stroke(
            &page,
            Stroke::default()
                .with_width(1.0)
                .with_color(self.theme.colors.border_strong),
        );

        let red_circle = Path::circle(Point::new(px(188.0), py(176.0)), 72.0 * scale);
        frame.fill(&red_circle, iced::Color::from_rgb8(0xE0, 0x48, 0x3D));

        let green_rect = Path::rectangle(
            Point::new(px(318.0), py(108.0)),
            Size::new(
                page_size.width * (274.0 / 800.0),
                page_size.height * (122.0 / 600.0),
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

        let zoom_bar = Path::rectangle(
            Point::new(
                page_x + page_size.width - 94.0,
                page_y + page_size.height + 16.0,
            ),
            Size::new((self.state.zoom * 34.0).clamp(24.0, 86.0), 3.0),
        );
        frame.fill(&zoom_bar, self.theme.colors.selected_bg);

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Action<CanvasAction>> {
        match event {
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                let local = DVec2::new(position.x as f64, position.y as f64);
                Some(Action::publish(CanvasAction::CursorMoved(local)).and_capture())
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(position) = cursor.position_in(bounds) {
                    let local = DVec2::new(position.x as f64, position.y as f64);
                    Some(Action::publish(CanvasAction::Pressed(local)).and_capture())
                } else {
                    None
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if let Some(position) = cursor.position_in(bounds) {
                    let local = DVec2::new(position.x as f64, position.y as f64);
                    Some(Action::publish(CanvasAction::Released(local)).and_capture())
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
                    let local = DVec2::new(position.x as f64, position.y as f64);
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
            }
            _ => None,
        }
    }
}
