use iced::{Align, Color, Column, Container, Length, Text};

use crate::bl3_ui::Message;
use crate::resources::fonts::JETBRAINS_MONO;

pub fn view<'a>() -> Container<'a, Message> {
    let initializating_text = Text::new("Initializing...")
        .font(JETBRAINS_MONO)
        .size(20)
        .color(Color::from_rgb8(220, 220, 220));

    Container::new(initializating_text)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Align::Center)
        .align_y(Align::Center)
}
