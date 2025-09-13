use godot::{
    classes::{
        AnimationPlayer, CharacterBody2D, ICharacterBody2D, Input, ProjectSettings, Sprite2D,
    },
    global,
    prelude::*,
};

enum State {
    Air,
    Floor,
}

const WALK_SPEED: f32 = 120.;
const ACCEL_SPEED: f32 = WALK_SPEED * 6.0;
const MIN_WALK_SPEED: f32 = 0.1;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,
    gravity: f64,
    state: State,
    sprite: OnReady<Gd<Sprite2D>>,
    animation_player: OnReady<Gd<AnimationPlayer>>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
            gravity: 0.,
            state: State::Air,
            sprite: OnReady::from_node("Sprite2D"),
            animation_player: OnReady::from_node("AnimationPlayer"),
        }
    }

    fn ready(&mut self) {
        let settings = ProjectSettings::singleton();
        self.gravity = settings.get("physics/2d/default_gravity").to::<f64>();
        godot_print!("Player ready")
    }

    fn physics_process(&mut self, delta: f64) {
        let mut velocity = self.base().get_velocity();
        velocity.y += self.gravity as f32 * delta as f32;

        match self.state {
            State::Air => {
                if self.base().is_on_floor() {
                    self.state = State::Floor;
                    return;
                }
            }
            State::Floor => {
                self.try_walk(&mut velocity, delta);
            }
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();

        let velocity_x = self.base().get_velocity().x;
        if !velocity_x.is_zero_approx() {
            self.sprite
                .set_scale(Vector2::new(velocity_x.signum(), 1.0));
        }

        let animation = self.get_new_animation();
        if !animation.is_empty() && animation != self.animation_player.get_current_animation() {
            self.animation_player.set_current_animation(&animation);
            self.animation_player.play();
        }
    }
}

impl Player {
    fn try_walk(&mut self, velocity: &mut Vector2, delta: f64) {
        let input = Input::singleton();
        let direction = input.get_axis("ui_left", "ui_right");
        velocity.x = global::move_toward(
            velocity.x as f64,
            (direction * WALK_SPEED) as f64,
            ACCEL_SPEED as f64 * delta,
        ) as f32;
    }

    fn get_new_animation(&mut self) -> GString {
        let animation = if let State::Floor = self.state {
            if self.base().get_velocity().abs().x > MIN_WALK_SPEED {
                "walk"
            } else {
                "idle"
            }
        } else {
            ""
        };
        GString::from(animation)
    }
}
