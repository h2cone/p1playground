use godot::{
    classes::{Node2D, PackedScene},
    prelude::*,
};
use std::collections::HashMap;

use crate::{player::Player, room_graph::RoomGraph};

const PLAYER_SCENE_PATH: &str = "res://player.tscn";
const INITIAL_PLAYER_POS: (f32, f32) = (8.0, 8.0);
const INITIAL_ROOM: &str = "room_1_0";
const ROOM_WIDTH: f32 = 480.0;
const PLAYER_WIDTH: f32 = 16.0;
const PLAYER_CROSS_THRESHOLD: f32 = 0.50;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct World {
    base: Base<Node2D>,
    room_graph: RoomGraph,
    room_scenes: HashMap<String, Gd<PackedScene>>,
    preloaded_rooms: HashMap<String, Gd<Node2D>>,
    current_room: Option<Gd<Node2D>>,
    current_room_name: String,
    player: Option<Gd<Player>>,
}

#[godot_api]
impl INode2D for World {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            room_graph: RoomGraph::new(),
            room_scenes: HashMap::new(),
            preloaded_rooms: HashMap::new(),
            current_room: None,
            current_room_name: String::new(),
            player: None,
        }
    }

    fn ready(&mut self) {
        for room_name in self.room_graph.all_rooms() {
            let path = format!("res://{}.tscn", room_name);
            let scene = try_load::<PackedScene>(&path).expect("failed to load scene");
            self.room_scenes.insert(room_name.to_string(), scene);
        }

        let player_scene =
            try_load::<PackedScene>(PLAYER_SCENE_PATH).expect("failed to load player scene");

        let mut initial_room = match self.spawn_room(INITIAL_ROOM) {
            Some(room) => room,
            None => {
                godot_error!("World failed to spawn initial room");
                return;
            }
        };

        let player_instance = player_scene
            .instantiate()
            .expect("failed to instantiate player scene");
        let mut player = player_instance
            .try_cast::<Player>()
            .expect("player root must be Player");

        player.set_global_position(Vector2::new(INITIAL_PLAYER_POS.0, INITIAL_PLAYER_POS.1));
        initial_room.add_child(&player);

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
    fn spawn_room(&mut self, room_name: &str) -> Option<Gd<Node2D>> {
        let room = match self.preloaded_rooms.remove(room_name) {
            Some(room) => room,
            None => self.instantiate_room(room_name)?,
        };

        if let Some(mut old_room) = self.current_room.take() {
            self.base_mut().remove_child(&old_room);
            old_room.queue_free();
        }

        self.base_mut().add_child(&room);
        self.current_room = Some(room.clone());
        self.current_room_name = room_name.to_string();
        self.preload_adjacent_rooms(room_name);

        Some(room)
    }

    fn instantiate_room(&self, room_name: &str) -> Option<Gd<Node2D>> {
        let scene = self.room_scenes.get(room_name)?;
        let instance = scene.instantiate().expect("failed to instantiate");
        Some(
            instance
                .try_cast::<Node2D>()
                .expect("room root must inherit Node2D"),
        )
    }

    fn preload_adjacent_rooms(&mut self, center_room: &str) {
        self.discard_far_rooms(center_room);

        let neighbors = self.room_graph.get_neighbors(center_room);
        for neighbor in [
            neighbors.left,
            neighbors.right,
            neighbors.up,
            neighbors.down,
        ] {
            let Some(room_name) = neighbor else {
                continue;
            };

            if self.preloaded_rooms.contains_key(room_name) {
                continue;
            }

            if let Some(room) = self.instantiate_room(room_name) {
                godot_print!("Preloaded room {}", room_name);
                self.preloaded_rooms.insert(room_name.to_string(), room);
            }
        }
    }

    fn discard_far_rooms(&mut self, center_room: &str) {
        let mut keep = vec![center_room.to_string()];

        let neighbors = self.room_graph.get_neighbors(center_room);
        for room_name in [
            neighbors.left,
            neighbors.right,
            neighbors.up,
            neighbors.down,
        ]
        .into_iter()
        .flatten()
        {
            keep.push(room_name.to_string());
        }

        self.preloaded_rooms.retain(|name, room| {
            if keep.contains(name) {
                true
            } else {
                godot_print!("Dropping stale preload {}", name);
                room.clone().queue_free();
                false
            }
        });
    }

    fn check_horizontal_transitions(&mut self, player: Gd<Player>) {
        let neighbors = self.room_graph.get_neighbors(&self.current_room_name);
        let (position, velocity_x) = {
            let player_ref = player.bind();
            let base = player_ref.base();
            (base.get_global_position(), base.get_velocity().x)
        };
        let half_width = PLAYER_WIDTH * 0.5;

        if let Some(target_room) = neighbors.right {
            let player_right = position.x + half_width;
            let overflow = player_right - ROOM_WIDTH;
            if velocity_x > 0.0 && self.should_trigger_transition(overflow) {
                self.transfer_player(player, target_room, HorizontalDirection::Right);
                return;
            }
        }

        if let Some(target_room) = neighbors.left {
            let player_left = position.x - half_width;
            let overflow = 0.0 - player_left;
            if velocity_x < 0.0 && self.should_trigger_transition(overflow) {
                self.transfer_player(player, target_room, HorizontalDirection::Left);
            }
        }
    }

    fn should_trigger_transition(&self, overflow: f32) -> bool {
        if overflow <= 0.0 {
            return false;
        }

        let ratio = overflow / PLAYER_WIDTH;
        ratio >= PLAYER_CROSS_THRESHOLD
    }

    fn transfer_player(
        &mut self,
        player: Gd<Player>,
        target_room: &str,
        direction: HorizontalDirection,
    ) {
        let mut player = player;
        let mut position = {
            let player_ref = player.bind();
            player_ref.base().get_global_position()
        };

        match direction {
            HorizontalDirection::Right => {
                position.x -= ROOM_WIDTH;
            }
            HorizontalDirection::Left => {
                position.x += ROOM_WIDTH;
            }
        }

        let player_node: Gd<Player> = player.clone();
        if let Some(mut parent) = player.get_parent() {
            parent.remove_child(&player_node);
        }

        let Some(mut new_room) = self.spawn_room(target_room) else {
            self.base_mut().add_child(&player_node);
            self.player = Some(player);
            return;
        };

        new_room.add_child(&player_node);
        player.set_global_position(position);
        self.player = Some(player);

        godot_print!("Transitioned to room {} via {:?}", target_room, direction);
    }
}
