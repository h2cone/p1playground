use godot::prelude::*;

mod game;
mod level;
mod level_portal;
mod player;
mod world;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
