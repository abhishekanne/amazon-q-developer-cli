---
doc_meta:
  validated: 2025-12-23
  commit: 57090ffe
  status: validated
  testable_headless: true
  category: command
  title: kiro-cli integrations
  description: Manage system integrations with install, uninstall, reinstall, and status operations
  keywords: [integrations, system, install, uninstall, status]
  related: [setup, inline]
---

# kiro-cli integrations

Manage system integrations with install, uninstall, reinstall, and status operations.

## Overview

The integrations command manages system-level integrations for Kiro CLI. Install, uninstall, reinstall, or check the status of various system integrations that enhance the CLI experience.

## Usage

```bash
kiro-cli integrations <subcommand>
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--verbose` | `-v` | Increase logging verbosity (can be repeated) |
| `--help` | `-h` | Print help information |

## Subcommands

### install

Install system integrations.

```bash
kiro-cli integrations install
```

### uninstall

Remove system integrations.

```bash
kiro-cli integrations uninstall
```

### reinstall

Reinstall system integrations.

```bash
kiro-cli integrations reinstall
```

### status

Show current status of integrations.

```bash
kiro-cli integrations status
```

## Examples

### Example 1: Install Integrations

```bash
kiro-cli integrations install
```

### Example 2: Check Status

```bash
kiro-cli integrations status
```

### Example 3: Reinstall Integrations

```bash
kiro-cli integrations reinstall
```

### Example 4: Uninstall Integrations

```bash
kiro-cli integrations uninstall
```

## Related Features

- [kiro-cli setup](setup.md) - Initial CLI setup
- [kiro-cli inline](inline.md) - Inline completions

## Limitations

- May require elevated permissions
- Platform-dependent functionality
- System restart may be needed

## Technical Details

**Purpose**: System-level integration management

**Scope**: System-wide integrations and enhancements

**Permissions**: May require elevated access for system modifications