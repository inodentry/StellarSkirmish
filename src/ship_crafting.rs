use crate::components::*;
use bevy::prelude::*;

/// This file contains non-system functions to load presets for various complete weapon systems,
/// such as loading specific settings for a type of missile. These systems server as baselines
/// that the player may later customize.

pub fn load_basic_rocket() -> PrimaryWeaponSystem {
    PrimaryWeaponSystem {
        speed: 1000.0,
        fuel: 300.0,
        proj_type: ProjectileType::Rocket,
        dmg_type: DamageType::Kinetic,
        sprite_path: "sprites/projectiles/spaceMissiles_001.png".to_string(),
        cooldown: 0.5,
        cd_timer: Timer::from_seconds(0.5, TimerMode::Once),
    }
}

pub fn load_basic_laser() -> SecondaryWeaponSystem {
    SecondaryWeaponSystem {
        speed: 1200.0,
        fuel: 400.0,
        proj_type: ProjectileType::Laser,
        dmg_type: DamageType::Radiant,
        sprite_path: "sprites/projectiles/laserBlue04.png".to_string(),
        cooldown: 0.2,
        cd_timer: Timer::from_seconds(0.2, TimerMode::Once),
    }
}

pub fn load_test_missile() -> PrimaryWeaponSystem {
    PrimaryWeaponSystem {
        speed: 600.0,
        fuel: 300.0,
        proj_type: ProjectileType::Missile,
        dmg_type: DamageType::Kinetic,
        sprite_path: "sprites/projectiles/spaceMissiles_020.png".to_string(),
        cooldown: 0.5,
        cd_timer: Timer::from_seconds(0.5, TimerMode::Once),
    }
}
