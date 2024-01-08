use crate::components::*;
use crate::ship_parts::*;

pub fn load_grunt_ship() -> Ship {
    Ship {
        turn_speed: f32::to_radians(1.25),
        primary_weapon: load_basic_torpedo(),
        secondary_weapon: load_blank_weapon(),
        tertiary_weapon: load_blank_weapon(),
    }
}

pub fn load_practice_ship() -> Ship {
    Ship {
        turn_speed: f32::to_radians(1.25),
        primary_weapon: load_practice_laser(),
        secondary_weapon: load_blank_weapon(),
        tertiary_weapon: load_blank_weapon(),
    }
}
