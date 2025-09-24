use godot::{
    classes::{Node, Node2D, PackedScene},
    prelude::*,
};

const LEVELS_LEN: usize = 3;
const PLAYER_SCENE_PATH: &str = "res://player.tscn";
const INITIAL_PLAYER_POS: (f32, f32) = (8.0, 8.0);
const LEVEL_WIDTH: f32 = 480.0;
const PLAYER_COLLISION_WIDTH: f32 = 16.0;
const PLAYER_CROSS_THRESHOLD: f32 = 0.50;

#[derive(Copy, Clone)]
struct LevelNeighbors {
    left: Option<usize>,
    right: Option<usize>,
}

const LEVEL_NEIGHBORS: [LevelNeighbors; LEVELS_LEN] = [
    LevelNeighbors {
        left: None,
        right: None,
    },
    LevelNeighbors {
        left: None,
        right: Some(2),
    },
    LevelNeighbors {
        left: Some(1),
        right: None,
    },
];

impl LevelNeighbors {
    fn for_level(index: usize) -> Self {
        LEVEL_NEIGHBORS[index]
    }
}

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct World {
    base: Base<Node2D>,
    level_scenes: [Option<Gd<PackedScene>>; LEVELS_LEN],
    preloaded_levels: [Option<Gd<Node2D>>; LEVELS_LEN],
    current_level: Option<Gd<Node2D>>,
    current_level_index: usize,
    player: Option<Gd<Node2D>>,
}

#[godot_api]
impl INode2D for World {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            level_scenes: std::array::from_fn(|_| None),
            preloaded_levels: std::array::from_fn(|_| None),
            current_level: None,
            current_level_index: 0,
            player: None,
        }
    }

    fn ready(&mut self) {
        for i in 1..LEVELS_LEN {
            let path = format!("res://level_{}.tscn", i);
            self.level_scenes[i] =
                Some(try_load::<PackedScene>(&path).expect("failed to load scene"));
        }

        let player_scene =
            try_load::<PackedScene>(PLAYER_SCENE_PATH).expect("failed to load player scene");

        let initial_level = match self.spawn_level(1) {
            Some(level) => level,
            None => {
                godot_error!("World failed to spawn initial level");
                return;
            }
        };

        let player_instance = player_scene
            .instantiate()
            .expect("failed to instantiate player scene");
        let mut player = player_instance
            .try_cast::<Node2D>()
            .expect("player root must inherit Node2D");

        let mut level_node = initial_level.clone().upcast::<Node>();
        level_node.add_child(&player.clone().upcast::<Node>());
        player.set_global_position(Vector2::new(INITIAL_PLAYER_POS.0, INITIAL_PLAYER_POS.1));

        self.player = Some(player);

        godot_print!("World ready");
    }

    fn physics_process(&mut self, _delta: f64) {
        let Some(player) = self.player.clone() else {
            return;
        };

        self.check_horizontal_transitions(player);
    }
}

#[derive(Copy, Clone, Debug)]
enum HorizontalDirection {
    Left,
    Right,
}

#[godot_api]
impl World {
    fn spawn_level(&mut self, level_no: usize) -> Option<Gd<Node2D>> {
        if level_no == 0 || level_no >= LEVELS_LEN {
            godot_warn!("Invalid level index {}", level_no);
            return None;
        }

        let level = match self.preloaded_levels[level_no].take() {
            Some(level) => level,
            None => self.instantiate_level(level_no)?,
        };

        if let Some(mut old_level) = self.current_level.take() {
            let old_level_node = old_level.clone().upcast::<Node>();
            self.base_mut().remove_child(&old_level_node);
            old_level.queue_free();
        }

        let level_node = level.clone().upcast::<Node>();
        self.base_mut().add_child(&level_node);
        self.current_level = Some(level.clone());
        self.current_level_index = level_no;
        self.preload_adjacent_levels(level_no);

        Some(level)
    }

    fn instantiate_level(&self, level_no: usize) -> Option<Gd<Node2D>> {
        let scene = self.level_scenes[level_no].as_ref()?;
        let instance = scene.instantiate().expect("failed to instantiate");
        Some(
            instance
                .try_cast::<Node2D>()
                .expect("level root must inherit Node2D"),
        )
    }

    fn preload_adjacent_levels(&mut self, center_level: usize) {
        self.discard_far_preloads(center_level);

        let neighbors = LevelNeighbors::for_level(center_level);
        for neighbor in [neighbors.left, neighbors.right] {
            let Some(index) = neighbor else {
                continue;
            };

            if self.preloaded_levels[index].is_some() {
                continue;
            }

            if let Some(level) = self.instantiate_level(index) {
                godot_print!("Preloaded level {}", index);
                self.preloaded_levels[index] = Some(level);
            }
        }
    }

    fn discard_far_preloads(&mut self, center_level: usize) {
        let mut keep = [false; LEVELS_LEN];

        if center_level < LEVELS_LEN {
            keep[center_level] = true;
        }

        let neighbors = LevelNeighbors::for_level(center_level);
        for neighbor in [neighbors.left, neighbors.right] {
            if let Some(index) = neighbor {
                keep[index] = true;
            }
        }

        for (idx, slot) in self.preloaded_levels.iter_mut().enumerate() {
            if idx == 0 {
                continue;
            }

            if !keep[idx] {
                if let Some(mut level) = slot.take() {
                    godot_print!("Dropping stale preload {}", idx);
                    level.queue_free();
                }
            }
        }
    }

    fn check_horizontal_transitions(&mut self, player: Gd<Node2D>) {
        let neighbors = LevelNeighbors::for_level(self.current_level_index);
        let position = player.get_global_position();
        let half_width = PLAYER_COLLISION_WIDTH * 0.5;

        if let Some(target_level) = neighbors.right {
            let player_right = position.x + half_width;
            let overflow = player_right - LEVEL_WIDTH;
            if self.should_trigger_transition(overflow) {
                self.transfer_player(player, target_level, HorizontalDirection::Right);
                return;
            }
        }

        if let Some(target_level) = neighbors.left {
            let player_left = position.x - half_width;
            let overflow = 0.0 - player_left;
            if self.should_trigger_transition(overflow) {
                self.transfer_player(player, target_level, HorizontalDirection::Left);
            }
        }
    }

    fn should_trigger_transition(&self, overflow: f32) -> bool {
        if overflow <= 0.0 {
            return false;
        }

        let ratio = overflow / PLAYER_COLLISION_WIDTH;
        ratio >= PLAYER_CROSS_THRESHOLD
    }

    fn transfer_player(
        &mut self,
        player: Gd<Node2D>,
        target_level: usize,
        direction: HorizontalDirection,
    ) {
        let mut player = player;
        let current_position = player.get_global_position();
        let mut new_position = current_position;

        match direction {
            HorizontalDirection::Right => {
                new_position.x -= LEVEL_WIDTH;
            }
            HorizontalDirection::Left => {
                new_position.x += LEVEL_WIDTH;
            }
        }

        let player_node: Gd<Node> = player.clone().upcast();
        if let Some(parent) = player.get_parent() {
            let mut parent = parent;
            parent.remove_child(&player_node);
        }

        let Some(new_level) = self.spawn_level(target_level) else {
            self.base_mut().add_child(&player_node);
            self.player = Some(player);
            return;
        };

        let mut level_node = new_level.clone().upcast::<Node>();
        level_node.add_child(&player_node);
        player.set_global_position(new_position);
        self.player = Some(player);

        godot_print!("Transitioned to level {} via {:?}", target_level, direction);
    }
}
