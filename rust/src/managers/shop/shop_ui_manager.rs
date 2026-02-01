use godot::classes::{Button, INode, Label, Node};
use godot::prelude::*;

use crate::types::save_game::SaveGame;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ShopUIManager {
    #[export]
    current_money: OnEditor<Gd<Label>>,
    #[export]
    current_upgrades: OnEditor<Gd<Label>>,

    #[export]
    upgrade_name_labels: Array<Gd<Label>>,
    #[export]
    upgrade_description_labels: Array<Gd<Label>>,
    #[export]
    upgrade_buy_buttons: Array<Gd<Button>>,

    #[export]
    reroll_button: OnEditor<Gd<Button>>,

    base: Base<Node>,
}

impl ShopUIManager {
    pub fn update_upgrades(&mut self, game: SaveGame) {
        self.current_money
            .set_text(&format!("wallet: ${}", game.money));
        self.current_upgrades
            .set_text(&format!("preview time: {:.1}s\ntop speed: {:.1}m/s\nacceleration: {:.1}m/s^2\nview distance: {:.1}m\nmass: {:.1}kg",
            	game.player_properties.warmup_time,
             	game.player_properties.max_speed,
              	game.player_properties.active_acceleration,
               	game.player_properties.view_distance,
                game.player_properties.stopping_mass
            ));
    }

    pub fn set_upgrade_info(
        &mut self,
        index: usize,
        name: String,
        description: String,
        price: i64,
        can_buy: bool,
        is_sold: bool,
    ) {
        self.upgrade_name_labels.get(index).unwrap().set_text(&name);
        self.upgrade_description_labels
            .get(index)
            .unwrap()
            .set_text(&description);
        self.upgrade_buy_buttons.get(index).unwrap().set_text(
            &(if is_sold {
                "sold".to_owned()
            } else {
                format!("buy ${}", price)
            }),
        );
        self.upgrade_buy_buttons
            .get(index)
            .unwrap()
            .set_disabled(!can_buy || is_sold);
    }

    pub fn set_reroll(&mut self, price: i64, can_buy: bool) {
        self.reroll_button.set_text(&format!("reroll ${price}"));
        self.reroll_button.set_disabled(!can_buy);
    }
}

#[godot_api]
impl INode for ShopUIManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            current_money: OnEditor::default(),
            current_upgrades: OnEditor::default(),

            upgrade_name_labels: Array::default(),
            upgrade_description_labels: Array::default(),
            upgrade_buy_buttons: Array::default(),

            reroll_button: OnEditor::default(),

            base,
        }
    }

    fn ready(&mut self) {}
}
