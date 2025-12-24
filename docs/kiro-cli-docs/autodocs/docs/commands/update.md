---
doc_meta:
  validated: 2025-12-23
  commit: 57090ffe
  status: validated
  testable_headless: true
  category: command
  title: kiro-cli update
  description: Update the Kiro application to the latest version
  keywords: [update, upgrade, version, install]
  related: [dashboard, whoami]
---

# kiro-cli update

Update the Kiro application to the latest version.

## Overview

The update command downloads and installs the latest version of Kiro CLI. Supports automatic confirmation and dashboard relaunching options.

## Usage

```bash
kiro-cli update [OPTIONS]
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--non-interactive` | `-y` | Don't prompt for confirmation |
| `--relaunch-dashboard` | | Relaunch into dashboard after update (false will launch in background) |
| `--rollout` | | Uses rollout |
| `--verbose` | `-v` | Increase logging verbosity (can be repeated) |
| `--help` | `-h` | Print help information |

## Examples

### Example 1: Interactive Update

```bash
kiro-cli update
```

Prompts for confirmation before updating.

### Example 2: Automatic Update

```bash
kiro-cli update -y
```

Updates without confirmation prompt.

### Example 3: Update with Dashboard Relaunch

```bash
kiro-cli update --relaunch-dashboard
```

Updates and relaunches dashboard interface.

### Example 4: Rollout Update

```bash
kiro-cli update --rollout
```

Uses rollout mechanism for update.

## Related Features

- [kiro-cli dashboard](dashboard.md) - Dashboard interface
- [kiro-cli whoami](whoami.md) - Check current version

## Limitations

- Requires internet connection
- May require elevated permissions
- Dashboard relaunch depends on system configuration

## Technical Details

**Update Source**: Downloads from official release channels

**Confirmation**: Interactive by default, bypass with `-y`

**Dashboard**: Optional relaunch after successful update