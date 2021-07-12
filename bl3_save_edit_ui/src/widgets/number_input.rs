use std::fmt::Display;
use std::str::FromStr;

use iced::{text_input, TextInput};

use crate::bl3_ui::InteractionMessage;

pub struct NumberInput<'a>(pub TextInput<'a, InteractionMessage>);

impl<'a> NumberInput<'a> {
    pub fn new<F, V>(
        state: &'a mut text_input::State,
        value: V,
        minimum_value: V,
        max_value: Option<V>,
        on_change: F,
    ) -> Self
    where
        F: 'static + Fn(V) -> InteractionMessage,
        V: 'static + Copy + Display + FromStr + PartialOrd,
    {
        let minimum_value_s = minimum_value.to_string();

        let input = TextInput::new(state, &minimum_value_s, &value.to_string(), move |s| {
            let value = if s.is_empty() {
                return InteractionMessage::Ignore;
            } else if let Ok(v) = s.parse::<V>() {
                if v < minimum_value {
                    return InteractionMessage::Ignore;
                }

                if let Some(max_value) = &max_value {
                    if v <= *max_value {
                        v
                    } else {
                        return InteractionMessage::Ignore;
                    }
                } else {
                    v
                }
            } else {
                return InteractionMessage::Ignore;
            };

            on_change(value)
        })
        .select_all_first_click(true);

        Self(input)
    }
}
