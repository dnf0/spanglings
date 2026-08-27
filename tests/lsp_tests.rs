use spanglings::lsp::LspServer;

#[test]
fn test_lsp_initialize_and_capabilities() {
    let mut server = LspServer::new();
    let init_req = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"capabilities":{}}}"#;
    let resp = server
        .handle_raw_message(init_req)
        .expect("should handle initialize");

    assert!(resp.contains(r#""hoverProvider":true"#));
    assert!(resp.contains(r#""completionProvider""#));
    assert!(resp.contains(r#""codeActionProvider":true"#));
    assert!(resp.contains("spanglings-lsp"));
}

#[test]
fn test_lsp_hover_grammar_card_and_conjugation() {
    let mut server = LspServer::new();
    let doc_uri = "file:///exercises/01_subjunctive.md";
    let doc_content = r#"---
id: b1_subj_01
title: Subjunctive Test
topic: subjunctive
level: B1
---
<!-- I AM NOT DONE -->
### Context
English: I want you to have it.
### Exercise
Espero que puedas haber llegado
"#;

    server
        .documents
        .insert(doc_uri.to_string(), doc_content.to_string());

    // Hover on 'subjunctive' (line 3, char 8)
    let hover_req = format!(
        r#"{{"jsonrpc":"2.0","id":2,"method":"textDocument/hover","params":{{"textDocument":{{"uri":"{}"}},"position":{{"line":3,"character":8}}}}}}"#,
        doc_uri
    );
    let hover_resp = server
        .handle_raw_message(&hover_req)
        .expect("should handle hover");
    assert!(hover_resp.contains("Spanglings Grammar Reference"));

    // Hover on 'haber' (line 10, char 22)
    let hover_req_verb = format!(
        r#"{{"jsonrpc":"2.0","id":3,"method":"textDocument/hover","params":{{"textDocument":{{"uri":"{}"}},"position":{{"line":10,"character":22}}}}}}"#,
        doc_uri
    );
    let hover_verb_resp = server
        .handle_raw_message(&hover_req_verb)
        .expect("should handle verb hover");
    assert!(hover_verb_resp.contains("he") || hover_verb_resp.contains("Present:"));
}

#[test]
fn test_lsp_live_diagnostics_and_did_open() {
    let mut server = LspServer::new();
    let doc_uri = "file:///exercises/01_subjunctive.md";
    let doc_content = r#"---
id: b1_subj_01
title: Subjunctive Test
topic: subjunctive
level: B1
type: cloze
---
<!-- I AM NOT DONE -->
### Context
English: I want you to come.
<!-- SOLUTION: vengas -->
### Exercise
Quiero que tú vienes
"#;

    let did_open = format!(
        r#"{{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{{"textDocument":{{"uri":"{}","languageId":"markdown","version":1,"text":{}}}}}}}"#,
        doc_uri,
        serde_json::to_string(doc_content).unwrap()
    );

    let notif_resp = server
        .handle_raw_message(&did_open)
        .expect("should emit publishDiagnostics");
    assert!(notif_resp.contains("textDocument/publishDiagnostics"));
    assert!(notif_resp.contains("INFO01") || notif_resp.contains("in-progress"));
}

#[test]
fn test_lsp_code_action_quickfix_done_marker() {
    let mut server = LspServer::new();
    let doc_uri = "file:///exercises/01_subjunctive.md";
    let doc_content = "---\nid: test\n---\n<!-- I AM NOT DONE -->\nQuiero que vengas";
    server
        .documents
        .insert(doc_uri.to_string(), doc_content.to_string());

    let code_action_req = format!(
        r#"{{"jsonrpc":"2.0","id":4,"method":"textDocument/codeAction","params":{{"textDocument":{{"uri":"{}"}},"range":{{"start":{{"line":2,"character":0}},"end":{{"line":2,"character":20}}}},"context":{{"diagnostics":[]}}}}}}"#,
        doc_uri
    );
    let resp = server
        .handle_raw_message(&code_action_req)
        .expect("should return code actions");
    assert!(resp.contains("Mark exercise as done"));
    assert!(resp.contains("quickfix"));
}

#[test]
fn test_lsp_completions() {
    let mut server = LspServer::new();
    let req = r#"{"jsonrpc":"2.0","id":5,"method":"textDocument/completion","params":{"textDocument":{"uri":"file:///test.md"},"position":{"line":0,"character":0}}}"#;
    let resp = server
        .handle_raw_message(req)
        .expect("should handle completion");
    assert!(resp.contains("desplegar"));
    assert!(resp.contains("conmutación por error"));
}

#[test]
fn test_lsp_shutdown() {
    let mut server = LspServer::new();
    let req = r#"{"jsonrpc":"2.0","id":6,"method":"shutdown"}"#;
    let resp = server
        .handle_raw_message(req)
        .expect("should handle shutdown");
    assert!(resp.contains(r#""result":null"#));
    assert!(server.is_shutdown);
}
