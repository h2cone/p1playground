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
  - Manages level loading/unloading with edge-based transitions
  - Preloads adjacent levels (left/right neighbors) and discards distant ones
  - Handles horizontal player transfers between levels when player crosses LEVEL_WIDTH boundaries
  - Uses threshold-based transitions (`PLAYER_CROSS_THRESHOLD` = 50% of player width must cross)
  - Maintains single active level in scene tree, with 1-2 preloaded but hidden neighbors

- **`player.rs`** (`Player` node): CharacterBody2D with movement and animation
  - State machine: `Air` (falling) and `Floor` (can walk)
  - Horizontal movement controlled via `ui_left`/`ui_right` input actions
  - Sprite direction flips based on velocity sign
  - Animation states: `idle`, `walk`

- **`level.rs`** (`Level` node): Minimal Node2D container for tilemap/collision scenes

- **`game.rs`** (`Game` node): Top-level Node, currently just a skeleton

- **`lib.rs`**: Registration point for all modules via `#[gdextension]`

### Level Transition System

Levels are defined by their index (1-based). The `LEVEL_NEIGHBORS` array in `world.rs` defines the connectivity graph (which levels are to the left/right of each level). When the player crosses a level boundary:

1. `check_horizontal_transitions()` detects when 50%+ of player width crosses `LEVEL_WIDTH`
2. `transfer_player()` reparents the player to the target level and adjusts position
3. `spawn_level()` removes the old level from the scene tree and adds the new one
4. `preload_adjacent_levels()` instantiates neighboring levels in the background

### Key Constants (world.rs)

- `LEVELS_LEN`: Total number of levels (currently 3)
- `LEVEL_WIDTH`: 480.0 (screen width in pixels)
- `PLAYER_COLLISION_WIDTH`: 16.0
- `PLAYER_CROSS_THRESHOLD`: 0.50 (50% of player must cross before transition)

## Godot Integration Notes

- After modifying exported Rust properties or signals, reopen the `.tscn` file in Godot to refresh bindings
- Portal/collision nodes must use correct collision layers/masks
- Use `godot_print!()` for runtime diagnostics (appears in Godot console)
- Level scenes are loaded from `res://level_N.tscn` where N is the level index
- Player scene path: `res://player.tscn`

## Code Style

- Four-space indentation (Rust and Godot scripts)
- Prefer `godot::prelude::*` imports
- Snake_case filenames, CamelCase Rust types, PascalCase node names, snake_case exported properties
- Always run `cargo fmt` and `cargo clippy` before committing
