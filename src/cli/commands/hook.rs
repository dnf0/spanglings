use colored::Colorize;
use std::fs;
use std::path::PathBuf;

const HOOK_START_MARKER: &str = "# --- SPANGLINGS HOOK START ---";
const HOOK_END_MARKER: &str = "# --- SPANGLINGS HOOK END ---";

pub fn run_hook_install(hook_type: &str) -> anyhow::Result<()> {
    let hooks_dir = find_git_hooks_dir()?;
    let hook_file = hooks_dir.join(hook_type);

    let script_snippet = format!(
        "{}\n# Spanglings pre-commit micro-drill hook\nif command -v spanglings >/dev/null 2>&1; then\n    echo -e \"\\033[1;36m🇪🇸 Spanglings Pre-Commit Micro-Drill\\033[0m\"\n    spanglings drill\n    if [ $? -ne 0 ]; then\n        echo -e \"\\033[1;31mCommit cancelled. Complete the drill to proceed (or use git commit --no-verify).\\033[0m\"\n        exit 1\n    fi\nfi\n{}\n",
        HOOK_START_MARKER, HOOK_END_MARKER
    );

    let mut existing_content = if hook_file.exists() {
        fs::read_to_string(&hook_file)?
    } else {
        "#!/usr/bin/env bash\n\n".to_string()
    };

    if existing_content.contains(HOOK_START_MARKER) {
        // Strip existing marker and replace
        existing_content = remove_spanglings_block(&existing_content);
    }

    if !existing_content.starts_with("#!") {
        existing_content = format!("#!/usr/bin/env bash\n\n{}", existing_content);
    }

    let final_content = format!("{}\n{}", existing_content.trim_end(), script_snippet);
    fs::write(&hook_file, final_content)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook_file)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook_file, perms)?;
    }

    println!(
        "{} Installed Spanglings {} hook at: {}",
        "✓".green().bold(),
        hook_type.cyan().bold(),
        hook_file.display().to_string().bold()
    );
    println!("  Every time you run 'git commit', you'll get a rapid-fire Spanish flashcard!");
    Ok(())
}

pub fn run_hook_uninstall(hook_type: &str) -> anyhow::Result<()> {
    let hooks_dir = find_git_hooks_dir()?;
    let hook_file = hooks_dir.join(hook_type);

    if !hook_file.exists() {
        println!("{} No {} hook file found.", "•".yellow(), hook_type);
        return Ok(());
    }

    let content = fs::read_to_string(&hook_file)?;
    if !content.contains(HOOK_START_MARKER) {
        println!(
            "{} Spanglings hook was not active in {}.",
            "•".yellow(),
            hook_type
        );
        return Ok(());
    }

    let cleaned = remove_spanglings_block(&content);
    if cleaned.trim() == "#!/usr/bin/env bash" || cleaned.trim().is_empty() {
        let _ = fs::remove_file(&hook_file);
        println!(
            "{} Removed empty {} hook file.",
            "✓".green().bold(),
            hook_type
        );
    } else {
        fs::write(&hook_file, cleaned)?;
        println!(
            "{} Removed Spanglings block from {}.",
            "✓".green().bold(),
            hook_type
        );
    }
    Ok(())
}

fn remove_spanglings_block(content: &str) -> String {
    let mut result = Vec::new();
    let mut skipping = false;

    for line in content.lines() {
        if line.contains(HOOK_START_MARKER) {
            skipping = true;
            continue;
        }
        if line.contains(HOOK_END_MARKER) {
            skipping = false;
            continue;
        }
        if !skipping {
            result.push(line);
        }
    }

    result.join("\n")
}

fn find_git_hooks_dir() -> anyhow::Result<PathBuf> {
    let mut curr = std::env::current_dir()?;
    loop {
        let git_dir = curr.join(".git");
        if git_dir.is_dir() {
            let hooks = git_dir.join("hooks");
            if !hooks.exists() {
                fs::create_dir_all(&hooks)?;
            }
            return Ok(hooks);
        }
        if !curr.pop() {
            break;
        }
    }
    anyhow::bail!("Not in a git repository. Navigate to a git repo before installing hooks.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_spanglings_block() {
        let content_with_markers = format!(
            "#!/usr/bin/env bash\necho 'pre-existing'\n{}\nspanglings drill\n{}\necho 'done'",
            HOOK_START_MARKER, HOOK_END_MARKER
        );
        let cleaned = remove_spanglings_block(&content_with_markers);
        assert!(cleaned.contains("echo 'pre-existing'"));
        assert!(cleaned.contains("echo 'done'"));
        assert!(!cleaned.contains(HOOK_START_MARKER));
        assert!(!cleaned.contains("spanglings drill"));
    }
}
