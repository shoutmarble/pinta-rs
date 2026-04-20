use iced::widget::{column, container, row, scrollable};
use iced::{Background, Element, Length};
use pinta_ui::widgets::{
    canvas_viewport::CanvasViewport, icon::IconKind, pad, shell, status_bar, toolbox,
};

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
            icon: IconKind::Effects,
            selected: false,
            on_press: None,
        },
    ];

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
                    icon: IconKind::Eye,
                    muted: false,
                },
                shell::ToolbarAction {
                    icon: IconKind::ImageLandscape,
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
                IconKind::ChevronDown,
                IconKind::ChevronUp,
                IconKind::WindowClose,
            ],
        },
    ));

    let tool_options = container(shell::tool_options_bar(
        theme,
        shell::ToolOptionsBar {
            tool_label: "Tool:",
            tool_icon: IconKind::Paintbrush,
            value_label: "Brush width:",
            value: state.brush_width,
            mode_label: "Type:",
            mode_value: "Normal",
            mode_width: 126.0,
            shape_icon: IconKind::LineCurve,
            shape_width: 48.0,
        },
    ));

    let viewport = CanvasViewport::new(theme.clone(), state.viewport.clone())
        .view()
        .map(AppMessage::from);

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

    let right_sidebar = column![
        container(pad::view(
            theme,
            "Layers",
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
        .height(Length::FillPortion(1)),
        container(pad::view(
            theme,
            "History",
            history_body,
            vec![IconKind::Undo, IconKind::Redo]
        ))
        .height(Length::FillPortion(1)),
    ]
    .spacing(0)
    .width(Length::Fixed(theme.sizing.right_sidebar_width as f32));

    let main = row![
        container(toolbox::view(theme, tools)).padding([theme.spacing.sm, 0.0]),
        container(viewport).width(Length::Fill).height(Length::Fill),
        container(right_sidebar).padding([theme.spacing.sm, 0.0]),
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
