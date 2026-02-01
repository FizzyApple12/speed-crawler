use crate::managers::save_manager::SaveManager;
use crate::managers::shop::shop_ui_manager::ShopUIManager;
use crate::types::save_game::SaveGame;
use crate::types::upgrades::UpgradeType;
use godot::classes::{INode, Node};
use godot::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

#[derive(GodotClass)]
#[class(base=Node)]
struct ShopManager {
    #[export]
    save_manager: OnEditor<Gd<SaveManager>>,

    #[export]
    shop_ui: OnEditor<Gd<ShopUIManager>>,

    current_game: SaveGame,

    upgrades: [(UpgradeType, bool); 3],

    reroll_cost: i64,

    base: Base<Node>,
}

#[godot_api]
impl ShopManager {
    #[func]
    fn start_game(&mut self) {
        self.current_game.in_shop = false;

        self.save_manager
            .bind_mut()
            .update_save_game(Some(self.current_game.clone()));

        if let Some(ref mut scene_tree) = self.base().get_tree() {
            scene_tree.change_scene_to_file("res://scenes/game/game.tscn");
        }
    }

    #[func]
    fn save_and_exit(&mut self) {
        self.save_manager
            .bind_mut()
            .update_save_game(Some(self.current_game.clone()));

        if let Some(ref mut scene_tree) = self.base().get_tree() {
            scene_tree.change_scene_to_file("res://scenes/main_menu/main_menu.tscn");
        }
    }

    #[func]
    pub fn reroll_shop(&mut self) {
        let cost = self.reroll_cost;

        if self.current_game.money >= cost {
            self.current_game.money -= cost;

            self.reroll_cost *= 2;

            self.shop_ui
                .bind_mut()
                .set_reroll(self.reroll_cost, self.reroll_cost < self.current_game.money);

            self.shop_ui
                .bind_mut()
                .update_upgrades(self.current_game.clone());

            self.current_game.mod_shop_page += 1;

            self.populate_shop();

            self.save_manager
                .bind_mut()
                .update_save_game(Some(self.current_game.clone()));
        }
    }

    #[func]
    pub fn buy_upgrade(&mut self, number: i32) {
        let number = number as usize;

        let cost = self.upgrades[number].0.get_price();

        if self.current_game.money >= cost && !self.upgrades[number].1 {
            self.current_game.money -= cost;
            self.upgrades[number].1 = true;

            for (i, (upgrade, sold)) in self.upgrades.iter().enumerate() {
                self.shop_ui.bind_mut().set_upgrade_info(
                    i,
                    upgrade.get_name(),
                    upgrade.get_description(),
                    upgrade.get_price(),
                    upgrade.get_price() < self.current_game.money,
                    *sold,
                );
            }

            self.current_game = self.upgrades[number]
                .0
                .clone()
                .apply_upgrade(self.current_game.clone());

            self.shop_ui
                .bind_mut()
                .update_upgrades(self.current_game.clone());

            self.save_manager
                .bind_mut()
                .update_save_game(Some(self.current_game.clone()));
        }
    }

    pub fn populate_shop(&mut self) {
        let mut rng = SmallRng::from_seed(self.current_game.get_rng_seed());

        // fixes rng somehow????
        for _ in 0..3 {
            let _ = UpgradeType::generate_random(&mut rng);
        }

        self.upgrades = [
            (UpgradeType::generate_random(&mut rng), false),
            (UpgradeType::generate_random(&mut rng), false),
            (UpgradeType::generate_random(&mut rng), false),
        ];

        for (i, (upgrade, sold)) in self.upgrades.iter().enumerate() {
            self.shop_ui.bind_mut().set_upgrade_info(
                i,
                upgrade.get_name(),
                upgrade.get_description(),
                upgrade.get_price(),
                upgrade.get_price() < self.current_game.money,
                *sold,
            );
        }
    }
}

#[godot_api]
impl INode for ShopManager {
    fn init(base: Base<Node>) -> Self {
        let mut rng = SmallRng::from_os_rng();

        let upgrades = [
            (UpgradeType::generate_random(&mut rng), false),
            (UpgradeType::generate_random(&mut rng), false),
            (UpgradeType::generate_random(&mut rng), false),
        ];

        Self {
            save_manager: OnEditor::default(),

            shop_ui: OnEditor::default(),

            current_game: SaveGame::new(0),

            upgrades,

            reroll_cost: 4,

            base,
        }
    }

    fn ready(&mut self) {
        if let Some(ref save_file) = self.save_manager.bind().save_game {
            godot_print!("{save_file:?}");

            self.current_game = save_file.clone();
        } else {
            if let Some(ref mut scene_tree) = self.base().get_tree() {
                scene_tree.change_scene_to_file("res://scenes/main_menu/main_menu.tscn");
            }
        }

        self.populate_shop();

        self.shop_ui
            .bind_mut()
            .set_reroll(self.reroll_cost, self.reroll_cost < self.current_game.money);

        self.shop_ui
            .bind_mut()
            .update_upgrades(self.current_game.clone());
    }
}
