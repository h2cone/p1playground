use godot::{
    classes::{Area2D, IArea2D, Node2D},
    prelude::*,
};

use crate::world::World;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct LevelPortal {
    base: Base<Area2D>,
    #[export]
    target_level: i64,
    #[export]
    spawn_point: NodePath,
}

#[godot_api]
impl IArea2D for LevelPortal {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            base,
            target_level: 0,
            spawn_point: NodePath::default(),
        }
    }

    fn ready(&mut self) {
        godot_print!(
            "LevelPortal ready -> target_level={}, spawn_point={}",
            self.target_level,
            self.spawn_point.to_string()
        );

        self.base()
            .signals()
            .body_entered()
            .connect_other(self, LevelPortal::on_body_entered);

        self.base()
            .signals()
            .body_exited()
            .connect_other(self, LevelPortal::on_body_exited);
    }
}

#[godot_api]
impl LevelPortal {
    #[func]
    fn on_body_entered(&mut self, body: Gd<Node2D>) {
        godot_print!(
            "LevelPortal body_entered: {} (target={}, spawn={})",
            body.get_name(),
            self.target_level,
            self.spawn_point.to_string()
        );
    }

    #[func]
    fn on_body_exited(&mut self, body: Gd<Node2D>) {
        if !body.is_class("Player") {
            return;
        }

        if self.target_level == 0 {
            godot_warn!(
                "LevelPortal {} missing target_level",
                self.base().get_name()
            );
            return;
        }

        let offset = body.get_global_position() - self.base().get_global_position();
        let portal_id = self.base().instance_id().to_i64();

        let Some(level_parent) = body.get_parent() else {
            return;
        };
        let Some(world_node) = level_parent.get_parent() else {
            return;
        };

        match world_node.try_cast::<World>() {
            Ok(mut world) => {
                godot_print!(
                    "LevelPortal requesting transfer via {} -> level {}",
                    self.base().get_name(),
                    self.target_level
                );

                let args = [
                    portal_id.to_variant(),
                    self.target_level.to_variant(),
                    self.spawn_point.to_variant(),
                    offset.to_variant(),
                    body.to_variant(),
                ];

                world.call_deferred("queue_portal_transfer", &args);
            }
            Err(_) => {
                godot_warn!(
                    "LevelPortal {} could not locate World node",
                    self.base().get_name()
                );
            }
        }
    }
}
