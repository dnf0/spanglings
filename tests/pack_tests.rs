use spanglings::cli::commands::pack::{run_pack_create, run_pack_validate};
use std::env;

#[test]
fn test_pack_create_and_validate() {
    let temp_dir = tempfile::tempdir().expect("temp dir");
    let old_dir = env::current_dir().expect("curr dir");
    env::set_current_dir(temp_dir.path()).expect("set temp dir");

    // Create a new pack
    let create_result = run_pack_create("legal-spanish");
    assert!(create_result.is_ok());

    // Validate the pack
    let validate_result = run_pack_validate("legal-spanish");
    assert!(validate_result.is_ok());
    assert!(validate_result.unwrap());

    env::set_current_dir(old_dir).expect("restore dir");
}
