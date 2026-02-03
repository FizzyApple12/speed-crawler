use crate::managers::game::game_manager::GameManager;
use crate::objects::map::floor::Floor;
use crate::types::rooms::RoomType;
use crate::types::save_game::SaveGame;
use godot::classes::{INode, Node};
use godot::prelude::*;
use rand::prelude::*;
use std::collections::HashMap;
use std::thread::JoinHandle;

pub type FloorLayout = HashMap<(i64, i64), RoomType>;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct FloorManager {
    #[export]
    game_manager: OnEditor<Gd<GameManager>>,

    floor_generation_thread: Option<JoinHandle<(FloorLayout, f64)>>,

    #[export]
    current_floor: OnEditor<Gd<Floor>>,

    pub current_floor_layout: HashMap<(i64, i64), RoomType>,
    pub estimated_completion_time: f64,

    base: Base<Node>,
}

impl FloorManager {
    pub fn setup_level(&mut self, save_game: SaveGame) {
        self.floor_generation_thread = Some(std::thread::spawn(move || {
            let mut rng = SmallRng::from_seed(save_game.get_rng_seed());

            if save_game.current_floor == 0 {
                return (
                    HashMap::from([
                        ((-1, 1), RoomType::Normal),
                        ((0, 1), RoomType::Normal),
                        ((0, 0), RoomType::Normal),
                        ((1, 0), RoomType::Normal),
                        ((2, 0), RoomType::Normal),
                        ((3, 0), RoomType::Normal),
                        ((4, 0), RoomType::Normal),
                        ((5, 0), RoomType::Normal),
                        ((5, -1), RoomType::Normal),
                    ]),
                    0.0,
                );
            }

            let mut rooms = HashMap::new();

            let mut position: (i64, i64) = (0, 0);

            let max_room_count = 20 + (save_game.current_floor.pow(2)) + rng.random_range(-10..=10);

            // fixes rng somehow????
            for _ in 0..3 {
                let _ = rng.random_range(-1..=1);
            }

            for _ in 0..max_room_count {
                rooms.insert(position, RoomType::Normal);

                let mut delta = (rng.random_range(-1..=1), rng.random_range(-1..=1));

                if delta.0 != 0 && delta.1 != 0 {
                    if rng.random_bool(0.5) {
                        delta.0 = 0;
                    } else {
                        delta.1 = 0;
                    }
                }

                position = (position.0 + delta.0, position.1 + delta.1);
            }

            let estimated_time = rooms.len() as f64
                * ((1.0 / ((save_game.current_floor.max(0) as f64 / 2.0) + 1.0)) * 2.0 + 1.0);

            (rooms, estimated_time)
        }));
    }

    pub fn get_completion_progress(&self) -> f64 {
        self.current_floor.bind().get_completion_progress()
    }
}

#[godot_api]
impl INode for FloorManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            game_manager: OnEditor::default(),

            floor_generation_thread: None,

            current_floor: OnEditor::default(),

            current_floor_layout: HashMap::new(),
            estimated_completion_time: 0.0,

            base,
        }
    }

    fn process(&mut self, _delta: f64) {
        if let Some(ref floor_generation_callback) = self.floor_generation_thread
            && floor_generation_callback.is_finished()
        {
            let result = self.floor_generation_thread.take();

            match result.unwrap().join() {
                Ok((floor_layout, estimated_completion_time)) => {
                    godot_print!("Success generating floor");

                    self.current_floor_layout = floor_layout;
                    self.estimated_completion_time = estimated_completion_time;

                    self.current_floor
                        .bind_mut()
                        .load_floor(self.current_floor_layout.clone());

                    self.game_manager.bind_mut().level_setup_complete(
                        self.current_floor_layout.clone(),
                        self.estimated_completion_time,
                    );
                }
                Err(err) => {
                    godot_error!("Error generating floor: {err:?}");

                    self.game_manager.bind_mut().level_setup_failed();
                }
            }
        }
    }
}
