use anyhow::Result;

use crate::bl3_profile::profile_currency::ProfileCurrency;
use crate::protos::oak_profile::Profile;

#[derive(Debug)]
pub struct ProfileData {
    pub profile: Profile,
    pub golden_keys: i32,
    pub diamond_keys: i32,
    pub vault_card_1_keys: i32,
}

impl ProfileData {
    pub fn from_profile(profile: Profile) -> Result<Self> {
        let golden_keys = ProfileCurrency::GoldenKey
            .get_profile_currency(&profile.bank_inventory_category_list)
            .unwrap_or(0);
        let diamond_keys = ProfileCurrency::DiamondKey
            .get_profile_currency(&profile.bank_inventory_category_list)
            .unwrap_or(0);
        let vault_card_1_keys = ProfileCurrency::VaultCardOneId
            .get_profile_currency(&profile.bank_inventory_category_list)
            .unwrap_or(0);

        dbg!(&golden_keys);
        dbg!(&diamond_keys);
        dbg!(&vault_card_1_keys);

        Ok(Self {
            profile,
            golden_keys,
            diamond_keys,
            vault_card_1_keys,
        })
    }
}
