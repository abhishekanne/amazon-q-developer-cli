---
doc_meta:
  validated: 2025-12-23
  commit: 57090ffe
  status: validated
  testable_headless: false
  category: command
  title: kiro-cli launch
  description: Launch the desktop app
  keywords: [launch, desktop, app, gui]
  related: [dashboard, quit, restart]
---

# kiro-cli launch

Launch the desktop app.

## Overview

The launch command starts the Kiro CLI desktop application.

## Usage

```bash
kiro-cli launch
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--verbose` | `-v` | Increase logging verbosity (can be repeated) |
| `--help` | `-h` | Print help information |

## Examples

### Example 1: Launch Desktop App

```bash
kiro-cli launch
```

Starts the desktop application.

## Related Features

- [kiro-cli dashboard](dashboard.md) - Alternative launch command
- [kiro-cli quit](quit.md) - Quit desktop app
- [kiro-cli restart](restart.md) - Restart desktop app

## Limitations

- Requires graphical environment
- Not available in headless/SSH environments

## Technical Details

**Purpose**: Start desktop application

**Interface**: Graphical user interface