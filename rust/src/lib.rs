use godot::prelude::*;

mod game;
mod world;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
