# Repository Guidelines

## Project Structure & Module Organization
- `src/main.rs`: CLI entrypoint and argument parsing.
- `src/printentry.rs`: output formatting and file entry rendering logic.
- `src/printentry/colors.rs`: terminal color helpers.
- `target/`: Cargo build artifacts (generated; do not edit).
- Root files: `Cargo.toml` (dependencies and package metadata), `README.md` (project overview).

Keep new features in focused modules under `src/` and keep CLI wiring in `main.rs` minimal.

## Build, Test, and Development Commands
- `cargo run -- --help`: run the CLI locally and show available flags.
- `cargo build`: compile debug build.
- `cargo build --release`: compile optimized binary.
- `cargo test`: run unit/integration tests.
- `cargo fmt`: format code with rustfmt.
- `cargo clippy -- -D warnings`: lint strictly and fail on warnings.

Run `cargo fmt` and `cargo clippy -- -D warnings` before opening a PR.

## Coding Style & Naming Conventions
- Follow Rust 2021 defaults and rustfmt output (4-space indentation, trailing commas where formatter adds them).
- Use `snake_case` for functions/modules/files, `PascalCase` for types/structs/enums, and `SCREAMING_SNAKE_CASE` for constants.
- Prefer small, single-purpose functions and explicit error handling (`Result` over panics in normal flow).
- Keep terminal output logic in `printentry`-related modules, not scattered across CLI parsing.

## Testing Guidelines
- Add tests alongside code (`#[cfg(test)] mod tests`) for unit behavior.
- Use integration tests under `tests/` for CLI-facing behavior as coverage grows.
- Name tests by behavior, e.g. `prints_empty_for_hidden_only_dir`.
- At minimum, run `cargo test` locally before committing.

## Commit & Pull Request Guidelines
- Match existing commit style: short, imperative summaries (examples in history: `Fixed symlink following`, `Unified output under one stdout instance`).
- Keep subject lines concise and focused on one change.
- PRs should include:
  - what changed and why,
  - notable CLI/output changes (example command + output snippet),
  - linked issue(s) when applicable.
- Prefer small, reviewable PRs over large mixed changes.

## Security & Configuration Tips
- Avoid following untrusted symlinks or assuming filesystem metadata is valid.
- Preserve cross-platform behavior (Windows/macOS/Linux) when adding path or terminal logic.
