use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerProperties {
    // pub has_move_buffer: bool,
    // pub momentum_redirector: Option<MomentumRedirectorProperties>,
    // pub extra_eye: Option<ExtraEyeProperties>,
    pub warmup_time: f64,

    pub max_speed: f64,
    pub active_acceleration: f64,

    pub view_distance: f64,

    pub stopping_mass: f64,
}

impl Default for PlayerProperties {
    fn default() -> Self {
        Self {
            // has_move_buffer: false,
            // momentum_redirector: None,
            // extra_eye: None,
            warmup_time: 5.0,

            max_speed: 24.0,
            active_acceleration: 12.0,

            view_distance: 10.0,

            stopping_mass: 128.0,
        }
    }
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct MomentumRedirectorProperties {
//     cooldown: f64,
// }

// impl Default for MomentumRedirectorProperties {
//     fn default() -> Self {
//         MomentumRedirectorProperties { cooldown: 10.0 }
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct ExtraEyeProperties {
//     count: i64,

//     initial_speed: f64,
// }

// impl Default for ExtraEyeProperties {
//     fn default() -> Self {
//         ExtraEyeProperties {
//             count: 1,

//             initial_speed: 16.0,
//         }
//     }
// }
