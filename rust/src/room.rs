use godot::{builtin::PackedVector2Array, classes::Line2D, prelude::*};

const ROOM_WIDTH: f32 = 480.0;
const ROOM_HEIGHT: f32 = 270.0;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Room {
    base: Base<Node2D>,

    #[export]
    debug_draw_bounds: bool,
    outline: Option<Gd<Line2D>>,
}

#[godot_api]
impl INode2D for Room {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            debug_draw_bounds: true,
            outline: None,
        }
    }

    fn ready(&mut self) {
        self.refresh_outline();
        godot_print!("Room ready");
    }
}

impl Room {
    fn refresh_outline(&mut self) {
        if !self.debug_draw_bounds {
            if let Some(mut outline) = self.outline.take() {
                outline.queue_free();
            }
            return;
        }

        if let Some(outline) = self.outline.as_mut() {
            outline.set_visible(true);
            return;
        }

        let mut outline = Line2D::new_alloc();
        outline.set_default_color(Color::from_rgb(1.0, 1.0, 0.0));
        outline.set_width(2.0);
        outline.set_antialiased(false);
        outline.set_closed(true);
        outline.set_z_as_relative(false);
        outline.set_z_index(1000);

        let mut points = PackedVector2Array::new();
        points.push(Vector2::new(0.0, 0.0));
        points.push(Vector2::new(ROOM_WIDTH, 0.0));
        points.push(Vector2::new(ROOM_WIDTH, ROOM_HEIGHT));
        points.push(Vector2::new(0.0, ROOM_HEIGHT));
        outline.set_points(&points);

        let outline_node: Gd<Node> = outline.clone().upcast();
        self.base_mut().add_child(&outline_node);
        self.outline = Some(outline);
    }
}
