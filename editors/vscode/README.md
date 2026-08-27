# Spanglings: Developer Spanish Learning (VS Code / Cursor Extension)

Official VS Code and Cursor extension for **[Spanglings](https://github.com/dnf0/spanglings)** — learn Spanish through compiler diagnostics, Language Server Protocol (LSP) feedback, and interactive exercises designed specifically for software engineers.

## Features

- **Real-Time Language Diagnostics**: As you edit exercises or Spanish markdown files, the built-in Spanglings LSP validates grammar, verb conjugations, and accent rules directly in your editor problems pane and inline squiggles.
- **Exercise Tree Explorer**: Browse tracks, view completion status, and jump straight to exercises from the Spanglings Activity Bar view.
- **Status Bar Progress & Streaks**: Monitor your current streak, SRS review status, and daily progress at a glance in the bottom status bar.
- **Command Palette Integration**:
  - `Spanglings: Open Next Exercise`: Quickly navigate to the next incomplete exercise.
  - `Spanglings: Conjugate Verb`: Look up verb conjugations in multiple tenses on the fly.
  - `Spanglings: Open Reference Browser`: Open Spanish grammar and technical terminology reference tables.
  - `Spanglings: Sync Progress / State`: Manually sync exercise state and refresh diagnostics.

## Requirements

- **Spanglings CLI**: Install the `spanglings` binary (e.g. via `cargo install spanglings` or from source) and ensure it is available in your `$PATH` or configured in extension settings.

## Extension Settings

This extension contributes the following settings:

- `spanglings.executablePath`: Path to the `spanglings` executable binary (default: `"spanglings"`).
- `spanglings.strictAccents`: Enforce strict accent mark checking during exercise compilation and LSP diagnostics (default: `false`).
- `spanglings.enableLsp`: Enable or disable the built-in Spanglings Language Server client (default: `true`).

## Development

```bash
# Install dependencies
npm install

# Compile the extension
npm run compile

# Watch for changes during development
npm run watch

# Package build
npm run vscode:prepublish
```

## License

Apache-2.0
