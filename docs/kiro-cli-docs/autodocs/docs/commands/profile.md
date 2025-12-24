---
doc_meta:
  validated: 2025-12-23
  commit: 57090ffe
  status: validated
  testable_headless: true
  category: command
  title: kiro-cli profile
  description: Show the profile associated with this IDC user
  keywords: [profile, user, idc, identity]
  related: [login, whoami]
---

# kiro-cli profile

Show the profile associated with this IDC user.

## Overview

The profile command displays profile information for the current IDC (Identity Center) user.

## Usage

```bash
kiro-cli profile
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--verbose` | `-v` | Increase logging verbosity (can be repeated) |
| `--help` | `-h` | Print help information |

## Examples

### Example 1: Show Profile

```bash
kiro-cli profile
```

Displays current user's IDC profile information.

## Related Features

- [kiro-cli login](login.md) - Authentication
- [kiro-cli whoami](whoami.md) - User information

## Technical Details

**Purpose**: Display IDC user profile information

**Authentication**: Requires active IDC session