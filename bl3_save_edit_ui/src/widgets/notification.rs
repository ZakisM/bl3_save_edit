use iced::alignment::{Horizontal, Vertical};
use iced::{button, container, svg, Alignment, Button, Color, Container, Length, Row, Svg, Text};

use crate::bl3_ui::Bl3Message;
use crate::resources::fonts::JETBRAINS_MONO_BOLD;
use crate::resources::svgs::{INFO_CLOSE, NEGATIVE_CLOSE, POSITIVE_CLOSE};

#[derive(Debug, Default)]
pub struct Notification {
    message: String,
    sentiment: NotificationSentiment,
    close_button_state: button::State,
}

#[derive(Debug, Copy, Clone)]
pub enum NotificationSentiment {
    Positive,
    Info,
    Negative,
}

impl std::default::Default for NotificationSentiment {
    fn default() -> Self {
        Self::Positive
    }
}

impl Notification {
    pub fn new<T: AsRef<str>>(message: T, sentiment: NotificationSentiment) -> Self {
        Notification {
            message: message.as_ref().to_owned(),
            sentiment,
            close_button_state: button::State::default(),
        }
    }

    pub fn view(&mut self) -> Container<Bl3Message> {
        let close_handle = match self.sentiment {
            NotificationSentiment::Positive => svg::Handle::from_memory(POSITIVE_CLOSE),
            NotificationSentiment::Info => svg::Handle::from_memory(INFO_CLOSE),
            NotificationSentiment::Negative => svg::Handle::from_memory(NEGATIVE_CLOSE),
        };

        let close_icon = Svg::new(close_handle)
            .height(Length::Units(18))
            .width(Length::Units(18));

        let close_button = Button::new(&mut self.close_button_state, close_icon)
            .on_press(Bl3Message::ClearNotification)
            .style(NotificationStyle {
                sentiment: self.sentiment,
            });

        let contents_row = Row::new()
            .push(
                Container::new(Text::new(&self.message).font(JETBRAINS_MONO_BOLD).size(18))
                    .width(Length::Fill),
            )
            .push(close_button)
            .align_items(Alignment::Center);

        Container::new(
            Container::new(contents_row)
                .width(Length::Fill)
                .padding(20)
                .style(NotificationStyle {
                    sentiment: self.sentiment,
                }),
        )
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .width(Length::Fill)
        .padding(1)
    }
}

struct NotificationStyle {
    sentiment: NotificationSentiment,
}

impl container::StyleSheet for NotificationStyle {
    fn style(&self) -> container::Style {
        match self.sentiment {
            NotificationSentiment::Positive => PositiveNotificationStyle.style(),
            NotificationSentiment::Info => InfoNotificationStyle.style(),
            NotificationSentiment::Negative => NegativeNotificationStyle.style(),
        }
    }
}

impl button::StyleSheet for NotificationStyle {
    fn active(&self) -> button::Style {
        match self.sentiment {
            NotificationSentiment::Positive => PositiveNotificationStyle.active(),
            NotificationSentiment::Info => InfoNotificationStyle.active(),
            NotificationSentiment::Negative => NegativeNotificationStyle.active(),
        }
    }
}

struct PositiveNotificationStyle;

impl container::StyleSheet for PositiveNotificationStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::from_rgb8(149, 240, 171)),
            background: Some(Color::from_rgb8(29, 54, 39).into()),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(36, 61, 46),
        }
    }
}

impl button::StyleSheet for PositiveNotificationStyle {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Default::default(),
            background: None,
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::from_rgb8(29, 54, 39),
            ..button::Style::default()
        }
    }
}

struct InfoNotificationStyle;

impl container::StyleSheet for InfoNotificationStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::from_rgb8(149, 187, 240)),
            background: Some(Color::from_rgb8(29, 39, 54).into()),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(36, 47, 61),
        }
    }
}

impl button::StyleSheet for InfoNotificationStyle {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Default::default(),
            background: None,
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::from_rgb8(29, 54, 39),
            ..button::Style::default()
        }
    }
}

struct NegativeNotificationStyle;

impl container::StyleSheet for NegativeNotificationStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::from_rgb8(240, 149, 149)),
            background: Some(Color::from_rgb8(54, 29, 29).into()),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(61, 36, 36),
        }
    }
}

impl button::StyleSheet for NegativeNotificationStyle {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Default::default(),
            background: None,
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::from_rgb8(54, 29, 29),
            ..button::Style::default()
        }
    }
}
