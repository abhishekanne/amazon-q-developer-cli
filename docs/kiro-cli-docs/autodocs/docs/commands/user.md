---
doc_meta:
  validated: 2025-12-23
  commit: 57090ffe
  status: validated
  testable_headless: true
  category: command
  title: kiro-cli user
  description: Manage your account with login, logout, whoami, and profile operations
  keywords: [user, account, login, logout, whoami, profile]
  related: [login, logout, whoami, profile]
---

# kiro-cli user

Manage your account with login, logout, whoami, and profile operations.

## Overview

The user command provides account management functionality including authentication, user information display, and profile access.

## Usage

```bash
kiro-cli user <COMMAND>
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--verbose` | `-v` | Increase logging verbosity (can be repeated) |
| `--help` | `-h` | Print help information |

## Subcommands

### login

Authenticate with Kiro CLI service.

```bash
kiro-cli user login
```

### logout

Sign out of Kiro CLI service.

```bash
kiro-cli user logout
```

### whoami

Display current user information.

```bash
kiro-cli user whoami
```

### profile

Show IDC user profile.

```bash
kiro-cli user profile
```

### help

Show help for user command or subcommands.

```bash
kiro-cli user help [SUBCOMMAND]
```

## Examples

### Example 1: Login

```bash
kiro-cli user login
```

### Example 2: Check Current User

```bash
kiro-cli user whoami
```

### Example 3: View Profile

```bash
kiro-cli user profile
```

### Example 4: Logout

```bash
kiro-cli user logout
```

## Related Features

- [kiro-cli login](login.md) - Direct login command
- [kiro-cli logout](logout.md) - Direct logout command
- [kiro-cli whoami](whoami.md) - Direct whoami command
- [kiro-cli profile](profile.md) - Direct profile command

## Technical Details

**Purpose**: Centralized account management

**Subcommands**: Wrapper for individual user-related commands