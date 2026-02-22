use std::{
    fs,
    process::{Command, Output},
};

use tempfile::tempdir;

fn run_cli(root: &std::path::Path, args: &[&str]) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_config-mapper"));
    cmd.current_dir(root).args(args).output().expect("run cli")
}

#[cfg(unix)]
#[test]
fn cli_creates_symlink_and_backup() {
    let workspace = tempdir().expect("tempdir");
    let root = workspace.path();

    fs::create_dir_all(root.join("source")).expect("create source dir");
    fs::create_dir_all(root.join("target")).expect("create target dir");

    fs::write(root.join("source/bashrc"), "new content").expect("write source file");
    fs::write(root.join("target/bashrc"), "old content").expect("write target file");

    fs::write(
        root.join("configs.toml"),
        "[configs]\n\"source/*\" = \"target\"\n",
    )
    .expect("write config file");

    let output = run_cli(root, &["--backup-dir", "backups"]);
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let target_path = root.join("target/bashrc");
    let metadata = fs::symlink_metadata(&target_path).expect("target metadata");
    assert!(metadata.file_type().is_symlink());

    let link_target = fs::read_link(&target_path).expect("read symlink");
    assert_eq!(link_target, root.join("source/bashrc"));

    let backup_entries = fs::read_dir(root.join("backups"))
        .expect("read backups")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect backup entries");
    assert_eq!(backup_entries.len(), 1);

    let backup_content = fs::read_to_string(backup_entries[0].path()).expect("read backup file");
    assert_eq!(backup_content, "old content");
}

#[test]
fn cli_dry_run_makes_no_changes() {
    let workspace = tempdir().expect("tempdir");
    let root = workspace.path();

    fs::create_dir_all(root.join("source")).expect("create source dir");
    fs::create_dir_all(root.join("target")).expect("create target dir");

    fs::write(root.join("source/gitconfig"), "new").expect("write source file");
    fs::write(root.join("target/gitconfig"), "old").expect("write target file");

    fs::write(
        root.join("configs.toml"),
        "[configs]\n\"source/*\" = \"target\"\n",
    )
    .expect("write config file");

    let output = run_cli(root, &["--dry-run", "--backup-dir", "backups"]);
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let target_path = root.join("target/gitconfig");
    assert!(target_path.exists());
    let metadata = fs::symlink_metadata(&target_path).expect("target metadata");
    assert!(!metadata.file_type().is_symlink());
    assert_eq!(
        fs::read_to_string(target_path).expect("read target file"),
        "old"
    );

    assert!(!root.join("backups").exists());
}

#[test]
fn cli_fails_when_glob_matches_nothing() {
    let workspace = tempdir().expect("tempdir");
    let root = workspace.path();

    fs::write(
        root.join("configs.toml"),
        "[configs]\n\"missing/*\" = \"target\"\n",
    )
    .expect("write config file");

    let output = run_cli(root, &[]);
    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    assert!(stderr.contains("matched no files"), "stderr was: {stderr}");
}
