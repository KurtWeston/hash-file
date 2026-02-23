use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_hash_calculation() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "Hello, World!").unwrap();

    let output = std::process::Command::new("cargo")
        .args(["run", "--", file_path.to_str().unwrap()])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("test.txt"));
}

#[test]
fn test_verification() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("verify.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "Test content").unwrap();

    let hash_output = std::process::Command::new("cargo")
        .args(["run", "--", "-q", file_path.to_str().unwrap()])
        .output()
        .unwrap();

    let hash = String::from_utf8_lossy(&hash_output.stdout).trim().to_string();

    let verify_output = std::process::Command::new("cargo")
        .args(["run", "--", "--verify", &hash, file_path.to_str().unwrap()])
        .output()
        .unwrap();

    assert!(verify_output.status.success());
    let stdout = String::from_utf8_lossy(&verify_output.stdout);
    assert!(stdout.contains("OK"));
}