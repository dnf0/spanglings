use crate::lsp::diagnostics::compute_diagnostics;
use crate::lsp::hover::compute_hover;
use crate::lsp::protocol::{
    CodeAction, CompletionItem, Diagnostic, JsonRpcRequest, JsonRpcResponse, Position, Range,
    TextEdit, WorkspaceEdit,
};
use anyhow::Result;
use std::collections::HashMap;
use std::io::{self, BufRead, Read, Write};

pub struct LspServer {
    pub documents: HashMap<String, String>,
    pub strict_accents: bool,
    pub is_shutdown: bool,
}

impl Default for LspServer {
    fn default() -> Self {
        Self::new()
    }
}

impl LspServer {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
            strict_accents: false,
            is_shutdown: false,
        }
    }

    pub fn validate_document(&self, uri: &str, content: &str) -> Vec<Diagnostic> {
        compute_diagnostics(uri, content, self.strict_accents)
    }

    pub fn handle_raw_message(&mut self, raw_json: &str) -> Option<String> {
        let req: JsonRpcRequest = match serde_json::from_str(raw_json) {
            Ok(r) => r,
            Err(_) => return None,
        };

        match req.method.as_str() {
            "initialize" => {
                let capabilities = serde_json::json!({
                    "capabilities": {
                        "textDocumentSync": 1, // Full sync
                        "hoverProvider": true,
                        "completionProvider": {
                            "triggerCharacters": [" ", "_", "@", "-"]
                        },
                        "codeActionProvider": true
                    },
                    "serverInfo": {
                        "name": "spanglings-lsp",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                });
                let resp = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id,
                    result: Some(capabilities),
                    error: None,
                };
                serde_json::to_string(&resp).ok()
            }
            "initialized" => None,
            "shutdown" => {
                self.is_shutdown = true;
                let resp = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id,
                    result: Some(serde_json::Value::Null),
                    error: None,
                };
                serde_json::to_string(&resp).ok()
            }
            "textDocument/didOpen" => {
                if let Some(params) = req.params {
                    if let (Some(uri), Some(text)) = (
                        params
                            .get("textDocument")
                            .and_then(|t| t.get("uri"))
                            .and_then(|u| u.as_str()),
                        params
                            .get("textDocument")
                            .and_then(|t| t.get("text"))
                            .and_then(|t| t.as_str()),
                    ) {
                        self.documents.insert(uri.to_string(), text.to_string());
                        let diags = self.validate_document(uri, text);
                        return Some(format_publish_diagnostics(uri, &diags));
                    }
                }
                None
            }
            "textDocument/didChange" => {
                if let Some(params) = req.params {
                    if let Some(uri) = params
                        .get("textDocument")
                        .and_then(|t| t.get("uri"))
                        .and_then(|u| u.as_str())
                    {
                        if let Some(changes) =
                            params.get("contentChanges").and_then(|c| c.as_array())
                        {
                            if let Some(last_change) = changes
                                .last()
                                .and_then(|c| c.get("text"))
                                .and_then(|t| t.as_str())
                            {
                                self.documents
                                    .insert(uri.to_string(), last_change.to_string());
                                let diags = self.validate_document(uri, last_change);
                                return Some(format_publish_diagnostics(uri, &diags));
                            }
                        }
                    }
                }
                None
            }
            "textDocument/hover" => {
                let hover_result = (|| {
                    let params = req.params.as_ref()?;
                    let uri = params.get("textDocument")?.get("uri")?.as_str()?;
                    let pos_json = params.get("position")?;
                    let pos: Position = serde_json::from_value(pos_json.clone()).ok()?;
                    let content = self.documents.get(uri)?;
                    compute_hover(content, pos)
                })();

                let resp = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id,
                    result: Some(
                        serde_json::to_value(hover_result).unwrap_or(serde_json::Value::Null),
                    ),
                    error: None,
                };
                serde_json::to_string(&resp).ok()
            }
            "textDocument/completion" => {
                let completions = vec![
                    CompletionItem {
                        label: "desplegar".to_string(),
                        kind: Some(3), // Function / Keyword
                        detail: Some("To deploy (software)".to_string()),
                        documentation: Some(crate::lsp::protocol::MarkupContent {
                            kind: "markdown".to_string(),
                            value: "Authentic technical Spanish for software deployment."
                                .to_string(),
                        }),
                        insert_text: Some("desplegar".to_string()),
                    },
                    CompletionItem {
                        label: "conmutación por error".to_string(),
                        kind: Some(3),
                        detail: Some("Failover (distributed systems)".to_string()),
                        documentation: Some(crate::lsp::protocol::MarkupContent {
                            kind: "markdown".to_string(),
                            value: "High-register translation for system failover.".to_string(),
                        }),
                        insert_text: Some("conmutación por error".to_string()),
                    },
                ];

                let resp = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id,
                    result: Some(
                        serde_json::to_value(completions).unwrap_or(serde_json::Value::Null),
                    ),
                    error: None,
                };
                serde_json::to_string(&resp).ok()
            }
            "textDocument/codeAction" => {
                let actions = (|| {
                    let params = req.params.as_ref()?;
                    let uri = params.get("textDocument")?.get("uri")?.as_str()?;
                    let content = self.documents.get(uri)?;
                    let mut action_list = Vec::new();

                    // Code Action: Remove `<!-- I AM NOT DONE -->`
                    if let Some(line_idx) = content
                        .lines()
                        .position(|l| l.contains("<!-- I AM NOT DONE -->"))
                    {
                        let mut changes = HashMap::new();
                        changes.insert(
                            uri.to_string(),
                            vec![TextEdit {
                                range: Range {
                                    start: Position {
                                        line: line_idx as u32,
                                        character: 0,
                                    },
                                    end: Position {
                                        line: (line_idx + 1) as u32,
                                        character: 0,
                                    },
                                },
                                new_text: "".to_string(),
                            }],
                        );
                        action_list.push(CodeAction {
                            title: "Mark exercise as done (remove <!-- I AM NOT DONE -->)"
                                .to_string(),
                            kind: Some("quickfix".to_string()),
                            edit: Some(WorkspaceEdit {
                                changes: Some(changes),
                            }),
                        });
                    }

                    Some(action_list)
                })()
                .unwrap_or_default();

                let resp = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id,
                    result: Some(serde_json::to_value(actions).unwrap_or(serde_json::Value::Null)),
                    error: None,
                };
                serde_json::to_string(&resp).ok()
            }
            _ => None,
        }
    }

    pub fn run_stdio_loop(&mut self) -> Result<()> {
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        let mut stdout = io::stdout();

        loop {
            if self.is_shutdown {
                break;
            }

            let mut header_line = String::new();
            let bytes_read = reader.read_line(&mut header_line)?;
            if bytes_read == 0 {
                break;
            }

            if let Some(stripped) = header_line.strip_prefix("Content-Length:") {
                let len_str = stripped.trim();
                let content_len: usize = match len_str.parse() {
                    Ok(l) => l,
                    Err(_) => continue,
                };

                // Read empty separator line
                let mut empty_line = String::new();
                reader.read_line(&mut empty_line)?;

                // Read payload body
                let mut body = vec![0u8; content_len];
                reader.read_exact(&mut body)?;

                let payload_str = String::from_utf8_lossy(&body);
                if let Some(response_json) = self.handle_raw_message(&payload_str) {
                    let formatted = format!(
                        "Content-Length: {}\r\n\r\n{}",
                        response_json.len(),
                        response_json
                    );
                    stdout.write_all(formatted.as_bytes())?;
                    stdout.flush()?;
                }
            }
        }

        Ok(())
    }
}

fn format_publish_diagnostics(uri: &str, diags: &[Diagnostic]) -> String {
    let notif = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/publishDiagnostics",
        "params": {
            "uri": uri,
            "diagnostics": diags
        }
    });
    serde_json::to_string(&notif).unwrap_or_default()
}
