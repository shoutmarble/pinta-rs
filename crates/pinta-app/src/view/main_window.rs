use iced::widget::{column, container, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length};
use pinta_ui::widgets::{canvas_viewport::CanvasViewport, icon::{self, IconKind}, pad, status_bar, toolbox};

use crate::message::AppMessage;
use crate::state::{AppState, ToolKind};

pub fn view(state: &AppState) -> Element<'_, AppMessage> {
    let theme = &state.theme;

    let tools = [
        toolbox::ToolboxItem {
            icon: IconKind::MovePixels,
            selected: false,
            on_press: None,
        },
        toolbox::ToolboxItem {
            icon: IconKind::MoveSelection,
            selected: false,
            on_press: None,
        },
        toolbox::ToolboxItem {
            icon: IconKind::Zoom,
            selected: state.active_tool == ToolKind::Zoom,
            on_press: Some(AppMessage::ToolSelected(ToolKind::Zoom)),
        },
        toolbox::ToolboxItem {
            icon: IconKind::Pan,
            selected: false,
            on_press: None,
        },
        toolbox::ToolboxItem {
            icon: IconKind::RectSelect,
            selected: false,
            on_press: None,
        },
        toolbox::ToolboxItem {
            icon: IconKind::EllipseSelect,
            selected: false,
            on_press: None,
        },
        toolbox::ToolboxItem {
            icon: IconKind::LassoSelect,
            selected: false,
            on_press: None,
        },
        toolbox::ToolboxItem {
            icon: IconKind::MagicWand,
            selected: false,
            on_press: None,
        },
        toolbox::ToolboxItem {
            icon: IconKind::Paintbrush,
            selected: state.active_tool == ToolKind::Paintbrush,
            on_press: Some(AppMessage::ToolSelected(ToolKind::Paintbrush)),
        },
        toolbox::ToolboxItem {
            icon: IconKind::Pencil,
            selected: state.active_tool == ToolKind::Pencil,
            on_press: Some(AppMessage::ToolSelected(ToolKind::Pencil)),
        },
        toolbox::ToolboxItem {
            icon: IconKind::Eraser,
            selected: state.active_tool == ToolKind::Eraser,
            on_press: Some(AppMessage::ToolSelected(ToolKind::Eraser)),
        },
        toolbox::ToolboxItem {
            icon: IconKind::PaintBucket,
            selected: false,
            on_press: None,
        },
        toolbox::ToolboxItem {
            icon: IconKind::Gradient,
            selected: false,
            on_press: None,
        },
        toolbox::ToolboxItem {
            icon: IconKind::ColorPicker,
            selected: false,
            on_press: None,
        },
        toolbox::ToolboxItem {
            icon: IconKind::Text,
            selected: false,
            on_press: None,
        },
        toolbox::ToolboxItem {
            icon: IconKind::LineCurve,
            selected: false,
            on_press: None,
        },
        toolbox::ToolboxItem {
            icon: IconKind::Rectangle,
            selected: false,
            on_press: None,
        },
        toolbox::ToolboxItem {
            icon: IconKind::RoundedRectangle,
            selected: false,
            on_press: None,
        },
        toolbox::ToolboxItem {
            icon: IconKind::Ellipse,
            selected: false,
            on_press: None,
        },
        toolbox::ToolboxItem {
            icon: IconKind::Freeform,
            selected: false,
            on_press: None,
        },
        toolbox::ToolboxItem {
            icon: IconKind::CloneStamp,
            selected: false,
            on_press: None,
        },
        toolbox::ToolboxItem {
            icon: IconKind::Recolor,
            selected: false,
            on_press: None,
        },
        toolbox::ToolboxItem {
            icon: IconKind::MagicWand,
            selected: false,
            on_press: None,
        },
    ];

    let header = container(
        row![
            row![
                chrome_button(theme, IconKind::DocumentNew, false),
                chrome_button(theme, IconKind::OpenImage, false),
                chrome_button(theme, IconKind::Save, false),
                chrome_button(theme, IconKind::Undo, true),
                chrome_button(theme, IconKind::Redo, true),
                chrome_button(theme, IconKind::Scissors, false),
                chrome_button(theme, IconKind::Duplicate, false),
            ]
            .spacing(theme.spacing.xs)
            .width(Length::FillPortion(2))
            .align_y(Alignment::Center),
            container(text(format!("{} - Pinta", state.document_name)).size(theme.typography.toolbar))
                .width(Length::FillPortion(5))
                .padding([0, theme.spacing.md])
                .center_y(Length::Fill),
            row![
                chrome_button(theme, IconKind::ImageLandscape, false),
                chrome_button(theme, IconKind::Adjustments, false),
                chrome_button(theme, IconKind::Effects, false),
                chrome_button(theme, IconKind::Menu, false),
                round_control(theme, IconKind::ChevronDown),
                round_control(theme, IconKind::ChevronUp),
                round_control(theme, IconKind::WindowClose),
            ]
            .spacing(theme.spacing.xs)
            .width(Length::FillPortion(2))
            .align_y(Alignment::Center),
        ]
        .align_y(Alignment::Center),
    )
    .padding([theme.spacing.xs, theme.spacing.md])
    .height(Length::Fixed(theme.sizing.top_bar_height as f32))
    .width(Length::Fill)
    .style(move |_| {
        iced::widget::container::Style::default()
            .background(Background::Color(theme.colors.toolbar_bg))
            .border(Border::default().width(1).color(theme.colors.border_subtle))
    });

    let tool_options = container(
        row![
            text("Tool:").size(theme.typography.toolbar),
            chrome_button(theme, IconKind::Paintbrush, false),
            text("Brush width:").size(theme.typography.toolbar),
            segmented_value(theme, state.brush_width),
            text("Type:").size(theme.typography.toolbar),
            dropdown_chip(theme, "Normal", 126.0),
            icon_dropdown_chip(theme, IconKind::LineCurve, 48.0),
        ]
        .spacing(theme.spacing.md)
        .align_y(Alignment::Center),
    )
    .padding([theme.spacing.xs, theme.spacing.md])
    .width(Length::Fill)
    .height(Length::Fixed(theme.sizing.tool_options_height as f32));

    let viewport = CanvasViewport::new(theme.clone(), state.viewport.clone())
        .view()
        .map(AppMessage::from);

    let layers_body: Element<'_, AppMessage> = column(
        state.layers.iter().map(|layer| {
            container(
                row![
                    icon::view(IconKind::Eye, 18.0, 18.0, theme.colors.text_muted),
                    text(layer.clone()).size(theme.typography.body),
                    container(icon::view(IconKind::ThumbnailSample, 60.0, 34.0, theme.colors.text_primary))
                        .width(Length::Fixed(60.0))
                        .height(Length::Fixed(34.0))
                        .style(move |_| {
                            iced::widget::container::Style::default()
                                .background(iced::Background::Color(theme.colors.canvas_page_bg))
                                .border(
                                    iced::Border::default()
                                        .width(1)
                                        .color(theme.colors.border_strong),
                                )
                        }),
                ]
                .spacing(theme.spacing.md)
                .align_y(Alignment::Center),
            )
            .height(Length::Fixed(theme.sizing.layer_row_height as f32))
            .width(Length::Fill)
            .padding([0, theme.spacing.md])
            .style(move |_| {
                iced::widget::container::Style::default()
                    .background(iced::Background::Color(theme.colors.selected_bg))
                    .border(
                        iced::Border::default()
                            .width(1)
                            .color(theme.colors.border_subtle),
                    )
            })
            .into()
        }),
    )
    .into();

    let history_body: Element<'_, AppMessage> = scrollable(
        column(
            state
                .history
                .iter()
                .cloned()
                .map(|entry| {
                    container(
                        row![
                            icon::view(IconKind::OpenImage, 14.0, 14.0, theme.colors.text_muted),
                            text(entry).size(theme.typography.caption),
                        ]
                        .spacing(theme.spacing.md)
                        .align_y(Alignment::Center),
                    )
                    .height(Length::Fixed(theme.sizing.history_row_height as f32))
                    .width(Length::Fill)
                    .padding([0, theme.spacing.md])
                    .style(move |_| {
                        iced::widget::container::Style::default()
                            .background(iced::Background::Color(theme.colors.panel_bg))
                    })
                    .into()
                }),
        )
        .spacing(theme.spacing.xs),
    )
    .into();

    let right_sidebar = column![
        container(pad::view(theme, "Layers", layers_body, vec![
            IconKind::Add,
            IconKind::Duplicate,
            IconKind::Delete,
            IconKind::Merge,
            IconKind::MoveUp,
            IconKind::MoveDown,
            IconKind::More,
        ])).height(Length::FillPortion(1)),
        container(pad::view(theme, "History", history_body, vec![IconKind::Undo, IconKind::Redo])).height(Length::FillPortion(1)),
    ]
    .spacing(0)
    .width(Length::Fixed(theme.sizing.right_sidebar_width as f32));

    let main = row![
        container(toolbox::view(theme, tools)).padding([theme.spacing.md, 0]),
        container(viewport).width(Length::Fill).height(Length::Fill),
        container(right_sidebar).padding([theme.spacing.md, 0]),
    ]
    .height(Length::Fill)
    .spacing(0);

    let footer = status_bar::view(
        theme,
        state.cursor_text.clone(),
        state.image_text.clone(),
        state.selection_text.clone(),
        format!("{}%", state.zoom_percent),
    );

    container(column![header, tool_options, main, footer])
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| {
            iced::widget::container::Style::default()
                .background(Background::Color(theme.colors.window_bg))
        })
        .into()
}

fn chrome_button<'a>(theme: &'a pinta_theme::PintaTheme, icon_kind: IconKind, muted: bool) -> Element<'a, AppMessage> {
    let icon_color = if muted {
        Color::from_rgba(
            theme.colors.text_muted.r,
            theme.colors.text_muted.g,
            theme.colors.text_muted.b,
            0.30,
        )
    } else {
        theme.colors.text_primary
    };

    container(icon::view(icon_kind, 18.0, 18.0, icon_color))
        .width(Length::Fixed(24.0))
        .height(Length::Fixed(24.0))
        .center(Length::Fill)
        .into()
}

fn round_control<'a>(theme: &'a pinta_theme::PintaTheme, icon_kind: IconKind) -> Element<'a, AppMessage> {
    container(icon::view(icon_kind, 14.0, 14.0, theme.colors.text_primary))
        .width(Length::Fixed(24.0))
        .height(Length::Fixed(24.0))
        .center(Length::Fill)
        .style(move |_| {
            iced::widget::container::Style::default()
                .background(Background::Color(theme.colors.hover_bg))
                .border(Border::default().rounded(theme.radii.sm))
        })
        .into()
}

fn segmented_value<'a>(theme: &'a pinta_theme::PintaTheme, value: u32) -> Element<'a, AppMessage> {
    container(
        row![
            container(text(value.to_string()).size(theme.typography.toolbar))
                .width(Length::FillPortion(2))
                .center_y(Length::Fill),
            stepper_button(theme, "-"),
            stepper_button(theme, "+"),
        ]
        .align_y(Alignment::Center),
    )
    .width(Length::Fixed(176.0))
    .height(Length::Fixed(36.0))
    .padding([0, theme.spacing.sm])
    .style(move |_| {
        iced::widget::container::Style::default()
            .background(Background::Color(theme.colors.hover_bg))
            .border(Border::default().rounded(theme.radii.md).width(1).color(theme.colors.border_subtle))
    })
    .into()
}

fn stepper_button<'a>(theme: &'a pinta_theme::PintaTheme, label: &'a str) -> Element<'a, AppMessage> {
    container(text(label).size(theme.typography.toolbar))
        .width(Length::Fixed(28.0))
        .center(Length::Fill)
        .style(move |_| {
            iced::widget::container::Style::default()
                .border(Border::default().width(1).color(theme.colors.border_subtle))
        })
        .into()
}

fn dropdown_chip<'a>(theme: &'a pinta_theme::PintaTheme, label: &'a str, width: f32) -> Element<'a, AppMessage> {
    container(
        row![
            text(label).size(theme.typography.toolbar),
            icon::view(IconKind::ChevronDown, 12.0, 12.0, theme.colors.text_primary),
        ]
        .spacing(theme.spacing.sm)
        .align_y(Alignment::Center),
    )
    .width(Length::Fixed(width))
    .height(Length::Fixed(36.0))
    .padding([0, theme.spacing.sm])
    .style(move |_| {
        iced::widget::container::Style::default()
            .background(Background::Color(theme.colors.hover_bg))
            .border(Border::default().rounded(theme.radii.md).width(1).color(theme.colors.border_subtle))
    })
    .into()
}

fn icon_dropdown_chip<'a>(theme: &'a pinta_theme::PintaTheme, icon_kind: IconKind, width: f32) -> Element<'a, AppMessage> {
    container(
        row![
            icon::view(icon_kind, 14.0, 14.0, theme.colors.text_primary),
            icon::view(IconKind::ChevronDown, 12.0, 12.0, theme.colors.text_primary),
        ]
        .spacing(theme.spacing.xs)
        .align_y(Alignment::Center),
    )
    .width(Length::Fixed(width))
    .height(Length::Fixed(36.0))
    .padding([0, theme.spacing.sm])
    .style(move |_| {
        iced::widget::container::Style::default()
            .background(Background::Color(theme.colors.hover_bg))
            .border(Border::default().rounded(theme.radii.md).width(1).color(theme.colors.border_subtle))
    })
    .into()
}
