use crate::objects::map::ROOM_GRID_BASIS;
use crate::objects::player::Player;
use crate::types::save_game::SaveGame;
use godot::classes::{INode3D, MeshInstance3D, Node3D};
use godot::prelude::*;

const HAS_CORRIDOR_HORIZONTAL: f32 = 2.5;
const HAS_CORRIDOR_VERTICAL: f32 = 0.1;

const NO_CORRIDOR_HORIZONTAL: f32 = 2.349;
const NO_CORRIDOR_VERTICAL: f32 = -0.1;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Room {
    #[export]
    pub player: Option<Gd<Player>>,

    #[export]
    center_fog: OnEditor<Gd<MeshInstance3D>>,

    #[export]
    top_left_fog: OnEditor<Gd<MeshInstance3D>>,
    #[export]
    top_right_fog: OnEditor<Gd<MeshInstance3D>>,
    #[export]
    bottom_left_fog: OnEditor<Gd<MeshInstance3D>>,
    #[export]
    bottom_right_fog: OnEditor<Gd<MeshInstance3D>>,

    #[export]
    left_corridor: OnEditor<Gd<MeshInstance3D>>,
    #[export]
    right_corridor: OnEditor<Gd<MeshInstance3D>>,
    #[export]
    top_corridor: OnEditor<Gd<MeshInstance3D>>,
    #[export]
    bottom_corridor: OnEditor<Gd<MeshInstance3D>>,

    room_completely_revealed: bool,

    seen_horizontal_corridors: bool,
    seen_vertical_corridors: bool,

    seen_center: bool,

    seen_top_left: bool,
    seen_top_right: bool,
    seen_bottom_left: bool,
    seen_bottom_right: bool,

    current_game: SaveGame,

    pub seen_progress: f64,

    base: Base<Node3D>,
}

impl Room {
    pub fn place(&mut self, (x, y): (i64, i64)) {
        self.base_mut().set_position(Vector3 {
            x: x as f32 * ROOM_GRID_BASIS,
            y: y as f32 * ROOM_GRID_BASIS,
            z: 0.0,
        });
    }

    pub fn reset(&mut self) {
        self.room_completely_revealed = false;

        self.seen_horizontal_corridors = false;
        self.seen_vertical_corridors = false;

        self.seen_center = false;

        self.seen_top_left = false;
        self.seen_top_right = false;
        self.seen_bottom_left = false;
        self.seen_bottom_right = false;

        self.seen_progress = 0.0;
    }

    pub fn set_corridors(&mut self, left: bool, right: bool, top: bool, bottom: bool) {
        self.left_corridor.set_position(Vector3::new(
            if left {
                -HAS_CORRIDOR_HORIZONTAL
            } else {
                -NO_CORRIDOR_HORIZONTAL
            },
            0.0,
            if left {
                HAS_CORRIDOR_VERTICAL
            } else {
                NO_CORRIDOR_VERTICAL
            },
        ));
        self.right_corridor.set_position(Vector3::new(
            if right {
                HAS_CORRIDOR_HORIZONTAL
            } else {
                NO_CORRIDOR_HORIZONTAL
            },
            0.0,
            if right {
                HAS_CORRIDOR_VERTICAL
            } else {
                NO_CORRIDOR_VERTICAL
            },
        ));
        self.top_corridor.set_position(Vector3::new(
            0.0,
            if top {
                -HAS_CORRIDOR_HORIZONTAL
            } else {
                -NO_CORRIDOR_HORIZONTAL
            },
            if top {
                HAS_CORRIDOR_VERTICAL
            } else {
                NO_CORRIDOR_VERTICAL
            },
        ));
        self.bottom_corridor.set_position(Vector3::new(
            0.0,
            if bottom {
                HAS_CORRIDOR_HORIZONTAL
            } else {
                NO_CORRIDOR_HORIZONTAL
            },
            if bottom {
                HAS_CORRIDOR_VERTICAL
            } else {
                NO_CORRIDOR_VERTICAL
            },
        ));
    }

    pub fn set_current_game(&mut self, current_game: SaveGame) {
        self.current_game = current_game;
    }

    pub fn update_fog(&mut self) {
        self.center_fog.set_instance_shader_parameter(
            "seen",
            &Variant::from(if self.seen_center { 1.0 } else { 0.0 }),
        );

        self.left_corridor.set_instance_shader_parameter(
            "seen",
            &Variant::from(if self.seen_horizontal_corridors {
                1.0
            } else {
                0.0
            }),
        );
        self.right_corridor.set_instance_shader_parameter(
            "seen",
            &Variant::from(if self.seen_horizontal_corridors {
                1.0
            } else {
                0.0
            }),
        );
        self.top_corridor.set_instance_shader_parameter(
            "seen",
            &Variant::from(if self.seen_vertical_corridors {
                1.0
            } else {
                0.0
            }),
        );
        self.bottom_corridor.set_instance_shader_parameter(
            "seen",
            &Variant::from(if self.seen_vertical_corridors {
                1.0
            } else {
                0.0
            }),
        );

        self.top_left_fog.set_instance_shader_parameter(
            "seen",
            &Variant::from(if self.seen_top_left { 1.0 } else { 0.0 }),
        );
        self.top_right_fog.set_instance_shader_parameter(
            "seen",
            &Variant::from(if self.seen_top_right { 1.0 } else { 0.0 }),
        );
        self.bottom_left_fog.set_instance_shader_parameter(
            "seen",
            &Variant::from(if self.seen_bottom_left { 1.0 } else { 0.0 }),
        );
        self.bottom_right_fog.set_instance_shader_parameter(
            "seen",
            &Variant::from(if self.seen_bottom_right { 1.0 } else { 0.0 }),
        );
    }
}

#[godot_api]
impl INode3D for Room {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            player: None,

            center_fog: OnEditor::default(),

            top_left_fog: OnEditor::default(),
            top_right_fog: OnEditor::default(),
            bottom_left_fog: OnEditor::default(),
            bottom_right_fog: OnEditor::default(),

            left_corridor: OnEditor::default(),
            right_corridor: OnEditor::default(),
            top_corridor: OnEditor::default(),
            bottom_corridor: OnEditor::default(),

            room_completely_revealed: false,

            seen_horizontal_corridors: false,
            seen_vertical_corridors: false,

            seen_center: false,

            seen_top_left: false,
            seen_top_right: false,
            seen_bottom_left: false,
            seen_bottom_right: false,

            current_game: SaveGame::new(0),

            seen_progress: 0.0,

            base,
        }
    }

    fn ready(&mut self) {}

    fn process(&mut self, _delta: f64) {
        let room_position = self.base().get_position();
        let player_position = if let Some(ref player) = self.player {
            player.get_position()
        } else {
            room_position
        };

        let player_distance = Vector2::new(room_position.x, room_position.y)
            .distance_to(Vector2::new(player_position.x, player_position.y));

        if self.room_completely_revealed
            || player_distance
                > (self.current_game.player_properties.view_distance as f32
                    + (ROOM_GRID_BASIS / 2.0))
        {
            return;
        }

        self.update_fog();
    }

    fn physics_process(&mut self, _delta: f64) {
        let room_position = self.base().get_position();
        let player_position = if let Some(ref player) = self.player {
            player.get_position()
        } else {
            room_position
        };

        let player_distance = Vector2::new(room_position.x, room_position.y)
            .distance_to(Vector2::new(player_position.x, player_position.y));

        let half_grid_basis = ROOM_GRID_BASIS / 2.0;

        if self.room_completely_revealed
            || player_distance
                > (self.current_game.player_properties.view_distance as f32 + half_grid_basis)
        {
            return;
        }

        if player_position.y < room_position.y + 1.5 && player_position.y > room_position.y - 1.5 {
            self.seen_horizontal_corridors = true;
            self.seen_center = true;
        }

        if player_position.x < room_position.x + 1.5 && player_position.x > room_position.x - 1.5 {
            self.seen_vertical_corridors = true;
            self.seen_center = true;
        }

        if player_position.x < room_position.x - half_grid_basis
            && player_position.y > room_position.y - half_grid_basis
            && player_position.y < room_position.y + half_grid_basis
        {
            self.seen_top_right = true;
            self.seen_bottom_right = true;
        } else if player_position.x > room_position.x - half_grid_basis
            && player_position.x < room_position.x + half_grid_basis
        {
            if player_position.y < room_position.y - half_grid_basis {
                self.seen_top_left = true;
                self.seen_top_right = true;
            } else if player_position.y > room_position.y - half_grid_basis
                && player_position.y < room_position.y + half_grid_basis
            {
                self.seen_top_left = true;
                self.seen_top_right = true;
                self.seen_bottom_left = true;
                self.seen_bottom_right = true;
            } else if player_position.y > room_position.y + half_grid_basis {
                self.seen_bottom_left = true;
                self.seen_bottom_right = true;
            }
        } else if player_position.x > room_position.x + half_grid_basis
            && player_position.y > room_position.y - half_grid_basis
            && player_position.y < room_position.y + half_grid_basis
        {
            self.seen_top_left = true;
            self.seen_bottom_left = true;
        }

        if player_distance < ROOM_GRID_BASIS {
            if player_position.x < room_position.x - half_grid_basis
                && player_position.y > room_position.y - half_grid_basis
                && player_position.y < room_position.y + half_grid_basis
            {
                self.seen_vertical_corridors = true;
            } else if player_position.x > room_position.x - half_grid_basis
                && player_position.x < room_position.x + half_grid_basis
            {
                if player_position.y < room_position.y - half_grid_basis {
                    self.seen_horizontal_corridors = true;
                } else if player_position.y > room_position.y - half_grid_basis
                    && player_position.y < room_position.y + half_grid_basis
                {
                    self.seen_vertical_corridors = true;
                    self.seen_horizontal_corridors = true;
                } else if player_position.y > room_position.y + half_grid_basis {
                    self.seen_horizontal_corridors = true;
                }
            } else if player_position.x > room_position.x + half_grid_basis
                && player_position.y > room_position.y - half_grid_basis
                && player_position.y < room_position.y + half_grid_basis
            {
                self.seen_vertical_corridors = true;
            }
        }

        self.seen_progress = if self.seen_horizontal_corridors {
            0.25
        } else {
            0.0
        } + if self.seen_vertical_corridors {
            0.25
        } else {
            0.0
        } + if self.seen_center { 0.3 } else { 0.0 }
            + if self.seen_top_left { 0.05 } else { 0.0 }
            + if self.seen_top_right { 0.05 } else { 0.0 }
            + if self.seen_bottom_left { 0.05 } else { 0.0 }
            + if self.seen_bottom_right { 0.05 } else { 0.0 };

        if self.seen_horizontal_corridors
            && self.seen_vertical_corridors
            && self.seen_center
            && self.seen_top_left
            && self.seen_top_right
            && self.seen_bottom_left
            && self.seen_bottom_right
        {
            self.room_completely_revealed = true;
            self.seen_progress = 1.0;

            self.update_fog();
        }
    }
}
