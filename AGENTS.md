# Repository Guidelines
## Project Structure & Module Organization
- `godot/` hosts the Godot project; keep each `.tscn` with its tileset, textures, and `.import` metadata so asset hashes stay stable.
- `rust/` stores the GDExtension crate; mirror the scene graph in `rust/src/` (example: `world.rs`, `player.rs`, `level.rs`) and register modules via `lib.rs`.
- Place gameplay-ready assets under `godot/`; co-locate helper scripts or shaders with the nodes that load them to simplify scene maintenance.

## Build, Test, and Development Commands
- `cargo build --manifest-path rust/Cargo.toml` compiles the extension DLL before launching Godot.
- `cargo test --manifest-path rust/Cargo.toml` runs unit and integration suites for the Rust side.
- `pwsh ./run.ps1 restart [--debug-collisions]` rebuilds the DLL and relaunches the editor; add the flag to visualize collision gizmos.
- `godot --headless --path godot` validates scenes and scripts in CI-safe mode.

## Coding Style & Naming Conventions
- Use four-space indentation in both Rust and Godot script files; keep files ASCII unless existing content says otherwise.
- Prefer `godot::prelude::*` imports, Snake_case filenames, CamelCase Rust types, PascalCase node names, and snake_case exported properties.
- Run `cargo fmt` and `cargo clippy --manifest-path rust/Cargo.toml --all-targets` before every commit.
- When creating new assets, match existing directory casing and node naming to maintain GDNative bindings.

## Testing Guidelines
- Favor inline `#[cfg(test)]` cases for engine-adjacent logic; store broader scenarios in `rust/tests/`.
- Execute `cargo test --manifest-path rust/Cargo.toml` prior to any push and rerun after touching gameplay-critical code.
- Document manual passes from `pwsh ./run.ps1 restart --debug-collisions`, especially around portal or trigger behavior.
- Keep test names descriptive (`test_portal_transfer_handles_spawn_marker`) and mirror scene element names.

## Commit & Pull Request Guidelines
- Write imperative, ≤72-character commit subjects (example: `Add portal overlap logging`); include bodies only when context would otherwise be lost.
- Reference new assets or external dependencies explicitly so reviewers can reproduce the setup.
- In PRs, describe gameplay impact, link issues or tasks, and outline validation steps; add screenshots or GIFs for visual changes.
- Mention when exported Rust fields or signals changed so reviewers reopen affected `.tscn` scenes to refresh bindings.

## Godot & Rust Bridge Tips
- Reopen scenes after adjusting exported fields or signals to refresh editor bindings.
- Ensure portal-like nodes use the correct collision layers and masks; rely on `godot_print!` for runtime diagnostics.
- Preload adjoining levels in `world.rs` and keep spawn markers inside each scene to avoid handoff gaps.
- When debugging transfers, compare collision gizmo output with expected layers before adjusting code.
