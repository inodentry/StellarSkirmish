use bevy::prelude::*;

pub trait Weapon<T> {
    fn fire(&mut self, vec_dir: Vec3, origin_speed: f32) -> T;
}
