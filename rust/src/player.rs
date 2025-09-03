use godot::{
    classes::{CharacterBody2D, ICharacterBody2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        godot_print!("Player ready")
    }

    fn physics_process(&mut self, _delta: f64) {}
}

impl Player {}
