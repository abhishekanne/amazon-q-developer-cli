---
doc_meta:
  validated: 2025-12-23
  commit: 57090ffe
  status: validated
  testable_headless: true
  category: command
  title: kiro-cli restart
  description: Restart the desktop app
  keywords: [restart, reboot, desktop, app]
  related: [dashboard, quit]
---

# kiro-cli restart

Restart the desktop app.

## Overview

The restart command restarts the Kiro CLI desktop application.

## Usage

```bash
kiro-cli restart
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--verbose` | `-v` | Increase logging verbosity (can be repeated) |
| `--help` | `-h` | Print help information |

## Examples

### Example 1: Restart Desktop App

```bash
kiro-cli restart
```

Restarts the desktop application.

## Related Features

- [kiro-cli dashboard](dashboard.md) - Launch desktop app
- [kiro-cli quit](quit.md) - Quit desktop app

## Technical Details

**Purpose**: Restarts desktop application process

**Scope**: Desktop app only (not chat sessions)