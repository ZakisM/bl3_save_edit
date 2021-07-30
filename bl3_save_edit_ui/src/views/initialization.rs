use iced::{Align, Color, Container, Length, Text};

use crate::bl3_ui::Message;
use crate::resources::fonts::JETBRAINS_MONO;

#[derive(Debug, Clone)]
pub enum InitializationMessage {
    LoadLazyData,
}

pub fn view<'a>() -> Container<'a, Message> {
    let initializing_text = Text::new("Initializing...")
        .font(JETBRAINS_MONO)
        .size(20)
        .color(Color::from_rgb8(220, 220, 220));

    Container::new(initializing_text)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Align::Center)
        .align_y(Align::Center)
}
