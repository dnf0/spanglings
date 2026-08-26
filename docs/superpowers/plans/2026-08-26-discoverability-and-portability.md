# Discoverability, Portability & Zero-Setup CLI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Provide zero-setup portability with embedded curriculum (`spanglings init`), in-memory fallback execution, JSON machine-readable output across commands, fuzzy topic/text search, and shell auto-completions.

**Architecture:** Embed all 116+ curriculum markdown files using `include_dir!`. Allow `curriculum::find_all_exercises` to fall back to in-memory embedded exercises when no local directory exists. Implement `init`, `search`, and `completions` subcommands with `clap_complete` and add `--json` serialization across all query and validation commands.

**Tech Stack:** Rust 2021, `include_dir` 0.7, `clap` 4.5, `clap_complete` 4.5, `serde` / `serde_json` 1.0, `colored` 2.1.

---

### Task 1: Embedded Exercises Catalog & In-Memory Fallback

**Files:**
- Modify: `Cargo.toml`
- Create: `src/core/embedded.rs`
- Modify: `src/core/mod.rs`
- Modify: `src/core/curriculum.rs`
- Test: `tests/embedded_tests.rs`

- [ ] **Step 1: Add `include_dir` and `clap_complete` dependencies to `Cargo.toml`**

```toml
include_dir = "0.7"
clap_complete = "4.5"
```

- [ ] **Step 2: Write failing test in `tests/embedded_tests.rs`**

```rust
use spanglings::core::curriculum::{find_all_exercises_or_embedded, Level};
use spanglings::core::embedded::{get_embedded_exercises, init_exercises_dir};
use tempfile::tempdir;

#[test]
fn test_get_embedded_exercises_loads_full_catalog() {
    let exercises = get_embedded_exercises().expect("Failed to load embedded exercises");
    assert!(exercises.len() >= 116, "Expected at least 116 embedded exercises, found {}", exercises.len());
    assert!(exercises.iter().any(|e| e.level == Level::B1));
    assert!(exercises.iter().any(|e| e.level == Level::B2));
    assert!(exercises.iter().any(|e| e.level == Level::C1));
}

#[test]
fn test_init_exercises_dir_writes_files() {
    let temp = tempdir().unwrap();
    let target = temp.path().join("exercises");
    let count = init_exercises_dir(&target, false).expect("Failed to init exercises");
    assert!(count >= 116);
    assert!(target.join("00_baseline").exists());
    assert!(target.join("03_subjunctive").exists());
    assert!(target.join("21_nuanced_collocations").exists());
}

#[test]
fn test_find_all_exercises_or_embedded_fallback() {
    let temp = tempdir().unwrap();
    let non_existent = temp.path().join("empty_dir/exercises");
    let exercises = find_all_exercises_or_embedded(&non_existent).expect("Fallback failed");
    assert!(exercises.len() >= 116);
}
```

- [ ] **Step 3: Run test to verify it fails**

Run: `cargo test --test embedded_tests`
Expected: Compilation failure (modules not found).

- [ ] **Step 4: Implement `src/core/embedded.rs`**

```rust
use crate::core::exercise::Exercise;
use include_dir::{include_dir, Dir};
use std::fs;
use std::path::Path;

pub static EMBEDDED_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/exercises");

pub fn get_embedded_exercises() -> anyhow::Result<Vec<Exercise>> {
    let mut exercises = Vec::new();
    collect_from_embedded_dir(&EMBEDDED_DIR, &mut exercises)?;
    exercises.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(exercises)
}

fn collect_from_embedded_dir(dir: &Dir, exercises: &mut Vec<Exercise>) -> anyhow::Result<()> {
    for file in dir.files() {
        if file.path().extension().is_some_and(|ext| ext == "md") {
            if let Some(content) = file.contents_utf8() {
                if let Ok(ex) = Exercise::from_markdown(file.path(), content) {
                    exercises.push(ex);
                }
            }
        }
    }
    for sub in dir.dirs() {
        collect_from_embedded_dir(sub, exercises)?;
    }
    Ok(())
}

pub fn init_exercises_dir<P: AsRef<Path>>(target_dir: P, force: bool) -> anyhow::Result<usize> {
    let target = target_dir.as_ref();
    if target.exists() && !force {
        let entries = fs::read_dir(target)?.count();
        if entries > 0 {
            anyhow::bail!(
                "Target directory '{}' already exists and is not empty. Use --force to overwrite.",
                target.display()
            );
        }
    }
    fs::create_dir_all(target)?;
    let count = extract_dir(&EMBEDDED_DIR, target)?;
    Ok(count)
}

fn extract_dir(dir: &Dir, target_root: &Path) -> anyhow::Result<usize> {
    let mut count = 0;
    for file in dir.files() {
        let out_path = target_root.join(file.path());
        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&out_path, file.contents())?;
        count += 1;
    }
    for sub in dir.dirs() {
        count += extract_dir(sub, target_root)?;
    }
    Ok(count)
}
```

- [ ] **Step 5: Export `embedded` and add `find_all_exercises_or_embedded` in `curriculum.rs`**

Modify `src/core/mod.rs`:
```rust
pub mod embedded;
```

Modify `src/core/curriculum.rs`:
```rust
pub fn find_all_exercises_or_embedded<P: AsRef<Path>>(root: P) -> anyhow::Result<Vec<Exercise>> {
    let root = root.as_ref();
    if root.exists() && root.is_dir() {
        let disk_exercises = find_all_exercises(root)?;
        if !disk_exercises.is_empty() {
            return Ok(disk_exercises);
        }
    }
    crate::core::embedded::get_embedded_exercises()
}
```

- [ ] **Step 6: Run tests and verify they pass**

Run: `cargo test --test embedded_tests`
Expected: PASS (all 3 tests pass).

- [ ] **Step 7: Commit**

```bash
git add Cargo.toml src/core/embedded.rs src/core/mod.rs src/core/curriculum.rs tests/embedded_tests.rs
git commit -m "feat(core): embed curriculum markdown and add in-memory fallback loader"
```

---

### Task 2: `spanglings init` Subcommand

**Files:**
- Create: `src/cli/commands/init.rs`
- Modify: `src/cli/commands/mod.rs`
- Modify: `src/cli/mod.rs`
- Modify: `src/main.rs`
- Test: `tests/init_tests.rs`

- [ ] **Step 1: Write failing test in `tests/init_tests.rs`**

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
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test init_tests`
Expected: FAIL (module not found).

- [ ] **Step 3: Implement `src/cli/commands/init.rs`**

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

- [ ] **Step 4: Wire `Init` command in `src/cli/mod.rs` and `src/main.rs`**

Add to `src/cli/mod.rs`:
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

Wire in `src/main.rs`:
```rust
Some(Commands::Init { path, force }) => {
    spanglings::cli::commands::init::run_init(path.as_deref(), force)?;
}
```

- [ ] **Step 5: Run tests and verify**

Run: `cargo test --test init_tests`
Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/cli/commands/init.rs src/cli/commands/mod.rs src/cli/mod.rs src/main.rs tests/init_tests.rs
git commit -m "feat(cli): add spanglings init command to scaffold exercise repository"
```

---

### Task 3: Machine-Readable JSON Output (`--json`)

**Files:**
- Modify: `src/cli/mod.rs`
- Modify: `src/cli/commands/list.rs`
- Modify: `src/cli/commands/progress.rs`
- Modify: `src/cli/commands/run.rs`
- Test: `tests/json_output_tests.rs`

- [ ] **Step 1: Write failing test in `tests/json_output_tests.rs`**

```rust
use spanglings::cli::commands::list::get_exercises_json;
use spanglings::cli::commands::progress::get_progress_json;

#[test]
fn test_list_exercises_json_serialization() {
    let json_str = get_exercises_json().expect("Failed to serialize exercises to JSON");
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    assert!(parsed.is_array());
    assert!(parsed.as_array().unwrap().len() >= 116);
    let first = &parsed[0];
    assert!(first.get("id").is_some());
    assert!(first.get("title").is_some());
    assert!(first.get("level").is_some());
    assert!(first.get("topic").is_some());
}

#[test]
fn test_progress_json_serialization() {
    let json_str = get_progress_json().expect("Failed to serialize progress to JSON");
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    assert!(parsed.get("total").is_some());
    assert!(parsed.get("completed").is_some());
    assert!(parsed.get("due_reviews").is_some());
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test json_output_tests`
Expected: FAIL (functions not defined).

- [ ] **Step 3: Update `src/cli/commands/list.rs` and `src/cli/commands/progress.rs`**

Implement `get_exercises_json() -> anyhow::Result<String>` and add `json: bool` parameter to `list_exercises(json: bool)`.
Implement `get_progress_json() -> anyhow::Result<String>` and add `json: bool` parameter to `show_progress(json: bool)`.

- [ ] **Step 4: Update `src/cli/mod.rs` to add `--json` flag to `Cli` or commands**

Add `--json` global flag to `Cli` struct:
```rust
#[arg(long, global = true, help = "Output result in JSON format")]
pub json: bool,
```

- [ ] **Step 5: Run tests and verify**

Run: `cargo test --test json_output_tests`
Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/cli/commands/list.rs src/cli/commands/progress.rs src/cli/mod.rs src/main.rs tests/json_output_tests.rs
git commit -m "feat(cli): add --json machine-readable output mode for list and progress"
```

---

### Task 4: Fuzzy Topic & Full-Text Search (`spanglings search`)

**Files:**
- Create: `src/cli/commands/search.rs`
- Modify: `src/cli/commands/mod.rs`
- Modify: `src/cli/mod.rs`
- Modify: `src/main.rs`
- Test: `tests/search_tests.rs`

- [ ] **Step 1: Write failing test in `tests/search_tests.rs`**

```rust
use spanglings::cli::commands::search::search_exercises;

#[test]
fn test_search_by_topic_and_keyword() {
    let results = search_exercises("subjunctive").expect("Search failed");
    assert!(!results.is_empty());
    assert!(results.iter().all(|e| {
        e.topic.to_lowercase().contains("subjunctive")
            || e.title.to_lowercase().contains("subjunctive")
            || e.prompt.to_lowercase().contains("subjunctive")
    }));
}

#[test]
fn test_search_by_level() {
    let results = search_exercises("C1").expect("Search failed");
    assert!(!results.is_empty());
    assert!(results.iter().any(|e| e.level.to_string() == "C1"));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test search_tests`
Expected: FAIL.

- [ ] **Step 3: Implement `src/cli/commands/search.rs`**

```rust
use crate::core::curriculum::find_all_exercises_or_embedded;
use crate::core::exercise::Exercise;
use colored::Colorize;

pub fn search_exercises(query: &str) -> anyhow::Result<Vec<Exercise>> {
    let all = find_all_exercises_or_embedded("exercises")?;
    let q = query.trim().to_lowercase();
    let matches: Vec<Exercise> = all
        .into_iter()
        .filter(|e| {
            e.id.to_lowercase().contains(&q)
                || e.title.to_lowercase().contains(&q)
                || e.topic.to_lowercase().contains(&q)
                || e.level.to_string().to_lowercase().contains(&q)
                || e.prompt.to_lowercase().contains(&q)
                || e.solution.to_lowercase().contains(&q)
                || e.grammar_rule.as_deref().unwrap_or("").to_lowercase().contains(&q)
        })
        .collect();
    Ok(matches)
}

pub fn run_search(query: &str, json: bool) -> anyhow::Result<()> {
    let results = search_exercises(query)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&results)?);
        return Ok(());
    }

    if results.is_empty() {
        println!("No exercises found matching query: '{}'", query.yellow());
        return Ok(());
    }

    println!(
        "Found {} exercise(s) matching '{}':\n",
        results.len().to_string().bold().green(),
        query.cyan()
    );
    for ex in results {
        println!(
            "  • [{}] {} (ID: {}) - Topic: {}",
            ex.level.to_string().magenta().bold(),
            ex.title.bold(),
            ex.id.cyan(),
            ex.topic.yellow()
        );
        println!("    Path: {}", ex.path.display().to_string().dimmed());
    }
    Ok(())
}
```

- [ ] **Step 4: Wire `Search` command into `src/cli/mod.rs` and `src/main.rs`**

- [ ] **Step 5: Run tests and verify**

Run: `cargo test --test search_tests`
Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/cli/commands/search.rs src/cli/commands/mod.rs src/cli/mod.rs src/main.rs tests/search_tests.rs
git commit -m "feat(cli): add spanglings search command for full-text query matching"
```

---

### Task 5: Shell Completions Generator (`spanglings completions`)

**Files:**
- Create: `src/cli/commands/completions.rs`
- Modify: `src/cli/commands/mod.rs`
- Modify: `src/cli/mod.rs`
- Modify: `src/main.rs`
- Test: `tests/completions_tests.rs`

- [ ] **Step 1: Write failing test in `tests/completions_tests.rs`**

```rust
use clap_complete::Shell;
use spanglings::cli::commands::completions::generate_completions_string;

#[test]
fn test_generate_bash_completions() {
    let output = generate_completions_string(Shell::Bash);
    assert!(output.contains("spanglings"));
    assert!(output.contains("watch"));
    assert!(output.contains("run"));
    assert!(output.contains("init"));
    assert!(output.contains("search"));
}

#[test]
fn test_generate_zsh_completions() {
    let output = generate_completions_string(Shell::Zsh);
    assert!(output.contains("spanglings"));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test completions_tests`
Expected: FAIL.

- [ ] **Step 3: Implement `src/cli/commands/completions.rs`**

```rust
use crate::cli::Cli;
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use std::io;

pub fn generate_completions_string(shell: Shell) -> String {
    let mut cmd = Cli::command();
    let mut buf = Vec::new();
    generate(shell, &mut cmd, "spanglings", &mut buf);
    String::from_utf8_lossy(&buf).to_string()
}

pub fn run_completions(shell: Shell) -> anyhow::Result<()> {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "spanglings", &mut io::stdout());
    Ok(())
}
```

- [ ] **Step 4: Wire `Completions` command into `src/cli/mod.rs` and `src/main.rs`**

- [ ] **Step 5: Run tests and verify**

Run: `cargo test --test completions_tests`
Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/cli/commands/completions.rs src/cli/commands/mod.rs src/cli/mod.rs src/main.rs tests/completions_tests.rs
git commit -m "feat(cli): add shell completion generator for bash, zsh, fish, and powershell"
```

---

### Task 6: Full Verification & Documentation Update

**Files:**
- Modify: `README.md`
- Run: Full test suite, linter, formatting, and build check

- [ ] **Step 1: Update `README.md` with new commands**
Document `spanglings init`, `spanglings search <query>`, `spanglings completions <shell>`, and `--json` flag.

- [ ] **Step 2: Run full verification checklist**
```bash
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo build --release
```

- [ ] **Step 3: Update knowledge graph**
```bash
uvx --from graphifyy graphify update .
```

- [ ] **Step 4: Commit**
```bash
git add README.md graphify-out/
git commit -m "docs: document init, search, completions, and json output features"
```
