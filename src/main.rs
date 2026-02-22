#![forbid(unsafe_code)]

use std::{env, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use config_mapper::{MapperOptions, run};

#[derive(Debug, Parser)]
#[command(
    name = "config-mapper",
    version,
    about = "Symlink files from configured sources into target directories"
)]
struct Cli {
    /// Path to TOML configuration file.
    #[arg(short, long, default_value = "configs.toml")]
    config: PathBuf,

    /// Working directory used to resolve relative source patterns and targets.
    #[arg(short = 'C', long)]
    working_dir: Option<PathBuf>,

    /// Simulate work without changing files.
    #[arg(long)]
    dry_run: bool,

    /// Optional backup directory. Defaults to working-dir/backup-<timestamp>.
    #[arg(long)]
    backup_dir: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let working_dir = cli
        .working_dir
        .unwrap_or(env::current_dir().map_err(anyhow::Error::from)?);

    let summary = run(&MapperOptions {
        working_dir,
        config_path: cli.config,
        dry_run: cli.dry_run,
        backup_dir: cli.backup_dir,
    })?;

    for operation in &summary.operations {
        match &operation.backup {
            Some(backup_path) => {
                println!(
                    "{} -> {} (backed up to {})",
                    operation.source.display(),
                    operation.target.display(),
                    backup_path.display()
                );
            }
            None => {
                println!(
                    "{} -> {}",
                    operation.source.display(),
                    operation.target.display()
                );
            }
        }
    }

    if summary.dry_run {
        println!(
            "Dry run complete: {} link(s) planned.",
            summary.operations.len()
        );
    } else {
        println!("Completed: {} link(s) created.", summary.operations.len());
    }

    if let Some(backup_dir) = summary.backup_dir {
        println!("Backups stored in {}", backup_dir.display());
    }

    Ok(())
}
