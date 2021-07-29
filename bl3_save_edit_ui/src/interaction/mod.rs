use iced::Element;

use crate::bl3_ui::{InteractionMessage, Message};

pub mod choose_save_directory;
pub mod manage_save;
pub mod save_file;

pub trait InteractionExt<'a, T>
where
    T: Into<Element<'a, InteractionMessage>>,
{
    fn into_element(self) -> Element<'a, Message>;
}

impl<'a, T> InteractionExt<'a, T> for T
where
    T: Into<Element<'a, InteractionMessage>>,
{
    fn into_element(self) -> Element<'a, Message> {
        let element: Element<'a, InteractionMessage> = self.into();
        element.map(Message::Interaction)
    }
}
