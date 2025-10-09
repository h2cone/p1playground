# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a 2D platformer game built with Godot 4 and Rust (via gdext GDExtension). The Rust code provides gameplay logic, while Godot handles rendering and the editor workflow.

## Build and Development Commands

**Building the Rust extension:**
```
cargo build --manifest-path rust/Cargo.toml
cargo test --manifest-path rust/Cargo.toml
cargo fmt
cargo clippy --manifest-path rust/Cargo.toml --all-targets
```

**Running the project:**
```
pwsh ./run.ps1 restart [--debug-collisions]
```
This rebuilds the Rust DLL and relaunches the Godot editor. Add `--debug-collisions` to visualize collision gizmos.

**CI validation:**
```
godot --headless --path godot
```

## Architecture

### Scene Graph and Rust Modules

The Rust modules in `rust/src/` mirror the Godot scene hierarchy:

- **`world.rs`** (`World` node): Root gameplay coordinator
  - Manages room loading/unloading with edge-based transitions
  - Preloads adjacent rooms (left/right/up/down neighbors) and discards distant ones
  - Handles horizontal player transfers between rooms when player crosses ROOM_WIDTH boundaries
  - Uses threshold-based transitions (`PLAYER_CROSS_THRESHOLD` = 50% of player width must cross)
  - Checks velocity direction to prevent backtracking during transitions
  - Maintains single active room in scene tree, with adjacent preloaded but hidden neighbors

- **`player.rs`** (`Player` node): CharacterBody2D with movement and animation
  - State machine: `Air` (falling/jumping) and `Floor` (can walk/jump)
  - Horizontal movement controlled via `walk_left`/`walk_right` input actions
  - Jump controlled via `jump` input action with velocity -300
  - Sprite direction flips based on velocity sign
  - Animation states: `idle`, `walk`, `jump`, `fall`

- **`room.rs`** (`Room` node): Node2D container for tilemap/collision scenes
  - Exported property `debug_draw_bounds` toggles yellow outline visualization
  - Displays room boundaries (480x270) via Line2D when debug mode enabled

- **`room_graph.rs`**: Room connectivity graph
  - Defines neighbor relationships for each room (left/right/up/down)
  - Rooms named with grid coordinates: `room_X_Y` (e.g., `room_0_0`, `room_1_0`)
  - Currently defines 4 rooms in a horizontal chain: `room_0_0` → `room_1_0` → `room_2_0`, plus `room_1_1`

- **`game.rs`** (`Game` node): Top-level Node, currently just a skeleton

- **`lib.rs`**: Registration point for all modules via `#[gdextension]`

### Room Transition System

Rooms are defined by grid coordinates in a naming pattern `room_X_Y`. The `RoomGraph` struct in `room_graph.rs` defines the connectivity graph (which rooms are adjacent). When the player crosses a room boundary:

1. `check_horizontal_transitions()` detects when 50%+ of player width crosses `ROOM_WIDTH` AND velocity is in transition direction
2. `transfer_player()` reparents the player to the target room and adjusts position by ±ROOM_WIDTH
3. `spawn_room()` removes the old room from the scene tree and adds the new one
4. `preload_adjacent_rooms()` instantiates neighboring rooms in the background (4-directional)
5. `discard_far_rooms()` cleans up rooms that are no longer adjacent

### Key Constants

**world.rs:**
- `ROOM_WIDTH`: 480.0 (screen width in pixels)
- `PLAYER_WIDTH`: 16.0 (collision width)
- `PLAYER_CROSS_THRESHOLD`: 0.50 (50% of player must cross before transition)
- `INITIAL_ROOM`: "room_1_0"
- `INITIAL_PLAYER_POS`: (8.0, 8.0)

**room.rs:**
- `ROOM_WIDTH`: 480.0
- `ROOM_HEIGHT`: 270.0

**player.rs:**
- `WALK_SPEED`: 120.0
- `ACCEL_SPEED`: 720.0 (6× walk speed)
- `JUMP_VELOCITY`: -300.0
- `MIN_WALK_SPEED`: 0.1 (threshold for walk animation)

## Godot Integration Notes

- After modifying exported Rust properties or signals, reopen the `.tscn` file in Godot to refresh bindings
- Use `godot_print!()` for runtime diagnostics (appears in Godot console)
- Room scenes are loaded from `res://room_X_Y.tscn` (e.g., `res://room_0_0.tscn`, `res://room_1_0.tscn`)
- Player scene path: `res://player.tscn`
- Tileset resource: `res://industrial.tres` (renamed from `room.tres`)
- Input actions used: `walk_left`, `walk_right`, `jump`

## Code Style

- Four-space indentation (Rust and Godot scripts)
- Prefer `godot::prelude::*` imports
- Snake_case filenames, CamelCase Rust types, PascalCase node names, snake_case exported properties
- Always run `cargo fmt` and `cargo clippy` before committing
