use bl3_save_edit_core::bl3_save::inventory_slot::InventorySlot;
use bl3_save_edit_core::bl3_save::sdu::SaveSduSlot;

use crate::views::manage_save::ManageSaveState;

pub fn map_character_state(manage_save_state: &mut ManageSaveState) {
    let save = &manage_save_state.current_file;

    manage_save_state.main_state.character_state.name_input = save
        .character_data
        .character
        .preferred_character_name
        .clone();

    manage_save_state
        .main_state
        .character_state
        .player_class_selected_class = save.character_data.player_class;

    manage_save_state.main_state.character_state.xp_level_input = save.character_data.player_level;

    manage_save_state.main_state.character_state.xp_points_input =
        save.character_data.character.experience_points;

    manage_save_state
        .main_state
        .character_state
        .skin_state
        .head_skin_selected = save.character_data.head_skin_selected;

    manage_save_state
        .main_state
        .character_state
        .skin_state
        .character_skin_selected = save.character_data.character_skin_selected;

    manage_save_state
        .main_state
        .character_state
        .skin_state
        .echo_theme_selected = save.character_data.echo_theme_selected;

    let mut gear_state =
        std::mem::take(&mut manage_save_state.main_state.character_state.gear_state);

    save.character_data
        .unlockable_inventory_slots
        .iter()
        .for_each(|s| match s.slot {
            InventorySlot::Weapon1 => {
                gear_state.unlock_weapon_1_slot = true;
            }
            InventorySlot::Weapon2 => {
                gear_state.unlock_weapon_2_slot = s.unlocked;
            }
            InventorySlot::Weapon3 => {
                gear_state.unlock_weapon_3_slot = s.unlocked;
            }
            InventorySlot::Weapon4 => {
                gear_state.unlock_weapon_4_slot = s.unlocked;
            }
            InventorySlot::Shield => {
                gear_state.unlock_shield_slot = s.unlocked;
            }
            InventorySlot::Grenade => {
                gear_state.unlock_grenade_slot = s.unlocked;
            }
            InventorySlot::ClassMod => {
                gear_state.unlock_class_mod_slot = s.unlocked;
            }
            InventorySlot::Artifact => {
                gear_state.unlock_artifact_slot = s.unlocked;
            }
        });

    manage_save_state.main_state.character_state.gear_state = gear_state;

    let mut sdu_state = std::mem::take(&mut manage_save_state.main_state.character_state.sdu_state);

    save.character_data
        .sdu_slots
        .iter()
        .for_each(|s| match s.slot {
            SaveSduSlot::Backpack => sdu_state.backpack_input = s.current,
            SaveSduSlot::Sniper => sdu_state.sniper_input = s.current,
            SaveSduSlot::Shotgun => sdu_state.shotgun_input = s.current,
            SaveSduSlot::Pistol => sdu_state.pistol_input = s.current,
            SaveSduSlot::Grenade => sdu_state.grenade_input = s.current,
            SaveSduSlot::Smg => sdu_state.smg_input = s.current,
            SaveSduSlot::Ar => sdu_state.assault_rifle_input = s.current,
            SaveSduSlot::Heavy => sdu_state.heavy_input = s.current,
        });

    manage_save_state.main_state.character_state.sdu_state = sdu_state;
}
