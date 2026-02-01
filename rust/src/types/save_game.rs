use serde::{Deserialize, Serialize};

use crate::types::player_properties::PlayerProperties;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SaveGame {
    pub level_seed: i64,

    pub current_floor: i64,

    pub in_shop: bool,
    pub mod_shop_page: i32,

    pub money: i64,

    pub player_properties: PlayerProperties,
}

impl SaveGame {
    pub fn new(seed: i64) -> SaveGame {
        SaveGame {
            level_seed: seed,

            current_floor: 0,

            in_shop: false,
            mod_shop_page: 0,

            money: 0,

            player_properties: PlayerProperties::default(),
        }
    }

    pub fn get_rng_seed(&self) -> [u8; 32] {
        self.level_seed
            .to_le_bytes()
            .iter()
            .chain(&self.current_floor.to_le_bytes())
            .chain(
                &(if self.in_shop {
                    (self.mod_shop_page as i64) + 1i64
                } else {
                    0i64
                })
                .to_le_bytes(),
            )
            .chain(&self.money.to_le_bytes())
            .copied()
            .collect::<Vec<u8>>()
            .as_slice()
            .try_into()
            .unwrap()
    }
}
