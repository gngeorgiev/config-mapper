# config-mapper

`config-mapper` is a Rust CLI that maps source files into target directories using symlinks, while backing up any existing target files first.

## Features

- Declarative TOML config (`[configs]`) for source-glob -> target-directory mappings.
- Expands `~` and environment variables in both source patterns and target paths.
- Safe replacement behavior with automatic backup of existing targets.
- `--dry-run` mode to preview operations without touching the filesystem.

## Installation

### Install from GitHub

```bash
cargo install --git https://github.com/gngeorgiev/config-mapper --locked
```

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
".config/*" = "~/.config"
"home/*" = "~"
"agentlb/*" = "~/.agentlb"
```

Each key is a glob pattern for source entries.
Each value is a target directory where matching filenames will be linked.

## Dotfiles Flow (`~/dot`)

Intended usage is to run this tool from your dotfiles repository root at `~/dot`.

Before running `config-mapper`:

```text
~/dot
├── .config/
│   ├── nvim/
│   └── starship.toml
├── home/
│   ├── .bashrc
│   └── .gitconfig
├── agentlb/
│   └── config.toml
└── configs.toml
```

After running `config-mapper`:

```text
~
├── .bashrc -> ~/dot/home/.bashrc
├── .gitconfig -> ~/dot/home/.gitconfig
├── .config/
│   ├── nvim -> ~/dot/.config/nvim
│   └── starship.toml -> ~/dot/.config/starship.toml
└── .agentlb/
    └── config.toml -> ~/dot/agentlb/config.toml
```

## Usage

```bash
# From ~/dot, using ./configs.toml
cd ~/dot
config-mapper

# Preview only
config-mapper --dry-run

# Use the shipped example config
config-mapper --config ./example/configs.example.toml

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
