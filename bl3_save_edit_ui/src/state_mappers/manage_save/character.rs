use anyhow::Result;

use bl3_save_edit_core::bl3_save::ammo::AmmoPool;
use bl3_save_edit_core::bl3_save::inventory_slot::InventorySlot;
use bl3_save_edit_core::bl3_save::sdu::SaveSduSlot;
use bl3_save_edit_core::bl3_save::Bl3Save;

use crate::views::manage_save::ManageSaveState;

pub fn map_save_to_character_state(manage_save_state: &mut ManageSaveState) {
    let save = &manage_save_state.current_file;

    manage_save_state.save_view_state.character_state.name_input = save
        .character_data
        .character
        .preferred_character_name
        .clone();

    manage_save_state
        .save_view_state
        .character_state
        .player_class_selected_class = save.character_data.player_class();

    manage_save_state
        .save_view_state
        .character_state
        .level_input = save.character_data.player_level();

    manage_save_state
        .save_view_state
        .character_state
        .experience_points_input = save.character_data.character.experience_points;

    manage_save_state
        .save_view_state
        .character_state
        .ability_points_input = save.character_data.ability_points();

    manage_save_state
        .save_view_state
        .character_state
        .skin_selectors
        .head_skin
        .selected = save.character_data.head_skin_selected();

    manage_save_state
        .save_view_state
        .character_state
        .skin_selectors
        .character_skin
        .selected = save.character_data.character_skin_selected();

    manage_save_state
        .save_view_state
        .character_state
        .skin_selectors
        .echo_theme
        .selected = save.character_data.echo_theme_selected();

    let mut gear_unlocker = std::mem::take(
        &mut manage_save_state
            .save_view_state
            .character_state
            .gear_unlocker,
    );

    save.character_data
        .unlockable_inventory_slots()
        .iter()
        .for_each(|s| match s.slot {
            InventorySlot::Grenade => {
                gear_unlocker.grenade.is_unlocked = s.unlocked;
                gear_unlocker.grenade.inv_slot = InventorySlot::Grenade;
            }
            InventorySlot::Shield => {
                gear_unlocker.shield.is_unlocked = s.unlocked;
                gear_unlocker.shield.inv_slot = InventorySlot::Shield;
            }
            InventorySlot::Weapon1 => {
                gear_unlocker.weapon_1.is_unlocked = s.unlocked;
                gear_unlocker.weapon_1.inv_slot = InventorySlot::Weapon1;
            }
            InventorySlot::Weapon2 => {
                gear_unlocker.weapon_2.is_unlocked = s.unlocked;
                gear_unlocker.weapon_2.inv_slot = InventorySlot::Weapon2;
            }
            InventorySlot::Weapon3 => {
                gear_unlocker.weapon_3.is_unlocked = s.unlocked;
                gear_unlocker.weapon_3.inv_slot = InventorySlot::Weapon3;
            }
            InventorySlot::Weapon4 => {
                gear_unlocker.weapon_4.is_unlocked = s.unlocked;
                gear_unlocker.weapon_4.inv_slot = InventorySlot::Weapon4;
            }
            InventorySlot::Artifact => {
                gear_unlocker.artifact.is_unlocked = s.unlocked;
                gear_unlocker.artifact.inv_slot = InventorySlot::Artifact;
            }
            InventorySlot::ClassMod => {
                gear_unlocker.class_mod.is_unlocked = s.unlocked;
                gear_unlocker.class_mod.inv_slot = InventorySlot::ClassMod;
            }
        });

    manage_save_state
        .save_view_state
        .character_state
        .gear_unlocker = gear_unlocker;

    let mut ammo_setter = std::mem::take(
        &mut manage_save_state
            .save_view_state
            .character_state
            .ammo_setter,
    );

    save.character_data
        .ammo_pools()
        .iter()
        .for_each(|s| match s.pool {
            AmmoPool::Sniper => ammo_setter.sniper.input = s.current,
            AmmoPool::Shotgun => ammo_setter.shotgun.input = s.current,
            AmmoPool::Pistol => ammo_setter.pistol.input = s.current,
            AmmoPool::Grenade => ammo_setter.grenade.input = s.current,
            AmmoPool::Smg => ammo_setter.smg.input = s.current,
            AmmoPool::Ar => ammo_setter.assault_rifle.input = s.current,
            AmmoPool::Heavy => ammo_setter.heavy.input = s.current,
        });

    manage_save_state
        .save_view_state
        .character_state
        .ammo_setter = ammo_setter;

    let mut sdu_unlocker = std::mem::take(
        &mut manage_save_state
            .save_view_state
            .character_state
            .sdu_unlocker,
    );

    save.character_data
        .sdu_slots()
        .iter()
        .for_each(|s| match s.sdu {
            SaveSduSlot::Backpack => sdu_unlocker.backpack.input = s.current,
            SaveSduSlot::Sniper => sdu_unlocker.sniper.input = s.current,
            SaveSduSlot::Shotgun => sdu_unlocker.shotgun.input = s.current,
            SaveSduSlot::Pistol => sdu_unlocker.pistol.input = s.current,
            SaveSduSlot::Grenade => sdu_unlocker.grenade.input = s.current,
            SaveSduSlot::Smg => sdu_unlocker.smg.input = s.current,
            SaveSduSlot::Ar => sdu_unlocker.assault_rifle.input = s.current,
            SaveSduSlot::Heavy => sdu_unlocker.heavy.input = s.current,
        });

    manage_save_state
        .save_view_state
        .character_state
        .sdu_unlocker = sdu_unlocker;
}

pub fn map_character_state_to_save(
    manage_save_state: &mut ManageSaveState,
    save: &mut Bl3Save,
) -> Result<()> {
    save.character_data.character.preferred_character_name = manage_save_state
        .save_view_state
        .character_state
        .name_input
        .clone();

    save.character_data.set_player_level(
        manage_save_state
            .save_view_state
            .character_state
            .experience_points_input,
    )?;

    save.character_data.set_player_class(
        manage_save_state
            .save_view_state
            .character_state
            .player_class_selected_class,
    )?;

    save.character_data.set_ability_points(
        manage_save_state
            .save_view_state
            .character_state
            .ability_points_input,
    )?;

    save.character_data.set_head_skin_selected(
        &manage_save_state
            .save_view_state
            .character_state
            .skin_selectors
            .head_skin
            .selected,
    );

    save.character_data.set_character_skin_selected(
        &manage_save_state
            .save_view_state
            .character_state
            .skin_selectors
            .character_skin
            .selected,
    );

    save.character_data.set_echo_theme_selected(
        &manage_save_state
            .save_view_state
            .character_state
            .skin_selectors
            .echo_theme
            .selected,
    );

    let gear_unlocker = &manage_save_state
        .save_view_state
        .character_state
        .gear_unlocker;

    let all_gear_unlock_boxes = [
        &gear_unlocker.artifact,
        &gear_unlocker.class_mod,
        &gear_unlocker.grenade,
        &gear_unlocker.shield,
        &gear_unlocker.weapon_1,
        &gear_unlocker.weapon_2,
        &gear_unlocker.weapon_3,
        &gear_unlocker.weapon_4,
    ];

    for s in all_gear_unlock_boxes {
        if s.is_unlocked {
            save.character_data.unlock_inventory_slot(&s.inv_slot)?;
        } else {
            save.character_data
                .remove_inventory_slot_if_exists(&s.inv_slot)?;
        }
    }

    let ammo_setter = &manage_save_state
        .save_view_state
        .character_state
        .ammo_setter;

    let all_ammo_pools = [
        &ammo_setter.grenade,
        &ammo_setter.assault_rifle,
        &ammo_setter.heavy,
        &ammo_setter.pistol,
        &ammo_setter.shotgun,
        &ammo_setter.smg,
        &ammo_setter.sniper,
    ];

    for a in all_ammo_pools {
        save.character_data.set_ammo_pool(&a.ammo_pool, a.input)?;
    }

    let sdu_unlocker = &manage_save_state
        .save_view_state
        .character_state
        .sdu_unlocker;

    let all_sdu_slots = [
        &sdu_unlocker.grenade,
        &sdu_unlocker.assault_rifle,
        &sdu_unlocker.heavy,
        &sdu_unlocker.pistol,
        &sdu_unlocker.shotgun,
        &sdu_unlocker.smg,
        &sdu_unlocker.sniper,
        &sdu_unlocker.backpack,
    ];

    for s in all_sdu_slots {
        save.character_data.set_sdu_slot(&s.sdu_slot, s.input);
    }

    Ok(())
}
