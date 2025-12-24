---
doc_meta:
  validated: 2025-12-23
  commit: 57090ffe
  status: validated
  testable_headless: true
  category: command
  title: kiro-cli theme
  description: Get or set the CLI theme for customizing appearance
  keywords: [theme, appearance, color, style, ui]
  related: [settings]
---

# kiro-cli theme

Get or set the CLI theme for customizing appearance.

## Overview

The theme command manages visual themes for Kiro CLI. View current theme, set new theme, list available themes, or open theme folder for customization.

## Usage

### Basic Usage

```bash
kiro-cli theme
```

Shows current theme.

### Set Theme

```bash
kiro-cli theme <theme-name>
```

Sets specified theme.

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--list` | | List all available themes |
| `--folder` | | Open theme folder |
| `--verbose` | `-v` | Increase logging verbosity (can be repeated) |
| `--help` | `-h` | Print help information |

## Examples

### Example 1: View Current Theme

```bash
kiro-cli theme
```

### Example 2: Set Theme

```bash
kiro-cli theme dark
```

### Example 3: List Available Themes

```bash
kiro-cli theme --list
```

### Example 4: Open Theme Folder

```bash
kiro-cli theme --folder
```

## Related Features

- [kiro-cli settings](settings.md) - General configuration

## Technical Details

**Scope**: User-wide theme settings

**Storage**: Theme configuration stored in user settings