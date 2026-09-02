pub mod core;
pub mod engine;
pub mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub mod cli;
#[cfg(not(target_arch = "wasm32"))]
pub mod lsp;
#[cfg(not(target_arch = "wasm32"))]
pub mod tui;
#[cfg(not(target_arch = "wasm32"))]
pub mod watcher;
