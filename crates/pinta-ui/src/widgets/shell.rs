use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, row, text};
use iced::{Alignment, Background, Border, Element, Length};
use pinta_theme::PintaTheme;

use crate::widgets::icon::{self, IconKind};

#[derive(Debug, Clone, Copy)]
pub struct ToolbarAction {
    pub icon: IconKind,
    pub muted: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct TitleBar<'a> {
    pub document_title: &'a str,
    pub leading_actions: &'a [ToolbarAction],
    pub trailing_actions: &'a [ToolbarAction],
    pub window_controls: &'a [IconKind],
}

#[derive(Debug, Clone, Copy)]
pub struct ToolOptionsBar<'a> {
    pub tool_label: &'a str,
    pub tool_icon: IconKind,
    pub value_label: &'a str,
    pub value: u32,
    pub mode_label: &'a str,
    pub mode_value: &'a str,
    pub mode_width: f32,
    pub shape_icon: IconKind,
    pub shape_width: f32,
}

pub fn title_bar<'a, Message: 'a>(
    theme: &'a PintaTheme,
    model: TitleBar<'a>,
) -> Element<'a, Message> {
    let leading = model.leading_actions.iter().fold(
        row!()
            .spacing(theme.spacing.xs)
            .width(Length::FillPortion(3))
            .align_y(Alignment::Center),
        |row, action| row.push(toolbar_icon(theme, action.icon, action.muted)),
    );

    let trailing_actions = model
        .trailing_actions
        .iter()
        .fold(row!().spacing(theme.spacing.xs), |row, action| {
            row.push(toolbar_icon(theme, action.icon, action.muted))
        });
    let window_controls = model
        .window_controls
        .iter()
        .fold(row!().spacing(theme.spacing.xs), |row, icon_kind| {
            row.push(round_control(theme, *icon_kind))
        });
    let trailing = row![trailing_actions, window_controls]
        .spacing(theme.spacing.xs)
        .width(Length::FillPortion(3))
        .align_y(Alignment::Center);

    container(
        row![
            leading,
            container(
                text(format!("{} - Pinta", model.document_title))
                    .size(theme.typography.toolbar)
                    .font(theme.typography.ui_medium())
                    .color(theme.colors.text_primary),
            )
            .width(Length::FillPortion(4))
            .padding([0.0, theme.spacing.md])
            .center_y(Length::Fill),
            trailing,
        ]
        .align_y(Alignment::Center),
    )
    .padding([theme.spacing.xs, theme.spacing.md])
    .height(Length::Fixed(theme.sizing.top_bar_height as f32))
    .width(Length::Fill)
    .style(move |_| {
        container::Style::default()
            .background(Background::Color(theme.colors.toolbar_bg))
            .border(Border::default().width(1).color(theme.colors.border_subtle))
    })
    .into()
}

pub fn tool_options_bar<'a, Message: 'a>(
    theme: &'a PintaTheme,
    model: ToolOptionsBar<'a>,
) -> Element<'a, Message> {
    container(
        row![
            text(model.tool_label)
                .size(theme.typography.toolbar)
                .font(theme.typography.ui_regular())
                .color(theme.colors.text_primary),
            toolbar_icon(theme, model.tool_icon, false),
            text(model.value_label)
                .size(theme.typography.toolbar)
                .font(theme.typography.ui_regular())
                .color(theme.colors.text_primary),
            segmented_value(theme, model.value),
            text(model.mode_label)
                .size(theme.typography.toolbar)
                .font(theme.typography.ui_regular())
                .color(theme.colors.text_primary),
            dropdown_chip(theme, model.mode_value, model.mode_width),
            icon_dropdown_chip(theme, model.shape_icon, model.shape_width),
        ]
        .spacing(theme.spacing.sm)
        .align_y(Alignment::Center),
    )
    .padding([theme.spacing.xs, theme.spacing.md])
    .width(Length::Fill)
    .height(Length::Fixed(theme.sizing.tool_options_height as f32))
    .into()
}

pub fn layer_row<'a, Message: 'a>(
    theme: &'a PintaTheme,
    label: &'a str,
    selected: bool,
) -> Element<'a, Message> {
    container(
        row![
            icon::view(IconKind::ViewReveal, 18.0, 18.0, theme.colors.text_primary),
            container(
                text(label)
                    .size(theme.typography.body)
                    .font(theme.typography.ui_regular())
                    .color(theme.colors.text_primary),
            )
            .width(Length::Fill)
            .center_y(Length::Fill),
            container(icon::view(
                IconKind::ThumbnailSample,
                60.0,
                30.0,
                theme.colors.text_primary,
            ))
            .width(Length::Fixed(60.0))
            .height(Length::Fixed(30.0))
            .style(move |_| {
                container::Style::default()
                    .background(Background::Color(theme.colors.canvas_page_bg))
                    .border(Border::default().width(1).color(theme.colors.border_subtle))
            }),
        ]
        .spacing(theme.spacing.sm)
        .align_y(Alignment::Center),
    )
    .height(Length::Fixed(theme.sizing.layer_row_height as f32))
    .width(Length::Fill)
    .padding([0.0, theme.spacing.xs])
    .style(move |_| {
        container::Style::default()
            .background(Background::Color(if selected {
                theme.colors.selected_bg
            } else {
                theme.colors.panel_bg
            }))
            .border(Border::default().width(1).color(theme.colors.border_subtle))
    })
    .into()
}

pub fn history_row<'a, Message: 'a>(
    theme: &'a PintaTheme,
    icon_kind: IconKind,
    label: &'a str,
) -> Element<'a, Message> {
    container(
        row![
            icon::view(icon_kind, 14.0, 14.0, theme.colors.text_primary),
            text(label)
                .size(theme.typography.caption)
                .font(theme.typography.ui_regular())
                .color(theme.colors.text_primary),
        ]
        .spacing(theme.spacing.md)
        .align_y(Alignment::Center),
    )
    .height(Length::Fixed(theme.sizing.history_row_height as f32))
    .width(Length::Fill)
    .padding([0.0, theme.spacing.xs])
    .style(move |_| {
        container::Style::default()
            .background(Background::Color(theme.colors.selected_bg))
            .border(Border::default().width(1).color(theme.colors.border_subtle))
    })
    .into()
}

fn toolbar_icon<'a, Message: 'a>(
    theme: &'a PintaTheme,
    icon_kind: IconKind,
    muted: bool,
) -> Element<'a, Message> {
    let icon_color = if muted {
        theme.colors.icon_disabled
    } else {
        theme.colors.text_primary
    };

    container(icon::view(icon_kind, 19.0, 19.0, icon_color))
        .width(Length::Fixed(22.0))
        .height(Length::Fixed(22.0))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
}

fn round_control<'a, Message: 'a>(
    theme: &'a PintaTheme,
    icon_kind: IconKind,
) -> Element<'a, Message> {
    container(icon::view(icon_kind, 14.0, 14.0, theme.colors.text_primary))
        .width(Length::Fixed(24.0))
        .height(Length::Fixed(24.0))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .style(move |_| {
            container::Style::default()
                .background(Background::Color(theme.colors.hover_bg))
                .border(Border::default().rounded(theme.radii.sm))
        })
        .into()
}

fn segmented_value<'a, Message: 'a>(theme: &'a PintaTheme, value: u32) -> Element<'a, Message> {
    container(
        row![
            container(
                text(value.to_string())
                    .size(theme.typography.toolbar)
                    .font(theme.typography.ui_regular())
                    .color(theme.colors.text_primary),
            )
            .width(Length::FillPortion(2))
            .center_y(Length::Fill),
            stepper_button(theme, "-"),
            stepper_button(theme, "+"),
        ]
        .align_y(Alignment::Center),
    )
    .width(Length::Fixed(168.0))
    .height(Length::Fixed(34.0))
    .padding([0.0, theme.spacing.sm])
    .style(move |_| {
        container::Style::default()
            .background(Background::Color(theme.colors.hover_bg))
            .border(
                Border::default()
                    .rounded(theme.radii.md)
                    .width(1)
                    .color(theme.colors.border_subtle),
            )
    })
    .into()
}

fn stepper_button<'a, Message: 'a>(theme: &'a PintaTheme, label: &'a str) -> Element<'a, Message> {
    container(
        text(label)
            .size(theme.typography.toolbar)
            .font(theme.typography.ui_medium())
            .color(theme.colors.text_primary),
    )
    .width(Length::Fixed(26.0))
    .center(Length::Fill)
    .style(move |_| {
        container::Style::default()
            .border(Border::default().width(1).color(theme.colors.border_subtle))
    })
    .into()
}

fn dropdown_chip<'a, Message: 'a>(
    theme: &'a PintaTheme,
    label: &'a str,
    width: f32,
) -> Element<'a, Message> {
    container(
        row![
            text(label)
                .size(theme.typography.toolbar)
                .font(theme.typography.ui_regular())
                .color(theme.colors.text_primary),
            icon::view(IconKind::ChevronDown, 12.0, 12.0, theme.colors.text_primary),
        ]
        .spacing(theme.spacing.sm)
        .align_y(Alignment::Center),
    )
    .width(Length::Fixed(width))
    .height(Length::Fixed(34.0))
    .padding([0.0, theme.spacing.sm])
    .style(move |_| {
        container::Style::default()
            .background(Background::Color(theme.colors.hover_bg))
            .border(
                Border::default()
                    .rounded(theme.radii.md)
                    .width(1)
                    .color(theme.colors.border_subtle),
            )
    })
    .into()
}

fn icon_dropdown_chip<'a, Message: 'a>(
    theme: &'a PintaTheme,
    icon_kind: IconKind,
    width: f32,
) -> Element<'a, Message> {
    container(
        row![
            icon::view(icon_kind, 14.0, 14.0, theme.colors.text_primary),
            icon::view(IconKind::ChevronDown, 12.0, 12.0, theme.colors.text_primary),
        ]
        .spacing(theme.spacing.xs)
        .align_y(Alignment::Center),
    )
    .width(Length::Fixed(width))
    .height(Length::Fixed(34.0))
    .padding([0.0, theme.spacing.sm])
    .style(move |_| {
        container::Style::default()
            .background(Background::Color(theme.colors.hover_bg))
            .border(
                Border::default()
                    .rounded(theme.radii.md)
                    .width(1)
                    .color(theme.colors.border_subtle),
            )
    })
    .into()
}
