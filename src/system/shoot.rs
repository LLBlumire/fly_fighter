use crate::prelude::*;
use bevy::prelude::*;

pub fn shoot(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    mut shooters: Query<(&Fighter, &mut Gun, &Player, &Transform)>,
) {
    for (_fighter, mut gun, player, transform) in shooters.iter_mut() {
        let shoot_keys: &[KeyCode] = match player {
            Player::Red => &[KeyCode::W],
            Player::Green => &[KeyCode::Up, KeyCode::I],
        };
        if gun.cooldown >= 0.0 {
            gun.cooldown -= time.delta_seconds();
        }
        if gun.cooldown < 0.0 && keys.any_pressed(shoot_keys.iter().copied()) {
            spawn_bullet(
                &mut commands,
                &mut meshes,
                &mut materials,
                transform.clone(),
                player.clone(),
            );
            gun.cooldown = crate::BULLET_CD;
        }
    }
}
