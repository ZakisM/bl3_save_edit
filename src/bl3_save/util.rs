use anyhow::{Context, Result};

use crate::bl3_save::character_data::Playthrough;
use crate::protos::oak_save::Character;

const REQUIRED_XP_LIST: [[i32; 2]; 80] = [
    [0, 1],
    [358, 2],
    [1241, 3],
    [2850, 4],
    [5376, 5],
    [8997, 6],
    [13886, 7],
    [20208, 8],
    [28126, 9],
    [37798, 10],
    [49377, 11],
    [63016, 12],
    [78861, 13],
    [97061, 14],
    [117757, 15],
    [141092, 16],
    [167206, 17],
    [196238, 18],
    [228322, 19],
    [263595, 20],
    [302190, 21],
    [344238, 22],
    [389873, 23],
    [439222, 24],
    [492414, 25],
    [549578, 26],
    [610840, 27],
    [676325, 28],
    [746158, 29],
    [820463, 30],
    [899363, 31],
    [982980, 32],
    [1071435, 33],
    [1164850, 34],
    [1263343, 35],
    [1367034, 36],
    [1476041, 37],
    [1590483, 38],
    [1710476, 39],
    [1836137, 40],
    [1967582, 41],
    [2104926, 42],
    [2248285, 43],
    [2397772, 44],
    [2553501, 45],
    [2715586, 46],
    [2884139, 47],
    [3059273, 48],
    [3241098, 49],
    [3429728, 50],
    [3625271, 51],
    [3827840, 52],
    [4037543, 53],
    [4254491, 54],
    [4478792, 55],
    [4710556, 56],
    [4949890, 57],
    [5196902, 58],
    [5451701, 59],
    [5714393, 60],
    [5985086, 61],
    [6263885, 62],
    [6550897, 63],
    [6846227, 64],
    [7149982, 65],
    [7462266, 66],
    [7783184, 67],
    [8112840, 68],
    [8451340, 69],
    [8798786, 70],
    [9155282, 71],
    [9520931, 72],
    [9895837, 73],
    [10280103, 74],
    [10673830, 75],
    [11077120, 76],
    [11490077, 77],
    [11912801, 78],
    [12345393, 79],
    [12787955, 80],
];

#[derive(Debug)]
pub enum Currency {
    Money,
    Eridium,
}

pub fn currency_amount_from_character(character: &Character, currency: &Currency) -> Result<i32> {
    character
        .inventory_category_list
        .iter()
        .find(|i| {
            i.base_category_definition_hash
                == match currency {
                    Currency::Money => 618814354,
                    Currency::Eridium => 3679636065,
                }
        })
        .map(|i| i.quantity)
        .context("could not find currency amount")
}

pub fn experience_to_level(experience: &i32) -> Result<i32> {
    REQUIRED_XP_LIST
        .iter()
        .rev()
        .find(|[xp, _]| experience >= xp)
        .map(|[_, level]| *level)
        .context("could not calculate level based off of experience")
}

pub fn read_playthroughs(character: &Character) -> Result<Vec<Playthrough>> {
    dbg!(&character.game_state_save_data_for_playthrough);
    dbg!(&character.last_active_travel_station_for_playthrough);

    Ok(Vec::new())
}
