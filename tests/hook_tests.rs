use std::fs;

#[test]
fn test_hook_helpers() {
    let temp_dir = tempfile::tempdir().expect("temp dir");
    let fake_git = temp_dir.path().join(".git").join("hooks");
    fs::create_dir_all(&fake_git).expect("create fake git hooks dir");

    let hook_file = fake_git.join("pre-commit");
    let script = "#!/usr/bin/env bash\n# --- SPANGLINGS HOOK START ---\nspanglings drill\n# --- SPANGLINGS HOOK END ---\n";
    fs::write(&hook_file, script).expect("write hook");

    let content = fs::read_to_string(&hook_file).expect("read hook");
    assert!(content.contains("SPANGLINGS HOOK START"));
    assert!(content.contains("spanglings drill"));
}
