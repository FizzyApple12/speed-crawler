use crate::managers::game::floor_manager::FloorLayout;
use crate::managers::game::game_manager::GameState;
use crate::objects::map::ROOM_GRID_BASIS;
use crate::types::input_bindings::InputBindings;
use crate::types::save_game::SaveGame;
use godot::classes::{AnimatedSprite3D, INode3D, Input, Node3D, TextureProgressBar};
use godot::prelude::*;

const UP_DIRECTION: (i64, i64) = (0, 1);
const DOWN_DIRECTION: (i64, i64) = (0, -1);
const LEFT_DIRECTION: (i64, i64) = (-1, 0);
const RIGHT_DIRECTION: (i64, i64) = (1, 0);

#[derive(Debug, PartialEq, Clone)]
enum PlayerStatus {
    Idle,
    Moving,
    Cooling,
}

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Player {
    #[export]
    sprite: OnEditor<Gd<AnimatedSprite3D>>,

    #[export]
    cooldown_ring: OnEditor<Gd<TextureProgressBar>>,

    game_state: GameState,

    current_game: SaveGame,

    floor_layout: FloorLayout,

    player_status: PlayerStatus,

    is_running_animation: bool,

    speed: f64,
    cooldown: f64,
    total_cooldown: f64,

    direction: (i64, i64),

    target_position: (i64, i64),

    base: Base<Node3D>,
}

impl Player {
    pub fn set_game_properties(&mut self, save_game: SaveGame, floor_layout: FloorLayout) {
        self.current_game = save_game;
        self.floor_layout = floor_layout;
    }

    pub fn reset(&mut self) {
        self.is_running_animation = false;

        self.speed = 0.0;
        self.cooldown = 0.0;
        self.total_cooldown = 0.0;

        self.direction = (0, 0);

        self.target_position = (0, 0);

        self.base_mut().set_position(Vector3::new(0.0, 0.0, 0.0));
        self.cooldown_ring.set_value(0.0);

        self.sprite.set_animation("default");
        self.sprite.play();
    }

    pub fn change_game_state(&mut self, next_game_state: GameState) {
        self.game_state = next_game_state;
    }

    fn calculate_stop_cooldown(&self) -> f64 {
        (self.speed.abs().powf(1.4) / 100.0)
            * (self.current_game.player_properties.stopping_mass / 128.0)
    }

    fn change_player_status(&mut self, next_player_status: PlayerStatus) {
        match next_player_status {
            PlayerStatus::Idle => {}
            PlayerStatus::Moving => {
                let last_target_position = self.target_position;

                self.target_position = (
                    last_target_position.0 + self.direction.0,
                    last_target_position.1 + self.direction.1,
                );

                if !self.floor_layout.contains_key(&self.target_position) {
                    self.target_position = last_target_position;
                    self.direction = (0, 0);

                    self.change_player_status(PlayerStatus::Cooling);

                    return;
                }
            }
            PlayerStatus::Cooling => {
                self.cooldown = self.calculate_stop_cooldown();
                self.total_cooldown = self.cooldown;
            }
        }

        self.player_status = next_player_status;
    }
}

#[godot_api]
impl INode3D for Player {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            sprite: OnEditor::default(),

            cooldown_ring: OnEditor::default(),

            game_state: GameState::Loading,

            current_game: SaveGame::new(0),

            floor_layout: FloorLayout::default(),

            player_status: PlayerStatus::Idle,

            is_running_animation: false,

            speed: 0.0,
            cooldown: 0.0,
            total_cooldown: 0.0,

            direction: (0, 0),

            target_position: (0, 0),

            base,
        }
    }

    fn ready(&mut self) {
        self.reset();
    }

    fn process(&mut self, _delta: f64) {
        let should_running_animation = self.speed.abs() > 0.1;

        if should_running_animation != self.is_running_animation {
            self.sprite.set_animation(if should_running_animation {
                "run"
            } else {
                "default"
            });

            self.is_running_animation = should_running_animation;
        }

        self.sprite
            .get_sprite_frames()
            .unwrap()
            .set_animation_speed("run", 5.0 + (self.speed * 2.0));

        match self.direction {
            (-1, 0) => {
                self.sprite
                    .set_rotation_degrees(Vector3::new(0.0, 0.0, 180.0));
            }
            (0, 1) => {
                self.sprite
                    .set_rotation_degrees(Vector3::new(0.0, 0.0, 90.0));
            }
            (0, -1) => {
                self.sprite
                    .set_rotation_degrees(Vector3::new(0.0, 0.0, 270.0));
            }
            _ => {
                self.sprite
                    .set_rotation_degrees(Vector3::new(0.0, 0.0, 0.0));
            }
        }

        match self.game_state {
            GameState::Running => {
                self.cooldown_ring
                    .set_value((self.cooldown / self.total_cooldown.max(0.001)) * 100.0);
            }
            _ => {
                self.cooldown_ring.set_value(0.0);
            }
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let is_pressing_up: bool =
            Input::singleton().is_action_pressed(Into::<&str>::into(InputBindings::PlayerMoveUp));
        let is_pressing_down: bool =
            Input::singleton().is_action_pressed(Into::<&str>::into(InputBindings::PlayerMoveDown));
        let is_pressing_left: bool =
            Input::singleton().is_action_pressed(Into::<&str>::into(InputBindings::PlayerMoveLeft));
        let is_pressing_right: bool = Input::singleton()
            .is_action_pressed(Into::<&str>::into(InputBindings::PlayerMoveRight));

        if self.game_state != GameState::Running {
            return;
        }

        match self.player_status {
            PlayerStatus::Idle => {
                if is_pressing_up {
                    self.direction = UP_DIRECTION;

                    self.change_player_status(PlayerStatus::Moving);
                } else if is_pressing_down {
                    self.direction = DOWN_DIRECTION;

                    self.change_player_status(PlayerStatus::Moving);
                } else if is_pressing_left {
                    self.direction = LEFT_DIRECTION;

                    self.change_player_status(PlayerStatus::Moving);
                } else if is_pressing_right {
                    self.direction = RIGHT_DIRECTION;

                    self.change_player_status(PlayerStatus::Moving);
                }
            }
            PlayerStatus::Moving => {
                self.speed = (self.speed
                    + (self.current_game.player_properties.active_acceleration * delta))
                    .min(self.current_game.player_properties.max_speed);

                let speed_vector = Vector3::new(
                    (self.speed as f32 * self.direction.0 as f32) * delta as f32,
                    (self.speed as f32 * self.direction.1 as f32) * delta as f32,
                    0.0,
                );

                let position = self.base().get_position();

                self.base_mut().set_position(position + speed_vector);

                let position = self.base().get_position();

                // have we passed the target position
                if (self.direction == UP_DIRECTION
                    && position.y >= self.target_position.1 as f32 * ROOM_GRID_BASIS)
                    || (self.direction == DOWN_DIRECTION
                        && position.y <= self.target_position.1 as f32 * ROOM_GRID_BASIS)
                    || (self.direction == LEFT_DIRECTION
                        && position.x <= self.target_position.0 as f32 * ROOM_GRID_BASIS)
                    || (self.direction == RIGHT_DIRECTION
                        && position.x >= self.target_position.0 as f32 * ROOM_GRID_BASIS)
                {
                    if is_pressing_up {
                        if self.direction != UP_DIRECTION {
                            self.change_player_status(PlayerStatus::Cooling);
                        }

                        self.direction = UP_DIRECTION;
                    } else if is_pressing_down {
                        if self.direction != DOWN_DIRECTION {
                            self.change_player_status(PlayerStatus::Cooling);
                        }

                        self.direction = DOWN_DIRECTION;
                    } else if is_pressing_left {
                        if self.direction != LEFT_DIRECTION {
                            self.change_player_status(PlayerStatus::Cooling);
                        }

                        self.direction = LEFT_DIRECTION;
                    } else if is_pressing_right {
                        if self.direction != RIGHT_DIRECTION {
                            self.change_player_status(PlayerStatus::Cooling);
                        }

                        self.direction = RIGHT_DIRECTION;
                    } else {
                        self.change_player_status(PlayerStatus::Cooling);

                        self.direction = (0, 0);
                    }

                    if self.direction != (0, 0) && self.player_status != PlayerStatus::Cooling {
                        self.change_player_status(PlayerStatus::Moving);
                    }
                }
            }
            PlayerStatus::Cooling => {
                let target_position = self.target_position;

                self.base_mut().set_position(Vector3::new(
                    target_position.0 as f32 * ROOM_GRID_BASIS,
                    target_position.1 as f32 * ROOM_GRID_BASIS,
                    0.0,
                ));

                self.speed = 0.0;

                if is_pressing_up {
                    self.direction = UP_DIRECTION;
                } else if is_pressing_down {
                    self.direction = DOWN_DIRECTION;
                } else if is_pressing_left {
                    self.direction = LEFT_DIRECTION;
                } else if is_pressing_right {
                    self.direction = RIGHT_DIRECTION;
                }

                if self.cooldown > 0.0 {
                    self.cooldown -= delta;
                } else if self.direction != (0, 0) {
                    self.change_player_status(PlayerStatus::Moving);
                } else {
                    self.change_player_status(PlayerStatus::Idle);
                }
            }
        }
    }
}
