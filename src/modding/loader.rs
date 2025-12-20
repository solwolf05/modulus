use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};

use rune::runtime::RuntimeContext;
use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Context, Diagnostics, Source, Sources, Vm};

use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Default, Resource)]
pub struct Mods {
    mods: Vec<Mod>,
    context: Context,
    runtime: Arc<RuntimeContext>,
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

pub fn load_mods(mut mods: ResMut<Mods>) {
    mods.context = Context::with_default_modules().unwrap();
    mods.runtime = Arc::new(mods.context.runtime().unwrap());

    for mod_data in mods.mods.iter() {
        load_mod(mod_data, &mods.context, mods.runtime.clone());
    }

    // let mut sources = Sources::new();
    // sources.insert(Source::from_path("path").unwrap()).unwrap();

    // let mut diagnostics = Diagnostics::new();

    // let result = rune::prepare(&mut sources)
    //     .with_context(&context)
    //     .with_diagnostics(&mut diagnostics)
    //     .build();

    // if !diagnostics.is_empty() {
    //     let mut writer = StandardStream::stderr(ColorChoice::Always);
    //     diagnostics.emit(&mut writer, &sources).unwrap();
    // }

    // let unit = result.unwrap();
    // let unit = Arc::new(unit);
    // let mut vm = Vm::new(runtime, unit);

    // let output = vm.call(["init"], (10i64, 20i64)).unwrap();
    // let output: i64 = rune::from_value(output).unwrap();

    // println!("{}", output);
}

fn load_mod(mod_data: &Mod, context: &Context, runtime: Arc<RuntimeContext>) {
    let mut sources = Sources::new();
    sources
        .insert(match Source::from_path(mod_data.path.join("main.rune")) {
            Ok(source) => source,
            Err(e) => {
                error!(
                    "error loading mod script for {}: {}",
                    mod_data.metadata.id, e
                );
                return;
            }
        })
        .unwrap();

    let mut diagnostics = Diagnostics::new();

    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        error!("failed to compile mod script:");
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics.emit(&mut writer, &sources).unwrap();
        return;
    }

    let unit = Arc::new(result.unwrap());
    let mut vm = Vm::new(runtime, unit);

    if let Err(e) = vm.call(["init"], ()) {
        error!(
            "error while executing {} main script: {}",
            mod_data.metadata.id, e
        );
    }
}
