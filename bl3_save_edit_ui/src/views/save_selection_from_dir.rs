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
            background: Some(Color::from_rgb8(26, 89, 150).into()),
            ..container::Style::default()
        }
    }
}

struct SaveSelectionStyle;

impl container::StyleSheet for SaveSelectionStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Color::from_rgb8(1, 41, 102).into()),
            border_color: Color::from_rgb8(26, 89, 150),
            border_width: 3.0,
            ..container::Style::default()
        }
    }
}

impl button::StyleSheet for SaveSelectionStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(7, 119, 227).into()),
            border_width: 2.0,
            border_radius: 5.0,
            border_color: Color::from_rgb8(57, 196, 245),
            text_color: Color::from_rgb8(250, 250, 250),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(191, 145, 0).into()),
            border_width: 2.0,
            border_radius: 5.0,
            border_color: Color::from_rgb8(234, 223, 145),
            text_color: Color::from_rgb8(222, 224, 229),
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(176, 133, 0).into()),
            border_width: 2.0,
            border_radius: 5.0,
            border_color: Color::from_rgb8(214, 204, 133),
            text_color: Color::from_rgb8(205, 207, 212),
            ..button::Style::default()
        }
    }
}

pub fn view(save_selection_button_state: &mut button::State) -> Container<Message> {
    let title = Container::new(
        Text::new("Select Character")
            .font(COMPACTA)
            .size(44)
            .vertical_alignment(VerticalAlignment::Center)
            .height(Length::Units(80))
            .color(Color::from_rgb8(213, 230, 234)),
    )
    .align_x(Align::Center)
    .width(Length::Fill)
    .style(SaveSelectionContainerStyle);

    let selection_card_1 = Button::new(
        save_selection_button_state,
        Row::new()
            .push(
                Text::new("FL4K")
                    .font(COMPACTA)
                    .size(32)
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .width(Length::Fill),
            )
            .push(
                Text::new("Level 50")
                    .font(COMPACTA)
                    .size(32)
                    .horizontal_alignment(HorizontalAlignment::Right)
                    .width(Length::Fill),
            )
            .width(Length::Fill),
    )
    .on_press(Message::SaveSelectionMessage(
        SaveSelectionMessage::SavePressed,
    ))
    .width(Length::Fill)
    .padding(10)
    .style(SaveSelectionStyle);

    let saves_list = Column::new()
        .align_items(Align::Center)
        .padding(20)
        .push(selection_card_1);

    let all_contents = Column::new()
        .align_items(Align::Center)
        .spacing(20)
        .push(title)
        .push(saves_list);

    Container::new(all_contents)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(SaveSelectionStyle)
}
