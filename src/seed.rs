//use bevy::prelude::*;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use sha3::{Digest, Sha3_256};

pub static SEED_ENV: Option<&'static str> = std::option_env!("COMPILE_TIME_SEED");
lazy_static! {
    pub static ref SEED: [u8; 32] = {
        if let Some(s) = SEED_ENV {
            let mut hasher = Sha3_256::new();
            hasher.update(s.as_bytes());
            hasher.finalize().into()
        }
        else {
            // Use fully random seed
            thread_rng().gen()
        }
    };
}
/*
#[derive(Resource)]
pub struct Seed([u8; 32]);

impl Seed {
    pub fn new(seed: [u8; 32]) -> Self {
        Self(seed)
    }

    pub fn hash_from_bytes(bytes: &[u8]) -> Self {
        let mut hasher = Sha3_256::new();
        hasher.update(bytes);
        Self(hasher.finalize().into())
    }

    pub fn get_seed(&self) -> &[u8; 32] {
        &self.0
    }

}

#[derive(Default)]
pub struct CompileTimeSeed;


impl Plugin for CompileTimeSeed {
    fn build(&self, app: &mut App) {
        let seed = if let Some(s) = SEED_ENV {
            Seed::hash_from_bytes(s.as_bytes())
        }
        else {
            // Use fully random seed
            Seed::new(thread_rng().gen())
        };
        app.insert_resource(seed);
    }
}

*/
