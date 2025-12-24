---
doc_meta:
  validated: 2025-12-23
  commit: 57090ffe
  status: validated
  testable_headless: true
  category: command
  title: kiro-cli inline
  description: Manage inline shell completions with enable, disable, status, and customization operations
  keywords: [inline, completions, shell, enable, disable, customization]
  related: [chat]
---

# kiro-cli inline

Manage inline shell completions with enable, disable, status, and customization operations.

## Overview

The inline command manages inline shell completions that provide AI-powered suggestions directly in your terminal. Enable or disable the feature, check status, and manage customizations for personalized completion behavior.

## Usage

```bash
kiro-cli inline <subcommand>
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--verbose` | `-v` | Increase logging verbosity (can be repeated) |
| `--help` | `-h` | Print help information |

## Subcommands

### enable

Enable inline shell completions.

```bash
kiro-cli inline enable
```

### disable

Disable inline shell completions.

```bash
kiro-cli inline disable
```

### status

Show current status of inline completions.

```bash
kiro-cli inline status
```

### set-customization

Select a customization if available.

```bash
kiro-cli inline set-customization
```

### show-customizations

Display available customizations.

```bash
kiro-cli inline show-customizations
```

## Examples

### Example 1: Enable Inline Completions

```bash
kiro-cli inline enable
```

### Example 2: Check Status

```bash
kiro-cli inline status
```

### Example 3: View Customizations

```bash
kiro-cli inline show-customizations
```

### Example 4: Set Customization

```bash
kiro-cli inline set-customization
```

### Example 5: Disable Feature

```bash
kiro-cli inline disable
```

## Related Features

- [kiro-cli chat](chat.md) - Main chat interface

## Limitations

- Requires compatible shell
- May need shell restart after enable/disable
- Customizations depend on availability

## Technical Details

**Shell Integration**: Integrates with shell for inline suggestions

**Customizations**: Personalized completion behavior based on available options

**Status**: Shows whether feature is currently active