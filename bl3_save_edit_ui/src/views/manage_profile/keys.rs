use iced::{Container, Text};

use crate::bl3_ui::Message;

#[derive(Debug, Default)]
pub struct KeysState {}

#[derive(Debug, Clone)]
pub enum ProfileKeysInteractionMessage {}

pub fn view(keys_state: &mut KeysState) -> Container<Message> {
    Container::new(Text::new("Keys"))
}
