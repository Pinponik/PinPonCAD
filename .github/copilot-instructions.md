# Copilot instructions for PinPonCAD

This file orients Copilot-powered assistants to work effectively in this repository.

## Build, test, and lint commands
- Build (debug):
  - cargo build
- Build (release):
  - cargo build --release
- Run:
  - cargo run
- Run tests (full suite):
  - cargo test
- Run a single test by name/pattern:
  - cargo test <pattern>
    - Example: cargo test my_test_name
- Formatting / linting:
  - Format: cargo fmt --all
  - Lint (Clippy): cargo clippy --all-targets --all-features -- -D warnings

Notes: cargo is the canonical entrypoint for building, running and testing. The project currently targets Rust edition 2024.

## High-level architecture
- Single binary crate named `PinPonCAD` (see Cargo.toml).
- Entry point: `src/main.rs` (small CLI-style program). For larger logic, prefer adding `src/lib.rs` and keep `main.rs` as an invocation shim.
- Dependencies: none declared in Cargo.toml at present; add crates in Cargo.toml.
- Build artifacts: `target/` contains compiled outputs and is expected to be ignored by git.

## Key repository conventions
- Source is under `src/`. New modules should follow standard Rust layout (module files or `mod` declarations in `lib.rs` / `main.rs`).
- Tests: place unit tests inside modules with `#[cfg(test)]` or as integration tests under `tests/`.
- CI / automation: none detected in repository. If adding workflows, place GitHub Actions under `.github/workflows/`.
- Edition: crate uses `edition = "2024"` in Cargo.toml — prefer modern Rust idioms and toolchain compatible with that edition.

## Files of interest (quick pointers)
- Cargo.toml — crate metadata and dependencies
- src/main.rs — program entrypoint
- README.md — project title and landing doc


---
If you'd like changes (add CI, recommended lints, or expand architecture notes), tell me which area to expand.