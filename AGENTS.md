# Repository Guidelines

## Project Structure & Module Organization
- Root: `Cargo.toml` (package, deps), `README.md`, `AGENTS.md`.
- Source: `src/main.rs` (binary) or `src/lib.rs` (library). Add modules as `src/<module>.rs` or `src/<module>/mod.rs` for nested trees.
- Tests: unit tests live next to code (`#[cfg(test)]`); integration tests in `tests/` as separate files.
- Examples: `examples/` for small runnable demos via `cargo run --example <name>`.
- Artifacts: `target/` is generated and ignored.
- Not initialized yet? Run `cargo new --bin .` to scaffold.

## Build, Test, and Development Commands
- Build: `cargo build` (debug) | `cargo build --release` (optimized).
- Run: `cargo run` (uses current binary crate).
- Test: `cargo test` (all tests) | `cargo test -- --nocapture` to see output.
- Format: `cargo fmt` (rustfmt) — required before pushing.
- Lint: `cargo clippy -D warnings` to lint and fail on warnings.
- Docs: `cargo doc --open` to build and open API docs.

## Coding Style & Naming Conventions
- Formatting: rustfmt defaults; 4 spaces, no tabs, 100–120 col soft limit.
- Naming: `snake_case` for files/functions, `CamelCase` for types/traits, `SCREAMING_SNAKE_CASE` for consts.
- Visibility: keep items private; `pub` only when needed. Prefer slices (`&[T]`, `&str`) in APIs.
- Errors: return `Result<_, _>`; bubble with `?`. For apps, `anyhow` is acceptable; for libs, prefer typed errors.
- Docs: use `///` for public items; include examples if practical.

## Testing Guidelines
- Unit tests: colocate with code; name tests `fn <subject>_works()`.
- Integration: place in `tests/`; test only public API.
- Coverage (optional): `cargo llvm-cov` or `tarpaulin` if configured.
- CI expectation: tests pass, no clippy warnings, formatted code.

## Commit & Pull Request Guidelines
- Commits: use Conventional Commits — e.g., `feat: add parser`, `fix: handle utf8`.
- Scope: small, focused diffs; include tests and doc updates.
- PRs: clear description, rationale, steps to reproduce/verify, linked issues.
- Check locally before opening: build, test, fmt, clippy clean.

## Security & Configuration
- Do not commit secrets. Use env vars or `.env` (git-ignored).
- Keep dependencies minimal and up to date; prefer stable editions (Rust 2021+).
