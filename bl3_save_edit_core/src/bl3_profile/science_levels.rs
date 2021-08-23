use anyhow::{bail, Context, Result};
use strum::Display;

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct BorderlandsScienceInfo {
    pub science_level: ScienceLevel,
    pub solves: i32,
    pub tokens: i32,
}

#[derive(Copy, Clone, Debug, Display, Eq, PartialEq, Ord, PartialOrd)]
#[strum(serialize_all = "title_case")]
pub enum ScienceLevel {
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
    Unknown,
}

impl std::default::Default for ScienceLevel {
    fn default() -> Self {
        Self::Claptrap
    }
}

const BORDERLANDS_SCIENCE_LEVELS: [(i32, ScienceLevel); 10] = [
    (5, ScienceLevel::Claptrap),
    (10, ScienceLevel::Brick),
    (15, ScienceLevel::Mordecai),
    (20, ScienceLevel::Torgue),
    (25, ScienceLevel::Marcus),
    (30, ScienceLevel::Ellie),
    (35, ScienceLevel::Lilith),
    (40, ScienceLevel::MadMoxxi),
    (50, ScienceLevel::Tannis),
    (0, ScienceLevel::TrueTannis),
];

impl ScienceLevel {
    pub fn from_solves(progression: &[i32]) -> Result<ScienceLevel> {
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
