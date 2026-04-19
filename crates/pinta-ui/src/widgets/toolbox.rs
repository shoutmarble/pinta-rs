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
    let mut rows = column!().spacing(0).padding(0);
    let mut pending: Option<ToolboxItem<Message>> = None;

    for item in items {
        if let Some(left) = pending.take() {
            rows = rows.push(
                row![tool_button(theme, left), tool_button(theme, item)]
                    .spacing(theme.spacing.xs)
                    .padding([0, theme.spacing.xs]),
            );
        } else {
            pending = Some(item);
        }
    }

    if let Some(item) = pending {
        rows = rows.push(
            row![tool_button(theme, item)]
                .padding([0, theme.spacing.xs]),
        );
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
    let base_bg = if selected {
        theme.colors.toolbox_selected_bg
    } else {
        theme.colors.panel_bg
    };

    button(
        container(icon::view(item.icon, 19.0, 19.0, if selected {
            theme.colors.text_primary
        } else {
            Color::from_rgb8(0x4A, 0x4A, 0x50)
        }))
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

            button::Style {
                background: Some(Background::Color(background)),
                text_color: if selected {
                    theme.colors.text_primary
                } else {
                    Color::from_rgb8(0x4A, 0x4A, 0x50)
                },
                border: Border::default()
                    .rounded(theme.radii.md)
                    .width(1)
                    .color(theme.colors.border_subtle),
                shadow: Default::default(),
            }
        })
        .into()
}
