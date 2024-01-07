use crate::components::*;
use crate::ship_parts::*;

pub fn load_grunt_ship() -> Ship {
    Ship {
        angle: 0.0,
        turn_speed: 100.0,
        primary_weapon: load_basic_torpedo(),
        secondary_weapon: load_blank_weapon(),
        tertiary_weapon: load_blank_weapon(),
    }
}
