use std::collections::{HashMap, hash_map};

use bevy::ecs::resource::Resource;

#[derive(Debug, Default, Resource)]
pub struct Registry<T> {
    map: HashMap<Id, T>,
}

impl<T> Registry<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn register(&mut self, interner: &mut IdInterner, path: &str, value: T) -> Option<Id> {
        let id = interner.intern(path)?;
        self.map.insert(id, value);
        Some(id)
    }

    pub fn get(&self, id: &Id) -> Option<&T> {
        self.map.get(id)
    }

    pub fn contains(&self, id: &Id) -> bool {
        self.map.contains_key(id)
    }

    pub fn iter<'a>(&'a self) -> hash_map::Iter<'a, Id, T> {
        self.map.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> hash_map::IterMut<'a, Id, T> {
        self.map.iter_mut()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub u32);

#[derive(Default, Resource)]
pub struct IdInterner {
    strings: Vec<Box<str>>,
    lookup: HashMap<Box<str>, Id>,
}

impl IdInterner {
    pub fn intern(&mut self, path: &str) -> Option<Id> {
        if let Some(&id) = self.lookup.get(path) {
            return Some(id);
        }

        if !Self::is_valid_path(path) {
            return None;
        }

        let id = Id(self.strings.len() as u32);
        let boxed: Box<str> = path.into();

        self.strings.push(boxed.clone());
        self.lookup.insert(boxed, id);

        Some(id)
    }

    pub fn resolve(&self, id: Id) -> &str {
        &self.strings[id.0 as usize]
    }

    fn is_valid_path(path: &str) -> bool {
        let segments = path.split("::");
        for segment in segments {
            if !Self::is_valid_segment(segment) {
                return false;
            }
        }
        true
    }

    fn is_valid_segment(segment: &str) -> bool {
        if segment.is_empty() || segment.starts_with('_') || segment.ends_with('_') {
            return false;
        }

        segment
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    }
}
