use crate::{
    input::InputManager,
    modding::{
        PostModLoad, PreModLoad,
        registry::{Id, IdInterner},
    },
};
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostModLoad, init_camera_input_ids)
            .add_systems(Startup, setup)
            .add_systems(Update, camera_control);
    }
}

#[derive(Debug, Resource)]
struct CameraInputIds {
    move_x: Id,
    move_y: Id,
    speed: Id,
    zoom: Id,
    pan: Id,
    mouse_x: Id,
    mouse_y: Id,
}

fn init_camera_input_ids(mut commands: Commands, mut interner: ResMut<IdInterner>) {
    let ids = CameraInputIds {
        move_x: interner.intern("base::input::x").unwrap(),
        move_y: interner.intern("base::input::y").unwrap(),
        speed: interner.intern("base::input::speed").unwrap(),
        zoom: interner.intern("base::input::zoom").unwrap(),
        pan: interner.intern("base::input::pan").unwrap(),
        mouse_x: interner.intern("base::input::mouse_x").unwrap(),
        mouse_y: interner.intern("base::input::mouse_y").unwrap(),
    };
    commands.insert_resource(ids);
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::FixedVertical {
                viewport_height: 256.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn camera_control(
    mut query: Query<(&Camera, &GlobalTransform, &mut Transform, &mut Projection)>,
    input: Res<InputManager>,
    ids: Res<CameraInputIds>,
    time: Res<Time>,
    mut last_pos: Local<Option<Vec2>>,
) {
    let (camera, global, mut transform, projection) = query.single_mut().unwrap();

    let Projection::Orthographic(ortho) = projection.into_inner() else {
        return;
    };

    key_pan(&input, &ids, &mut transform, &ortho, &time);
    cursor_zoom(&input, &ids, camera, global, &mut transform, ortho);
    drag_pan(&input, &ids, camera, global, &mut transform, &mut last_pos);
}

fn key_pan(
    input: &InputManager,
    ids: &CameraInputIds,
    transform: &mut Transform,
    ortho: &OrthographicProjection,
    time: &Time,
) {
    let dir = input.vec2_or_default(ids.move_x, ids.move_y);

    if dir == Vec2::ZERO {
        return;
    }

    let mut speed = 128.0;
    if input.bool_or_default(ids.speed) {
        speed *= 4.0;
    }

    transform.translation +=
        dir.normalize_or_zero().extend(0.0) * speed * ortho.scale * time.delta_secs();
}

fn cursor_zoom(
    input: &InputManager,
    ids: &CameraInputIds,
    camera: &Camera,
    global: &GlobalTransform,
    transform: &mut Transform,
    ortho: &mut OrthographicProjection,
) {
    let zoom = input.axis_or_default(ids.zoom);
    if zoom == 0.0 {
        return;
    }

    let cursor = input.vec2_or_default(ids.mouse_x, ids.mouse_y);

    let Ok(world_before) = camera.viewport_to_world_2d(global, cursor) else {
        return;
    };

    // exponential zoom (smooth + symmetric)
    let zoom_factor = (1.0 - zoom * 0.15).clamp(0.1, 10.0);
    ortho.scale *= zoom_factor;

    let Ok(world_after) = camera.viewport_to_world_2d(global, cursor) else {
        return;
    };

    transform.translation += (world_before - world_after).extend(0.0);
}

fn drag_pan(
    input: &InputManager,
    ids: &CameraInputIds,
    camera: &Camera,
    global: &GlobalTransform,
    transform: &mut Transform,
    last_pos: &mut Option<Vec2>,
) {
    if !input.bool_or_default(ids.pan) {
        *last_pos = None;
        return;
    }

    let cursor = input.vec2_or_default(ids.mouse_x, ids.mouse_y);

    // If the cursor is zero things get weird
    if cursor.length_squared() == 0.0 {
        return;
    }

    if let Some(prev) = *last_pos {
        let (Ok(a), Ok(b)) = (
            camera.viewport_to_world_2d(global, cursor),
            camera.viewport_to_world_2d(global, prev),
        ) else {
            return;
        };

        transform.translation -= (a - b).extend(0.0);
    }

    *last_pos = Some(cursor);
}
