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

    pub fn progression(&self) -> Vec<i32> {
        if *self == BorderlandsScienceLevel::None {
            return Vec::new();
        }

        let required_level_index = BORDERLANDS_SCIENCE_LEVELS
            .iter()
            .position(|(_, l)| l == self)
            .expect("Failed to find corresponding Borderlands Science Level");

        let mut levels = BORDERLANDS_SCIENCE_LEVELS[0..required_level_index]
            .iter()
            .map(|(i, _)| *i)
            .collect::<Vec<_>>();

        levels.resize(BORDERLANDS_SCIENCE_LEVELS.len(), 0);

        levels
    }
}

#[cfg(test)]
mod tests {
    use crate::bl3_profile::science_levels::BorderlandsScienceLevel;

    #[test]
    pub fn test_science_level_progression() {
        assert_eq!(
            BorderlandsScienceLevel::None.progression(),
            Vec::<i32>::new()
        );
        assert_eq!(
            BorderlandsScienceLevel::Claptrap.progression(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            BorderlandsScienceLevel::Brick.progression(),
            vec![5, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            BorderlandsScienceLevel::Mordecai.progression(),
            vec![5, 10, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            BorderlandsScienceLevel::Torgue.progression(),
            vec![5, 10, 15, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            BorderlandsScienceLevel::Marcus.progression(),
            vec![5, 10, 15, 20, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            BorderlandsScienceLevel::Ellie.progression(),
            vec![5, 10, 15, 20, 25, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            BorderlandsScienceLevel::Lilith.progression(),
            vec![5, 10, 15, 20, 25, 30, 0, 0, 0, 0]
        );
        assert_eq!(
            BorderlandsScienceLevel::MadMoxxi.progression(),
            vec![5, 10, 15, 20, 25, 30, 35, 0, 0, 0]
        );
        assert_eq!(
            BorderlandsScienceLevel::Tannis.progression(),
            vec![5, 10, 15, 20, 25, 30, 35, 40, 0, 0]
        );
        assert_eq!(
            BorderlandsScienceLevel::TrueTannis.progression(),
            vec![5, 10, 15, 20, 25, 30, 35, 40, 50, 0]
        );
    }
}
