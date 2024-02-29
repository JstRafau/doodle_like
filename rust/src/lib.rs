use godot::{
    prelude::gdextension,
    private::class_macros::init::ExtensionLibrary,
};

pub mod character;
pub mod rooms;
pub mod projectile;
pub mod run;

struct DoodleLikeRs;

#[gdextension]
unsafe impl ExtensionLibrary for DoodleLikeRs {
}

pub struct Defaults {
    pub speed: f32, 
}

impl Defaults {
    pub const fn new() -> Self {
        Self {
            speed: 350.,
        }
    }
}

const DEFAULTS: Defaults = Defaults::new();

