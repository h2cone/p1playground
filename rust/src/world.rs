use godot::prelude::*;

const LEVELS_LEN: usize = 3;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct World {
    base: Base<Node2D>,
    levels: [Option<Gd<PackedScene>>; LEVELS_LEN],
}

#[godot_api]
impl INode2D for World {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            levels: std::array::from_fn(|_| None),
        }
    }

    fn ready(&mut self) {
        for i in 1..LEVELS_LEN {
            let path = format!("res://level_{}.tscn", i);
            self.levels[i] = Some(try_load::<PackedScene>(&path).expect("failed to load scene"));
        }
        let level_no = 1;
        if let Some(scene) = &self.levels[level_no] {
            let new_level = scene.instantiate().expect("failed to instantiate");
            self.base_mut().add_child(&new_level);
        }
        godot_print!("World ready")
    }
}
