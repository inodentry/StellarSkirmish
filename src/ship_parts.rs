use crate::components::*;
use bevy::prelude::*;

/// This file contains non-system functions to load presets for various complete weapon systems,
/// such as loading specific settings for a type of missile. These systems server as baselines
/// that the player may later customize.

//-------------
//-- Weapons --
//-------------
pub fn load_basic_torpedo() -> WeaponSystem {
    WeaponSystem {
        name: "Basic Torpedo".to_string(),
        value: 10.0,
        proj_speed: 1000.0,
        proj_fuel: 300.0,
        proj_type: ProjectileType::Torpedo,
        proj_mass: 1.0,
        dmg_type: DamageType::Kinetic,
        sprite_path: "sprites/projectiles/spaceMissiles_001.png".to_string(),
        cooldown: 0.5,
        cd_timer: Timer::from_seconds(0.5, TimerMode::Once),
    }
}

pub fn load_basic_laser() -> WeaponSystem {
    WeaponSystem {
        name: "Basic Laser".to_string(),
        value: 10.0,
        proj_speed: 1200.0,
        proj_fuel: 400.0,
        proj_type: ProjectileType::Laser,
        proj_mass: 0.0,
        dmg_type: DamageType::Radiant,
        sprite_path: "sprites/projectiles/laserBlue04.png".to_string(),
        cooldown: 0.2,
        cd_timer: Timer::from_seconds(0.2, TimerMode::Once),
    }
}

pub fn load_basic_cannon() -> WeaponSystem {
    WeaponSystem {
        name: "Basic Turret".to_string(),
        value: 10.0,
        proj_speed: 1600.0,
        proj_fuel: 300.0,
        proj_type: ProjectileType::Shells,
        dmg_type: DamageType::Kinetic,
        proj_mass: 1.0,
        sprite_path: "sprites/projectiles/laserGreen14.png".to_string(),
        cooldown: 0.1,
        cd_timer: Timer::from_seconds(0.1, TimerMode::Once),
    }
}

pub fn load_blank_weapon() -> WeaponSystem {
    WeaponSystem {
        name: "None".to_string(),
        value: 0.0,
        proj_speed: 0.0,
        proj_fuel: 0.0,
        proj_type: ProjectileType::Shells,
        dmg_type: DamageType::Kinetic,
        proj_mass: 0.0,
        // Valid path supplied temporarily
        sprite_path: "sprites/projectiles/laserGreen14.png".to_string(),
        cooldown: 0.0,
        cd_timer: Timer::from_seconds(0.0, TimerMode::Once),
    }
}

pub fn load_test_torpedo() -> WeaponSystem {
    WeaponSystem {
        name: "Test Torpedo".to_string(),
        value: 0.0,
        proj_speed: 600.0,
        proj_fuel: 300.0,
        proj_type: ProjectileType::Torpedo,
        proj_mass: 1.0,
        dmg_type: DamageType::Kinetic,
        sprite_path: "sprites/projectiles/spaceMissiles_020.png".to_string(),
        cooldown: 0.5,
        cd_timer: Timer::from_seconds(0.5, TimerMode::Once),
    }
}

//---------------
//-- Thrusters --
//---------------
pub fn load_basic_thruster() -> Thruster {
    Thruster {
        name: "Basic Thruster".to_string(),
        value: 10.0,
        force: 100.0,
    }
}
