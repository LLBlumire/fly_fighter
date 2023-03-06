use bevy::{prelude::*, window::WindowResized};
use crate::prelude::Viewport;

pub fn adjust_screen_resize(
    mut resized: EventReader<WindowResized>,
    mut camera: Query<(&mut OrthographicProjection, &Viewport)>,
) {
    if let Some(resized) = resized.iter().last() {
        for (mut projection, viewport) in camera.iter_mut() {
            let width_scale = viewport.width / resized.width;
            let height_scale = viewport.height / resized.height;
            projection.scale = f32::max(width_scale, height_scale);
        }
    }
}
