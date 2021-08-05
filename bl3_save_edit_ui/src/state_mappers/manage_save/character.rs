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
        .skin_selectors
        .head_skin
        .selected = save.character_data.head_skin_selected;

    manage_save_state
        .main_state
        .character_state
        .skin_selectors
        .character_skin
        .selected = save.character_data.character_skin_selected;

    manage_save_state
        .main_state
        .character_state
        .skin_selectors
        .echo_theme
        .selected = save.character_data.echo_theme_selected;

    let mut gear_unlocker =
        std::mem::take(&mut manage_save_state.main_state.character_state.gear_unlocker);

    save.character_data
        .unlockable_inventory_slots
        .iter()
        .for_each(|s| match s.slot {
            InventorySlot::Grenade => {
                gear_unlocker.grenade.is_unlocked = s.unlocked;
            }
            InventorySlot::Shield => {
                gear_unlocker.shield.is_unlocked = s.unlocked;
            }
            InventorySlot::Weapon1 => {
                gear_unlocker.weapon_1.is_unlocked = s.unlocked;
            }
            InventorySlot::Weapon2 => {
                gear_unlocker.weapon_2.is_unlocked = s.unlocked;
            }
            InventorySlot::Weapon3 => {
                gear_unlocker.weapon_3.is_unlocked = s.unlocked;
            }
            InventorySlot::Weapon4 => {
                gear_unlocker.weapon_4.is_unlocked = s.unlocked;
            }
            InventorySlot::Artifact => {
                gear_unlocker.artifact.is_unlocked = s.unlocked;
            }
            InventorySlot::ClassMod => {
                gear_unlocker.class_mod.is_unlocked = s.unlocked;
            }
        });

    manage_save_state.main_state.character_state.gear_unlocker = gear_unlocker;

    let mut sdu_unlocker =
        std::mem::take(&mut manage_save_state.main_state.character_state.sdu_unlocker);

    save.character_data
        .sdu_slots
        .iter()
        .for_each(|s| match s.slot {
            SaveSduSlot::Backpack => sdu_unlocker.backpack.input = s.current,
            SaveSduSlot::Sniper => sdu_unlocker.sniper.input = s.current,
            SaveSduSlot::Shotgun => sdu_unlocker.shotgun.input = s.current,
            SaveSduSlot::Pistol => sdu_unlocker.pistol.input = s.current,
            SaveSduSlot::Grenade => sdu_unlocker.grenade.input = s.current,
            SaveSduSlot::Smg => sdu_unlocker.smg.input = s.current,
            SaveSduSlot::Ar => sdu_unlocker.assault_rifle.input = s.current,
            SaveSduSlot::Heavy => sdu_unlocker.heavy.input = s.current,
        });

    manage_save_state.main_state.character_state.sdu_unlocker = sdu_unlocker;
}
