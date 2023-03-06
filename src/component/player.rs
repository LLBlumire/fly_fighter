use bevy::prelude::*;

/// Player is attached to any entity with alliegence to a particular side (fighters, bullets).
#[derive(Component, Clone, PartialEq)]
pub enum Player {
    /// The red player.
    Red,
    /// The green player.
    Green,
}

impl Player {
    /// Get the color for materials with this player color.
    pub fn color(&self) -> Color {
        match self {
            Player::Red => Color::RED,
            Player::Green => Color::GREEN,
        }
    }
    /// Get the bullet firing controls for the player.
    pub fn fire_ctrl(&self) -> KeyCode {
        match self {
            Player::Red => KeyCode::W,
            Player::Green => KeyCode::Up,
        }
    }
    /// Get the fighters start position and rotation.
    pub fn starts_at(&self) -> (Vec3, f32) {
        match self {
            Player::Red => (Vec3::new(-200.0, -150.0, 0.0), f32::to_radians(270.0)),
            Player::Green => (Vec3::new(200.0, 150.0, 0.0), f32::to_radians(90.0)),
        }
    }
}

