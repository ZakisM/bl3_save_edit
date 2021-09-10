use iced::{text_input, TextInput};

use crate::bl3_ui::InteractionMessage;

pub struct TextInputLimited<'a>(pub TextInput<'a, InteractionMessage>);

impl<'a> TextInputLimited<'a> {
    pub fn new<F>(
        state: &'a mut text_input::State,
        placeholder: &str,
        value: &str,
        max_length: usize,
        on_change: F,
    ) -> Self
    where
        F: 'static + Fn(String) -> InteractionMessage,
    {
        let value_len = value.len();

        let input = TextInput::new(state, placeholder, value, move |s| {
            if s.len() <= max_length && value_len <= max_length {
                on_change(s)
            } else {
                InteractionMessage::Ignore
            }
        });

        Self(input)
    }
}
