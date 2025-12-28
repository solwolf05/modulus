use std::collections::{HashMap, HashSet};

use bevy::{
    input::{InputSystems, mouse::MouseWheel},
    prelude::*,
};

use crate::modding::{
    PostModLoad,
    registry::{Id, Registry},
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<InputMap>()
            .init_resource::<Registry<InputMapping>>()
            .add_systems(PreUpdate, input_state_system.after(InputSystems))
            .add_systems(PostModLoad, setup_input_map);
    }
}

fn input_state_system(
    state: ResMut<InputState>,
    map: Res<InputMap>,
    key_buttons: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    scroll: MessageReader<MouseWheel>,
    window: Query<&Window>,
) {
    let state = state.into_inner();
    update_buttons(
        state,
        map.into_inner(),
        key_buttons.into_inner(),
        mouse_buttons.into_inner(),
    );
    update_scroll(state, scroll);
    update_mouse(state, window.single_inner().unwrap());
}

fn update_buttons(
    state: &mut InputState,
    map: &InputMap,
    key_buttons: &ButtonInput<KeyCode>,
    mouse_buttons: &ButtonInput<MouseButton>,
) {
    state.clear();

    for (&id, input) in map.map.iter() {
        // Check if all modifiers are pressed
        if !input.modifiers.iter().all(|&m| key_buttons.pressed(m)) {
            continue;
        }

        match input.input_type {
            InputType::None => {}
            InputType::KeyButton(key_code) => {
                if key_buttons.just_pressed(key_code) {
                    state.press(id);
                } else if key_buttons.just_released(key_code) {
                    state.release(id);
                }
            }
            InputType::MouseButton(mouse_button) => {
                if mouse_buttons.just_pressed(mouse_button) {
                    state.press(id);
                } else if mouse_buttons.just_released(mouse_button) {
                    state.release(id);
                }
            }
        }
    }
}

fn update_scroll(state: &mut InputState, mut scroll: MessageReader<MouseWheel>) {
    state.scroll = scroll.read().fold(0.0, |sum, event| sum + event.y);
}

fn update_mouse(state: &mut InputState, window: &Window) {
    state.mouse = window.cursor_position();
}

fn setup_input_map(mut map: ResMut<InputMap>, registry: Res<Registry<InputMapping>>) {
    // TODO: Implement loading from save file (serialisation)
    for (&id, input) in registry.iter() {
        map.insert(id, input.default.clone());
    }
}

/// Contains the states for each input mapping
#[derive(Debug, Default, Resource)]
pub struct InputState {
    pressed: HashSet<Id>,
    just_pressed: HashSet<Id>,
    just_released: HashSet<Id>,
    mouse: Option<Vec2>,
    scroll: f32,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
            mouse: None,
            scroll: 0.0,
        }
    }

    pub fn clear(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    pub fn press(&mut self, id: Id) {
        self.pressed.insert(id);
        self.just_pressed.insert(id);
    }

    pub fn release(&mut self, id: Id) {
        self.pressed.remove(&id);
        self.just_released.insert(id);
    }

    pub fn pressed(&self, id: Id) -> bool {
        self.pressed.contains(&id)
    }

    pub fn just_pressed(&self, id: Id) -> bool {
        self.just_pressed.contains(&id)
    }

    pub fn just_released(&self, id: Id) -> bool {
        self.just_released.contains(&id)
    }

    pub fn axis(&self, positive: Id, negative: Id) -> f32 {
        let positive = self.pressed.contains(&positive) as i8;
        let negative = self.pressed.contains(&negative) as i8;
        (positive - negative) as f32
    }

    pub fn vec2(&self, positive_x: Id, negative_x: Id, positive_y: Id, negative_y: Id) -> Vec2 {
        let x = self.axis(positive_x, negative_x);
        let y = self.axis(positive_y, negative_y);
        Vec2::new(x, y)
    }

    pub fn mouse(&self) -> Option<Vec2> {
        self.mouse
    }

    pub fn scroll(&self) -> f32 {
        self.scroll
    }
}

/// Contains the mappings of ids to physical inputs
#[derive(Debug, Default, Resource)]
pub struct InputMap {
    map: HashMap<Id, Input>,
}

impl InputMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, id: Id) -> Option<&Input> {
        self.map.get(&id)
    }

    pub fn insert(&mut self, id: Id, input: Input) {
        self.map.insert(id, input);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Input {
    input_type: InputType,
    modifiers: HashSet<KeyCode>,
}

impl Input {
    pub fn new(input_type: InputType, modifiers: HashSet<KeyCode>) -> Self {
        Self {
            input_type,
            modifiers,
        }
    }

    pub fn from_input_type(input_type: InputType) -> Self {
        Self {
            input_type,
            modifiers: HashSet::new(),
        }
    }

    pub fn none() -> Self {
        Self {
            input_type: InputType::None,
            modifiers: HashSet::new(),
        }
    }

    pub fn key(key_code: KeyCode) -> Self {
        Self {
            input_type: InputType::KeyButton(key_code),
            modifiers: HashSet::new(),
        }
    }

    pub fn mouse(mouse_button: MouseButton) -> Self {
        Self {
            input_type: InputType::MouseButton(mouse_button),
            modifiers: HashSet::new(),
        }
    }

    pub fn with_modifier(mut self, modifier: KeyCode) -> Self {
        self.modifiers.insert(modifier);
        self
    }

    pub fn with_lshift(self) -> Self {
        self.with_modifier(KeyCode::ShiftLeft)
    }

    pub fn with_rshift(self) -> Self {
        self.with_modifier(KeyCode::ShiftRight)
    }

    pub fn with_lctrl(self) -> Self {
        self.with_modifier(KeyCode::ControlLeft)
    }

    pub fn with_rctrl(self) -> Self {
        self.with_modifier(KeyCode::ControlRight)
    }

    pub fn with_lalt(self) -> Self {
        self.with_modifier(KeyCode::AltLeft)
    }

    pub fn with_ralt(self) -> Self {
        self.with_modifier(KeyCode::AltRight)
    }
}

impl From<InputType> for Input {
    fn from(value: InputType) -> Self {
        Self::from_input_type(value)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputType {
    #[default]
    None,
    KeyButton(KeyCode),
    MouseButton(MouseButton),
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InputMapping {
    name: String,
    default: Input,
}

impl InputMapping {
    pub fn new(name: &str, default: Input) -> Self {
        Self {
            name: name.to_string(),
            default,
        }
    }

    pub(crate) fn nameless(default: Input) -> Self {
        Self::new("", default)
    }
}

impl From<Input> for InputMapping {
    fn from(value: Input) -> Self {
        Self::nameless(value)
    }
}
