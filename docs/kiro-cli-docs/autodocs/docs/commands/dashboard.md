---
doc_meta:
  validated: 2025-12-23
  commit: 57090ffe
  status: validated
  testable_headless: false
  category: command
  title: kiro-cli dashboard
  description: Open the Kiro CLI dashboard application
  keywords: [dashboard, gui, application, interface]
  related: [chat, login]
---

# kiro-cli dashboard

Open the Kiro CLI dashboard application.

## Overview

The dashboard command launches the Kiro CLI graphical user interface. Provides a visual interface for interacting with Kiro CLI features and managing conversations.

## Usage

### Basic Usage

```bash
kiro-cli dashboard
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--verbose` | `-v` | Increase logging verbosity (can be repeated) |
| `--help` | `-h` | Print help information |

## Examples

### Example 1: Open Dashboard

```bash
kiro-cli dashboard
```

Opens the Kiro CLI dashboard application.

## Related Features

- [kiro-cli chat](chat.md) - Command-line chat interface
- [kiro-cli login](login.md) - Authentication required

## Limitations

- Requires graphical environment
- Not available in headless/SSH environments
- May require authentication

## Technical Details

**Interface**: Graphical user interface for Kiro CLI

**Platform**: Desktop application