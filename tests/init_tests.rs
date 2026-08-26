use spanglings::cli::commands::init::run_init;
use tempfile::tempdir;

#[test]
fn test_run_init_creates_exercise_workspace() {
    let temp = tempdir().unwrap();
    let target = temp.path().join("my_exercises");
    let result = run_init(Some(target.to_str().unwrap()), false);
    assert!(result.is_ok());
    assert!(target.join("00_baseline").exists());
    assert!(target.join("01_ser_vs_estar").exists());
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
