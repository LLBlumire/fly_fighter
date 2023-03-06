use bevy::{prelude::*, window::close_on_esc};

/// The base speed of the fighters in pixels per second.
const FIGHTER_BASE_SPEED: f32 = 150.0;

/// The breaking speed of the fighters in pixels per second.
const FIGHTER_BREAK_SPEED: f32 = 45.0;

/// The speed bullets move at in pixels per second.
const BULLET_SPEED: f32 = 450.0;

/// The minimum time between bullets from a fighter in seconds.
const BULLET_CD: f32 = 1.0;

/// The initial screen width and target viewport
const SCREEN_WIDTH: f32 = 800.0;

/// The initial screen height and target viewport
const SCREEN_HEIGHT: f32 = 600.0;

mod component;
mod entity;
mod system;
pub mod util;

pub mod prelude {
    pub use crate::{
        component::{Bullet, Fighter, Gun, Kinematic, Player, Viewport},
        entity::{spawn_bullet, spawn_fighter},
        system::{air_brake, fighter_death, fix_viewport, kinematics, shoot, yaw},
    };
}

use prelude::*;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Fly Fighter".to_string(),
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(close_on_esc)
        .add_system(kinematics)
        .add_system(air_brake)
        .add_system(yaw)
        .add_system(shoot)
        .add_system(fighter_death)
        .add_system(fix_viewport)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default()).insert(Viewport {
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
    });
    spawn_fighter(Player::Red, &mut commands, &mut meshes, &mut materials);
    spawn_fighter(Player::Green, &mut commands, &mut meshes, &mut materials);
}
