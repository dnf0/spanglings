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
