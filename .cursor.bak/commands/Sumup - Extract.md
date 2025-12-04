# Extract: Dynamic Topic Extraction

## Purpose

Extract all information relating to a specific bullet point topic from the conversation and create a comprehensive document.

## Execution Steps

**CRITICAL: Execution Expectation** → MANDATORY: This command MUST be executed exactly as written with NO DEVIATION, NO SKIPPING, and NO MODIFICATION.

1. **Parse bullet point**: Extract the topic text from the provided bullet point (remove `- **` and `**:`)
2. **Search conversation**: Extract ALL mentions, discussions, implementations, and details related to the topic
3. **Comprehensive extraction**: Include:
    - Initial requests or requirements
    - Implementation approaches discussed
    - Code snippets or configurations
    - Technical decisions made
    - Challenges encountered and solutions
    - Current status or completion state
    - Any related context or background information
4. **Create extraction document**: Write comprehensive extraction to `.cursor/ADHOC/extracted___{TOPIC}.md`
5. **Document structure**: Use clear headings and preserve all technical details
6. **Display confirmation**: Show the created file path and brief summary of extracted content

## File Naming Convention

- Extract topic text from bullet point
- Convert to kebab case (spaces → hyphens, remove special characters)
- Prefix with `extracted___`
- Example: `- **MCP Documentation Server Creation**:` → `extracted___MCP-Documentation-Server-Creation.md`

## Output Format

```markdown
# Extracted: {TOPIC}

## Overview

[Brief description of what the topic involves]

## Initial Requirements/Requests

[What was originally requested or needed]

## Implementation Details

[Technical approaches, code, configurations discussed]

## Technical Decisions

[Key decisions made during implementation]

## Challenges & Solutions

[Problems encountered and how they were resolved]

## Current Status

[Where the implementation stands now]

## Related Context

[Any additional relevant information]
```

## Usage

Execute with: `/@Sumup - Extract.md - **{TOPIC}**:`

Example: `/@Sumup - Extract.md - **MCP Documentation Server Creation**:`
