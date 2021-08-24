use crate::game_data::{
    PROFILE_ECHO_THEMES, PROFILE_ECHO_THEMES_DEFAULTS, PROFILE_EMOTES, PROFILE_HEADS,
    PROFILE_HEADS_DEFAULTS, PROFILE_ROOM_DECORATIONS, PROFILE_SKINS, PROFILE_SKINS_DEFAULTS,
    PROFILE_WEAPON_SKINS, PROFILE_WEAPON_TRINKETS,
};

#[derive(Debug, Default, Clone)]
pub struct ProfileSkinData {
    pub skin_type: ProfileSkinType,
    pub current: usize,
    pub max: usize,
}

impl ProfileSkinData {
    pub fn new(skin_type: ProfileSkinType, current: usize) -> Self {
        let max = skin_type.maximum();

        ProfileSkinData {
            skin_type,
            current,
            max,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ProfileSkinType {
    CharacterSkins,
    CharacterHeads,
    EchoThemes,
    Emotes,
    RoomDecorations,
    WeaponSkins,
    WeaponTrinkets,
}

impl std::default::Default for ProfileSkinType {
    fn default() -> Self {
        Self::CharacterSkins
    }
}

impl ProfileSkinType {
    pub fn maximum(&self) -> usize {
        match *self {
            ProfileSkinType::CharacterSkins => PROFILE_SKINS.len() + PROFILE_SKINS_DEFAULTS.len(),
            ProfileSkinType::CharacterHeads => PROFILE_HEADS.len() + PROFILE_HEADS_DEFAULTS.len(),
            ProfileSkinType::EchoThemes => {
                PROFILE_ECHO_THEMES.len() + PROFILE_ECHO_THEMES_DEFAULTS.len()
            }
            ProfileSkinType::Emotes => PROFILE_EMOTES.len() + PROFILE_ECHO_THEMES_DEFAULTS.len(),
            ProfileSkinType::RoomDecorations => PROFILE_ROOM_DECORATIONS.len(),
            ProfileSkinType::WeaponSkins => PROFILE_WEAPON_SKINS.len(),
            ProfileSkinType::WeaponTrinkets => PROFILE_WEAPON_TRINKETS.len(),
        }
    }
}
