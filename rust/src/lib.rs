use godot::prelude::*;

mod game;
mod player;
mod room;
mod room_graph;
mod world;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
