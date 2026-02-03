use crate::managers::game::floor_manager::FloorLayout;
use crate::objects::map::room::Room;
use crate::objects::player::Player;
use crate::types::rooms::RoomType;
use godot::classes::{INode3D, Node3D};
use godot::prelude::*;
use std::collections::HashMap;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Floor {
    #[export]
    player: OnEditor<Gd<Player>>,

    floor_objects: HashMap<(i64, i64), Gd<Room>>,

    normal_room_scene: Gd<PackedScene>,

    base: Base<Node3D>,
}

impl Floor {
    pub fn load_floor(&mut self, layout: FloorLayout) {
        for (_, node) in self.floor_objects.drain() {
            node.free();
        }

        for (position, room_type) in layout.iter() {
            match room_type {
                RoomType::Normal => {
                    let mut room = self.normal_room_scene.instantiate_as::<Room>();

                    room.bind_mut().player = Some(self.player.clone());

                    self.base_mut().add_child(&room);

                    room.bind_mut().place(*position);

                    let has_room_left = layout.contains_key(&(position.0 - 1, position.1));
                    let has_room_right = layout.contains_key(&(position.0 + 1, position.1));
                    let has_room_top = layout.contains_key(&(position.0, position.1 - 1));
                    let has_room_bottom = layout.contains_key(&(position.0, position.1 + 1));
                    room.bind_mut().set_corridors(
                        has_room_left,
                        has_room_right,
                        has_room_top,
                        has_room_bottom,
                    );

                    room.bind_mut().reset();

                    self.floor_objects.insert(*position, room);
                }
            }
        }
    }

    pub fn get_completion_progress(&self) -> f64 {
        let mut progress_accumulator = 0.0;
        let mut number_objects = 0.0;

        for (_, room) in self.floor_objects.iter() {
            progress_accumulator += room.bind().seen_progress;
            number_objects += 1.0;
        }

        progress_accumulator / number_objects
    }
}

#[godot_api]
impl INode3D for Floor {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            player: OnEditor::default(),

            floor_objects: HashMap::new(),

            normal_room_scene: load::<PackedScene>("res://objects/room/room.tscn"),

            base,
        }
    }
}
