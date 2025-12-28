use bevy::{app::MainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};

use registry::IdInterner;

use crate::modding::loader::{Mods, load_mods, preload_mods};

pub mod loader;
pub mod registry;

/// Loads mods at the start of the game and registers their types in the registry.
pub struct ModPlugin;

impl Plugin for ModPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_schedule(PreModLoad)
            .init_schedule(ModLoad)
            .init_schedule(PostModLoad)
            .init_resource::<Mods>()
            .add_systems(PreModLoad, preload_mods)
            .add_systems(ModLoad, load_mods);

        app.world_mut()
            .resource_mut::<MainScheduleOrder>()
            .insert_startup_before(Startup, PreModLoad);

        app.world_mut()
            .resource_mut::<MainScheduleOrder>()
            .insert_startup_after(PreModLoad, ModLoad);

        app.world_mut()
            .resource_mut::<MainScheduleOrder>()
            .insert_startup_after(ModLoad, PostModLoad);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct PreModLoad;

#[derive(Debug, Clone, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct ModLoad;

#[derive(Debug, Clone, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct PostModLoad;
