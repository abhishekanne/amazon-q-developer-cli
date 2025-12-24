---
doc_meta:
  validated: 2025-12-19
  commit: 57090ffe
  status: validated
  testable_headless: false
  category: slash_command
  title: /agent list
  description: List all available agents
  keywords: [agent, list, available, show]
  related: [agent-switch, agent-generate, cmd-agent, agent-config]
---

# /agent list

List all available agents.

## Overview

The `/agent list` command displays all available agent configurations with their paths and marks the currently active agent.

## Usage

```
/agent list
```

## Options

- `-h, --help` - Print help

## Examples

### Example 1: Basic List

```
/agent list
```

**Output**:
```
* rust-expert    ~/.kiro/agents
  python-dev     ~/.kiro/agents
  default        (Built-in)
```

The `*` indicates the currently active agent.

## Output Format

- **Active Agent**: Marked with `*` prefix
- **Agent Name**: Configuration name
- **Path**: Location of agent configuration
  - `~/.kiro/agents/` - Global agents
  - `.kiro/agents/` - Local agents  
  - `(Built-in)` - Default system agents

## Agent Resolution Order

Agents are discovered from:
1. **Local**: `.kiro/agents/` in current directory
2. **Global**: `~/.kiro/agents/` in home directory
3. **Built-in**: Default system agents

## Related Commands

- [/agent](agent-switch.md) - Switch to different agent
- [/agent generate](agent-generate.md) - Generate new agent
- [kiro-cli agent](../commands/agent.md) - CLI agent management

## Technical Details

**Discovery**: Scans agent directories in resolution order and includes built-in agents.

**Active Detection**: Compares current session agent with discovered agents to mark active status.