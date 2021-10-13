use iced::alignment::{Horizontal, Vertical};
use iced::{Color, Container, Length, Text};

use crate::bl3_ui::Bl3Message;
use crate::resources::fonts::JETBRAINS_MONO;

pub fn view<'a>() -> Container<'a, Bl3Message> {
    let loading_text = Text::new("Loading...")
        .font(JETBRAINS_MONO)
        .size(20)
        .color(Color::from_rgb8(220, 220, 220));

    Container::new(loading_text)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
}
