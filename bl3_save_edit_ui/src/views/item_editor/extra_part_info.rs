use iced::{container, Color, Column};

use bl3_save_edit_core::resources::ResourcePartInfo;

use crate::bl3_ui::InteractionMessage;
use crate::resources::fonts::JETBRAINS_MONO_LIGHT_ITALIC;
use crate::widgets::text_margin::TextMargin;

pub fn add_extra_part_info<'a>(
    part_contents_col: Column<'a, InteractionMessage>,
    part_info: &'a ResourcePartInfo,
) -> Column<'a, InteractionMessage> {
    let mut part_contents_col = part_contents_col;

    if let Some(effects) = &part_info.effects {
        part_contents_col = part_contents_col.push(
            TextMargin::new(effects, 1)
                .0
                .font(JETBRAINS_MONO_LIGHT_ITALIC)
                .color(Color::from_rgb8(180, 180, 180))
                .size(16),
        );
    }

    let mut positives_negatives = Vec::new();

    if let Some(positives) = &part_info.positives {
        positives
            .split(", ")
            .for_each(|p| positives_negatives.push(p));
    }

    if let Some(negatives) = &part_info.negatives {
        negatives
            .split(", ")
            .for_each(|n| positives_negatives.push(n));
    }

    let positives_negatives = positives_negatives.join(", ");

    if !positives_negatives.is_empty() {
        part_contents_col = part_contents_col.push(
            TextMargin::new(positives_negatives, 1)
                .0
                .font(JETBRAINS_MONO_LIGHT_ITALIC)
                .color(Color::from_rgb8(180, 180, 180))
                .size(16),
        );
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
