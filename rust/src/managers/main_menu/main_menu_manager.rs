use crate::managers::main_menu::main_menu_ui_manager::{MainMenuUIManager, MenuUIView};
use crate::managers::save_manager::SaveManager;
use crate::types::save_game::SaveGame;
use godot::classes::{INode, Node, Time};
use godot::global::{randi, seed};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
struct MainMenuManager {
    #[export]
    save_manager: OnEditor<Gd<SaveManager>>,

    #[export]
    main_menu_ui: OnEditor<Gd<MainMenuUIManager>>,

    base: Base<Node>,
}

#[godot_api]
impl MainMenuManager {
    #[func]
    fn open_start_game_menu(&mut self) {
        self.main_menu_ui
            .bind_mut()
            .open_ui_view(MenuUIView::GameSetup);
    }

    #[func]
    fn reroll_seed(&mut self) {
        let new_seed = randi();

        self.main_menu_ui.bind_mut().set_seed(new_seed);
    }

    #[func]
    fn start_game(&mut self) {
        let new_game_seed = self.main_menu_ui.bind_mut().get_seed();

        self.save_manager
            .bind_mut()
            .update_save_game(Some(SaveGame::new(new_game_seed)));

        if let Some(ref mut scene_tree) = self.base().get_tree() {
            scene_tree.change_scene_to_file("res://scenes/game/game.tscn");
        }
    }

    #[func]
    fn resume_game(&mut self) {
        let in_shop = match self.save_manager.bind().save_game {
            Some(ref save_game) => save_game.in_shop,
            None => false,
        };

        if let Some(ref mut scene_tree) = self.base().get_tree() {
            if in_shop {
                scene_tree.change_scene_to_file("res://scenes/shop/shop.tscn");
            } else {
                scene_tree.change_scene_to_file("res://scenes/game/game.tscn");
            }
        }
    }
}

#[godot_api]
impl INode for MainMenuManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            save_manager: OnEditor::default(),

            main_menu_ui: OnEditor::default(),

            base,
        }
    }

    fn ready(&mut self) {
        let system_seed = Time::singleton().get_unix_time_from_system() as i64;

        seed(system_seed);

        self.reroll_seed();

        self.main_menu_ui
            .bind_mut()
            .allow_resume(self.save_manager.bind_mut().save_game.is_some());
    }
}
