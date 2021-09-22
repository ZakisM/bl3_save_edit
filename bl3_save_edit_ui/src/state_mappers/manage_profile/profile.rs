use bl3_save_edit_core::bl3_profile::sdu::ProfileSduSlot;
use bl3_save_edit_core::bl3_profile::Bl3Profile;

use crate::views::manage_profile::profile::skin_unlocker::SkinUnlocker;
use crate::views::manage_profile::ManageProfileState;

pub fn map_profile_to_profile_state(manage_profile_state: &mut ManageProfileState) {
    let profile = &manage_profile_state.current_file;

    manage_profile_state
        .profile_view_state
        .profile_state
        .guardian_rank_tokens_input = profile.profile_data.guardian_rank_tokens();

    manage_profile_state
        .profile_view_state
        .profile_state
        .science_level_selected = profile
        .profile_data
        .borderlands_science_info()
        .science_level;

    manage_profile_state
        .profile_view_state
        .profile_state
        .science_tokens_input = profile.profile_data.borderlands_science_info().tokens;

    let mut skin_unlocker = SkinUnlocker::default();

    skin_unlocker.character_heads.skin_data.current =
        profile.profile_data.character_heads_unlocked();

    skin_unlocker.character_skins.skin_data.current =
        profile.profile_data.character_skins_unlocked();

    skin_unlocker.echo_themes.skin_data.current = profile.profile_data.echo_themes_unlocked();

    skin_unlocker.emotes.skin_data.current = profile.profile_data.profile_emotes_unlocked();

    skin_unlocker.room_decorations.skin_data.current =
        profile.profile_data.room_decorations_unlocked();

    skin_unlocker.weapon_skins.skin_data.current = profile.profile_data.weapon_skins_unlocked();

    skin_unlocker.weapon_trinkets.skin_data.current =
        profile.profile_data.weapon_trinkets_unlocked();

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
        .sdu_slots()
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

pub fn map_profile_state_to_profile(
    manage_profile_state: &mut ManageProfileState,
    profile: &mut Bl3Profile,
) {
    let profile_state = &manage_profile_state.profile_view_state.profile_state;

    profile
        .profile_data
        .set_borderlands_science_level(&profile_state.science_level_selected);

    profile
        .profile_data
        .set_borderlands_science_tokens(profile_state.science_tokens_input);

    let skin_unlocker = &profile_state.skin_unlocker;

    let all_skin_unlock_boxes = [
        &skin_unlocker.character_skins,
        &skin_unlocker.character_heads,
        &skin_unlocker.echo_themes,
        &skin_unlocker.emotes,
        &skin_unlocker.room_decorations,
        &skin_unlocker.weapon_skins,
        &skin_unlocker.weapon_trinkets,
    ];

    for s in all_skin_unlock_boxes {
        if s.is_unlocked {
            profile.profile_data.unlock_skin_set(&s.skin_data.skin_type)
        }
    }

    let sdu_unlocker = &profile_state.sdu_unlocker;

    let all_sdu_slots = [&sdu_unlocker.lost_loot, &sdu_unlocker.bank];

    for s in all_sdu_slots {
        profile.profile_data.set_sdu_slot(&s.sdu_slot, s.input);
    }
}
