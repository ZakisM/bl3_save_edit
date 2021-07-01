use anyhow::{Context, Result};
use protobuf::RepeatedField;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use strum::Display;

use crate::bl3_profile::util::get_checksum_hash;
use crate::protos::oak_shared::InventoryCategorySaveData;

#[derive(Debug, Display)]
pub enum ProfileCurrency {
    #[strum(
        to_string = "/Game/Gear/_Shared/_Design/InventoryCategories/InventoryCategory_GoldenKey"
    )]
    GoldenKey,
    #[strum(
        to_string = "/Game/Gear/_Shared/_Design/InventoryCategories/InventoryCategory_DiamondKey"
    )]
    DiamondKey,
    #[strum(
        to_string = "/Game/Gear/_Shared/_Design/InventoryCategories/InventoryCategory_VaultCard1Key"
    )]
    VaultCardOneId,
}

impl ProfileCurrency {
    pub fn get_profile_currency(
        &self,
        bicl: &RepeatedField<InventoryCategorySaveData>,
    ) -> Result<i32> {
        let hash = get_checksum_hash(&self.to_string())?;

        let amount = bicl
            .par_iter()
            .find_first(|i| i.base_category_definition_hash as usize == hash)
            .map(|i| i.quantity)
            .with_context(|| format!("failed to read profile_currency amount for: {:?}", self))?;

        Ok(amount)
    }
}
