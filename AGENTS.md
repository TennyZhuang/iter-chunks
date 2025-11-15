# Repository Guidelines

## Project Structure & Module Organization
The crate is small: `src/lib.rs` contains the `IterChunks` trait, the `Chunks` adapter, and inline `#[cfg(test)]` suites. Workspace metadata and feature flags live in `Cargo.toml`, while `rustfmt.toml` pins formatting behavior. There are no asset folders; documentation belongs in `README.md` and rustdoc comments directly beside the code they explain.

## Build, Test, and Development Commands
- `cargo check` verifies the library compiles quickly against Rust 1.85+.
- `cargo test` (or `cargo test --lib`) runs the lending-iterator scenarios in `src/lib.rs` and must be clean before every push.
- `cargo clippy --no-deps` enforces lint hygiene; fix or allow lints explicitly when behavior is intentional.
- `cargo fmt` applies the repo's formatting rules; the CI equivalent is `cargo fmt -- --check`.
- `cargo doc --no-deps --open` builds local API docs so you can verify examples and public comments render correctly.

## Coding Style & Naming Conventions
Follow standard Rust 2024 defaults: four-space indentation, `snake_case` for items, and `UpperCamelCase` for types and traits. Keep implementations small and prefer helper functions over long blocks. Run `cargo fmt` so doc comments stay wrapped and normalized per `rustfmt.toml`. All new APIs need rustdoc examples (see `IterChunks::chunks`) that compile via doctests.

## Testing Guidelines
Unit tests live next to the code that they cover; create additional `#[cfg(test)] mod tests` blocks if new modules appear. Name tests after the behavior under examination (e.g., `test_chunks_resumable`). Every change should extend the happy-path chunks loop test plus edge cases such as zero-length slices or resumable iterators. Use `cargo test -- --nocapture` when debugging to see assertion context. Favor concise fixtures and leverage iterators or generators over manual vectors for clarity.

## Commit & Pull Request Guidelines
Recent history mixes Conventional Commits (`feat: implement size_hint`) with release labels (`release: v0.2.2`, `Release 0.3.0`); stay consistent by using imperative summaries under 60 characters, preferring `feat:`, `fix:`, or `docs:` prefixes. Reference issue numbers or PR IDs in the body when applicable. Pull requests should include: a problem statement, a short testing summary (`cargo test`, `cargo clippy`), and screenshots or doc preview links if the change touches documentation. Keep diffs focused and call out any follow-up work so reviewers can prioritize accordingly.
