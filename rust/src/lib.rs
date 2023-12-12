use godot::{
    prelude::gdextension,
    private::class_macros::init::ExtensionLibrary,
};

pub mod character;
pub mod rooms;

struct DoodleLikeRs;

#[gdextension]
unsafe impl ExtensionLibrary for DoodleLikeRs {
}

