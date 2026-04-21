use iced::widget::{column, container, row, scrollable};
use iced::{Background, Border, Element, Length};
use pinta_ui::widgets::{
    canvas_viewport::CanvasViewport, icon::IconKind, pad, shell, status_bar, toolbox,
};

use crate::message::AppMessage;
use crate::state::{AppState, ToolKind};

pub fn view(state: &AppState) -> Element<'_, AppMessage> {
    let theme = &state.theme;

    let tools = ToolKind::toolbox_order()
        .iter()
        .copied()
        .map(|tool| toolbox::ToolboxItem {
            icon: tool.icon_kind(),
            selected: state.active_tool == tool,
            on_press: Some(AppMessage::ToolSelected(tool)),
        })
        .chain(std::iter::once(toolbox::ToolboxItem {
            icon: IconKind::Effects,
            selected: false,
            on_press: None,
        }));

    let header = container(shell::title_bar(
        theme,
        shell::TitleBar {
            document_title: &state.document_name,
            leading_actions: &[
                shell::ToolbarAction {
                    icon: IconKind::DocumentNew,
                    muted: false,
                },
                shell::ToolbarAction {
                    icon: IconKind::OpenImage,
                    muted: false,
                },
                shell::ToolbarAction {
                    icon: IconKind::Save,
                    muted: false,
                },
                shell::ToolbarAction {
                    icon: IconKind::Undo,
                    muted: true,
                },
                shell::ToolbarAction {
                    icon: IconKind::Redo,
                    muted: true,
                },
                shell::ToolbarAction {
                    icon: IconKind::Scissors,
                    muted: false,
                },
                shell::ToolbarAction {
                    icon: IconKind::Clipboard,
                    muted: false,
                },
            ],
            trailing_actions: &[
                shell::ToolbarAction {
                    icon: IconKind::ViewReveal,
                    muted: false,
                },
                shell::ToolbarAction {
                    icon: IconKind::ImageGeneric,
                    muted: false,
                },
                shell::ToolbarAction {
                    icon: IconKind::Adjustments,
                    muted: false,
                },
                shell::ToolbarAction {
                    icon: IconKind::Effects,
                    muted: false,
                },
                shell::ToolbarAction {
                    icon: IconKind::Menu,
                    muted: false,
                },
            ],
            window_controls: &[
                IconKind::WindowMinimize,
                IconKind::WindowMaximize,
                IconKind::WindowClose,
            ],
        },
    ));

    let tool_options = container(shell::tool_options_bar(
        theme,
        shell::ToolOptionsBar {
            tool_label: "Tool:",
            tool_icon: state.active_tool.icon_kind(),
            value_label: "Brush width:",
            value: state.brush_width,
            mode_label: "Type:",
            mode_value: "Normal",
            mode_width: 126.0,
            shape_icon: IconKind::LineCurve,
            shape_width: 48.0,
        },
    ));

    let viewport = CanvasViewport::new(
        theme.clone(),
        state.viewport.clone(),
        state.active_tool.icon_kind(),
        state.mock_scenario.active,
    )
        .view()
        .map(AppMessage::from);

    let workspace = container(viewport)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| {
            iced::widget::container::Style::default()
                .background(Background::Color(theme.colors.canvas_surround_bg))
                .border(
                    Border::default()
                        .width(1)
                        .color(theme.colors.border_strong),
                )
        });

    let layers_body: Element<'_, AppMessage> = column(
        state
            .layers
            .iter()
            .map(|layer| shell::layer_row(theme, layer, true)),
    )
    .into();

    let history_body: Element<'_, AppMessage> = scrollable(
        column(
            state
                .history
                .iter()
                .map(|entry| shell::history_row(theme, IconKind::OpenImage, entry)),
        )
        .spacing(theme.spacing.xs),
    )
    .into();

    let sidebar_top_inset = theme.sizing.right_sidebar_top_inset as f32;
    let sidebar_gap = theme.sizing.right_sidebar_gap as f32;

    let right_sidebar = column![
        container(column![]).height(Length::Fixed(sidebar_top_inset)),
        container(pad::view(
            theme,
            "Layers",
            Some(IconKind::Duplicate),
            layers_body,
            vec![
                IconKind::Add,
                IconKind::Duplicate,
                IconKind::Delete,
                IconKind::Merge,
                IconKind::MoveUp,
                IconKind::MoveDown,
                IconKind::More,
            ]
        ))
        .height(Length::Fixed(theme.sizing.layers_pad_height as f32)),
        container(pad::view(
            theme,
            "History",
            Some(IconKind::HistoryList),
            history_body,
            vec![IconKind::Undo, IconKind::Redo]
        ))
        .height(Length::Fixed(theme.sizing.history_pad_height as f32)),
    ]
    .spacing(sidebar_gap)
    .width(Length::Fixed(theme.sizing.right_sidebar_width as f32));

    let main = row![
        container(toolbox::view(theme, tools)).padding([theme.spacing.sm, 0.0]),
        workspace,
        container(right_sidebar),
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

    let footer = container(column![
        container(column![]).height(Length::Fixed(theme.sizing.footer_inset_top as f32)),
        footer,
    ])
        .height(Length::Fixed(theme.sizing.footer_height as f32))
        .width(Length::Fill);

    container(column![header, tool_options, main, footer])
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| {
            iced::widget::container::Style::default()
                .background(Background::Color(theme.colors.window_bg))
        })
        .into()
}
