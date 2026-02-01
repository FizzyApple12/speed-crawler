use rand::Rng;

use crate::types::save_game::SaveGame;

struct UpgradeTable;

// source of truth
impl UpgradeTable {
    fn add_warmup(mut save: SaveGame, time: f64) -> SaveGame {
        save.player_properties.warmup_time += time;

        save
    }

    fn multiply_speed(mut save: SaveGame, factor: f64) -> SaveGame {
        save.player_properties.max_speed *= factor;

        save
    }
    fn add_speed(mut save: SaveGame, speed: f64) -> SaveGame {
        save.player_properties.max_speed += speed;

        save
    }

    fn multiply_acceleration(mut save: SaveGame, factor: f64) -> SaveGame {
        save.player_properties.active_acceleration *= factor;

        save
    }
    fn add_acceleration(mut save: SaveGame, acceleration: f64) -> SaveGame {
        save.player_properties.active_acceleration += acceleration;

        save
    }

    fn add_view_distance(mut save: SaveGame, distance: f64) -> SaveGame {
        save.player_properties.view_distance *= distance;

        save
    }

    fn divide_mass(mut save: SaveGame, divisor: f64) -> SaveGame {
        save.player_properties.stopping_mass /= divisor;

        save
    }
}

// use proc macro to generate

#[derive(Clone, Debug)]
pub enum UpgradeType {
    AddWarmup(f64),

    MultiplySpeed(f64),
    AddSpeed(f64),

    MultiplyAcceleration(f64),
    AddAcceleration(f64),

    AddViewDistance(f64),

    DivideMass(f64),
}

impl UpgradeType {
    pub fn apply_upgrade(self, save: SaveGame) -> SaveGame {
        match self {
            UpgradeType::AddWarmup(time) => UpgradeTable::add_warmup(save, time),

            UpgradeType::MultiplySpeed(factor) => UpgradeTable::multiply_speed(save, factor),
            UpgradeType::AddSpeed(speed) => UpgradeTable::add_speed(save, speed),

            UpgradeType::MultiplyAcceleration(factor) => {
                UpgradeTable::multiply_acceleration(save, factor)
            }
            UpgradeType::AddAcceleration(acceleration) => {
                UpgradeTable::add_acceleration(save, acceleration)
            }

            UpgradeType::AddViewDistance(distance) => {
                UpgradeTable::add_view_distance(save, distance)
            }

            UpgradeType::DivideMass(divisor) => UpgradeTable::divide_mass(save, divisor),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            UpgradeType::AddWarmup(_time) => "Temporal Rift",

            UpgradeType::MultiplySpeed(_factor) => "Rocket Boots",
            UpgradeType::AddSpeed(_speed) => "Cheetah Soul",

            UpgradeType::MultiplyAcceleration(_factor) => "Steroids",
            UpgradeType::AddAcceleration(_acceleration) => "Leg Workout",

            UpgradeType::AddViewDistance(_distance) => "Enhanced Eyes",

            UpgradeType::DivideMass(_divisor) => "Gym Membership",
        }
        .to_owned()
    }

    pub fn get_description(&self) -> String {
        match self {
            UpgradeType::AddWarmup(time) => format!("preview time +{time:.1}s"),

            UpgradeType::MultiplySpeed(factor) => format!("top speed x{factor:.1}"),
            UpgradeType::AddSpeed(speed) => format!("top speed +{speed:.1}m/s"),

            UpgradeType::MultiplyAcceleration(factor) => format!("acceleration x{factor:.1}"),
            UpgradeType::AddAcceleration(acceleration) => {
                format!("acceleration +{acceleration:.1}m/s^2")
            }

            UpgradeType::AddViewDistance(distance) => format!("view distance +{distance:.1}m"),

            UpgradeType::DivideMass(divisor) => format!("mass /{divisor:.1}"),
        }
    }

    pub fn get_price(&self) -> i64 {
        match self {
            UpgradeType::AddWarmup(time) => 2 + (time.floor() as i64),

            UpgradeType::MultiplySpeed(factor) => 4 + (factor.floor() as i64),
            UpgradeType::AddSpeed(speed) => 2 + (speed.floor() as i64),

            UpgradeType::MultiplyAcceleration(factor) => 4 + (factor.floor() as i64),
            UpgradeType::AddAcceleration(acceleration) => 2 + (acceleration.floor() as i64),

            UpgradeType::AddViewDistance(distance) => 4 + (distance.floor() as i64),

            UpgradeType::DivideMass(divisor) => 6 + (divisor.floor() as i64),
        }
    }

    pub fn get_probability(&self) -> f64 {
        match self {
            UpgradeType::AddWarmup(time) => 0.2 / time,

            UpgradeType::MultiplySpeed(factor) => 0.02 / factor,
            UpgradeType::AddSpeed(speed) => 0.1 / speed,

            UpgradeType::MultiplyAcceleration(factor) => 0.01 / factor,
            UpgradeType::AddAcceleration(acceleration) => 0.05 / acceleration,

            UpgradeType::AddViewDistance(distance) => 0.1 / distance,

            UpgradeType::DivideMass(divisor) => 0.1 / divisor,
        }
    }

    pub fn generate_random(random: &mut impl Rng) -> Self {
        let upgrade_number = random.random_range(0..=6);

        match upgrade_number {
            0 => UpgradeType::AddWarmup(random.random_range(1.0..=5.0)),

            1 => UpgradeType::MultiplySpeed(random.random_range(1.2..=3.0)),
            2 => UpgradeType::AddSpeed(random.random_range(1.0..=5.0)),

            3 => UpgradeType::MultiplyAcceleration(random.random_range(1.2..=3.0)),
            4 => UpgradeType::AddAcceleration(random.random_range(1.0..=5.0)),

            5 => UpgradeType::AddViewDistance(random.random_range(2.0..=10.0)),

            6 => UpgradeType::DivideMass(random.random_range(1.2..=2.5)),

            x => panic!("impossible upgrade {}", x),
        }
    }
}
