use bevy::prelude::*;

pub trait Weapon<T> {
    fn fire(&self, vel_dir: Vec3, origin_speed: f32) -> T;
}
