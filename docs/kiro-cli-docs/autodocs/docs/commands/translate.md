---
doc_meta:
  validated: 2025-12-23
  commit: 57090ffe
  status: validated
  testable_headless: true
  category: command
  title: kiro-cli translate
  description: Natural Language to Shell translation
  keywords: [translate, natural, language, shell, command]
  related: [chat]
---

# kiro-cli translate

Natural Language to Shell translation.

## Overview

The translate command converts natural language descriptions into shell commands. Provides AI-powered command generation from plain English descriptions.

## Usage

```bash
kiro-cli translate [INPUT]...
```

## Arguments

| Argument | Description |
|----------|-------------|
| `[INPUT]...` | Natural language description of desired command |

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--verbose` | `-v` | Increase logging verbosity (can be repeated) |
| `--help` | `-h` | Print help information |

## Examples

### Example 1: File Operations

```bash
kiro-cli translate "list all files in current directory"
```

### Example 2: Process Management

```bash
kiro-cli translate "find all running processes with name node"
```

### Example 3: Network Commands

```bash
kiro-cli translate "check if port 8080 is open"
```

### Example 4: Git Operations

```bash
kiro-cli translate "show git status and recent commits"
```

## Related Features

- [kiro-cli chat](chat.md) - Interactive AI assistance

## Limitations

- Requires natural language input
- Command accuracy depends on description clarity
- Generated commands should be reviewed before execution

## Technical Details

**Purpose**: AI-powered natural language to shell command translation

**Input**: Plain English descriptions of desired operations

**Output**: Corresponding shell commands