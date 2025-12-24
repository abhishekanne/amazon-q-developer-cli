---
doc_meta:
  validated: 2025-12-23
  commit: 57090ffe
  status: validated
  testable_headless: true
  category: command
  title: kiro-cli doctor
  description: Fix and diagnose common issues with automated health checks and repairs
  keywords: [doctor, fix, diagnose, health, check, repair]
  related: [diagnostic, debug]
---

# kiro-cli doctor

Fix and diagnose common issues with automated health checks and repairs.

## Overview

The doctor command runs automated health checks to identify and fix common Kiro CLI issues. Performs diagnostics and applies fixes automatically unless running in check-only mode.

## Usage

### Basic Usage

```bash
kiro-cli doctor
```

### Check Only (No Fixes)

```bash
kiro-cli doctor --all
```

### Strict Mode

```bash
kiro-cli doctor --strict
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--all` | `-a` | Run all doctor tests, with no fixes |
| `--strict` | `-s` | Error on warnings |
| `--verbose` | `-v` | Increase logging verbosity (can be repeated) |
| `--help` | `-h` | Print help information |

## Common Use Cases

#### Use Case 1: Fix Issues Automatically

```bash
kiro-cli doctor
```

**What this does**: Runs health checks and automatically applies fixes for detected issues.

#### Use Case 2: Check Without Fixing

```bash
kiro-cli doctor --all
```

**What this does**: Identifies issues but doesn't apply fixes. Useful for assessment.

#### Use Case 3: Strict Validation

```bash
kiro-cli doctor --strict
```

**What this does**: Treats warnings as errors for stricter validation.

## Examples

### Example 1: Basic Health Check

```bash
kiro-cli doctor
```

**Expected Output**:
```
Running health checks...
✓ Configuration valid
✓ Database accessible
⚠ MCP servers need restart - fixing...
✓ All checks passed
```

### Example 2: Assessment Mode

```bash
kiro-cli doctor --all
```

**Expected Output**:
```
Running diagnostic checks (no fixes)...
✓ Configuration valid
✗ Database corruption detected
⚠ Outdated MCP servers
2 issues found, 1 warning
```

### Example 3: Verbose Output

```bash
kiro-cli doctor -v
```

Shows detailed information about each check and fix applied.

## What It Checks

- Configuration file validity
- Database integrity
- MCP server status
- File permissions
- Network connectivity
- Authentication status

## Troubleshooting

### Issue: Doctor Fails to Run

**Symptom**: Command exits with error  
**Cause**: Severe system issue  
**Solution**: Check basic installation with `kiro-cli --version`

### Issue: Fixes Don't Persist

**Symptom**: Same issues return after doctor run  
**Cause**: Underlying system problem  
**Solution**: Run with `--verbose` to see detailed fix attempts

### Issue: Strict Mode Too Restrictive

**Symptom**: Doctor fails on minor warnings  
**Cause**: `--strict` flag treats warnings as errors  
**Solution**: Run without `--strict` or address all warnings

## Related Features

- [kiro-cli diagnostic](diagnostic.md) - System diagnostics
- [kiro-cli debug](debug.md) - Debug utilities

## Limitations

- Some fixes require elevated permissions
- Network-dependent checks may fail offline
- Cannot fix all possible issues automatically

## Technical Details

**Checks Performed**: Configuration, database, MCP servers, permissions, connectivity

**Fix Strategy**: Applies safe, non-destructive fixes automatically

**Exit Codes**: 0 for success, non-zero for failures or warnings (in strict mode)