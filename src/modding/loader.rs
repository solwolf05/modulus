use std::{collections::HashMap, fs, path::PathBuf};

use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Default, Resource)]
pub struct Mods {
    mods: Vec<Mod>,
}

#[derive(Debug)]
pub struct Mod {
    metadata: ModMetadata,
    path: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct ModMetadata {
    id: String,
    name: String,
    description: String,
    version: String,
    dependencies: HashMap<String, String>,
}

pub fn preload_mods(mut mods: ResMut<Mods>) {
    // Temporary
    let mods_dir = "/home/solwolf/dev/modulus/mods";

    let entries = fs::read_dir(mods_dir).expect("unable to read mods dir");

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let mod_toml = path.join("mod.toml");
        if !mod_toml.exists() {
            continue;
        }

        let metadata_bytes = match fs::read(mod_toml) {
            Ok(m) => m,
            Err(e) => {
                error!("unable to read mod.toml: {}", e);
                continue;
            }
        };

        let metadata: ModMetadata = match toml::from_slice(&metadata_bytes) {
            Ok(m) => m,
            Err(e) => {
                error!("error parsing mod.toml:\n{}", e);
                continue;
            }
        };
        mods.mods.push(Mod { metadata, path });
    }
}

pub fn load_mods(mut mods: ResMut<Mods>) {}

fn load_mod(mod_data: &Mod) {}
