use iced::{text_input, TextInput};

use crate::bl3_ui::Message;

pub struct NumberInput<'a>(pub TextInput<'a, Message>);

impl<'a> NumberInput<'a> {
    pub fn new<F>(
        state: &'a mut text_input::State,
        placeholder: &str,
        value: usize,
        max_value: Option<usize>,
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
                    if let Some(max_value) = max_value {
                        if s <= max_value {
                            s
                        } else {
                            return Message::Ignore;
                        }
                    } else {
                        s
                    }
                } else {
                    return Message::Ignore;
                };

                on_change(v)
            },
        );

        Self(input)
    }
}
