use std::rc::Rc;

use derivative::Derivative;
use iced::alignment::{Horizontal, Vertical};
use iced::{
    button, text_input, tooltip, Alignment, Button, Color, Column, Container, Length, Row, Text,
    Tooltip,
};

use bl3_save_edit_core::bl3_profile::guardian_reward::GuardianReward;

use crate::bl3_ui::{Bl3Message, InteractionMessage};
use crate::bl3_ui_style::{Bl3UiStyle, Bl3UiTooltipStyle};
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_profile::profile::{GuardianRewardMessage, ProfileInteractionMessage};
use crate::views::manage_profile::ManageProfileInteractionMessage;
use crate::views::InteractionExt;
use crate::widgets::number_input::NumberInput;
use crate::widgets::text_margin::TextMargin;

#[derive(Derivative)]
#[derivative(Debug, Default)]
pub struct GuardianRewardField {
    name: String,
    text_margin: usize,
    pub guardian_reward: GuardianReward,
    pub input: i32,
    input_state: text_input::State,
    #[derivative(
        Debug = "ignore",
        Default(value = "Rc::new(GuardianRewardMessage::Accuracy)")
    )]
    on_changed: Rc<dyn Fn(i32) -> GuardianRewardMessage>,
}

impl GuardianRewardField {
    pub fn new<F>(text_margin: usize, guardian_reward: GuardianReward, on_changed: F) -> Self
    where
        F: 'static + Fn(i32) -> GuardianRewardMessage,
    {
        GuardianRewardField {
            name: guardian_reward.to_string(),
            text_margin,
            guardian_reward,
            on_changed: Rc::new(on_changed),
            ..Default::default()
        }
    }

    pub fn view(&mut self) -> Row<Bl3Message> {
        let on_changed = self.on_changed.clone();
        let minimum = 0;
        let maximum = i32::MAX;

        Row::new()
            .push(
                TextMargin::new(&self.name, self.text_margin)
                    .0
                    .font(JETBRAINS_MONO)
                    .size(17)
                    .color(Color::from_rgb8(220, 220, 220))
                    .width(Length::FillPortion(6)),
            )
            .push(
                Tooltip::new(
                    NumberInput::new(
                        &mut self.input_state,
                        self.input,
                        minimum,
                        Some(maximum),
                        move |v| {
                            InteractionMessage::ManageProfileInteraction(
                                ManageProfileInteractionMessage::Profile(
                                    ProfileInteractionMessage::GuardianRewardMessage(on_changed(v)),
                                ),
                            )
                        },
                    )
                    .0
                    .width(Length::FillPortion(3))
                    .font(JETBRAINS_MONO)
                    .padding(10)
                    .size(17)
                    .style(Bl3UiStyle)
                    .into_element(),
                    format!("Reward must be between {} and {}", minimum, maximum),
                    tooltip::Position::Top,
                )
                .gap(10)
                .padding(10)
                .font(JETBRAINS_MONO)
                .size(17)
                .style(Bl3UiTooltipStyle),
            )
            .width(Length::Fill)
            .align_items(Alignment::Center)
    }
}

#[derive(Debug)]
pub struct GuardianRewardUnlocker {
    pub accuracy: GuardianRewardField,
    pub action_skill_cooldown: GuardianRewardField,
    pub critical_damage: GuardianRewardField,
    pub elemental_damage: GuardianRewardField,
    pub ffyl_duration: GuardianRewardField,
    pub ffyl_movement_speed: GuardianRewardField,
    pub grenade_damage: GuardianRewardField,
    pub gun_damage: GuardianRewardField,
    pub gun_fire_rate: GuardianRewardField,
    pub max_health: GuardianRewardField,
    pub melee_damage: GuardianRewardField,
    pub rarity_rate: GuardianRewardField,
    pub recoil_reduction: GuardianRewardField,
    pub reload_speed: GuardianRewardField,
    pub shield_capacity: GuardianRewardField,
    pub shield_recharge_delay: GuardianRewardField,
    pub shield_recharge_rate: GuardianRewardField,
    pub vehicle_damage: GuardianRewardField,
    unlock_all_button_state: button::State,
}

impl std::default::Default for GuardianRewardUnlocker {
    fn default() -> Self {
        Self {
            accuracy: GuardianRewardField::new(
                3,
                GuardianReward::Accuracy,
                GuardianRewardMessage::Accuracy,
            ),
            action_skill_cooldown: GuardianRewardField::new(
                3,
                GuardianReward::ActionSkillCooldown,
                GuardianRewardMessage::ActionSkillCooldown,
            ),
            critical_damage: GuardianRewardField::new(
                0,
                GuardianReward::CriticalDamage,
                GuardianRewardMessage::CriticalDamage,
            ),
            elemental_damage: GuardianRewardField::new(
                3,
                GuardianReward::ElementalDamage,
                GuardianRewardMessage::ElementalDamage,
            ),
            ffyl_duration: GuardianRewardField::new(
                3,
                GuardianReward::FFYLDuration,
                GuardianRewardMessage::FFYLDuration,
            ),
            ffyl_movement_speed: GuardianRewardField::new(
                3,
                GuardianReward::FFYLMovementSpeed,
                GuardianRewardMessage::FFYLMovementSpeed,
            ),
            grenade_damage: GuardianRewardField::new(
                0,
                GuardianReward::GrenadeDamage,
                GuardianRewardMessage::GrenadeDamage,
            ),
            gun_damage: GuardianRewardField::new(
                0,
                GuardianReward::GunDamage,
                GuardianRewardMessage::GunDamage,
            ),
            gun_fire_rate: GuardianRewardField::new(
                0,
                GuardianReward::GunFireRate,
                GuardianRewardMessage::GunFireRate,
            ),
            max_health: GuardianRewardField::new(
                3,
                GuardianReward::MaxHealth,
                GuardianRewardMessage::MaxHealth,
            ),
            melee_damage: GuardianRewardField::new(
                0,
                GuardianReward::MeleeDamage,
                GuardianRewardMessage::MeleeDamage,
            ),
            rarity_rate: GuardianRewardField::new(
                3,
                GuardianReward::RarityRate,
                GuardianRewardMessage::RarityRate,
            ),
            recoil_reduction: GuardianRewardField::new(
                3,
                GuardianReward::RecoilReduction,
                GuardianRewardMessage::RecoilReduction,
            ),
            reload_speed: GuardianRewardField::new(
                3,
                GuardianReward::ReloadSpeed,
                GuardianRewardMessage::ReloadSpeed,
            ),
            shield_capacity: GuardianRewardField::new(
                3,
                GuardianReward::ShieldCapacity,
                GuardianRewardMessage::ShieldCapacity,
            ),
            shield_recharge_delay: GuardianRewardField::new(
                3,
                GuardianReward::ShieldRechargeDelay,
                GuardianRewardMessage::ShieldRechargeDelay,
            ),
            shield_recharge_rate: GuardianRewardField::new(
                3,
                GuardianReward::ShieldRechargeRate,
                GuardianRewardMessage::ShieldRechargeRate,
            ),
            vehicle_damage: GuardianRewardField::new(
                0,
                GuardianReward::VehicleDamage,
                GuardianRewardMessage::VehicleDamage,
            ),
            unlock_all_button_state: button::State::default(),
        }
    }
}

impl GuardianRewardUnlocker {
    pub fn view(&mut self) -> Container<Bl3Message> {
        Container::new(
            Column::new()
                .push(
                    Container::new(
                        Text::new("Guardian Rewards")
                            .font(JETBRAINS_MONO_BOLD)
                            .size(17)
                            .color(Color::from_rgb8(242, 203, 5)),
                    )
                    .padding(10)
                    .align_x(Horizontal::Center)
                    .width(Length::Fill)
                    .style(Bl3UiStyle),
                )
                .push(
                    Container::new(
                        Column::new()
                            .push(
                                Row::new()
                                    .push(self.critical_damage.view())
                                    .push(self.ffyl_duration.view())
                                    .push(self.accuracy.view()),
                            )
                            .push(
                                Row::new()
                                    .push(self.grenade_damage.view())
                                    .push(self.ffyl_movement_speed.view())
                                    .push(self.action_skill_cooldown.view()),
                            )
                            .push(
                                Row::new()
                                    .push(self.gun_damage.view())
                                    .push(self.max_health.view())
                                    .push(self.rarity_rate.view()),
                            )
                            .push(
                                Row::new()
                                    .push(self.gun_fire_rate.view())
                                    .push(self.shield_capacity.view())
                                    .push(self.recoil_reduction.view()),
                            )
                            .push(
                                Row::new()
                                    .push(self.melee_damage.view())
                                    .push(self.shield_recharge_delay.view())
                                    .push(self.reload_speed.view()),
                            )
                            .push(
                                Row::new()
                                    .push(self.vehicle_damage.view())
                                    .push(self.shield_recharge_rate.view())
                                    .push(self.elemental_damage.view()),
                            )
                            .push(
                                Container::new(
                                    Button::new(
                                        &mut self.unlock_all_button_state,
                                        Text::new("Max All Guardian Rewards")
                                            .font(JETBRAINS_MONO_BOLD)
                                            .size(17),
                                    )
                                    .on_press(InteractionMessage::ManageProfileInteraction(
                                        ManageProfileInteractionMessage::Profile(
                                            ProfileInteractionMessage::MaxGuardianRewardsPressed,
                                        ),
                                    ))
                                    .padding(10)
                                    .style(Bl3UiStyle)
                                    .into_element(),
                                )
                                .height(Length::Fill)
                                .align_y(Vertical::Bottom)
                                .padding(5),
                            )
                            .align_items(Alignment::Center)
                            .spacing(15),
                    )
                    .padding(15)
                    .height(Length::Units(524))
                    .style(Bl3UiStyle),
                ),
        )
    }

    pub fn all_rewards(&self) -> [&GuardianRewardField; 18] {
        [
            &self.accuracy,
            &self.action_skill_cooldown,
            &self.critical_damage,
            &self.elemental_damage,
            &self.ffyl_duration,
            &self.ffyl_movement_speed,
            &self.grenade_damage,
            &self.gun_damage,
            &self.gun_fire_rate,
            &self.max_health,
            &self.melee_damage,
            &self.rarity_rate,
            &self.recoil_reduction,
            &self.reload_speed,
            &self.shield_capacity,
            &self.shield_recharge_delay,
            &self.shield_recharge_rate,
            &self.vehicle_damage,
        ]
    }
}
