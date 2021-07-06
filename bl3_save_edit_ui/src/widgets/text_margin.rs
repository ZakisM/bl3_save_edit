use std::fmt::Display;

use iced::Text;

pub struct TextMargin(pub Text);

impl TextMargin {
    pub fn new<S: Display>(label: S, margin: usize) -> Self {
        let text = Text::new(format!("{:width$}{}", " ", label, width = margin));

        Self(text)
    }
}
