pub mod diagnostics;
pub mod hover;
pub mod protocol;
pub mod server;

pub use server::LspServer;

pub fn start_lsp_server(strict_accents: bool) -> anyhow::Result<()> {
    let mut server = LspServer::new();
    server.strict_accents = strict_accents;
    server.run_stdio_loop()
}
