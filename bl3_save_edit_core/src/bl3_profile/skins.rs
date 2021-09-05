use crate::game_data::{
    GameDataKv, PROFILE_ECHO_THEMES, PROFILE_ECHO_THEMES_DEFAULTS, PROFILE_EMOTES,
    PROFILE_EMOTES_DEFAULTS, PROFILE_HEADS, PROFILE_HEADS_DEFAULTS, PROFILE_ROOM_DECORATIONS,
    PROFILE_SKINS, PROFILE_SKINS_DEFAULTS, PROFILE_WEAPON_SKINS, PROFILE_WEAPON_TRINKETS,
};

#[derive(Debug, Default, Clone)]
pub struct ProfileSkinData {
    pub skin_type: ProfileSkinType,
    pub current: usize,
}

impl ProfileSkinData {
    pub fn new(skin_type: ProfileSkinType, current: usize) -> Self {
        ProfileSkinData { skin_type, current }
    }
}

#[derive(Debug, Clone)]
pub enum ProfileSkinType {
    Regular(SkinSet),
    Weapon(WeaponSkinSet),
}

impl std::default::Default for ProfileSkinType {
    fn default() -> Self {
        Self::Regular(SkinSet::CharacterSkins)
    }
}

#[derive(Debug, Clone)]
pub enum SkinSet {
    CharacterSkins,
    CharacterHeads,
    EchoThemes,
    Emotes,
    RoomDecorations,
}

#[derive(Debug, Clone)]
pub enum WeaponSkinSet {
    WeaponSkins,
    WeaponTrinkets,
}

impl ProfileSkinType {
    pub fn maximum(&self) -> usize {
        match self {
            ProfileSkinType::Regular(regular_skin_set) => match regular_skin_set {
                SkinSet::CharacterSkins => PROFILE_SKINS.len() + PROFILE_SKINS_DEFAULTS.len(),
                SkinSet::CharacterHeads => PROFILE_HEADS.len() + PROFILE_HEADS_DEFAULTS.len(),
                SkinSet::EchoThemes => {
                    PROFILE_ECHO_THEMES.len() + PROFILE_ECHO_THEMES_DEFAULTS.len()
                }
                SkinSet::Emotes => PROFILE_EMOTES.len() + PROFILE_EMOTES_DEFAULTS.len(),
                SkinSet::RoomDecorations => PROFILE_ROOM_DECORATIONS.len(),
            },
            ProfileSkinType::Weapon(weapon_skin_set) => match weapon_skin_set {
                WeaponSkinSet::WeaponSkins => PROFILE_WEAPON_SKINS.len(),
                WeaponSkinSet::WeaponTrinkets => PROFILE_WEAPON_TRINKETS.len(),
            },
        }
    }

    pub fn skin_set(&self) -> Vec<GameDataKv> {
        match self {
            ProfileSkinType::Regular(regular_skin_set) => match regular_skin_set {
                SkinSet::CharacterSkins => {
                    [PROFILE_SKINS.to_vec(), PROFILE_SKINS_DEFAULTS.to_vec()].concat()
                }
                SkinSet::CharacterHeads => {
                    [PROFILE_HEADS.to_vec(), PROFILE_HEADS_DEFAULTS.to_vec()].concat()
                }
                SkinSet::EchoThemes => [
                    PROFILE_ECHO_THEMES.to_vec(),
                    PROFILE_ECHO_THEMES_DEFAULTS.to_vec(),
                ]
                .concat(),
                SkinSet::Emotes => {
                    [PROFILE_EMOTES.to_vec(), PROFILE_EMOTES_DEFAULTS.to_vec()].concat()
                }
                SkinSet::RoomDecorations => PROFILE_ROOM_DECORATIONS.to_vec(),
            },
            ProfileSkinType::Weapon(weapon_skin_set) => match weapon_skin_set {
                WeaponSkinSet::WeaponSkins => PROFILE_WEAPON_SKINS.to_vec(),
                WeaponSkinSet::WeaponTrinkets => PROFILE_WEAPON_TRINKETS.to_vec(),
            },
        }
    }
}
