use bevy::prelude::*;
use bevy::color::palettes::basic;

use bevy_rapier2d::prelude::*;

pub fn raycast(
    mut commands: Commands,
    windows: Query<&Window, /* With<PrimaryWindow> */>,
    rapier_context: ReadRapierContext,
    cameras: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
) {
    let window = windows.single();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // We will color in read the colliders hovered by the mouse.
    for (camera, camera_transform) in &cameras {
        // First, compute a ray from the mouse position.
        let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };
        let context = rapier_context.single();
        // Then cast the ray.
        let hit = context.cast_ray(
            ray.origin.xy(),
            ray.direction.xy(),
            f32::MAX,
            true,
            QueryFilter::default(),
        );

        if let Some((entity, _toi)) = hit {
            // Color in blue the entity we just hit.
            // Because of the query filter, only colliders attached to a dynamic body
            // will get an event.
            let color = basic::BLUE.into();
            commands.entity(entity).insert(ColliderDebugColor(color));
        }
    }
}

// Example plugin that shows how to run raycasts from the mouse location
// It converts the cursor_position from and the camera into world coordinates
// and casts a ray. Entities collided with have their rapier debug color changed to blue
pub struct MouseHoverPlugin;

impl Plugin for MouseHoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, raycast);
    }
}
