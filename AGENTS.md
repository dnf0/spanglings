# Spanglings Agent Guidance

## Core Architecture & Guidelines
- **Dual-Layer Explanations**: Every concept, frame, showdown duel, and specialized drill engine MUST provide both an intuitive communicative mental model (`💡 Meaning / Context:`) and a structural rule (`📐 Grammar Rule:`).
- **Full UI Lifecycle Audit**: When adding or modifying interactive formatting or learning explanations, audit ALL interaction surfaces across the full lifecycle:
  1. Pre-session cheat sheets and topic explainers (`spanglings explain <topic>`).
  2. Live turn-by-turn prompts (both `✓ CORRECT!` and `✗ INCORRECT!` feedback branches).
  3. Interactive in-session hints (`?` / `hint`).
  4. Interactive TUI modals, review dialogs, and recap screens.
  5. End-of-session summary reports (CLI & JSON serialization).

## Release & Publishing Protocol
1. **Verify Remote Registries Before Bumping**: Always query remote package registries (e.g. `cargo search spanglings`, crates.io API, VS Code Marketplace) and remote git tags (`git ls-remote --tags origin`) before deciding on the next release version number. Never assume or guess version numbers.
2. **Version Manifest Alignment**: Synchronize version bumps across `Cargo.toml`, `Cargo.lock`, `editors/vscode/package.json`, and `CHANGELOG.md`.
3. **Automated Publishing Watch**: Trigger releases via semantic tags (`vX.Y.Z`) and watch the GitHub Actions workflow until all 5 platform builds, crates.io publishing, and VS Code Marketplace publishing succeed.
4. **Local Installation Transparency**: If validating a newly published version by running `cargo install spanglings --force` locally, explicitly notify the user that their local environment binary has been updated.
