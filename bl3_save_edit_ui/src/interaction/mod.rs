use iced::Element;

use crate::bl3_ui::{InteractionMessage, Message};

pub mod choose_save_directory;

pub trait InteractionExt<'a, T>
where
    Element<'a, InteractionMessage>: std::convert::From<T>,
{
    fn into_element(self) -> Element<'a, Message>;
}

impl<'a, T> InteractionExt<'a, T> for T
where
    Element<'a, InteractionMessage>: std::convert::From<T>,
{
    fn into_element(self) -> Element<'a, Message> {
        Element::from(self).map(Message::InteractionMessage)
    }
}
