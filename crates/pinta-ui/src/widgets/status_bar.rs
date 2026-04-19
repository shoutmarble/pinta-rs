use iced::widget::{column, container, row, text};
use iced::{Background, Border, Element, Length};
use pinta_theme::PintaTheme;

use crate::widgets::icon::{self, IconKind};

pub fn view<'a, Message: 'a>(
    theme: &'a PintaTheme,
    cursor_text: String,
    image_text: String,
    selection_text: String,
    zoom_text: String,
) -> Element<'a, Message> {
    let primary = swatch(theme, [0x00, 0x00, 0x00], 22.0, 22.0);
    let secondary = swatch(theme, [0xFF, 0xFF, 0xFF], 22.0, 22.0);

    let mini_swatches = row![
        swatch(theme, [0xE0, 0x48, 0x3D], 14.0, 14.0),
        swatch(theme, [0x5A, 0x8D, 0x4B], 14.0, 14.0),
        swatch(theme, [0x3F, 0x66, 0xB6], 14.0, 14.0),
        swatch(theme, [0xF0, 0xD1, 0x58], 14.0, 14.0),
        swatch(theme, [0x7E, 0x56, 0xAF], 14.0, 14.0),
        swatch(theme, [0xF8, 0xF8, 0xF8], 14.0, 14.0),
    ]
    .spacing(theme.spacing.xs);

    let palette_lead = container(
        row![
            column![primary, secondary].spacing(theme.spacing.xs),
            mini_swatches,
        ]
        .spacing(theme.spacing.sm)
        .align_y(iced::Alignment::Center),
    )
    .width(Length::Fixed(theme.sizing.palette_lead_width as f32));

    let zoom_box = container(text(zoom_text).size(theme.typography.caption))
        .padding([theme.spacing.xs, theme.spacing.sm])
        .style(move |_| {
            container::Style::default()
                .background(Background::Color(theme.colors.panel_bg))
                .border(
                    Border::default()
                        .rounded(theme.radii.md)
                        .width(1)
                        .color(theme.colors.border_subtle),
                )
        });

    let content = row![
        palette_lead,
        icon::view(IconKind::MovePixels, 14.0, 14.0, theme.colors.text_muted),
        text(cursor_text).size(theme.typography.caption),
        icon::view(IconKind::RectSelect, 14.0, 14.0, theme.colors.text_muted),
        text(selection_text).size(theme.typography.caption),
        icon::view(IconKind::ThumbnailSample, 14.0, 14.0, theme.colors.text_muted),
        text(image_text).size(theme.typography.caption),
        zoom_box,
    ]
    .spacing(theme.spacing.md)
    .padding([theme.spacing.xs, theme.spacing.md]);

    container(content)
        .width(Length::Fill)
        .height(Length::Fixed(theme.sizing.footer_height as f32))
        .style(move |_| {
            container::Style::default()
                .background(Background::Color(theme.colors.status_bg))
                .border(
                    Border::default()
                        .width(1)
                        .color(theme.colors.border_subtle),
                )
        })
        .into()
}

fn swatch<'a, Message: 'a>(
    theme: &'a PintaTheme,
    rgb: [u8; 3],
    width: f32,
    height: f32,
) -> Element<'a, Message> {
    container(text(""))
        .width(Length::Fixed(width))
        .height(Length::Fixed(height))
        .style(move |_| {
            container::Style::default()
                .background(Background::Color(iced::Color::from_rgb8(rgb[0], rgb[1], rgb[2])))
                .border(
                    Border::default()
                        .rounded(theme.radii.sm)
                        .width(1)
                        .color(theme.colors.border_strong),
                )
        })
        .into()
}
