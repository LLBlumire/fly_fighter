use crate::prelude::*;
use bevy::prelude::*;

/// The air brake system handles slowing down player fighters when they break.
pub fn air_brake(
    keys: Res<Input<KeyCode>>,
    mut breakers: Query<(With<Fighter>, &Player, &mut Kinematic)>,
) {
    for (_fighter, player, mut kinematic) in breakers.iter_mut() {
        let break_keys: &[KeyCode] = match player {
            Player::Red => &[KeyCode::S],
            Player::Green => &[KeyCode::J, KeyCode::Down],
        };
        if keys.any_just_pressed(break_keys.iter().copied()) {
            kinematic.velocity = crate::FIGHTER_BREAK_SPEED;
        }
        if keys.any_just_released(break_keys.iter().copied()) {
            kinematic.velocity = crate::FIGHTER_BASE_SPEED;
        }
    }
}
