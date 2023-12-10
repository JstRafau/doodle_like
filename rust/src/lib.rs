use godot::{
    prelude::gdextension,
    private::class_macros::init::ExtensionLibrary,
};

pub mod character;

struct DoodleLikeRs;

#[gdextension]
unsafe impl ExtensionLibrary for DoodleLikeRs {
}

