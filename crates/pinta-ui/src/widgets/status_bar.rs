use iced::widget::{column, container, mouse_area, row, stack, text};
use iced::{Background, Border, Element, Length, mouse};
use pinta_theme::PintaTheme;

use crate::widgets::icon::{self, IconKind};

const CURRENT_COLOR_CONTROL_SIZE: f32 = 42.0;
const PALETTE_ROW_HEIGHT: f32 = 20.0;
const PALETTE_ROW_GAP: f32 = 2.0;
const PALETTE_BLOCK_HEIGHT: f32 = (PALETTE_ROW_HEIGHT * 2.0) + PALETTE_ROW_GAP;
const RECENT_COLOR_COLUMNS: usize = 5;

const FIXED_PALETTE: [[u8; 3]; 24] = [
    [0xFF, 0xFF, 0xFF],
    [0x00, 0x00, 0x00],
    [0xA0, 0xA0, 0xA0],
    [0x80, 0x80, 0x80],
    [0x40, 0x40, 0x40],
    [0x30, 0x30, 0x30],
    [0xFF, 0x00, 0x00],
    [0xFF, 0x7F, 0x7F],
    [0xFF, 0x6A, 0x00],
    [0xFF, 0xB2, 0x7F],
    [0xFF, 0xD8, 0x00],
    [0xFF, 0xE9, 0x7F],
    [0xB6, 0xFF, 0x00],
    [0xDA, 0xFF, 0x7F],
    [0x4C, 0xFF, 0x00],
    [0xA5, 0xFF, 0x7F],
    [0x00, 0xFF, 0x21],
    [0x7F, 0xFF, 0x8E],
    [0x00, 0xFF, 0x90],
    [0x7F, 0xFF, 0xC5],
    [0x00, 0xFF, 0xFF],
    [0x7F, 0xFF, 0xFF],
    [0x00, 0x94, 0xFF],
    [0x7F, 0xC9, 0xFF],
];

pub fn view<'a, Message: Clone + 'a>(
    theme: &'a PintaTheme,
    primary_color: [u8; 3],
    secondary_color: [u8; 3],
    recent_colors: &'a [[u8; 3]],
    cursor_text: String,
    image_text: String,
    selection_text: String,
    zoom_text: String,
    on_pick_primary: impl Fn([u8; 3]) -> Message + Copy + 'a,
    on_pick_secondary: impl Fn([u8; 3]) -> Message + Copy + 'a,
    on_swap: Message,
    on_reset: Message,
) -> Element<'a, Message> {
    let palette_lead = row![
        color_stack_panel(
            theme,
            primary_color,
            secondary_color,
            on_swap.clone(),
            on_reset.clone(),
        ),
        quick_palette_panel(theme, recent_colors, on_pick_primary, on_pick_secondary),
        palette_grid(theme, on_pick_primary, on_pick_secondary),
    ]
    .spacing(theme.spacing.sm)
    .align_y(iced::Alignment::Center);

    let zoom_controls = row![
        flat_control(theme, IconKind::ValueDecrease, 26.0),
        zoom_display(theme, zoom_text),
        flat_control(theme, IconKind::ValueIncrease, 26.0),
    ]
    .spacing(0)
    .align_y(iced::Alignment::Center);

    let mut content = row![
        container(palette_lead).width(Length::Fixed(theme.sizing.palette_lead_width as f32)),
        icon::view(IconKind::CursorArrow, 15.0, 15.0, theme.colors.status_fg),
        metric_text(theme, cursor_text),
    ]
    .spacing(theme.spacing.md)
    .padding([2.0, theme.spacing.md])
    .align_y(iced::Alignment::Center);

    if selection_text != "0, 0, 0, 0" {
        content = content
            .push(icon::view(
                IconKind::RectSelect,
                14.0,
                14.0,
                theme.colors.status_fg,
            ))
            .push(metric_text(theme, selection_text));
    }

    content = content
        .push(metric_text(theme, image_text))
        .push(container(text("")).width(Length::Fill))
        .push(container(zoom_controls).width(Length::Shrink));

    container(content)
        .width(Length::Fill)
        .height(Length::Fixed(
            theme
                .sizing
                .footer_height
                .saturating_sub(theme.sizing.footer_inset_top) as f32,
        ))
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
    .color(theme.colors.status_fg)
        .into()
}

fn quick_palette_panel<'a, Message: Clone + 'a>(
    theme: &'a PintaTheme,
    recent_colors: &'a [[u8; 3]],
    on_pick_primary: impl Fn([u8; 3]) -> Message + Copy + 'a,
    on_pick_secondary: impl Fn([u8; 3]) -> Message + Copy + 'a,
) -> Element<'a, Message> {
    let recent_top = recent_row(theme, recent_colors, 0, on_pick_primary, on_pick_secondary);
    let recent_bottom = recent_row(
        theme,
        recent_colors,
        RECENT_COLOR_COLUMNS,
        on_pick_primary,
        on_pick_secondary,
    );

    container(column![recent_top, recent_bottom].spacing(PALETTE_ROW_GAP))
        .width(Length::Shrink)
        .height(Length::Fixed(PALETTE_BLOCK_HEIGHT))
        .into()
}

fn palette_grid<'a, Message: Clone + 'a>(
    theme: &'a PintaTheme,
    on_pick_primary: impl Fn([u8; 3]) -> Message + Copy + 'a,
    on_pick_secondary: impl Fn([u8; 3]) -> Message + Copy + 'a,
) -> Element<'a, Message> {
    let top = palette_row(
        theme,
        &FIXED_PALETTE,
        0,
        on_pick_primary,
        on_pick_secondary,
    );

    let bottom = palette_row(
        theme,
        &FIXED_PALETTE,
        1,
        on_pick_primary,
        on_pick_secondary,
    );

    container(column![top, bottom].spacing(PALETTE_ROW_GAP))
        .width(Length::Shrink)
        .height(Length::Fixed(PALETTE_BLOCK_HEIGHT))
        .into()
}

fn palette_row<'a, Message: Clone + 'a>(
    theme: &'a PintaTheme,
    palette: &'a [[u8; 3]],
    row_index: usize,
    on_pick_primary: impl Fn([u8; 3]) -> Message + Copy + 'a,
    on_pick_secondary: impl Fn([u8; 3]) -> Message + Copy + 'a,
) -> Element<'a, Message> {
    let mut row_widget = row!().spacing(0).align_y(iced::Alignment::Center);

    for index in (row_index..palette.len()).step_by(2) {
        let color = palette[index];
        row_widget = row_widget.push(interactive_swatch(
            theme,
            color,
            PALETTE_ROW_HEIGHT,
            PALETTE_ROW_HEIGHT,
            on_pick_primary(color),
            on_pick_secondary(color),
        ));
    }

    if palette.len() % 2 != 0 && row_index == 1 {
        row_widget = row_widget.push(empty_recent_swatch(
            theme,
            PALETTE_ROW_HEIGHT,
            PALETTE_ROW_HEIGHT,
        ));
    }

    row_widget
        .height(Length::Fixed(PALETTE_ROW_HEIGHT))
    .spacing(0)
        .into()
}

fn recent_row<'a, Message: Clone + 'a>(
    theme: &'a PintaTheme,
    recent_colors: &'a [[u8; 3]],
    start: usize,
    on_pick_primary: impl Fn([u8; 3]) -> Message + Copy + 'a,
    on_pick_secondary: impl Fn([u8; 3]) -> Message + Copy + 'a,
) -> Element<'a, Message> {
    let mut row_widget = row!().spacing(0).align_y(iced::Alignment::Center);

    for index in start..start + RECENT_COLOR_COLUMNS {
        row_widget = row_widget.push(match recent_colors.get(index).copied() {
            Some(color) => interactive_swatch(
                theme,
                color,
                PALETTE_ROW_HEIGHT,
                PALETTE_ROW_HEIGHT,
                on_pick_primary(color),
                on_pick_secondary(color),
            ),
            None => empty_recent_swatch(theme, PALETTE_ROW_HEIGHT, PALETTE_ROW_HEIGHT),
        });
    }

    row_widget.height(Length::Fixed(PALETTE_ROW_HEIGHT)).into()
}

fn color_stack_panel<'a, Message: Clone + 'a>(
    theme: &'a PintaTheme,
    primary_color: [u8; 3],
    secondary_color: [u8; 3],
    on_swap: Message,
    on_reset: Message,
) -> Element<'a, Message> {
    stack![
        empty_recent_swatch(theme, CURRENT_COLOR_CONTROL_SIZE, CURRENT_COLOR_CONTROL_SIZE),
        positioned_layer(
            current_color_swatch(theme, secondary_color),
            17.0,
            16.0,
            24.0,
            24.0,
        ),
        positioned_layer(
            current_color_swatch(theme, primary_color),
            4.0,
            3.0,
            24.0,
            24.0,
        ),
        positioned_layer(
            color_action_button(theme, IconKind::ColorSwap, on_swap, 15.0, 15.0),
            27.0,
            2.0,
            15.0,
            15.0,
        ),
        positioned_layer(
            color_action_button(theme, IconKind::ColorReset, on_reset, 15.0, 15.0),
            2.0,
            27.0,
            15.0,
            15.0,
        ),
    ]
    .width(Length::Fixed(CURRENT_COLOR_CONTROL_SIZE))
    .height(Length::Fixed(CURRENT_COLOR_CONTROL_SIZE))
    .into()
}

fn color_action_button<'a, Message: Clone + 'a>(
    theme: &'a PintaTheme,
    icon_kind: IconKind,
    on_press: Message,
    width: f32,
    height: f32,
) -> Element<'a, Message> {
    mouse_area(
        container(icon::view(icon_kind, 10.0, 10.0, theme.colors.status_fg))
            .width(Length::Fixed(width))
            .height(Length::Fixed(height))
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
    )
    .on_press(on_press)
    .interaction(mouse::Interaction::Pointer)
    .into()
}

fn current_color_swatch<'a, Message: 'a>(
    _theme: &'a PintaTheme,
    rgb: [u8; 3],
) -> Element<'a, Message> {
    container(container(text("")).width(Length::Fill).height(Length::Fill).style(
        move |_| {
            container::Style::default().background(Background::Color(iced::Color::from_rgb8(
                rgb[0], rgb[1], rgb[2],
            )))
        },
    ))
    .width(Length::Fixed(24.0))
    .height(Length::Fixed(24.0))
    .padding(1.0)
    .style(move |_| {
        container::Style::default()
            .background(Background::Color(iced::Color::WHITE))
            .border(Border::default().width(1).color(iced::Color::BLACK))
    })
    .into()
}

fn positioned_layer<'a, Message: 'a>(
    child: Element<'a, Message>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> Element<'a, Message> {
    container(column![
        container(text("")).height(Length::Fixed(y)),
        row![
            container(text("")).width(Length::Fixed(x)),
            container(child)
                .width(Length::Fixed(width))
                .height(Length::Fixed(height)),
        ]
    ])
    .width(Length::Fixed(CURRENT_COLOR_CONTROL_SIZE))
    .height(Length::Fixed(CURRENT_COLOR_CONTROL_SIZE))
    .into()
}

fn empty_recent_swatch<'a, Message: 'a>(theme: &'a PintaTheme, width: f32, height: f32) -> Element<'a, Message> {
    container(text(""))
        .width(Length::Fixed(width))
        .height(Length::Fixed(height))
        .style(move |_| {
            container::Style::default()
                .background(Background::Color(theme.colors.hover_bg))
                .border(Border::default().width(1).color(theme.colors.border_subtle))
        })
        .into()
}

fn interactive_swatch<'a, Message: Clone + 'a>(
    theme: &'a PintaTheme,
    rgb: [u8; 3],
    width: f32,
    height: f32,
    on_left_press: Message,
    on_right_press: Message,
) -> Element<'a, Message> {
    mouse_area(swatch(theme, rgb, width, height))
        .on_press(on_left_press)
        .on_right_press(on_right_press)
        .interaction(mouse::Interaction::Pointer)
        .into()
}

fn flat_control<'a, Message: 'a>(
    theme: &'a PintaTheme,
    icon_kind: IconKind,
    width: f32,
) -> Element<'a, Message> {
    container(icon::view(icon_kind, 12.0, 12.0, theme.colors.status_fg))
        .width(Length::Fixed(width))
        .height(Length::Fixed(28.0))
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
                    .color(theme.colors.status_fg),
            )
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center),
            container(icon::view(
                IconKind::ChevronDown,
                10.0,
                10.0,
                theme.colors.status_fg,
            ))
            .width(Length::Fixed(14.0))
            .align_x(iced::alignment::Horizontal::Center),
        ]
        .align_y(iced::Alignment::Center),
    )
    .width(Length::Fixed(theme.sizing.zoom_control_width as f32))
    .height(Length::Fixed(28.0))
    .padding([theme.spacing.xs, theme.spacing.xs])
    .style(move |_| {
        container::Style::default()
            .background(Background::Color(theme.colors.status_bg))
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
                .border(Border::default().rounded(0.0).width(0).color(theme.colors.border_subtle))
        })
        .into()
}
