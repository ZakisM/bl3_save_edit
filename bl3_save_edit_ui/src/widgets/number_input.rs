use iced::{text_input, TextInput};

use crate::bl3_ui::Message;
use crate::views::manage_save::character::CharacterMessage;
use crate::views::manage_save::ManageSaveMessage;

pub struct NumberInput<'a>(pub TextInput<'a, Message>);

impl<'a> NumberInput<'a> {
    pub fn new<F>(
        state: &'a mut text_input::State,
        placeholder: &str,
        value: usize,
        on_change: F,
    ) -> Self
    where
        Message: Clone,
        F: 'static + Fn(usize) -> Message,
    {
        let input = TextInput::new(
            state,
            placeholder,
            value.to_string().trim_start_matches('0'),
            move |s| {
                let v = if s.is_empty() {
                    0
                } else if let Ok(s) = s.parse::<usize>() {
                    s
                } else {
                    return Message::Ignore;
                };

                on_change(v)
            },
        );

        Self(input)
    }
}
