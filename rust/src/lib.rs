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

