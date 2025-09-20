# Repository Guidelines

## Project Structure & Module Organization
Godot content lives in godot/ (scenes, tilesets, .import metadata) while the Rust GDExtension crate sits in 
ust/. Gameplay logic mirrors the scene graph: world.rs loads level_*.tscn, player.rs drives the character, and level_portal.rs exposes LevelPortal triggers for scene swaps. Keep new Rust modules under 
ust/src/ and register them in lib.rs; place new scenes or textures alongside their .tscn/.tres files so imports stay co-located.

## Build, Test, and Development Commands
- cargo build --manifest-path rust/Cargo.toml — compile the extension DLL before launching Godot.
- pwsh ./run.ps1 restart [--debug-collisions] — rebuild Rust, relaunch the editor, and optionally show collision gizmos.
- pwsh ./run.ps1 start|stop [extra flags] — attach or detach the running editor without rebuilding.
- godot --headless --path godot — run CI-safe scene validation.

## Coding Style & Naming Conventions
Format with cargo fmt; four-space indents, snake_case files, and CamelCase types are the norm. Run cargo clippy --manifest-path rust/Cargo.toml --all-targets to catch unsafe GDNative usage. In Godot, name nodes in PascalCase, exported variables in snake_case, and keep textures beside their scenes to preserve import hashes.

## Testing Guidelines
Prefer inline unit tests with #[cfg(test)] for engine-adjacent logic and add broader coverage under 
ust/tests/. Execute cargo test --manifest-path rust/Cargo.toml before every PR. For gameplay changes, document a manual playtest using pwsh ./run.ps1 restart --debug-collisions and note observed behaviour (e.g., LevelPortal handoffs, physics edges) in the PR description.

## Commit & Pull Request Guidelines
Write short, imperative commit subjects ("Add portal overlap logging"), keeping them near 72 characters. Use bodies sparingly to capture rationale or follow-up tasks. PRs should summarise gameplay impact, link the tracked issue or task, list reproduction steps, and attach screenshots/gifs when visuals change. Call out new assets or external dependencies to simplify reviewer setup.

## Godot & Rust Bridge Notes
After tweaking exported Rust fields or signal signatures, reopen the affected .tscn to refresh extension metadata. Document new signals or groups directly in the scene tree. When adding triggers, verify collision layers/masks so LevelPortal nodes fire body_entered events; the Godot Output panel should show the Rust-side godot_print! diagnostics.
