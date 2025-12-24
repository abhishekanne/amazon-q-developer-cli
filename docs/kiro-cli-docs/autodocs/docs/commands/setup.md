---
doc_meta:
  validated: 2025-12-23
  commit: 57090ffe
  status: validated
  testable_headless: true
  category: command
  title: kiro-cli setup
  description: Setup CLI components including shell integrations and input methods
  keywords: [setup, install, shell, integration, dotfiles]
  related: [login, inline]
---

# kiro-cli setup

Setup CLI components including shell integrations and input methods.

## Overview

The setup command installs and configures Kiro CLI components including shell integrations, dotfiles, and input methods. Provides options for selective installation and global configuration.

## Usage

```bash
kiro-cli setup [OPTIONS]
```

## Options

| Option | Description |
|--------|-------------|
| `--dotfiles` | Install only the shell integrations |
| `--input-method` | Prompt input method installation |
| `--no-confirm` | Don't confirm automatic installation |
| `--force` | Force installation of q |
| `--global` | Install q globally |
| `--verbose` | Increase logging verbosity (can be repeated) |
| `--help` | Print help information |

## Examples

### Example 1: Full Setup

```bash
kiro-cli setup
```

Installs all components with confirmation prompts.

### Example 2: Shell Integration Only

```bash
kiro-cli setup --dotfiles
```

Installs only shell integrations and dotfiles.

### Example 3: Automatic Installation

```bash
kiro-cli setup --no-confirm
```

Installs without confirmation prompts.

### Example 4: Force Global Installation

```bash
kiro-cli setup --force --global
```

Forces global installation of q component.

### Example 5: Input Method Setup

```bash
kiro-cli setup --input-method
```

Prompts for input method installation.

## Related Features

- [kiro-cli login](login.md) - Authentication setup
- [kiro-cli inline](inline.md) - Inline completions

## Limitations

- May require elevated permissions
- Shell restart may be needed after setup
- Platform-dependent components

## Technical Details

**Components**: Shell integrations, dotfiles, input methods

**Installation**: Local by default, global with `--global`

**Confirmation**: Interactive by default, bypass with `--no-confirm`