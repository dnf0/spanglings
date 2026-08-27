# Focus Area 5: Verb Conjugator, Git Workflow Hooks, and Custom Packs Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement automated Crates.io release pipeline (`SPANG-040`), high-precision terminal verb conjugator (`SPANG-041`), Git commit learning hook (`SPANG-042`), and custom curriculum pack scaffolder/validator (`SPANG-043`).

---

### Task 1: Automated Release Pipeline & Crates.io Publishing (`SPANG-040`)

**Files:**
- Modify: `Cargo.toml` (bump version to `0.2.0`)
- Modify: `.github/workflows/release.yaml`

- [ ] **Step 1: Bump version in `Cargo.toml` to `0.2.0`**
- [ ] **Step 2: Add `publish-crates-io` job to `.github/workflows/release.yaml`**
- [ ] **Step 3: Verify local package verification with `cargo publish --dry-run`**
- [ ] **Step 4: Commit**
```bash
git add Cargo.toml .github/workflows/release.yaml
git commit --no-gpg-sign -m "chore(release): configure automated crates.io publishing and bump version to 0.2.0"
```

---

### Task 2: High-Precision Terminal Verb Conjugator (`SPANG-041`)

**Files:**
- Create: `src/core/conjugator.rs`
- Update: `src/core/mod.rs`
- Create: `src/cli/commands/conjugate.rs`
- Update: `src/cli/commands/mod.rs`
- Update: `src/cli/mod.rs`
- Update: `src/main.rs`
- Create: `tests/conjugator_tests.rs`

- [ ] **Step 1: Implement Conjugation Models & Database in `src/core/conjugator.rs`**
- [ ] **Step 2: Implement CLI Command `spanglings conjugate <verb> [tense] [--json]` in `src/cli/commands/conjugate.rs`**
- [ ] **Step 3: Wire CLI into `src/cli/mod.rs` and `src/main.rs`**
- [ ] **Step 4: Create integration tests in `tests/conjugator_tests.rs`**
- [ ] **Step 5: Verify and commit**

---

### Task 3: Git Pre-Commit / Pre-Push Learning Hook (`SPANG-042`)

**Files:**
- Create: `src/cli/commands/hook.rs`
- Update: `src/cli/commands/mod.rs`
- Update: `src/cli/mod.rs`
- Update: `src/main.rs`
- Create: `tests/hook_tests.rs`

- [ ] **Step 1: Implement Git Hook Manager in `src/cli/commands/hook.rs`**
- [ ] **Step 2: Wire `Commands::Hook` in `src/cli/mod.rs` and `src/main.rs`**
- [ ] **Step 3: Integration tests in `tests/hook_tests.rs`**
- [ ] **Step 4: Verify and commit**

---

### Task 4: Custom Curriculum Pack Scaffolder & Validator (`SPANG-043`)

**Files:**
- Create: `src/cli/commands/pack.rs`
- Update: `src/cli/commands/mod.rs`
- Update: `src/cli/mod.rs`
- Update: `src/main.rs`
- Create: `tests/pack_tests.rs`

- [ ] **Step 1: Implement Pack Subcommands in `src/cli/commands/pack.rs`**
- [ ] **Step 2: Wire `Commands::Pack` in `src/cli/mod.rs` and `src/main.rs`**
- [ ] **Step 3: Unit and integration tests in `tests/pack_tests.rs`**
- [ ] **Step 4: Verify and commit**

---

### Task 5: Documentation, Graphify Update & PR Creation

- [ ] **Step 1: Update `README.md` and `docs/BACKLOG.md`**
- [ ] **Step 2: Full test and linter verification**
- [ ] **Step 3: Knowledge graph update**
- [ ] **Step 4: Push branch and create PR #3**
