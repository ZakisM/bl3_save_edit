use iced::{container, Color, Column, Container, Length, Row, Text};

use bl3_save_edit_core::resources::ResourcePartInfo;

use crate::bl3_ui::InteractionMessage;
use crate::resources::fonts::JETBRAINS_MONO;
use crate::widgets::row_margin::RowMargin;

pub fn add_extra_part_info<'a>(
    part_contents_col: Column<'a, InteractionMessage>,
    part_info: &'a ResourcePartInfo,
) -> Column<'a, InteractionMessage> {
    let mut part_contents_col = part_contents_col;

    let mut has_extra_info = false;

    if let Some(effects) = &part_info.effects {
        has_extra_info = true;

        part_contents_col = part_contents_col.push(RowMargin::create(
            Container::new(Text::new(effects).font(JETBRAINS_MONO).size(15))
                .padding(5)
                .style(PartInfoEffectStyle),
            2,
        ));
    }

    let mut positives_negatives_row = Row::new().width(Length::Fill).spacing(2);

    if let Some(positives) = &part_info.positives {
        has_extra_info = true;

        positives_negatives_row = positives_negatives_row.push(RowMargin::create(
            Container::new(Text::new(positives).font(JETBRAINS_MONO).size(15))
                .padding(5)
                .style(PartInfoPositiveStyle),
            2,
        ));
    }

    if let Some(negatives) = &part_info.negatives {
        has_extra_info = true;

        positives_negatives_row = positives_negatives_row.push(RowMargin::create(
            Container::new(Text::new(negatives).font(JETBRAINS_MONO).size(15))
                .padding(5)
                .style(PartInfoNegativeStyle),
            2,
        ));
    }

    if has_extra_info {
        part_contents_col = part_contents_col.push(positives_negatives_row);
    }

    part_contents_col
}

struct PartInfoEffectStyle;

impl container::StyleSheet for PartInfoEffectStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::from_rgb8(224, 224, 224)),
            background: Some(Color::from_rgb8(38, 38, 38).into()),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(46, 46, 46),
        }
    }
}

struct PartInfoPositiveStyle;

impl container::StyleSheet for PartInfoPositiveStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::from_rgb8(149, 240, 171)),
            background: Some(Color::from_rgb8(29, 54, 39).into()),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(36, 61, 46),
        }
    }
}

struct PartInfoNegativeStyle;

impl container::StyleSheet for PartInfoNegativeStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::from_rgb8(240, 149, 149)),
            background: Some(Color::from_rgb8(54, 29, 29).into()),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(61, 36, 36),
        }
    }
}
