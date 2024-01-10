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
        proj_speed: 300.0, // m/s
        proj_fuel: 300.0,
        proj_type: ProjectileType::Torpedo,
        proj_mass: 2.2, // kg
        dmg_type: DamageType::Kinetic,
        dmg: 0.0,
        sprite_path: "sprites/projectiles/spaceMissiles_001.png".to_string(),
        on_spawn_sprite_path: "".to_string(),
        sound_path: "".to_string(),
        cooldown: 0.5,
        cd_timer: Timer::from_seconds(1.0, TimerMode::Once),
    }
}

pub fn load_lunker_torpedo() -> WeaponSystem {
    WeaponSystem {
        name: "Basic Torpedo".to_string(),
        value: 5.0,
        proj_speed: 250.0, // m/s
        proj_fuel: 300.0,
        proj_type: ProjectileType::Torpedo,
        proj_mass: 5.0, // kg
        dmg_type: DamageType::Kinetic,
        dmg: 0.0,
        sprite_path: "sprites/projectiles/spaceMissiles_001.png".to_string(),
        on_spawn_sprite_path: "".to_string(),
        sound_path: "".to_string(),
        cooldown: 0.5,
        cd_timer: Timer::from_seconds(2.5, TimerMode::Once),
    }
}

pub fn load_basic_laser() -> WeaponSystem {
    WeaponSystem {
        name: "Basic Laser".to_string(),
        value: 10.0,
        proj_speed: 400.0, // m/s
        proj_fuel: 400.0,
        proj_type: ProjectileType::Laser,
        proj_mass: 0.0,
        dmg_type: DamageType::Radiant,
        dmg: 50.0,
        sprite_path: "sprites/projectiles/laserBlue04.png".to_string(),
        on_spawn_sprite_path: "".to_string(),
        sound_path: "sounds/laserSmall_002.ogg".to_string(),
        cooldown: 0.25,
        cd_timer: Timer::from_seconds(0.2, TimerMode::Once),
    }
}

pub fn load_basic_cannon() -> WeaponSystem {
    WeaponSystem {
        name: "Basic Turret".to_string(),
        value: 10.0,
        proj_speed: 1000.0, // m/s
        proj_fuel: 300.0,
        proj_type: ProjectileType::Shells,
        dmg_type: DamageType::Kinetic,
        dmg: 0.5,
        proj_mass: 0.040, // kg
        sprite_path: "".to_string(),
        on_spawn_sprite_path: "sprites/effects/fire07.png".to_string(),
        sound_path: "sounds/light_shells.ogg".to_string(),
        cooldown: 0.15,
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
        dmg: 0.0,
        proj_mass: 0.0,
        sprite_path: "".to_string(),
        on_spawn_sprite_path: "".to_string(),
        sound_path: "".to_string(),
        cooldown: 0.0,
        cd_timer: Timer::from_seconds(0.0, TimerMode::Once),
    }
}

pub fn load_test_torpedo() -> WeaponSystem {
    WeaponSystem {
        name: "Test Torpedo".to_string(),
        value: 0.0,
        proj_speed: 300.0, // m/s
        proj_fuel: 300.0,
        proj_type: ProjectileType::Torpedo,
        proj_mass: 5.0, // kg
        dmg_type: DamageType::Kinetic,
        dmg: 0.0,
        sprite_path: "sprites/projectiles/spaceMissiles_020.png".to_string(),
        on_spawn_sprite_path: "".to_string(),
        sound_path: "".to_string(),
        cooldown: 0.5,
        cd_timer: Timer::from_seconds(0.5, TimerMode::Once),
    }
}

pub fn load_practice_laser() -> WeaponSystem {
    WeaponSystem {
        name: "Practice Laser".to_string(),
        value: 0.0,
        proj_speed: 400.0,
        proj_fuel: 300.0,
        proj_type: ProjectileType::Laser,
        proj_mass: 0.0,
        dmg_type: DamageType::Radiant,
        dmg: 0.0,
        sprite_path: "sprites/projectiles/laserRed09.png".to_string(),
        on_spawn_sprite_path: "".to_string(),
        sound_path: "".to_string(),
        cooldown: 2.0,
        cd_timer: Timer::from_seconds(0.5, TimerMode::Once),
    }
}

pub fn load_drone_laser() -> WeaponSystem {
    WeaponSystem {
        name: "Drone Laser".to_string(),
        value: 10.0,
        proj_speed: 300.0,
        proj_fuel: 100.0,
        proj_type: ProjectileType::Laser,
        proj_mass: 0.0,
        dmg_type: DamageType::Radiant,
        dmg: 1.0,
        sprite_path: "sprites/projectiles/drone_laser.png".to_string(),
        on_spawn_sprite_path: "".to_string(),
        sound_path: "".to_string(),
        cooldown: 1.0,
        cd_timer: Timer::from_seconds(0.75, TimerMode::Once),
    }
}

//---------------
//-- Thrusters --
//---------------
pub fn load_basic_thruster() -> Thruster {
    Thruster {
        name: "Basic Thruster".to_string(),
        value: 10.0,
        force: 100000.0,
    }
}

pub fn load_drone_thruster() -> Thruster {
    Thruster {
        name: "Drone Thruster".to_string(),
        value: 10.0,
        force: 800.0,
    }
}

pub fn load_missile_thruster() -> Thruster {
    Thruster {
        name: "Missile Thruster".to_string(),
        value: 5.0,
        force: 100.0,
    }
}
