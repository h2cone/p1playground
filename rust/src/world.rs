use godot::{
    classes::{Node, Node2D},
    obj::InstanceId,
    prelude::*,
};

use crate::level_portal::LevelPortal;

const LEVELS_LEN: usize = 3;
const PORTAL_COOLDOWN_FRAMES: i32 = 6;
const PLAYER_SCENE_PATH: &str = "res://player.tscn";
const INITIAL_PLAYER_POS: (f32, f32) = (8.0, 8.0);

struct PortalCooldown {
    portal_id: InstanceId,
    frames_left: i32,
}

impl PortalCooldown {
    fn new(portal_id: InstanceId, frames_left: i32) -> Self {
        Self {
            portal_id,
            frames_left,
        }
    }

    fn tick(&mut self) -> bool {
        if self.frames_left > 0 {
            self.frames_left -= 1;
        }
        self.frames_left <= 0
    }

    fn matches(&self, portal_id: InstanceId) -> bool {
        self.portal_id == portal_id
    }
}

struct PendingTransfer {
    portal_id: InstanceId,
    target_level: i64,
    spawn_point: NodePath,
    offset: Vector2,
    player: Gd<Node2D>,
}

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct World {
    base: Base<Node2D>,
    levels: [Option<Gd<PackedScene>>; LEVELS_LEN],
    current_level: Option<Gd<Node2D>>,
    level_no: usize,
    arrival_cooldown: Option<PortalCooldown>,
    player: Option<Gd<Node2D>>,
    pending_transfer: Option<PendingTransfer>,
}

#[godot_api]
impl INode2D for World {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            levels: std::array::from_fn(|_| None),
            current_level: None,
            level_no: 0,
            arrival_cooldown: None,
            player: None,
            pending_transfer: None,
        }
    }

    fn ready(&mut self) {
        for i in 1..LEVELS_LEN {
            let path = format!("res://level_{}.tscn", i);
            self.levels[i] = Some(try_load::<PackedScene>(&path).expect("failed to load scene"));
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
        if let Some(cooldown) = self.arrival_cooldown.as_mut() {
            if cooldown.tick() {
                self.arrival_cooldown = None;
            }
        }

        if let Some(pending) = self.pending_transfer.take() {
            self.execute_pending_transfer(pending);
        }
    }
}

#[godot_api]
impl World {
    #[func]
    fn queue_portal_transfer(
        &mut self,
        portal_id: InstanceId,
        target_level: i64,
        spawn_point: NodePath,
        offset: Vector2,
        player: Gd<Node2D>,
    ) {
        if self.should_ignore_portal(portal_id) {
            return;
        }

        self.player = Some(player.clone());
        self.pending_transfer = Some(PendingTransfer {
            portal_id,
            target_level,
            spawn_point,
            offset,
            player,
        });
    }
}

impl World {
    fn execute_pending_transfer(&mut self, pending: PendingTransfer) {
        let PendingTransfer {
            portal_id,
            target_level,
            spawn_point,
            offset,
            player,
        } = pending;

        self.request_portal_transfer(portal_id, target_level, spawn_point, offset, player);
    }

    fn request_portal_transfer(
        &mut self,
        portal_id: InstanceId,
        target_level: i64,
        spawn_point: NodePath,
        offset: Vector2,
        mut player: Gd<Node2D>,
    ) {
        if self.should_ignore_portal(portal_id) {
            return;
        }

        if target_level <= 0 {
            godot_warn!(
                "Ignoring portal transfer with non-positive target level {}",
                target_level
            );
            return;
        }

        let target_level = target_level as usize;
        if target_level >= LEVELS_LEN {
            godot_error!("Portal target level {} is out of range", target_level);
            return;
        }

        if self.level_no == target_level {
            return;
        }

        let player_node: Gd<Node> = player.clone().upcast();

        if let Some(parent) = player.get_parent() {
            let mut parent = parent;
            parent.remove_child(&player_node);
        }

        if let Some(mut old_level) = self.current_level.take() {
            let old_level_node = old_level.clone().upcast::<Node>();
            self.base_mut().remove_child(&old_level_node);
            old_level.queue_free();
        }

        let Some(new_level) = self.spawn_level(target_level) else {
            godot_error!("Failed to spawn target level {}", target_level);
            self.base_mut().add_child(&player_node);
            return;
        };

        let (spawn_anchor, arrival_portal) = Self::resolve_spawn_targets(&new_level, &spawn_point);

        let anchor_position = spawn_anchor
            .as_ref()
            .map(|anchor| anchor.get_global_position())
            .unwrap_or_else(|| new_level.get_global_position());

        {
            let mut new_level_node = new_level.clone().upcast::<Node>();
            new_level_node.add_child(&player_node);
        }

        player.set_global_position(anchor_position + offset);
        self.player = Some(player.clone());

        if let Some(portal) = arrival_portal {
            self.arrival_cooldown = Some(PortalCooldown::new(
                portal.instance_id(),
                PORTAL_COOLDOWN_FRAMES,
            ));
        } else {
            self.arrival_cooldown = None;
        }
    }

    fn should_ignore_portal(&self, portal_id: InstanceId) -> bool {
        self.arrival_cooldown
            .as_ref()
            .map(|cooldown| cooldown.matches(portal_id))
            .unwrap_or(false)
    }

    fn spawn_level(&mut self, level_no: usize) -> Option<Gd<Node2D>> {
        let scene = self.levels[level_no].as_ref()?;
        let instance = scene.instantiate().expect("failed to instantiate");
        let level = instance
            .try_cast::<Node2D>()
            .expect("level root must inherit Node2D");

        let level_node = level.clone().upcast::<Node>();
        self.base_mut().add_child(&level_node);
        self.current_level = Some(level.clone());
        self.level_no = level_no;
        Some(level)
    }

    fn resolve_spawn_targets(
        level: &Gd<Node2D>,
        spawn_point: &NodePath,
    ) -> (Option<Gd<Node2D>>, Option<Gd<LevelPortal>>) {
        let mut level_node = level.clone().upcast::<Node>();

        if !spawn_point.is_empty() {
            let path_text = spawn_point.to_string();
            let path_variant = spawn_point.to_variant();
            let args = [path_variant.clone()];
            let node_variant = level_node.call("get_node_or_null", &args);

            if !node_variant.is_nil() {
                if let Ok(node) = node_variant.try_to::<Gd<Node>>() {
                    if let Ok(portal) = node.clone().try_cast::<LevelPortal>() {
                        let portal_node: Gd<Node2D> = portal.clone().upcast::<Node2D>();
                        return (Some(portal_node), Some(portal));
                    }

                    if let Ok(node2d) = node.try_cast::<Node2D>() {
                        return (Some(node2d), None);
                    }
                }
            } else {
                godot_warn!(
                    "Spawn point {} not found in level {}",
                    path_text,
                    level.get_name()
                );
            }
        }

        if let Some(portal) = Self::find_portal_recursive(&level_node) {
            let portal_node: Gd<Node2D> = portal.clone().upcast::<Node2D>();
            return (Some(portal_node), Some(portal));
        }

        (Some(level.clone()), None)
    }

    fn find_portal_recursive(node: &Gd<Node>) -> Option<Gd<LevelPortal>> {
        if let Ok(portal) = node.clone().try_cast::<LevelPortal>() {
            return Some(portal);
        }

        let child_count = node.get_child_count();
        for idx in 0..child_count {
            if let Some(child) = node.get_child(idx) {
                if let Some(portal) = Self::find_portal_recursive(&child) {
                    return Some(portal);
                }
            }
        }

        None
    }
}
