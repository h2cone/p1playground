use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct World {
    base: Base<Node>,
}

#[godot_api]
impl INode for World {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        godot_print!("World ready!")
    }
}
