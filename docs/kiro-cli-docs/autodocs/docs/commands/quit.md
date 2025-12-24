---
doc_meta:
  validated: 2025-12-23
  commit: 57090ffe
  status: validated
  testable_headless: true
  category: command
  title: kiro-cli quit
  description: Quit the desktop app
  keywords: [quit, exit, close, desktop, app]
  related: [dashboard]
---

# kiro-cli quit

Quit the desktop app.

## Overview

The quit command terminates the Kiro CLI desktop application.

## Usage

```bash
kiro-cli quit
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--verbose` | `-v` | Increase logging verbosity (can be repeated) |
| `--help` | `-h` | Print help information |

## Examples

### Example 1: Quit Desktop App

```bash
kiro-cli quit
```

Terminates the desktop application.

## Related Features

- [kiro-cli dashboard](dashboard.md) - Launch desktop app

## Technical Details

**Purpose**: Terminates desktop application process

**Scope**: Desktop app only (not chat sessions)