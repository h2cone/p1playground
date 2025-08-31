# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Godot game project using Rust for game logic through the godot-rust (gdext) bindings. The project structure separates the Godot project files from the Rust codebase.

## Architecture

- **Godot Project** (`./godot/`): Contains the main game scenes and assets
  - `game.tscn`: Main game scene with Game node (Rust-based) and World node (Rust-based)
- **Rust Extension** (`./rust/`): Contains the Rust code compiled as a Godot extension
  - `lib.rs`: Extension entry point and registration
  - `game.rs`: Game node implementation (inherits from Node)
  - `world.rs`: World node implementation (inherits from Node2D)

## Development Commands

### Build and Run
```bash
# Build Rust extension and start Godot
./run.ps1 restart

# Just start Godot (without building)
./run.ps1 start

# Stop running Godot process
./run.ps1 stop

# Manual build (from rust directory)
cd rust && cargo build
```

### Core Development
```bash
# Build Rust extension
cargo build --manifest-path ./rust/Cargo.toml

# Run Godot project
godot --path ./godot

# Run Godot editor
godot --path ./godot --editor
```

## Project Structure

```
├── godot/                 # Godot project directory
│   ├── game.tscn         # Main game scene
│   └── (other Godot assets)
├── rust/                 # Rust extension source
│   ├── src/
│   │   ├── lib.rs        # Extension entry point
│   │   ├── game.rs       # Game node logic
│   │   └── world.rs      # World node logic
│   └── Cargo.toml        # Rust dependencies
└── run.ps1              # PowerShell build/run script
```

## Technology Stack

- **Godot Engine**: Game engine and scene management
- **Rust**: Game logic via godot-rust bindings
- **gdext**: Official Godot Rust bindings (git dependency)

## Key Files

- `rust/src/lib.rs`: Extension registration point
- `rust/src/game.rs`: Main game node (Node-based)
- `rust/src/world.rs`: 2D world node (Node2D-based)
- `godot/game.tscn`: Main scene containing both nodes
- `run.ps1`: PowerShell script for build/run automation