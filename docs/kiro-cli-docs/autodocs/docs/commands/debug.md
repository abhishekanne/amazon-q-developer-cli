---
doc_meta:
  validated: 2025-12-23
  commit: 57090ffe
  status: validated
  testable_headless: true
  category: command
  title: kiro-cli debug
  description: Debug the app with various diagnostic and troubleshooting tools
  keywords: [debug, troubleshoot, logs, diagnostics, devtools]
  related: [diagnostic, logdump]
---

# kiro-cli debug

Debug the app with various diagnostic and troubleshooting tools.

## Overview

The debug command provides various debugging and diagnostic utilities for troubleshooting Kiro CLI issues. Includes app debugging, log viewing, accessibility testing, and system diagnostics.

## Usage

```bash
kiro-cli debug <subcommand>
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--verbose` | `-v` | Increase logging verbosity (can be repeated) |
| `--help` | `-h` | Print help information |

## Subcommands

### app
Debug the app.

```bash
kiro-cli debug app
```

### build
Switch to another branch of a Fig.js app.

```bash
kiro-cli debug build
```

### autocomplete-window
Toggle/set autocomplete window debug mode.

```bash
kiro-cli debug autocomplete-window
```

### logs
Show debug logs.

```bash
kiro-cli debug logs
```

### input-method
Input method debugger.

```bash
kiro-cli debug input-method
```

### prompt-accessibility
Prompt accessibility testing.

```bash
kiro-cli debug prompt-accessibility
```

### sample
Sample desktop process.

```bash
kiro-cli debug sample
```

### verify-codesign
Debug application codesigning.

```bash
kiro-cli debug verify-codesign
```

### accessibility
Accessibility debugging.

```bash
kiro-cli debug accessibility
```

### key-tester
Key tester utility.

```bash
kiro-cli debug key-tester
```

### diagnostics
Watch diagnostics.

```bash
kiro-cli debug diagnostics
```

### query-index
Query remote repository for updates.

```bash
kiro-cli debug query-index
```

### devtools
Open devtools of specific webview.

```bash
kiro-cli debug devtools
```

### get-index
Display remote index.

```bash
kiro-cli debug get-index
```

### list-intellij-variants
List installed IntelliJ variants.

```bash
kiro-cli debug list-intellij-variants
```

### shell
Use minimal shell config.

```bash
kiro-cli debug shell
```

### fix-permissions
Fix shell config permissions.

```bash
kiro-cli debug fix-permissions
```

### refresh-auth-token
Refresh authentication token.

```bash
kiro-cli debug refresh-auth-token
```

## Examples

### Example 1: View Debug Logs

```bash
kiro-cli debug logs
```

### Example 2: Test Accessibility

```bash
kiro-cli debug accessibility
```

### Example 3: Fix Permissions

```bash
kiro-cli debug fix-permissions
```

## Related Features

- [kiro-cli diagnostic](diagnostic.md) - System diagnostics
- [/logdump](../slash-commands/logdump.md) - Create log archive

## Limitations

- Debug commands are for troubleshooting only
- Some commands may require elevated permissions
- Output varies by platform and configuration

## Technical Details

**Purpose**: Debugging and diagnostic utilities

**Platform**: Cross-platform debug tools

**Permissions**: Some commands may require elevated access