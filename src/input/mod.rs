use std::collections::{HashMap, HashSet};

use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::modding::{
    PostModLoad,
    registry::{Id, Registry},
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputManager>()
            .add_systems(PostModLoad, setup)
            .add_systems(PreUpdate, update);
    }
}

fn setup(mut input_manager: ResMut<InputManager>, registry: Res<Registry<InputType>>) {
    for (id, _) in registry.iter() {
        input_manager.inputs.insert(id.clone(), 0.0);
    }
}

fn update(
    mut input_manager: ResMut<InputManager>,
    registry: Res<Registry<InputType>>,
    key_inputs: Res<ButtonInput<KeyCode>>,
    mouse_inputs: Res<ButtonInput<MouseButton>>,
    mut scroll_input: MessageReader<MouseWheel>,
    windows: Query<&Window>,
) {
    for (id, input) in registry.iter() {
        let mut value = 0.0;

        match input {
            InputType::None => {}
            InputType::KeyPressed(key_code) => {
                if key_inputs.pressed(*key_code) {
                    value = 1.0;
                }
            }
            InputType::KeyJustPressed(key_code) => {
                if key_inputs.just_pressed(*key_code) {
                    value = 1.0;
                }
            }
            InputType::KeyJustReleased(key_code) => {
                if key_inputs.just_released(*key_code) {
                    value = 1.0;
                }
            }
            InputType::KeyAxis(positive, negative) => {
                let positive = key_inputs.pressed(*positive);
                let negative = key_inputs.pressed(*negative);

                if positive && negative {
                } else if positive {
                    value = 1.0;
                } else if negative {
                    value = -1.0;
                }
            }
            InputType::Scroll => {
                for event in scroll_input.read() {
                    value += event.y;
                }
            }
            InputType::MouseX => {
                value = windows
                    .single()
                    .unwrap()
                    .cursor_position()
                    .unwrap_or_default()
                    .x
            }
            InputType::MouseY => {
                value = windows
                    .single()
                    .unwrap()
                    .cursor_position()
                    .unwrap_or_default()
                    .y
            }
            InputType::MousePressed(mouse_button) => {
                if mouse_inputs.pressed(*mouse_button) {
                    value = 1.0;
                }
            }
            InputType::MouseJustPressed(mouse_button) => {
                if mouse_inputs.just_pressed(*mouse_button) {
                    value = 1.0;
                }
            }
            InputType::MouseJustReleased(mouse_button) => {
                if mouse_inputs.just_released(*mouse_button) {
                    value = 1.0;
                }
            }
        }

        input_manager.inputs.insert(id.clone(), value);
    }
}

#[derive(Debug, Default)]
pub enum InputType {
    #[default]
    None,
    KeyPressed(KeyCode),
    KeyJustPressed(KeyCode),
    KeyJustReleased(KeyCode),
    KeyAxis(KeyCode, KeyCode),
    Scroll,
    MouseX,
    MouseY,
    MousePressed(MouseButton),
    MouseJustPressed(MouseButton),
    MouseJustReleased(MouseButton),
}

pub struct Input {
    /// The base input type being detected
    input_type: InputType,
    /// A set of modifiers that must all be pressed for the input to be detected
    modifiers: HashSet<KeyCode>,
}

#[derive(Debug, Default, Resource)]
pub struct InputManager {
    inputs: HashMap<Id, f32>,
}

impl InputManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn axis(&self, id: Id) -> Option<f32> {
        self.inputs.get(&id).copied()
    }

    pub fn bool(&self, id: Id) -> Option<bool> {
        self.inputs.get(&id).map(|v| *v != 0.0)
    }

    pub fn vec2(&self, x: Id, y: Id) -> Option<Vec2> {
        Some(Vec2 {
            x: self.axis(x)?,
            y: self.axis(y)?,
        })
    }

    pub fn vec3(&self, x: Id, y: Id, z: Id) -> Option<Vec3> {
        Some(Vec3 {
            x: self.axis(x)?,
            y: self.axis(y)?,
            z: self.axis(z)?,
        })
    }

    pub fn axis_or(&self, id: Id, default: f32) -> f32 {
        self.axis(id).unwrap_or(default)
    }

    pub fn bool_or(&self, id: Id, default: bool) -> bool {
        self.bool(id).unwrap_or(default)
    }

    pub fn vec2_or(&self, x: Id, y: Id, default: Vec2) -> Vec2 {
        self.vec2(x, y).unwrap_or(default)
    }

    pub fn vec3_or(&self, x: Id, y: Id, z: Id, default: Vec3) -> Vec3 {
        self.vec3(x, y, z).unwrap_or(default)
    }

    pub fn axis_or_default(&self, id: Id) -> f32 {
        self.axis(id).unwrap_or_default()
    }

    pub fn bool_or_default(&self, id: Id) -> bool {
        self.bool(id).unwrap_or_default()
    }

    pub fn vec2_or_default(&self, x: Id, y: Id) -> Vec2 {
        self.vec2(x, y).unwrap_or_default()
    }

    pub fn vec3_or_default(&self, x: Id, y: Id, z: Id) -> Vec3 {
        self.vec3(x, y, z).unwrap_or_default()
    }
}
