use std::fmt::Display;

use iced::{Color, Element, Length, Row};

use crate::bl3_ui::Bl3Message;
use crate::resources::fonts::JETBRAINS_MONO_BOLD;
use crate::widgets::text_margin::TextMargin;

#[derive(Debug)]
pub struct LabelledElement;

impl LabelledElement {
    pub fn create<'a, S, E>(label: S, label_width: Length, element: E) -> Row<'a, Bl3Message>
    where
        S: Display,
        E: Into<Element<'a, Bl3Message>>,
    {
        Row::new()
            .push(
                TextMargin::new(label, 2)
                    .0
                    .font(JETBRAINS_MONO_BOLD)
                    .size(17)
                    .color(Color::from_rgb8(242, 203, 5))
                    .width(label_width),
            )
            .push(element)
    }
}
