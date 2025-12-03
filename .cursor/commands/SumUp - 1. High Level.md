# Summarize: High Level

## Purpose

Create a concise, high-level summary of the conversation topics.

## **REFERENCE FILES**

- **SUMMARY_OUTPUT**: `.cursor/ADHOC/Summary.md`

## Execution Steps

**CRITICAL: Execution Expectation** → MANDATORY: This command MUST be executed exactly as written with NO DEVIATION, NO SKIPPING, and NO MODIFICATION.

1. **Clear existing summary**: Delete **SUMMARY_OUTPUT** file
2. **COMPLETE CONVERSATION SCOPE**: Summary MUST reflect the ENTIRE conversation from initial prompt to completion, NOT just since "Summarizing Chat Context" creation.
3. **Identify main topics**: Extract the key discussion themes (exclude summary requests)
4. **Create summary**: Write a brief, bullet-point summary
5. **Output**: Save to **SUMMARY_OUTPUT**
6. **Display outline** → MANDATORY: Show ONLY the `### Outline` section from the created summary file in a copy-pasteable markdown code block with:
    - STRICT 2-space indentation (no tabs, exactly 2 spaces per level)
    - FORMATTING RULES for display block:
        - Make items concise first, use sub-bullets only when conciseness isn't possible
        - Preserve original summary file content unchanged

## Output Format

```markdown
# Conversation Summary - High Level

## Topics Discussed

### Outline

- **[Initial Request/Task 1]**:
  - **[Sub-category 1]**:
    - [Sub-task/related work 1]
    - [Sub-task/related work 2]
  - **[Sub-category 2]**:
    - [Sub-task/related work 1]
    - [Sub-task/related work 2]
  - **[Implementation Process]**:
    - [Work done to fulfill the request]
    - **[Current Status]**:
    - [Where things stand now]
        
- **[Only create separate top-level bullets for truly separate requests/tasks]**:
  - [Sub-task/related work 1]
  - [Sub-task/related work 2]

### Chronological (With Concise Topic Points)

- **[Main topic 1 - first discussed]**: [Brief description]
- **[Main topic 2 - second discussed]**: [Brief description]
- **[Additional topics in chronological order]**: [Brief descriptions]

## Summary Text

[Timestamp]: Conversation summary created covering [X] messages.
```

## Usage

Execute when user requests a high-level overview of conversation topics.
