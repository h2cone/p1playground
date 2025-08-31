use godot::{classes::InputEvent, prelude::*};

const LEVELS_LEN: usize = 3;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct World {
    base: Base<Node2D>,
    levels: [Option<Gd<PackedScene>>; LEVELS_LEN],
    level_no: usize,
    level_idx: Option<i32>,
}

#[godot_api]
impl INode2D for World {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            level_no: 0,
            level_idx: None,
            levels: [None, None, None],
        }
    }

    fn ready(&mut self) {
        for i in 1..LEVELS_LEN {
            let path = format!("res://level_{}.tscn", i);
            self.levels[i] = Some(try_load::<PackedScene>(&path).expect("failed to load scene"));
        }
        godot_print!("World ready")
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        let level_no = if event.is_action_pressed("ui_left") {
            1
        } else if event.is_action_pressed("ui_right") {
            2
        } else {
            return;
        };
        if level_no == self.level_no {
            return;
        }
        if self.level_no != 0 {
            if let Some(mut old_level) = self
                .base()
                .get_child(self.level_idx.expect("requires level index"))
            {
                self.base_mut().remove_child(&old_level);
                old_level.queue_free();
            }
        }
        if let Some(scene) = &self.levels[level_no] {
            let new_level = scene.instantiate().expect("failed to instantiate");
            self.base_mut().add_child(&new_level);
            self.level_idx = Some(new_level.get_index());
            self.level_no = level_no;
        }
    }
}
