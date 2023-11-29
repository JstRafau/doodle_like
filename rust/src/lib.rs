use godot::prelude::*;

struct MyExt;

#[gdextension]
unsafe impl ExtensionLibrary for MyExt {}
