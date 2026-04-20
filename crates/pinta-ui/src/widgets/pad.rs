use iced::widget::{column, container, row, text};
use iced::{Background, Border, Element, Length};
use pinta_theme::PintaTheme;

use crate::widgets::icon::{self, IconKind};

pub fn view<'a, Message: 'a>(
    theme: &'a PintaTheme,
    title: &'a str,
    body: Element<'a, Message>,
    footer_icons: Vec<IconKind>,
) -> Element<'a, Message> {
    let header = container(
        row![
            text(title).size(theme.typography.panel_title),
            container(icon::view(
                IconKind::ChevronDown,
                12.0,
                12.0,
                theme.colors.text_muted
            ))
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Right),
        ]
        .align_y(iced::Alignment::Center)
        .padding([theme.spacing.sm, theme.spacing.md]),
    )
    .width(Length::Fill)
    .style(move |_| {
        container::Style::default()
            .background(Background::Color(theme.colors.panel_header_bg))
            .border(Border::default().width(1).color(theme.colors.border_subtle))
            .color(theme.colors.text_primary)
    });

    let footer = footer_icons.into_iter().fold(
        row!()
            .spacing(theme.spacing.xs)
            .padding([theme.spacing.xs, theme.spacing.sm]),
        |row, icon_kind| {
            row.push(
                container(icon::view(icon_kind, 14.0, 14.0, theme.colors.panel_icon))
                    .width(Length::Fixed((theme.sizing.dock_header_height - 6) as f32))
                    .height(Length::Fixed((theme.sizing.dock_header_height - 6) as f32))
                    .style(move |_| {
                        container::Style::default()
                            .background(Background::Color(theme.colors.panel_header_bg))
                            .border(
                                Border::default()
                                    .rounded(theme.radii.sm)
                                    .width(1)
                                    .color(theme.colors.border_subtle),
                            )
                            .color(theme.colors.panel_icon)
                    }),
            )
        },
    );

    container(column![
        container(header).height(Length::Fixed(theme.sizing.dock_header_height as f32)),
        body,
        container(footer)
            .style(move |_| {
                container::Style::default()
                    .background(Background::Color(theme.colors.panel_header_bg))
                    .border(Border::default().width(1).color(theme.colors.border_subtle))
            })
            .height(Length::Fixed(theme.sizing.dock_toolbar_height as f32))
    ])
    .width(Length::Fill)
    .padding([0.0, theme.spacing.xs])
    .style(move |_| {
        container::Style::default()
            .background(Background::Color(theme.colors.panel_bg))
            .border(Border::default().width(1).color(theme.colors.border_subtle))
    })
    .into()
}
