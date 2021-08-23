use anyhow::{bail, Context, Result};
use strum::Display;

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct BorderlandsScienceInfo {
    pub science_level: BorderlandsScienceLevel,
    pub solves: i32,
    pub tokens: i32,
}

#[derive(Copy, Clone, Debug, Display, Eq, PartialEq, Ord, PartialOrd)]
#[strum(serialize_all = "title_case")]
pub enum BorderlandsScienceLevel {
    Claptrap,
    Brick,
    Mordecai,
    Torgue,
    Marcus,
    Ellie,
    Lilith,
    MadMoxxi,
    Tannis,
    TrueTannis,
    None,
}

impl std::default::Default for BorderlandsScienceLevel {
    fn default() -> Self {
        Self::Claptrap
    }
}

impl BorderlandsScienceLevel {
    pub const ALL: [BorderlandsScienceLevel; 11] = [
        BorderlandsScienceLevel::None,
        BorderlandsScienceLevel::Claptrap,
        BorderlandsScienceLevel::Brick,
        BorderlandsScienceLevel::Mordecai,
        BorderlandsScienceLevel::Torgue,
        BorderlandsScienceLevel::Marcus,
        BorderlandsScienceLevel::Ellie,
        BorderlandsScienceLevel::Lilith,
        BorderlandsScienceLevel::MadMoxxi,
        BorderlandsScienceLevel::Tannis,
        BorderlandsScienceLevel::TrueTannis,
    ];
}

const BORDERLANDS_SCIENCE_LEVELS: [(i32, BorderlandsScienceLevel); 10] = [
    (5, BorderlandsScienceLevel::Claptrap),
    (10, BorderlandsScienceLevel::Brick),
    (15, BorderlandsScienceLevel::Mordecai),
    (20, BorderlandsScienceLevel::Torgue),
    (25, BorderlandsScienceLevel::Marcus),
    (30, BorderlandsScienceLevel::Ellie),
    (35, BorderlandsScienceLevel::Lilith),
    (40, BorderlandsScienceLevel::MadMoxxi),
    (50, BorderlandsScienceLevel::Tannis),
    (0, BorderlandsScienceLevel::TrueTannis),
];

impl BorderlandsScienceLevel {
    pub fn from_solves(progression: &[i32]) -> Result<BorderlandsScienceLevel> {
        for (i, completions) in progression.iter().enumerate() {
            let (required_completions, science_level) =
                BORDERLANDS_SCIENCE_LEVELS.get(i).with_context(|| {
                    format!("Failed to read Borderlands Science Level for index: {}", i)
                })?;

            if completions < required_completions || *required_completions == 0 {
                return Ok(science_level.to_owned());
            }
        }

        bail!("Failed to read Borderlands Science Level.")
    }
}
