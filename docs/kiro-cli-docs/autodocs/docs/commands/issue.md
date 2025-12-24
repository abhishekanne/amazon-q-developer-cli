---
doc_meta:
  validated: 2025-12-23
  commit: 57090ffe
  status: validated
  testable_headless: true
  category: command
  title: kiro-cli issue
  description: Create a new GitHub issue with optional description and force creation
  keywords: [issue, github, bug, report, create]
  related: [slash-issue, report-issue]
---

# kiro-cli issue

Create a new GitHub issue with optional description and force creation.

## Overview

The issue command creates a new GitHub issue. Optionally provide description and force creation without prompts.

## Usage

### Basic Usage

```bash
kiro-cli issue
```

Creates GitHub issue with interactive prompts.

### With Description

```bash
kiro-cli issue "Bug in authentication flow"
```

Creates issue with specified description.

## Arguments

| Argument | Description |
|----------|-------------|
| `[DESCRIPTION]...` | Issue description (optional) |

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--force` | `-f` | Force issue creation without prompts |
| `--verbose` | `-v` | Increase logging verbosity (can be repeated) |
| `--help` | `-h` | Print help information |

## Examples

### Example 1: Create Issue Interactively

```bash
kiro-cli issue
```

### Example 2: Create with Description

```bash
kiro-cli issue "Command fails with permission error"
```

### Example 3: Force Creation

```bash
kiro-cli issue --force "Critical bug in file handling"
```

## Related Features

- [/issue](../slash-commands/issue.md) - Slash command version
- [report_issue](../tools/report-issue.md) - Tool version

## Limitations

- Requires GitHub account
- Opens browser for issue creation
- Not available in headless environments

## Technical Details

**Platform**: Opens GitHub issue creation page in browser

**Force Mode**: Skips confirmation prompts