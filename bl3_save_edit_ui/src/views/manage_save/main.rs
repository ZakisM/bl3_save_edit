use iced::image::viewer::Renderer;
use iced::{
    button, container, Align, Button, Color, Column, Container, HorizontalAlignment, Length, Row,
    Text, VerticalAlignment,
};

use crate::bl3_ui::{Message, SaveSelectionMessage};
use crate::fonts::COMPACTA;

struct SaveSelectionContainerStyle;

impl container::StyleSheet for SaveSelectionContainerStyle {
    fn style(&self) -> container::Style {
        container::Style {
            // background: Some(Color::from_rgb8(26, 89, 150).into()),
            ..container::Style::default()
        }
    }
}

struct SaveSelectionStyle;

impl button::StyleSheet for SaveSelectionStyle {
    fn active(&self) -> button::Style {
        button::Style {
            // background: Some(Color::from_rgb8(7, 119, 227).into()),
            // border_width: 2.0,
            // border_radius: 5.0,
            // border_color: Color::from_rgb8(57, 196, 245),
            // text_color: Color::from_rgb8(250, 250, 250),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            // background: Some(Color::from_rgb8(191, 145, 0).into()),
            // border_width: 2.0,
            // border_radius: 5.0,
            // border_color: Color::from_rgb8(234, 223, 145),
            // text_color: Color::from_rgb8(222, 224, 229),
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            // background: Some(Color::from_rgb8(176, 133, 0).into()),
            // border_width: 2.0,
            // border_radius: 5.0,
            // border_color: Color::from_rgb8(214, 204, 133),
            // text_color: Color::from_rgb8(205, 207, 212),
            ..button::Style::default()
        }
    }
}

pub fn view() -> Container<'static, Message> {
    let title = Container::new(
        Text::new("Manage Build")
            .font(COMPACTA)
            .size(44)
            .vertical_alignment(VerticalAlignment::Center)
            .height(Length::Units(80))
            .color(Color::from_rgb8(213, 230, 234)),
    )
    .align_x(Align::Center)
    .width(Length::Fill)
    .style(SaveSelectionContainerStyle);

    let all_contents = Column::new()
        .align_items(Align::Center)
        .spacing(20)
        .push(title);

    Container::new(all_contents).width(Length::Fill)
}
