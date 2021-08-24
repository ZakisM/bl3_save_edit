use bl3_save_edit_core::bl3_profile::sdu::ProfileSduSlot;

use crate::views::manage_profile::ManageProfileState;

pub fn map_profile_to_profile_state(manage_profile_state: &mut ManageProfileState) {
    let profile = &manage_profile_state.current_file;

    manage_profile_state
        .profile_view_state
        .profile_state
        .guardian_rank_input = profile.profile_data.guardian_rank;

    manage_profile_state
        .profile_view_state
        .profile_state
        .guardian_rank_tokens_input = profile.profile_data.guardian_rank_tokens;

    manage_profile_state
        .profile_view_state
        .profile_state
        .science_tokens_input = profile.profile_data.borderlands_science_info.tokens;

    let mut skin_unlocker = std::mem::take(
        &mut manage_profile_state
            .profile_view_state
            .profile_state
            .skin_unlocker,
    );

    skin_unlocker.character_heads.skin_data.current = profile.profile_data.character_heads_unlocked;
    skin_unlocker.character_skins.skin_data.current = profile.profile_data.character_skins_unlocked;
    skin_unlocker.echo_themes.skin_data.current = profile.profile_data.echo_themes_unlocked;
    skin_unlocker.emotes.skin_data.current = profile.profile_data.profile_emotes_unlocked;
    skin_unlocker.room_decorations.skin_data.current =
        profile.profile_data.room_decorations_unlocked;
    skin_unlocker.weapon_skins.skin_data.current = profile.profile_data.weapon_skins_unlocked;
    skin_unlocker.weapon_trinkets.skin_data.current = profile.profile_data.weapon_trinkets_unlocked;

    manage_profile_state
        .profile_view_state
        .profile_state
        .skin_unlocker = skin_unlocker;

    let mut sdu_unlocker = std::mem::take(
        &mut manage_profile_state
            .profile_view_state
            .profile_state
            .sdu_unlocker,
    );

    profile
        .profile_data
        .sdu_slots
        .iter()
        .for_each(|s| match s.slot {
            ProfileSduSlot::Bank => sdu_unlocker.bank.input = s.current,
            ProfileSduSlot::LostLoot => sdu_unlocker.lost_loot.input = s.current,
        });

    manage_profile_state
        .profile_view_state
        .profile_state
        .sdu_unlocker = sdu_unlocker;
}
