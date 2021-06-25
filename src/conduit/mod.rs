use std::collections::HashMap;

use crate::models::gi_fast_travel::GiFastTravel;
use crate::models::gi_mission::GiMission;
use crate::to_hash_map;

pub mod gi_fast_travel;
pub mod gi_mission;

pub trait ConduitExt {
    fn to_hash_map(&self) -> HashMap<String, String>;
}

to_hash_map!(GiFastTravel, raw, fullname);
to_hash_map!(GiMission, raw, fullname);

#[macro_export]
macro_rules! to_hash_map {
    ($struct_name:path, $field_1:ident, $field_2:ident) => {
        impl ConduitExt for Vec<$struct_name> {
            fn to_hash_map(&self) -> HashMap<String, String> {
                self.iter().fold(HashMap::new(), |mut curr, next| {
                    curr.insert(next.$field_1.to_owned(), next.$field_2.to_owned());
                    curr
                })
            }
        }
    };
}
