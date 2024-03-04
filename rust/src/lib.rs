use godot::{
    prelude::gdextension,
    private::class_macros::init::ExtensionLibrary,
};

pub mod character;
pub mod level;
pub mod projectile;
pub mod rooms;
pub mod run;

struct DoodleLikeRs;

#[gdextension]
unsafe impl ExtensionLibrary for DoodleLikeRs {
}

pub struct DDLDefaults {
    pub speed: f32, 
}

impl DDLDefaults {
    pub const fn new() -> Self {
        Self {
            speed: 350.,
        }
    }
}

const DEFAULTS: DDLDefaults = DDLDefaults::new();

