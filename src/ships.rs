use crate::components::*;
use crate::ship_parts::*;

pub fn load_grunt_ship() -> Ship {
    Ship {
        turn_speed: 100.0,
        primary_weapon: load_basic_torpedo(),
        secondary_weapon: load_blank_weapon(),
        tertiary_weapon: load_blank_weapon(),
    }
}
