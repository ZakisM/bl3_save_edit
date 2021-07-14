use iced::{
    button, pick_list, scrollable, Align, Button, Checkbox, Color, Column, Container, Length,
    PickList, Row, Scrollable, Text,
};
use strum::Display;

use bl3_save_edit_core::game_data::{GameDataKv, FAST_TRAVEL};

use crate::bl3_ui::{InteractionMessage, Message};
use crate::bl3_ui_style::Bl3UiStyle;
use crate::interaction::InteractionExt;
use crate::resources::fonts::{JETBRAINS_MONO, JETBRAINS_MONO_BOLD};
use crate::views::manage_save::{ManageSaveInteractionMessage, ManageSaveMessage};
use crate::widgets::text_margin::TextMargin;

#[derive(Debug, Clone, Display, Eq, PartialEq)]
pub enum PlaythroughType {
    Normal,
    #[strum(to_string = "TVHM")]
    Tvhm,
}

impl PlaythroughType {
    pub const ALL: [PlaythroughType; 2] = [PlaythroughType::Normal, PlaythroughType::Tvhm];
}

#[derive(Debug, Default)]
pub struct VisitedTeleporter {
    pub game_data: GameDataKv,
    pub visited: bool,
}

#[derive(Debug)]
pub struct FastTravelState {
    pub playthrough_type_selector: pick_list::State<PlaythroughType>,
    pub playthrough_type_selected: PlaythroughType,
    pub last_visited_teleporter_selector: pick_list::State<GameDataKv>,
    pub last_visited_teleporter_selected: GameDataKv,
    pub fast_travel_locations: Vec<GameDataKv>,
    pub visited_teleporters_list: Vec<VisitedTeleporter>,
    pub visited_teleporters_list_scrollable_state: scrollable::State,
    pub visited_teleporters_list_check_none_button_state: button::State,
    pub visited_teleporters_list_check_all_button_state: button::State,
}

impl std::default::Default for FastTravelState {
    fn default() -> Self {
        let mut fast_travel_locations = FAST_TRAVEL
            .iter()
            .filter(|gd| gd.ident.contains("gamedata/fasttravel/fts_"))
            .cloned()
            .collect::<Vec<_>>();

        fast_travel_locations.sort();
        // fast_travel_locations.dedup();

        let visited_teleporters_list = fast_travel_locations
            .iter()
            .cloned()
            .map(|game_data| VisitedTeleporter {
                game_data,
                visited: false,
            })
            .collect::<Vec<_>>();

        dbg!(&visited_teleporters_list.len());

        Self {
            playthrough_type_selector: Default::default(),
            playthrough_type_selected: PlaythroughType::Normal,
            last_visited_teleporter_selector: Default::default(),
            last_visited_teleporter_selected: Default::default(),
            fast_travel_locations,
            visited_teleporters_list,
            visited_teleporters_list_scrollable_state: Default::default(),
            visited_teleporters_list_check_none_button_state: Default::default(),
            visited_teleporters_list_check_all_button_state: Default::default(),
        }
    }
}

#[derive(Debug)]
pub enum FastTravelMessage {
    LastVisitedTeleporterSelected(GameDataKv),
    PlaythroughSelected(PlaythroughType),
    VisitedTeleportersListUpdated((usize, bool)),
}

#[derive(Debug, Clone)]
pub enum FastTravelInteractionMessage {
    CheckNoneVisitedTeleporterList,
    CheckAllVisitedTeleporterList,
}

pub fn view(fast_travel_state: &mut FastTravelState) -> Container<Message> {
    let playthrough_selector = Container::new(
        Row::new()
            .push(
                TextMargin::new("Playthrough", 2)
                    .0
                    .font(JETBRAINS_MONO)
                    .size(17)
                    .color(Color::from_rgb8(242, 203, 5))
                    .width(Length::Units(115)),
            )
            .push(
                PickList::new(
                    &mut fast_travel_state.playthrough_type_selector,
                    &PlaythroughType::ALL[..],
                    Some(fast_travel_state.playthrough_type_selected.clone()),
                    |s| {
                        Message::ManageSave(ManageSaveMessage::FastTravel(
                            FastTravelMessage::PlaythroughSelected(s),
                        ))
                    },
                )
                .font(JETBRAINS_MONO)
                .text_size(17)
                .width(Length::Fill)
                .padding(10)
                .style(Bl3UiStyle),
            )
            .align_items(Align::Center),
    )
    .width(Length::FillPortion(2))
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let pre_selected_last_visited_teleporter =
        match fast_travel_state.last_visited_teleporter_selected {
            GameDataKv { ident, name: _ } if ident.is_empty() => {
                Some(fast_travel_state.fast_travel_locations[0])
            }
            current => Some(current),
        };

    let last_visited_teleporter_selector = Container::new(
        Row::new()
            .push(
                TextMargin::new("Last Visited Teleporter", 2)
                    .0
                    .font(JETBRAINS_MONO)
                    .size(17)
                    .color(Color::from_rgb8(242, 203, 5))
                    .width(Length::Units(210)),
            )
            .push(
                PickList::new(
                    &mut fast_travel_state.last_visited_teleporter_selector,
                    &fast_travel_state.fast_travel_locations,
                    pre_selected_last_visited_teleporter,
                    |s| {
                        Message::ManageSave(ManageSaveMessage::FastTravel(
                            FastTravelMessage::LastVisitedTeleporterSelected(s),
                        ))
                    },
                )
                .font(JETBRAINS_MONO)
                .text_size(17)
                .width(Length::Fill)
                .padding(10)
                .style(Bl3UiStyle),
            )
            .align_items(Align::Center),
    )
    .width(Length::FillPortion(7))
    .height(Length::Units(36))
    .style(Bl3UiStyle);

    let playthrough_last_visited_row = Row::new()
        .push(playthrough_selector)
        .push(last_visited_teleporter_selector)
        .spacing(20);

    let mut teleporter_unlocker_checkboxes = Column::new().spacing(15);

    for (i, teleporter) in fast_travel_state
        .visited_teleporters_list
        .iter()
        .enumerate()
    {
        teleporter_unlocker_checkboxes = teleporter_unlocker_checkboxes.push(
            Checkbox::new(teleporter.visited, teleporter.game_data.name, move |b| {
                Message::ManageSave(ManageSaveMessage::FastTravel(
                    FastTravelMessage::VisitedTeleportersListUpdated((i, b)),
                ))
            })
            .size(20)
            .font(JETBRAINS_MONO)
            .text_color(Color::from_rgb8(220, 220, 220))
            .text_size(17)
            .style(Bl3UiStyle),
        );
    }

    let check_buttons_row = Row::new()
        .push(
            Button::new(
                &mut fast_travel_state.visited_teleporters_list_check_all_button_state,
                Text::new("Check All").font(JETBRAINS_MONO_BOLD).size(17),
            )
            .on_press(InteractionMessage::ManageSaveInteraction(
                ManageSaveInteractionMessage::FastTravel(
                    FastTravelInteractionMessage::CheckAllVisitedTeleporterList,
                ),
            ))
            .padding(10)
            .style(Bl3UiStyle)
            .into_element(),
        )
        .push(
            Button::new(
                &mut fast_travel_state.visited_teleporters_list_check_none_button_state,
                Text::new("Uncheck All").font(JETBRAINS_MONO_BOLD).size(17),
            )
            .on_press(InteractionMessage::ManageSaveInteraction(
                ManageSaveInteractionMessage::FastTravel(
                    FastTravelInteractionMessage::CheckNoneVisitedTeleporterList,
                ),
            ))
            .padding(10)
            .style(Bl3UiStyle)
            .into_element(),
        )
        .spacing(20);

    let teleporter_unlocker = Container::new(
        Column::new()
            .push(
                Container::new(
                    TextMargin::new("Visited Teleporters", 2)
                        .0
                        .font(JETBRAINS_MONO)
                        .size(17)
                        .color(Color::from_rgb8(242, 203, 5)),
                )
                .padding(10)
                .align_x(Align::Center)
                .width(Length::Fill)
                .style(Bl3UiStyle),
            )
            .push(
                Container::new(
                    Column::new().push(
                        Column::new()
                            .push(
                                Scrollable::new(
                                    &mut fast_travel_state
                                        .visited_teleporters_list_scrollable_state,
                                )
                                .push(teleporter_unlocker_checkboxes)
                                .height(Length::FillPortion(8))
                                .width(Length::Fill),
                            )
                            .push(check_buttons_row)
                            .align_items(Align::Center)
                            .spacing(15),
                    ),
                )
                .padding(20)
                .style(Bl3UiStyle),
            ),
    )
    .width(Length::Fill);

    let all_contents = Column::new()
        .push(playthrough_last_visited_row)
        .push(teleporter_unlocker)
        .spacing(20);

    Container::new(all_contents).padding(30)
}
