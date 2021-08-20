use iced::Element;
use iced::Row;

use crate::bl3_ui::InteractionMessage;
use crate::widgets::text_margin::TextMargin;

//Just a lil hack to give us a left margin
pub struct RowMargin;

impl RowMargin {
    pub fn create<'a, E: Into<Element<'a, InteractionMessage>>>(
        content: E,
        margin: usize,
    ) -> Row<'a, InteractionMessage> {
        Row::new()
            .push(TextMargin::new("", margin).0)
            .push(content.into())
    }
}
