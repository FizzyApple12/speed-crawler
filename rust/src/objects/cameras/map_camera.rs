use crate::objects::map::ROOM_GRID_BASIS;
use godot::classes::input::CursorShape;
use godot::classes::{
    Camera3D, ICamera3D, Input, InputEvent, InputEventMouseButton, InputEventMouseMotion,
};
use godot::global::MouseButton;
use godot::obj::WithBaseField;
use godot::prelude::*;

const SCROLL_SENSITIVITY: f32 = 1.2;
const MIN_SCROLL: f32 = 1.0;
const MAX_SCROLL: f32 = ROOM_GRID_BASIS * 32.0;

#[derive(GodotClass)]
#[class(base=Camera3D)]
struct MapCamera {
    position: Vector2,

    view_range: f32,

    is_holding: bool,

    base: Base<Camera3D>,
}

#[godot_api]
impl ICamera3D for MapCamera {
    fn init(base: Base<Camera3D>) -> Self {
        Self {
            position: Vector2 { x: 0.0, y: 0.0 },

            view_range: 16.0,

            is_holding: false,

            base,
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        Input::singleton()
            .set_default_cursor_shape_ex()
            .shape(CursorShape::DRAG)
            .done();

        if let Ok(mouse_event) = event.clone().try_cast::<InputEventMouseButton>() {
            match mouse_event.get_button_index() {
                MouseButton::LEFT => {
                    self.is_holding = mouse_event.is_pressed();
                }
                MouseButton::WHEEL_UP => {
                    let view_range = (self.view_range / SCROLL_SENSITIVITY).max(MIN_SCROLL);

                    self.view_range = view_range;
                    self.base_mut().set_size(view_range);
                }
                MouseButton::WHEEL_DOWN => {
                    let view_range = (self.view_range * SCROLL_SENSITIVITY).min(MAX_SCROLL);

                    self.view_range = view_range;
                    self.base_mut().set_size(view_range);
                }
                _ => {}
            }
        }

        if self.is_holding
            && let Ok(mouse_event) = event.try_cast::<InputEventMouseMotion>()
        {
            let camera_size = self.base().get_viewport().unwrap().get_visible_rect().size;

            let relative = mouse_event.get_relative();

            let position = self.position
                + Vector2::new(
                    -(relative.x / camera_size.y) * self.view_range,
                    (relative.y / camera_size.y) * self.view_range,
                );

            self.position = position;

            self.base_mut()
                .set_position(Vector3::new(position.x, position.y, 0.0));
        }
    }
}
