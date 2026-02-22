# config-mapper

`config-mapper` is a Rust CLI that maps source files into target directories using symlinks, while backing up any existing target files first.

## Features

- Declarative TOML config (`[configs]`) for source-glob -> target-directory mappings.
- Expands `~` and environment variables in both source patterns and target paths.
- Safe replacement behavior with automatic backup of existing targets.
- `--dry-run` mode to preview operations without touching the filesystem.
- Unit and integration test coverage.

## Installation

### Build from source

```bash
cargo install --path .
```

### Run without installing

```bash
cargo run -- --help
```

## Configuration

Create `configs.toml` in your working directory, or pass a custom path with `--config`.

```toml
[configs]
"dotfiles/.config/*" = "~/.config"
"dotfiles/home/*" = "~"
```

Each key is a glob pattern for source entries.
Each value is a target directory where matching filenames will be linked.

## Usage

```bash
# Uses current directory and ./configs.toml
config-mapper

# Preview only
config-mapper --dry-run

# Use a different config file
config-mapper --config ./configs.example.toml

# Resolve relative paths from a different working directory
config-mapper --working-dir /path/to/workspace

# Set explicit backup directory
config-mapper --backup-dir ./backups
```

## Backup Behavior

If target files already exist, `config-mapper` moves them into a backup directory before creating symlinks.

- Default backup directory: `<working-dir>/backup-<unix-millis>`
- You can override this with `--backup-dir`

## Development

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Contributing

See `CONTRIBUTING.md` for development workflow and PR expectations.

## Platform Notes

- Unix: symlink behavior works out-of-the-box.
- Windows: symlink creation may require Developer Mode or elevated privileges.

## License

MIT. See `LICENSE`.
