use bevy::prelude::*;
use crate::prelude::*;

pub fn fighter_death(
    mut commands: Commands,
    bullets: Query<(Entity, &Bullet, &Transform, &Player)>,
    fighters: Query<(Entity, &Fighter, &Transform, &Player)>,
) {
    for (bullet_entity, _bullet, bullet_transform, bullet_player) in bullets.iter() {
        for (fighter_entity, _fighter, fighter_transform, fighter_player) in fighters.iter() {
            if fighter_player != bullet_player {
                let distance = bullet_transform
                    .translation
                    .distance(fighter_transform.translation);
                if distance < 15.0 {
                    commands.entity(fighter_entity).despawn();
                    commands.entity(bullet_entity).despawn();
                }
            }
        }
    }
}
