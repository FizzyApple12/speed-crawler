use godot::classes::{Camera3D, ICamera3D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Camera3D)]
struct FollowCamera {
    #[export]
    follow_target: Option<Gd<Node3D>>,

    base: Base<Camera3D>,
}

#[godot_api]
impl ICamera3D for FollowCamera {
    fn init(base: Base<Camera3D>) -> Self {
        Self {
            follow_target: None,
            base,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        if let Some(ref mut follow_target) = self.follow_target {
            let position = follow_target.get_position();

            self.base_mut()
                .set_position(Vector3::new(position.x, position.y, 0.0));
        }
    }
}
