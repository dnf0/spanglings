pub mod core;
pub mod engine;
pub mod wasm;

pub use core::reference;
pub use engine as evaluator;

#[cfg(not(target_arch = "wasm32"))]
pub mod cli;
#[cfg(not(target_arch = "wasm32"))]
pub mod lsp;
