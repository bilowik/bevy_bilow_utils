use bevy::prelude::*;

use rand::{distributions::WeightedIndex, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub trait Modifier {
    fn initialize(&self, app: &mut App);
}

trait ChooseModifier {
    fn choose_modifier<T: Rng>(&self, rng: &mut T) -> Option<(&'static str, &Box<dyn Modifier>)>;
}

impl<T: Plugin + Clone> Modifier for T {
    fn initialize(&self, app: &mut App) {
        app.add_plugins(self.clone());
    }
}

pub struct ModifierSingle {
    #[allow(dead_code)]
    name: &'static str,
    modifier: Box<dyn Modifier>,
}

#[derive(Default)]
pub struct ModifierGroup {
    #[allow(dead_code)]
    name: &'static str,
    modifiers: Vec<(ModifierEntryInner, u32)>,
}

enum ModifierEntryInner {
    Single(ModifierSingle),
    Group(ModifierGroup),
}

pub struct ModifierEntry {
    modifier: ModifierEntryInner,
    chance_for_modifier: f32,
}

impl ModifierSingle {
    pub fn new<T>(name: &'static str, modifier: T) -> Self
    where
        for<'a> T: Modifier + 'a,
    {
        Self {
            name,
            modifier: Box::new(modifier) as Box<dyn Modifier>,
        }
    }
}

impl ModifierGroup {
    pub fn new(name: &'static str) -> Self {
        Self { name, ..default() }
    }

    pub fn with_single(&mut self, entry: ModifierSingle, weight: u32) -> &mut Self {
        self.modifiers
            .push((ModifierEntryInner::Single(entry), weight));
        self
    }

    pub fn with_group(&mut self, entry: ModifierGroup, weight: u32) -> &mut Self {
        self.modifiers
            .push((ModifierEntryInner::Group(entry), weight));
        self
    }

    fn get_modifier_weights(&self) -> Vec<u32> {
        self.modifiers.iter().map(|modifier| modifier.1).collect()
    }
}

impl ChooseModifier for ModifierSingle {
    fn choose_modifier<T: Rng>(&self, _: &mut T) -> Option<(&'static str, &Box<dyn Modifier>)> {
        Some((self.name, &self.modifier))
    }
}

impl ChooseModifier for ModifierGroup {
    fn choose_modifier<T: Rng>(&self, rng: &mut T) -> Option<(&'static str, &Box<dyn Modifier>)> {
        let weights = self.get_modifier_weights();
        let dist = WeightedIndex::new(&weights).ok()?;
        self.modifiers[rng.sample(&dist)].0.choose_modifier(rng)
    }
}

impl ChooseModifier for ModifierEntryInner {
    fn choose_modifier<T: Rng>(&self, rng: &mut T) -> Option<(&'static str, &Box<dyn Modifier>)> {
        match self {
            ModifierEntryInner::Single(modifier) => modifier.choose_modifier(rng),
            ModifierEntryInner::Group(modifier) => modifier.choose_modifier(rng),
        }
    }
}

impl ChooseModifier for ModifierEntry {
    fn choose_modifier<T: Rng>(&self, rng: &mut T) -> Option<(&'static str, &Box<dyn Modifier>)> {
        if (rng.gen::<f32>() * 100.0) < self.chance_for_modifier {
            self.modifier.choose_modifier(rng)
        } else {
            None
        }
    }
}

impl ModifierEntry {
    pub fn single<T>(name: &'static str, modifier: T, chance_for_modifier: f32) -> Self
    where
        for<'a> T: Modifier + 'a,
    {
        Self {
            modifier: ModifierEntryInner::Single(ModifierSingle::new(name, modifier)),
            chance_for_modifier,
        }
    }
    pub fn group(modifier: ModifierGroup, chance_for_modifier: f32) -> Self {
        Self {
            modifier: ModifierEntryInner::Group(modifier),
            chance_for_modifier,
        }
    }
}

pub struct Modifiers {
    modifiers: Vec<ModifierEntry>,
    seed: [u8; 32],
}

#[derive(Default, Clone, Resource)]
pub struct ActiveModifiers {
    modifiers: Vec<&'static str>,
}

impl ActiveModifiers {
    pub fn new(modifiers: &[&'static str]) -> Self {
        Self {
            modifiers: modifiers.to_vec(),
        }
    }

    pub fn get(&self) -> &[&'static str] {
        &self.modifiers
    }
}

impl Modifiers {
    pub fn new(seed: [u8; 32]) -> Self {
        Self {
            modifiers: Vec::new(),
            seed,
        }
    }

    pub fn with_modifier_entry(mut self, entry: ModifierEntry) -> Self {
        self.modifiers.push(entry);
        self
    }

    // Uses the given seed to init an rng to determine which modifier out of
    pub fn apply(&self, app: &mut App) -> ActiveModifiers {
        let mut rng = ChaCha8Rng::from_seed(self.seed.clone());
        let mut modifier_names = Vec::new();
        for entry in self.modifiers.iter() {
            if let Some((name, modifier)) = entry.choose_modifier(&mut rng) {
                modifier.initialize(app);
                modifier_names.push(name);
            }
        }
        ActiveModifiers::new(&modifier_names)
    }
}
