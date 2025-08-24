use godot::prelude::*;

mod level;
mod world;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
