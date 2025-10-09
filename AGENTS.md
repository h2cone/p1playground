# Repository Guidelines

## Project Structure & Module Organization
- `godot/` contains the Godot editor project. Keep each scene (`*.tscn`) beside its textures, tilesets, and `.import` metadata so asset GUIDs remain stable. Add gameplay-ready resources here; bundle helper scripts or shaders near the nodes that consume them.
- `rust/` holds the GDExtension crate. Mirror the scene graph under `rust/src/` (`world.rs`, `player.rs`, `room.rs`, etc.) and expose new modules through `lib.rs`. Scene-specific logic should live beside its counterpart node in Rust to keep the bridge maintainable.
- Temporary build products live under `godot/.godot/`. Do not commit this folder; the editor recreates it.

## Build, Test, and Development Commands
- `cargo build --manifest-path rust/Cargo.toml` — compiles the native extension DLL; run before launching Godot.
- `cargo test --manifest-path rust/Cargo.toml` — executes unit and integration tests for the Rust crate.
- `cargo clippy --manifest-path rust/Cargo.toml --all-targets` — lints with repository defaults; required before any PR.
- `pwsh ./run.ps1 restart [--debug-collisions]` — rebuilds the DLL and relaunches the editor; add the flag to visualize collision layers.
- `godot --headless --path godot` — validates scenes/scripts in CI-safe mode.

## Coding Style & Naming Conventions
- Use four-space indentation in both Rust and GDScript. Stick to ASCII unless an existing file demonstrates otherwise.
- Prefer `godot::prelude::*` imports, snake_case filenames, CamelCase Rust types, PascalCase node names, and snake_case exported properties.
- Run `cargo fmt` after edits; do not hand-format. Check in `.gd` or `.tscn` files exactly as Godot writes them.

## Testing Guidelines
- Keep lightweight engine-adjacent tests inline with `#[cfg(test)]`; larger scenarios go in `rust/tests/`.
- Name tests after the gameplay behavior they protect (e.g., `test_transition_stops_horizontal_oscillation`).
- Re-run `cargo test` plus a manual pass of `pwsh ./run.ps1 restart --debug-collisions` whenever room transfer logic or collision layers change.

## Commit & Pull Request Guidelines
- Use imperative, ≤72-character commit subjects (e.g., `Guard room swap by movement direction`). Provide a short body if the context is not obvious.
- Reference new assets or external dependencies so reviewers can reproduce the setup.
- PRs should describe gameplay impact, link issues/tasks, list validation steps, and include screenshots or clips for visual adjustments. Mention when exported Rust fields or Godot signals change so reviewers reopen affected scenes.

## Godot & Rust Bridge Tips
- The world stores the player as `Gd<Player>`; call `player.bind().base()` when you need `CharacterBody2D` operations. Reparent with `player.clone().upcast::<Node2D>()`.
- Avoid oscillation at room edges: require directional input before triggering transfers and keep portals on correct collision layers. Use `godot_print!` generously during debugging.
