---
doc_meta:
  validated: 2025-12-23
  commit: 57090ffe
  status: validated
  testable_headless: true
  category: command
  title: kiro-cli init
  description: Generate dotfiles for shell integration with pre/post command hooks
  keywords: [init, dotfiles, shell, integration, hooks]
  related: [setup, integrations]
---

# kiro-cli init

Generate dotfiles for shell integration with pre/post command hooks.

## Overview

The init command generates shell-specific dotfiles for integrating Kiro CLI with your shell. Supports bash, zsh, fish, and nu shells with pre/post command hook generation.

## Usage

```bash
kiro-cli init <SHELL> <WHEN> [OPTIONS]
```

## Arguments

| Argument | Description | Values |
|----------|-------------|---------|
| `<SHELL>` | The shell to generate dotfiles for | `bash`, `zsh`, `fish`, `nu` |
| `<WHEN>` | When to generate the dotfiles for | `pre`, `post` |

## Options

| Option | Description |
|--------|-------------|
| `--rcfile <RCFILE>` | Specify custom RC file path |
| `--verbose` | Increase logging verbosity (can be repeated) |
| `--help` | Print help information |

## Shell Support

- **bash**: Bash shell integration
- **zsh**: Zsh shell integration  
- **fish**: Fish shell integration
- **nu**: Nu shell integration

## Hook Types

- **pre**: Generate pre-command hooks
- **post**: Generate post-command hooks

## Examples

### Example 1: Bash Pre-Command Hook

```bash
kiro-cli init bash pre
```

### Example 2: Zsh Post-Command Hook

```bash
kiro-cli init zsh post
```

### Example 3: Fish with Custom RC File

```bash
kiro-cli init fish pre --rcfile ~/.config/fish/config.fish
```

### Example 4: Nu Shell Integration

```bash
kiro-cli init nu post
```

## Related Features

- [kiro-cli setup](setup.md) - Complete CLI setup
- [kiro-cli integrations](integrations.md) - System integrations

## Limitations

- Shell-specific syntax requirements
- May require shell restart after integration
- Custom RC file paths must be valid

## Technical Details

**Purpose**: Shell integration via dotfile generation

**Hook Types**: Pre-command and post-command execution hooks

**Shell Support**: Cross-shell compatibility with major shells