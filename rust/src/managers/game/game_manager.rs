use crate::managers::game::floor_manager::{FloorLayout, FloorManager};
use crate::managers::game::game_ui_manager::{GameUIManager, GameUIView};
use crate::managers::save_manager::SaveManager;
use crate::objects::player::Player;
use crate::types::input_bindings::InputBindings;
use crate::types::save_game::SaveGame;
use godot::classes::{INode, InputEvent, Node};
use godot::prelude::*;

const MAX_MONEY_GAIN: i64 = 30;
const MAX_MONEY_LOSS: i64 = -20;

#[derive(Debug, PartialEq, Clone)]
pub enum GameState {
    Loading,
    WarmUp,
    Running,
    Scoring,
    Paused,
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameManager {
    #[export]
    save_manager: OnEditor<Gd<SaveManager>>,

    #[export]
    game_ui: OnEditor<Gd<GameUIManager>>,

    #[export]
    player: OnEditor<Gd<Player>>,
    #[export]
    floor_manager: OnEditor<Gd<FloorManager>>,

    pub game_state: GameState,

    pub current_game: SaveGame,

    floor_layout: FloorLayout,

    estimated_completion_time: f64,

    run_timer: bool,
    warmup_timer: f64,
    game_timer: f64,

    base: Base<Node>,
}

#[godot_api]
impl GameManager {
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
    fn clear_and_exit(&mut self) {
        self.save_manager.bind_mut().update_save_game(None);

        if let Some(ref mut scene_tree) = self.base().get_tree() {
            scene_tree.change_scene_to_file("res://scenes/main_menu/main_menu.tscn");
        }
    }

    #[func]
    fn restart_level(&mut self) {
        self.change_game_state(GameState::Loading);
    }

    #[func]
    fn start_shop(&mut self) {
        self.save_manager
            .bind_mut()
            .update_save_game(Some(self.current_game.clone()));

        if let Some(ref mut scene_tree) = self.base().get_tree() {
            scene_tree.change_scene_to_file("res://scenes/shop/shop.tscn");
        }
    }

    fn change_game_state(&mut self, next_game_state: GameState) {
        godot_print!("setting game state to: {next_game_state:?}");

        match next_game_state {
            GameState::Loading => {
                self.setup_level();

                self.game_ui.bind_mut().open_ui_view(GameUIView::Loading);
            }
            GameState::WarmUp => {
                self.run_timer = true;
                self.warmup_timer = self.current_game.player_properties.warmup_time;

                self.game_ui
                    .bind_mut()
                    .set_game_time(0.0, self.estimated_completion_time);
                self.game_ui.bind_mut().open_ui_view(GameUIView::Countdown);
            }
            GameState::Running => {
                self.run_timer = true;
                self.game_timer = 0.0;

                self.game_ui.bind_mut().open_ui_view(GameUIView::Playing);
            }
            GameState::Scoring => {
                let game_over = self.score_run();

                self.game_ui
                    .bind_mut()
                    .open_ui_view(GameUIView::Scoring(game_over));
            }
            GameState::Paused => {
                self.game_ui.bind_mut().open_ui_view(GameUIView::Paused);
            }
        }

        self.game_state = next_game_state.clone();

        self.player.bind_mut().change_game_state(next_game_state);
    }

    fn setup_level(&mut self) {
        self.floor_manager
            .bind_mut()
            .setup_level(self.current_game.clone());
    }

    pub fn level_setup_complete(
        &mut self,
        floor_layout: FloorLayout,
        estimated_completion_time: f64,
    ) {
        self.floor_layout = floor_layout;
        self.estimated_completion_time = estimated_completion_time;

        self.player
            .bind_mut()
            .set_game_properties(self.current_game.clone(), self.floor_layout.clone());

        self.player.bind_mut().reset();

        self.change_game_state(GameState::WarmUp);
    }

    pub fn level_setup_failed(&mut self) {
        self.change_game_state(GameState::Loading);
    }

    fn score_run(&mut self) -> bool {
        self.current_game.in_shop = true;

        let logical_estimated_time = self.estimated_completion_time.max(0.1);
        let logical_completion_time = self.game_timer.max(0.1);

        let money_delta = if self.current_game.current_floor == 0 {
            10
        } else {
            ((((logical_estimated_time - logical_completion_time) / logical_completion_time) * 10.0)
                .round() as i64)
                .clamp(MAX_MONEY_LOSS, MAX_MONEY_GAIN)
        };

        let starting_money = self.current_game.money;

        let ending_money = starting_money + money_delta;

        self.current_game.money = ending_money;

        self.game_ui.bind_mut().set_scores(
            self.game_timer,
            self.estimated_completion_time,
            starting_money,
            ending_money,
        );

        self.current_game.current_floor += 1;

        if ending_money <= 0 {
            self.save_manager.bind_mut().update_save_game(None);

            true
        } else {
            self.save_manager
                .bind_mut()
                .update_save_game(Some(self.current_game.clone()));

            false
        }
    }
}

#[godot_api]
impl INode for GameManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            save_manager: OnEditor::default(),

            game_ui: OnEditor::default(),

            player: OnEditor::default(),
            floor_manager: OnEditor::default(),

            game_state: GameState::Loading,

            current_game: SaveGame::new(0),

            floor_layout: FloorLayout::new(),

            estimated_completion_time: 0.0,

            run_timer: false,
            warmup_timer: 0.0,
            game_timer: 0.0,

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

        self.change_game_state(GameState::Loading);
    }

    fn process(&mut self, _delta: f64) {
        match self.game_state {
            GameState::WarmUp => {
                self.game_ui.bind_mut().set_countdown_progress(
                    self.warmup_timer,
                    self.current_game.player_properties.warmup_time,
                );

                if !self.run_timer {
                    self.change_game_state(GameState::Running);
                }
            }
            GameState::Running => {
                self.game_ui
                    .bind_mut()
                    .set_game_time(self.game_timer, self.estimated_completion_time);

                if !self.run_timer {
                    self.change_game_state(GameState::Scoring);
                }
            }
            _ => {}
        }
    }

    fn physics_process(&mut self, delta: f64) {
        match self.game_state {
            GameState::WarmUp => {
                if self.run_timer {
                    self.warmup_timer -= delta;
                }

                if self.warmup_timer <= 0.0 {
                    self.run_timer = false;
                }
            }
            GameState::Running => {
                self.game_timer += delta;

                let complete_progress = self.floor_manager.bind().get_completion_progress();

                self.game_ui
                    .bind_mut()
                    .set_exploration_progress(complete_progress);

                if complete_progress >= 1.0 {
                    self.run_timer = false;
                }
            }
            _ => {}
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event
            .is_action_pressed_ex(Into::<&str>::into(InputBindings::GamePause))
            .allow_echo(false)
            .exact_match(true)
            .done()
            && self.game_state != GameState::Scoring
            && self.game_state != GameState::Paused
        {
            self.change_game_state(GameState::Paused);
        }
    }
}
