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
    let palette_lead = row![
        color_stack_panel(theme),
        blank_chip(theme, 70.0),
        swatch(theme, [0x00, 0x00, 0x00], 18.0, 18.0),
        swatch_row(
            theme,
            &[
                [0x6D, 0x6D, 0x6D],
                [0x8B, 0x8B, 0x8B],
                [0xB5, 0xB5, 0xB5],
                [0xD6, 0xD6, 0xD6],
            ],
            14.0,
            18.0,
        ),
        swatch_row(
            theme,
            &[
                [0xFF, 0x2B, 0x20],
                [0xFF, 0x7D, 0x14],
                [0xFF, 0xCF, 0x11],
                [0xB5, 0xFA, 0x18],
                [0x37, 0xF0, 0x1D],
                [0x16, 0xD8, 0x4F],
                [0x22, 0xDA, 0xD7],
                [0x22, 0x89, 0xF0],
                [0x2C, 0x35, 0xF0],
                [0x8D, 0x25, 0xE8],
                [0xEA, 0x20, 0xF0],
                [0xFF, 0x2F, 0xA9],
            ],
            12.0,
            18.0,
        ),
    ]
    .spacing(theme.spacing.xs)
    .align_y(iced::Alignment::Center);

    let zoom_controls = row![
        flat_control(theme, IconKind::ValueDecrease, 24.0),
        zoom_display(theme, zoom_text),
        flat_control(theme, IconKind::ValueIncrease, 24.0),
    ]
    .spacing(0)
    .align_y(iced::Alignment::Center);

    let mut content = row![
        container(palette_lead).width(Length::Fixed(theme.sizing.palette_lead_width as f32)),
        icon::view(IconKind::CursorArrow, 15.0, 15.0, theme.colors.text_muted),
        metric_text(theme, cursor_text),
    ]
    .spacing(theme.spacing.sm)
    .padding([theme.spacing.xs, theme.spacing.sm])
    .align_y(iced::Alignment::Center);

    if selection_text != "0, 0, 0, 0" {
        content = content
            .push(icon::view(
                IconKind::RectSelect,
                14.0,
                14.0,
                theme.colors.text_muted,
            ))
            .push(metric_text(theme, selection_text));
    }

    content = content
        .push(metric_text(theme, image_text))
        .push(container(zoom_controls).width(Length::Shrink));

    container(content)
        .width(Length::Fill)
        .height(Length::Fixed(theme.sizing.footer_height as f32))
        .style(move |_| {
            container::Style::default()
                .background(Background::Color(theme.colors.status_bg))
                .border(Border::default().width(1).color(theme.colors.border_subtle))
        })
        .into()
}

fn metric_text<'a, Message: 'a>(theme: &'a PintaTheme, value: String) -> Element<'a, Message> {
    text(value)
        .size(theme.typography.caption)
        .font(theme.typography.ui_regular())
        .color(theme.colors.text_primary)
        .into()
}

fn color_stack_panel<'a, Message: 'a>(theme: &'a PintaTheme) -> Element<'a, Message> {
    let back = swatch(theme, [0xFF, 0xFF, 0xFF], 22.0, 22.0);
    let front = swatch(theme, [0x00, 0x00, 0x00], 22.0, 22.0);

    let stacked = container(
        column![
            row![container(back).width(Length::Fixed(28.0))],
            row![container(front).width(Length::Fixed(28.0))],
        ]
        .spacing(-10.0),
    )
    .width(Length::Fixed(32.0))
    .height(Length::Fixed(28.0));

    row![
        stacked,
        column![
            color_action_button(theme, IconKind::ColorSwap),
            color_action_button(theme, IconKind::ColorReset),
        ]
        .spacing(1.0),
    ]
    .spacing(theme.spacing.xs)
    .align_y(iced::Alignment::Center)
    .into()
}

fn color_action_button<'a, Message: 'a>(
    theme: &'a PintaTheme,
    icon_kind: IconKind,
) -> Element<'a, Message> {
    container(icon::view(icon_kind, 10.0, 10.0, theme.colors.text_muted))
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(11.0))
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .into()
}

fn blank_chip<'a, Message: 'a>(theme: &'a PintaTheme, width: f32) -> Element<'a, Message> {
    container(text(""))
        .width(Length::Fixed(width))
        .height(Length::Fixed(28.0))
        .style(move |_| {
            container::Style::default().background(Background::Color(theme.colors.hover_bg))
        })
        .into()
}

fn swatch_row<'a, Message: 'a>(
    theme: &'a PintaTheme,
    colors: &'a [[u8; 3]],
    width: f32,
    height: f32,
) -> Element<'a, Message> {
    let row = colors.iter().fold(row!().spacing(0), |row, rgb| {
        row.push(swatch(theme, *rgb, width, height))
    });

    row.into()
}

fn flat_control<'a, Message: 'a>(
    theme: &'a PintaTheme,
    icon_kind: IconKind,
    width: f32,
) -> Element<'a, Message> {
    container(icon::view(icon_kind, 12.0, 12.0, theme.colors.text_primary))
    .width(Length::Fixed(width))
    .height(Length::Fixed(24.0))
    .center(Length::Fill)
    .into()
}

fn zoom_display<'a, Message: 'a>(
    theme: &'a PintaTheme,
    zoom_text: String,
) -> Element<'a, Message> {
    container(
        row![
            container(
                text(zoom_text)
                    .size(theme.typography.caption)
                    .font(theme.typography.ui_regular())
                    .color(theme.colors.text_primary),
            )
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center),
            container(icon::view(
                IconKind::ChevronDown,
                10.0,
                10.0,
                theme.colors.text_primary,
            ))
            .width(Length::Fixed(14.0))
            .align_x(iced::alignment::Horizontal::Center),
        ]
        .align_y(iced::Alignment::Center),
    )
    .width(Length::Fixed(theme.sizing.zoom_control_width as f32))
    .height(Length::Fixed(24.0))
    .padding([theme.spacing.xs, theme.spacing.xs])
    .style(move |_| {
        container::Style::default()
            .background(Background::Color(theme.colors.panel_bg))
            .border(Border::default().width(1).color(theme.colors.border_subtle))
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
                .background(Background::Color(iced::Color::from_rgb8(
                    rgb[0], rgb[1], rgb[2],
                )))
                .border(
                    Border::default()
                        .rounded(2.0)
                        .width(1)
                        .color(theme.colors.border_strong),
                )
        })
        .into()
}
