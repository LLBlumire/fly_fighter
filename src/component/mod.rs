//! This modules stores all components which power the game.

use bevy::prelude::*;

mod player;

pub use player::Player;

/// Kinematic is attached to entities which move.
#[derive(Component)]
pub struct Kinematic {
    /// The speed an entity should move, in the direction indicated by its transform.
    pub velocity: f32,
}

/// Fighter is attached to each of the two player fighter entities.
#[derive(Component)]
pub struct Fighter;

/// Bullets are attached to each bullet entity.
#[derive(Component)]
pub struct Bullet;

/// Gun is attached to the fighter entities.
#[derive(Component)]
pub struct Gun {
    /// How many seconds until the next bullet can be fired.
    pub cooldown: f32,
}

#[derive(Debug, Component)]
pub struct Viewport {
    pub width: f32,
    pub height: f32
}
