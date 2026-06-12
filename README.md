# menv-rs

A dotfiles manager tool written in Rust.

menv-rs helps you manage your dotfiles repositories, system tools, and setup scripts in one place. It reads configuration from TOML files and provides commands to initialize, sync, and link your dotfiles, as well as install and track system packages.

## Features

- **Repos Management** - Clone, pull, and symlink dotfile repositories in parallel
- **Tools Management** - Check, list, and batch-install system packages via pacman/paru
- **Scripts** - Run common setup scripts (locale, swap, trim, hibernate, services, etc.)
- **Status Overview** - Quick summary of which tools and repos are set up
- **Self Update** - Built-in updater via cargo-dist

## Installation

### Shell (macOS & Linux)

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/moecly/menv-rs/releases/latest/download/menv-rs-installer.sh | sh
```

### PowerShell (Windows)

```powershell
irm https://github.com/moecly/menv-rs/releases/latest/download/menv-rs-installer.ps1 | iex
```

### From Source

```sh
git clone https://github.com/moecly/menv-rs.git
cd menv-rs
cargo build --release
```

## Configuration

menv-rs reads configuration from TOML files in the `config/` directory under the working directory:

- `config/repos.toml` - Dotfile repositories to manage
- `config/tools.toml` - System tools/packages to track

### repos.toml

```toml
[[repos]]
name = "dotfiles"
git_url = "https://github.com/user/dotfiles.git"

[[repos]]
name = "nvim-config"
git_url = "https://github.com/user/nvim-config.git"
```

Each repo should contain a `link.sh` script for the `repos link` command to execute.

### tools.toml

```toml
[[tools]]
tool_name = "neovim"
package_name = "neovim"
package_source = "pacman"
category = "editor"
desc = "Hyperextensible text editor"
command = "nvim"
```

`package_source` can be `pacman` or `paru`.

## Usage

```
menv <command>

Commands:
  status              Show overview of tools and repos status
  repos <subcommand>  Manage dotfile repositories
  tools <subcommand>  Manage system tools/packages
  scripts <subcommand> Run setup scripts
  upgrade             Self-update to the latest version
```

### Repos

```
menv repos init     # Clone all configured repositories
menv repos pull     # Pull latest changes for all repositories
menv repos link     # Run link.sh in each repository to create symlinks
menv repos list     # List all configured repositories and their status
```

### Tools

```
menv tools install  # Batch install missing packages via pacman/paru
menv tools list     # List all configured tools and their install status
```

### Scripts

```
menv scripts boot-default   # Configure boot default
menv scripts hibernate      # Configure hibernate
menv scripts locale         # Set up locale
menv scripts paru           # Install paru AUR helper
menv scripts service        # Configure services
menv scripts swapfile       # Set up swapfile
menv scripts trim           # Configure SSD TRIM
```

### Status

```
menv status  # Show how many tools are installed and repos are cloned
```

## License

MIT
