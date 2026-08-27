# Spanglings Init Subcommand Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the `spanglings init` subcommand to allow users to scaffold their exercise workspace.

**Architecture:** 
The command will be added to the clap CLI parser as a `Init` variant of the `Commands` enum in `src/cli/mod.rs`. It will take an optional path argument (defaulting to `./exercises`) and a boolean `--force` flag. The subcommand execution logic in `src/main.rs` will call `run_init` defined in `src/cli/commands/init.rs`, which delegates to the embedded helper `init_exercises_dir` in `src/core/embedded.rs`.

**Tech Stack:** Rust, Clap (CLI arguments parsing), Colored (CLI output coloring), include_dir (for embedded files handling).

---

### Task 1: Create Integration Tests

**Files:**
- Create: `tests/init_tests.rs`

- [ ] **Step 1: Write integration tests in `tests/init_tests.rs`**

```rust
use spanglings::cli::commands::init::run_init;
use tempfile::tempdir;

#[test]
fn test_run_init_creates_exercise_workspace() {
    let temp = tempdir().unwrap();
    let target = temp.path().join("my_exercises");
    let result = run_init(Some(target.to_str().unwrap()), false);
    assert!(result.is_ok());
    assert!(target.join("00_baseline").exists());
    assert!(target.join("01_ser_estar").exists());
    assert!(target.join("03_subjunctive_weirdo").exists());
}

#[test]
fn test_run_init_force_flag() {
    let temp = tempdir().unwrap();
    let target = temp.path().join("my_exercises");
    let res1 = run_init(Some(target.to_str().unwrap()), false);
    assert!(res1.is_ok());

    // Without force, should fail
    let res2 = run_init(Some(target.to_str().unwrap()), false);
    assert!(res2.is_err());

    // With force, should succeed
    let res3 = run_init(Some(target.to_str().unwrap()), true);
    assert!(res3.is_ok());
}
```

- [ ] **Step 2: Run test to verify it fails to compile**

Run: `cargo test --test init_tests`
Expected: Compilation failure because `run_init` is not implemented yet.

---

### Task 2: Implement Init Subcommand Command Handler

**Files:**
- Create: `src/cli/commands/init.rs`
- Modify: `src/cli/commands/mod.rs`

- [ ] **Step 1: Write the implementation in `src/cli/commands/init.rs`**

```rust
use crate::core::embedded::init_exercises_dir;
use colored::Colorize;
use std::path::PathBuf;

pub fn run_init(target_path: Option<&str>, force: bool) -> anyhow::Result<()> {
    let target = match target_path {
        Some(p) => PathBuf::from(p),
        None => PathBuf::from("exercises"),
    };

    println!("{}", "Initializing Spanglings curriculum workspace...".bold().cyan());
    let count = init_exercises_dir(&target, force)?;

    println!(
        "{} Initialized {} exercises in '{}'!",
        "✔".bold().green(),
        count.to_string().bold().yellow(),
        target.display().to_string().bold()
    );
    println!();
    println!("{}", "Next steps:".bold());
    println!("  1. Run {} to start interactive TUI mode", "spanglings".cyan().bold());
    println!("  2. Or run {} to start live file-watcher mode", "spanglings watch".cyan().bold());
    println!("  3. List available exercises with {}", "spanglings list".cyan().bold());

    Ok(())
}
```

- [ ] **Step 2: Export module in `src/cli/commands/mod.rs`**

Add `pub mod init;` to `src/cli/commands/mod.rs`.

---

### Task 3: Update CLI Definition and Main Dispatcher

**Files:**
- Modify: `src/cli/mod.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Add `Init` subcommand to `Commands` enum in `src/cli/mod.rs`**

Add the `Init` variant:
```rust
    /// Initialize exercises in the current directory or target path
    Init {
        /// Directory path to extract exercises into (defaults to ./exercises)
        path: Option<String>,
        /// Overwrite existing files if directory is not empty
        #[arg(short, long)]
        force: bool,
    },
```

- [ ] **Step 2: Add match dispatch in `src/main.rs`**

Add matching branch:
```rust
        Some(Commands::Init { path, force }) => {
            spanglings::cli::commands::init::run_init(path.as_deref(), force)?;
        }
```

---

### Task 4: Verification and Code Quality

- [ ] **Step 1: Run tests**

Run: `cargo test --all-targets`
Expected: All tests pass.

- [ ] **Step 2: Run clippy**

Run: `cargo clippy --all-targets -- -D warnings`
Expected: Clean build without warnings.

- [ ] **Step 3: Run cargo fmt**

Run: `cargo fmt --check`
Expected: Code matches format standard.

---

### Task 5: Commit changes

- [ ] **Step 1: Commit with conventional message**

Run: `git add src/cli/commands/init.rs src/cli/commands/mod.rs src/cli/mod.rs src/main.rs tests/init_tests.rs docs/superpowers/plans/2026-08-26-spanglings-init-subcommand.md`
Run: `git commit -m "feat(cli): add spanglings init command to scaffold exercise repository" --no-gpg-sign`
Expected: Successful atomic commit.
