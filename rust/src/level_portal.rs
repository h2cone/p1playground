use godot::{
    classes::{Area2D, IArea2D, Node2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct LevelPortal {
    base: Base<Area2D>,
    #[export]
    target_level: i64,
    #[export]
    spawn_point: GString,
}

#[godot_api]
impl IArea2D for LevelPortal {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            base,
            target_level: 0,
            spawn_point: GString::default(),
        }
    }

    fn ready(&mut self) {
        godot_print!(
            "LevelPortal ready -> target_level={}, spawn_point={}",
            self.target_level,
            self.spawn_point
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
            self.spawn_point
        );
    }

    #[func]
    fn on_body_exited(&mut self, body: Gd<Node2D>) {
        godot_print!("LevelPortal body_exited: {}", body.get_name());
    }
}
