use iced::widget::{button, column, container, row};
use iced::{Background, Border, Color, Element, Length};
use pinta_theme::PintaTheme;

use crate::widgets::icon::{self, IconKind};

#[derive(Debug, Clone)]
pub struct ToolboxItem<Message> {
    pub icon: IconKind,
    pub selected: bool,
    pub on_press: Option<Message>,
}

pub fn view<'a, Message: Clone + 'a>(
    theme: &'a PintaTheme,
    items: impl IntoIterator<Item = ToolboxItem<Message>>,
) -> Element<'a, Message> {
    let mut rows = column!().spacing(theme.spacing.xxs).padding(0);
    let mut pending: Option<ToolboxItem<Message>> = None;

    for item in items {
        if let Some(left) = pending.take() {
            rows = rows.push(
                row![tool_button(theme, left), tool_button(theme, item)]
                    .spacing(theme.spacing.xs)
                    .padding([0.0, theme.spacing.xxs]),
            );
        } else {
            pending = Some(item);
        }
    }

    if let Some(item) = pending {
        rows = rows.push(row![tool_button(theme, item)].padding([0.0, theme.spacing.xxs]));
    }

    container(rows)
        .width(Length::Fixed(theme.sizing.left_toolbar_width as f32))
        .into()
}

fn tool_button<'a, Message: Clone + 'a>(
    theme: &'a PintaTheme,
    item: ToolboxItem<Message>,
) -> Element<'a, Message> {
    let selected = item.selected;
    let icon_size = tool_icon_size(item.icon);
    let base_bg = if selected {
        theme.colors.toolbox_selected_bg
    } else {
        Color::TRANSPARENT
    };

    button(
        container(icon::view(
            item.icon,
            icon_size,
            icon_size,
            if selected {
                theme.colors.text_primary
            } else {
                theme.colors.icon_subtle
            },
        ))
        .width(Length::Fill)
        .center(Length::Fill),
    )
    .width(Length::Fixed(theme.sizing.toolbox_button_size as f32))
    .height(Length::Fixed((theme.sizing.toolbox_button_size - 8) as f32))
    .padding(0)
    .on_press_maybe(item.on_press)
    .style(move |_theme, status| {
        let background = match status {
            button::Status::Hovered => theme.colors.toolbox_hover_bg,
            button::Status::Pressed => theme.colors.selected_bg,
            button::Status::Disabled => base_bg,
            button::Status::Active => base_bg,
        };

        let border_width =
            if selected || matches!(status, button::Status::Hovered | button::Status::Pressed) {
                1
            } else {
                0
            };

        button::Style {
            background: Some(Background::Color(background)),
            text_color: if selected {
                theme.colors.text_primary
            } else {
                theme.colors.icon_subtle
            },
            border: Border::default()
                .rounded(theme.radii.md)
                .width(border_width)
                .color(theme.colors.border_subtle),
            shadow: Default::default(),
            snap: true,
        }
    })
    .into()
}

fn tool_icon_size(kind: IconKind) -> f32 {
    match kind {
        IconKind::Pan => 28.0,
        IconKind::Eraser => 24.0,
        IconKind::PaintBucket => 24.0,
        IconKind::EllipseSelect => 24.0,
        _ => 22.0,
    }
}
