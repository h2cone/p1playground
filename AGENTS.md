# Repository Guidelines

## Project Structure & Module Organization
- godot/ contains the Godot project: each scene (*.tscn) keeps its tileset, textures, and .import metadata beside it so asset hashes remain stable.
- ust/ hosts the GDExtension crate. Modules in ust/src/ mirror the scene graph (world.rs, player.rs, level.rs) and register through lib.rs.
- Keep new gameplay assets under godot/ and place auxiliary scripts or helpers next to the nodes that consume them.

## Build, Test, and Development Commands
- cargo build --manifest-path rust/Cargo.toml — compiles the Rust extension DLL before launching Godot.
- cargo test --manifest-path rust/Cargo.toml — runs unit/integration tests for the extension.
- pwsh ./run.ps1 restart [--debug-collisions] — rebuilds the DLL and relaunches the editor, optionally toggling collision gizmos.
- godot --headless --path godot — validates scenes/scripts in CI-safe mode.

## Coding Style & Naming Conventions
- Use four-space indentation across Rust and Godot scripts; keep files ASCII unless existing content dictates otherwise.
- Favor godot::prelude::* imports; register new modules in lib.rs.
- Snake_case filenames, CamelCase types, PascalCase node names, and snake_case exported properties.
- Run cargo fmt and cargo clippy --manifest-path rust/Cargo.toml --all-targets before committing.

## Testing Guidelines
- Prefer inline #[cfg(test)] cases for engine-adjacent logic; place wider coverage in ust/tests/.
- Execute cargo test --manifest-path rust/Cargo.toml prior to any push.
- For gameplay changes, perform a manual pass via pwsh ./run.ps1 restart --debug-collisions and note observed portal/trigger behavior in PR notes.

## Commit & Pull Request Guidelines
- Commit subjects: imperative mood, ≤72 characters (e.g., Add portal overlap logging). Include a body only when the context would otherwise be lost.
- PRs should describe gameplay impact, link issues/tasks, outline reproduction or verification steps, and attach screenshots/GIFs for visual tweaks.
- Call out new assets or external dependencies so reviewers can reproduce the setup quickly.

## Godot & Rust Bridge Tips
- After editing exported Rust fields or signals, reopen the affected .tscn to refresh bindings.
- Ensure Portal-like nodes use correct collision layers/masks; rely on godot_print! output while debugging transfers.
- When adding new levels, preload adjacent scenes in world.rs and keep spawn markers inside the scene for consistent handoffs.
