use crate::{
    input::{InputMapping, InputState},
    modding::{
        PostModLoad,
        registry::{Id, Registry},
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
    up: Id,
    down: Id,
    left: Id,
    right: Id,
    speed: Id,
    zoom_in: Id,
    zoom_out: Id,
    pan: Id,
}

fn init_camera_input_ids(mut commands: Commands, inputs: Res<Registry<InputMapping>>) {
    let ids = CameraInputIds {
        up: inputs.lookup("base::input::up").unwrap(),
        down: inputs.lookup("base::input::down").unwrap(),
        left: inputs.lookup("base::input::left").unwrap(),
        right: inputs.lookup("base::input::right").unwrap(),
        speed: inputs.lookup("base::input::speed").unwrap(),
        zoom_in: inputs.lookup("base::input::zoom_in").unwrap(),
        zoom_out: inputs.lookup("base::input::zoom_out").unwrap(),
        pan: inputs.lookup("base::input::pan").unwrap(),
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
    input: Res<InputState>,
    ids: Res<CameraInputIds>,
    time: Res<Time>,
    mut last_pos: Local<Option<Vec2>>,
) {
    let (camera, global, mut transform, projection) = query.single_mut().unwrap();

    let Projection::Orthographic(ortho) = projection.into_inner() else {
        return;
    };

    key_pan(&input, &ids, &mut transform, &ortho, &time);
    cursor_zoom(&input, &ids, ortho);
    drag_pan(&input, &ids, camera, global, &mut transform, &mut last_pos);
}

fn key_pan(
    input: &InputState,
    ids: &CameraInputIds,
    transform: &mut Transform,
    ortho: &OrthographicProjection,
    time: &Time,
) {
    let dir = input.vec2(ids.right, ids.left, ids.up, ids.down);

    if dir == Vec2::ZERO {
        return;
    }

    let mut speed = 128.0;
    if input.pressed(ids.speed) {
        speed *= 4.0;
    }

    transform.translation +=
        dir.normalize_or_zero().extend(0.0) * speed * ortho.scale * time.delta_secs();
}

fn cursor_zoom(input: &InputState, ids: &CameraInputIds, ortho: &mut OrthographicProjection) {
    let mut zoom = input.scroll();
    if input.just_pressed(ids.zoom_in) {
        zoom += 1.0;
    }
    if input.just_pressed(ids.zoom_out) {
        zoom -= 1.0;
    }

    // Apply zoom (exponential)
    let zoom_factor = 0.25;
    ortho.scale *= 1.0 - zoom_factor * zoom;
}

fn drag_pan(
    input: &InputState,
    ids: &CameraInputIds,
    camera: &Camera,
    global: &GlobalTransform,
    transform: &mut Transform,
    last_pos: &mut Option<Vec2>,
) {
    if !input.pressed(ids.pan) {
        *last_pos = None;
        return;
    }

    let Some(cursor) = input.mouse() else {
        return;
    };

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
