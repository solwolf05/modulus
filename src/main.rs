use avian2d::prelude::*;
use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};

use crate::{
    camera::CameraPlugin,
    input::{InputPlugin, InputType},
    modding::{
        ModLoad, ModPlugin,
        registry::{IdInterner, Registry},
    },
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
            #[cfg(debug_assertions)]
            PhysicsDebugPlugin,
            ModPlugin,
            CameraPlugin,
            InputPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, esc_exit)
        // Temporary mod loading
        .add_systems(ModLoad, reg_setup)
        .init_resource::<Registry<InputType>>()
        .run()
}

fn reg_setup(mut registry: ResMut<Registry<InputType>>, interner: ResMut<IdInterner>) {
    let interner = interner.into_inner();
    registry.register(
        interner,
        "base::input::y",
        InputType::KeyAxis(KeyCode::KeyW, KeyCode::KeyS),
    );
    registry.register(
        interner,
        "base::input::x",
        InputType::KeyAxis(KeyCode::KeyD, KeyCode::KeyA),
    );
    registry.register(
        interner,
        "base::input::speed",
        InputType::KeyPressed(KeyCode::ShiftLeft),
    );
    registry.register(interner, "base::input::zoom", InputType::Scroll);
    registry.register(interner, "base::input::mouse_x", InputType::MouseX);
    registry.register(interner, "base::input::mouse_y", InputType::MouseY);
    registry.register(
        interner,
        "base::input::pan",
        InputType::MousePressed(MouseButton::Middle),
    );
}

fn setup(mut commands: Commands) {
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
