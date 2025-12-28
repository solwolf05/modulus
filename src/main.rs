use avian2d::prelude::*;
use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};

use crate::{
    camera::CameraPlugin,
    input::{Input, InputMapping, InputPlugin, InputState},
    modding::{ModLoad, ModPlugin, registry::Registry},
};

mod camera;
mod input;
mod modding;

/// The number of pixels to a metre
const UNIT: usize = 16;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoVsync,
                        mode: WindowMode::Windowed,
                        title: "Modulus".to_owned(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default().with_length_unit(UNIT as f32),
            PhysicsPickingPlugin,
            #[cfg(debug_assertions)]
            PhysicsDebugPlugin,
            ModPlugin,
            InputPlugin,
            CameraPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (esc_exit, cursor_system))
        // Temporary mod loading
        .add_systems(ModLoad, reg_setup)
        .run()
}

fn reg_setup(mut input: ResMut<Registry<InputMapping>>) {
    input
        .register("base::input::up", Input::key(KeyCode::KeyW).into())
        .unwrap();
    input
        .register("base::input::down", Input::key(KeyCode::KeyS).into())
        .unwrap();
    input
        .register("base::input::left", Input::key(KeyCode::KeyA).into())
        .unwrap();
    input
        .register("base::input::right", Input::key(KeyCode::KeyD).into())
        .unwrap();

    input
        .register("base::input::speed", Input::key(KeyCode::ShiftLeft).into())
        .unwrap();

    input
        .register(
            "base::input::zoom_in",
            Input::key(KeyCode::Equal).with_lshift().into(),
        )
        .unwrap();
    input
        .register(
            "base::input::zoom_out",
            Input::key(KeyCode::Minus).with_lshift().into(),
        )
        .unwrap();

    input
        .register("base::input::pan", Input::mouse(MouseButton::Middle).into())
        .unwrap();

    input
        .register(
            "base::input::select",
            Input::mouse(MouseButton::Left).into(),
        )
        .unwrap();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Cursor,
        Sprite::from_color(Color::hsl(0.0, 0.0, 1.0), Vec2::ONE),
        RigidBody::Kinematic,
        Collider::rectangle(1.0, 1.0),
    ));

    // Red physics box
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Dynamic,
        Collider::rectangle(16.0, 16.0),
        Sprite::from_color(Color::hsl(0.0, 1.0, 0.5), Vec2::new(16.0, 16.0)),
    ));

    // Platform
    commands.spawn((
        Transform::from_xyz(0.0, -128.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(256.0, 16.0),
        Sprite::from_color(Color::hsl(90.0, 1.0, 0.5), Vec2::new(256.0, 16.0)),
    ));
}

fn esc_exit(input: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

#[derive(Debug, Component)]
#[require(Transform)]
struct Cursor;

fn cursor_system(
    mut cursor: Query<&mut Transform, With<Cursor>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    input: Res<InputState>,
) {
    let Some(mouse) = input.mouse() else {
        return;
    };

    let camera = camera.single().unwrap();
    let camera_transform = camera.1;
    let camera = camera.0;
    if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, mouse) {
        cursor.single_mut().unwrap().translation = world_pos.extend(0.0);
    }
}
