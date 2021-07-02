use std::fmt::Display;

use iced::{text_input, Text, TextInput};

use crate::bl3_ui::Message;
use crate::views::manage_save::character::CharacterMessage;
use crate::views::manage_save::ManageSaveMessage;

pub struct TextMargin(pub Text);

impl TextMargin {
    pub fn new<S: Display>(label: S, margin: usize) -> Self {
        let text = Text::new(format!("{:width$}{}", " ", label, width = margin));

        Self(text)
    }
}
