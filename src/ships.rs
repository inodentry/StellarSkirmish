use crate::components::*;
use crate::ship_parts::*;
use bevy::prelude::{Timer, TimerMode};

pub fn load_speedy_ship() -> (
    Ship,
    Enemy,
    Thruster,
    Clipping,
    Drag,
    CollisionBox,
    Health,
    Mass,
    EntityType,
    AITimer,
    AITimer2,
    SpeedyAI,
) {
    (
        Ship {
            turn_speed: f32::to_radians(1.25),
            primary_weapon: load_practice_laser(),
            secondary_weapon: load_blank_weapon(),
            tertiary_weapon: load_blank_weapon(),
        },
        Enemy {},
        load_basic_thruster(),
        Clipping {
            cd_timer: Timer::from_seconds(0.1, TimerMode::Once),
        },
        Drag {
            dampening_factor: 0.995,
        },
        CollisionBox {
            shape: Shape::Circle,
            width_radius: 38.0 * GLOBAL_RESCALE_C,
            height: 38.0 * GLOBAL_RESCALE_C,
        },
        Health { value: 10.0 },
        Mass { value: 80000.0 },
        EntityType::Ship,
        AITimer {
            cd_timer: Timer::from_seconds(2.0, TimerMode::Once),
        },
        AITimer2 {
            cd_timer: Timer::from_seconds(1.0, TimerMode::Once),
        },
        SpeedyAI {},
    )
}

pub fn load_drone_ship() -> (
    Ship,
    Enemy,
    Thruster,
    Clipping,
    Drag,
    CollisionBox,
    Health,
    Mass,
    EntityType,
    DroneAI,
) {
    (
        Ship {
            turn_speed: f32::to_radians(1.0),
            primary_weapon: load_drone_laser(),
            secondary_weapon: load_blank_weapon(),
            tertiary_weapon: load_blank_weapon(),
        },
        Enemy {},
        load_drone_thruster(),
        Clipping {
            cd_timer: Timer::from_seconds(0.1, TimerMode::Once),
        },
        Drag {
            dampening_factor: 0.995,
        },
        CollisionBox {
            shape: Shape::Circle,
            width_radius: 38.0 * GLOBAL_RESCALE_C,
            height: 38.0 * GLOBAL_RESCALE_C,
        },
        Health { value: 100.0 },
        Mass { value: 1000.0 },
        EntityType::Ship,
        DroneAI {},
    )
}

pub fn load_rammer_ship() -> (
    Ship,
    Enemy,
    Thruster,
    Clipping,
    Drag,
    CollisionBox,
    Health,
    Mass,
    EntityType,
    AITimer,
    AITimer2,
    RammerAI,
) {
    (
        Ship {
            turn_speed: f32::to_radians(1.25),
            primary_weapon: load_practice_laser(),
            secondary_weapon: load_blank_weapon(),
            tertiary_weapon: load_blank_weapon(),
        },
        Enemy {},
        load_basic_thruster(),
        Clipping {
            cd_timer: Timer::from_seconds(1.0, TimerMode::Once),
        },
        Drag {
            dampening_factor: 0.995,
        },
        CollisionBox {
            shape: Shape::Circle,
            width_radius: 38.0 * GLOBAL_RESCALE_C,
            height: 38.0 * GLOBAL_RESCALE_C,
        },
        Health { value: 200.0 },
        Mass { value: 200000.0 },
        EntityType::Ship,
        AITimer {
            cd_timer: Timer::from_seconds(0.5, TimerMode::Once),
        },
        AITimer2 {
            cd_timer: Timer::from_seconds(3.0, TimerMode::Once),
        },
        RammerAI {},
    )
}

pub fn load_lunker_ship() -> (
    Ship,
    Enemy,
    Thruster,
    Clipping,
    Drag,
    CollisionBox,
    Health,
    Mass,
    EntityType,
    AITimer,
    AITimer2,
    SpeedyAI,
) {
    (
        Ship {
            turn_speed: f32::to_radians(1.25),
            primary_weapon: load_basic_torpedo(),
            secondary_weapon: load_blank_weapon(),
            tertiary_weapon: load_blank_weapon(),
        },
        Enemy {},
        load_basic_thruster(),
        Clipping {
            cd_timer: Timer::from_seconds(0.1, TimerMode::Once),
        },
        Drag {
            dampening_factor: 0.995,
        },
        CollisionBox {
            shape: Shape::Circle,
            width_radius: 57.0 * GLOBAL_RESCALE_C,
            height: 57.0 * GLOBAL_RESCALE_C,
        },
        Health { value: 100.0 },
        Mass { value: 140000.0 },
        EntityType::Ship,
        AITimer {
            cd_timer: Timer::from_seconds(2.0, TimerMode::Once),
        },
        AITimer2 {
            cd_timer: Timer::from_seconds(1.0, TimerMode::Once),
        },
        SpeedyAI {},
    )
}

pub fn load_minelayer_ship() -> (
    Ship,
    Enemy,
    Thruster,
    Clipping,
    Drag,
    CollisionBox,
    Health,
    Mass,
    EntityType,
    AITimer,
    MineLayerAI,
) {
    (
        Ship {
            turn_speed: f32::to_radians(2.0),
            primary_weapon: load_practice_laser(),
            secondary_weapon: load_blank_weapon(),
            tertiary_weapon: load_blank_weapon(),
        },
        Enemy {},
        load_basic_thruster(),
        Clipping {
            cd_timer: Timer::from_seconds(0.1, TimerMode::Once),
        },
        Drag {
            dampening_factor: 0.995,
        },
        CollisionBox {
            shape: Shape::Circle,
            width_radius: 38.0 * GLOBAL_RESCALE_C,
            height: 38.0 * GLOBAL_RESCALE_C,
        },
        Health { value: 200.0 },
        Mass { value: 250000.0 },
        EntityType::Ship,
        AITimer {
            cd_timer: Timer::from_seconds(2.5, TimerMode::Once),
        },
        MineLayerAI {},
    )
}

pub fn load_turret_ship() -> (
    Ship,
    Enemy,
    Thruster,
    Clipping,
    Drag,
    CollisionBox,
    Health,
    Mass,
    EntityType,
    TurretAI,
) {
    (
        Ship {
            turn_speed: f32::to_radians(1.25),
            primary_weapon: load_practice_laser(),
            secondary_weapon: load_blank_weapon(),
            tertiary_weapon: load_blank_weapon(),
        },
        Enemy {},
        load_basic_thruster(),
        Clipping {
            cd_timer: Timer::from_seconds(0.1, TimerMode::Once),
        },
        Drag {
            dampening_factor: 0.995,
        },
        CollisionBox {
            shape: Shape::Circle,
            width_radius: 38.0 * GLOBAL_RESCALE_C,
            height: 38.0 * GLOBAL_RESCALE_C,
        },
        Health { value: 50.0 },
        Mass { value: 100000.0 },
        EntityType::Ship,
        TurretAI {},
    )
}

pub fn load_picket_ship() -> (
    Ship,
    Enemy,
    Thruster,
    Clipping,
    Drag,
    CollisionBox,
    Health,
    Mass,
    EntityType,
    AITimer,
    PicketAI,
) {
    (
        Ship {
            turn_speed: f32::to_radians(2.0),
            primary_weapon: load_practice_laser(),
            secondary_weapon: load_blank_weapon(),
            tertiary_weapon: load_blank_weapon(),
        },
        Enemy {},
        load_picket_thruster(),
        Clipping {
            cd_timer: Timer::from_seconds(0.1, TimerMode::Once),
        },
        Drag {
            dampening_factor: 0.995,
        },
        CollisionBox {
            shape: Shape::Circle,
            width_radius: 38.0 * GLOBAL_RESCALE_C,
            height: 38.0 * GLOBAL_RESCALE_C,
        },
        Health { value: 100.0 },
        Mass { value: 80000.0 },
        EntityType::Ship,
        AITimer {
            cd_timer: Timer::from_seconds(3.5, TimerMode::Once),
        },
        PicketAI {},
    )
}

pub fn load_boss_ship() -> (
    Ship,
    Enemy,
    Clipping,
    Drag,
    CollisionBox,
    Health,
    Mass,
    EntityType,
    AITimer,
    AITimer2,
    BossAI,
) {
    (
        Ship {
            turn_speed: f32::to_radians(1.25),
            primary_weapon: load_basic_torpedo(),
            secondary_weapon: load_blank_weapon(),
            tertiary_weapon: load_blank_weapon(),
        },
        Enemy {},
        Clipping {
            cd_timer: Timer::from_seconds(1.0, TimerMode::Once),
        },
        Drag {
            dampening_factor: 1.0,
        },
        CollisionBox {
            shape: Shape::Circle,
            width_radius: 450.0 * GLOBAL_RESCALE_C,
            height: 450.0 * GLOBAL_RESCALE_C,
        },
        Health { value: 5000.0 },
        Mass { value: 100000000.0 },
        EntityType::Ship,
        AITimer {
            cd_timer: Timer::from_seconds(1.0, TimerMode::Once),
        },
        AITimer2 {
            cd_timer: Timer::from_seconds(3.0, TimerMode::Once),
        },
        BossAI {},
    )
}
