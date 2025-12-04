# Summarize: Testing

## Purpose

Create a comprehensive testing-focused summary of code changes, test assumptions, and lessons learned from the conversation.

## **REFERENCE FILES**

- **SUMMARY_OUTPUT**: `.cursor/ADHOC/testing-summary.md`

## Execution Steps

**CRITICAL: Execution Expectation** → MANDATORY: This command MUST be executed exactly as written with NO DEVIATION, NO SKIPPING, and NO MODIFICATION.

1. **CONVERSATION SCOPE**:
    - If this command was NOT called before in this conversation: Reflect the ENTIRE conversation from initial prompt to completion
    - If this command WAS called before in this conversation: Start from that previous command execution point, NOT from initial prompt
2. **Identify testing-related content**: Extract code changes, test assumptions, and lessons learned
3. **Prevent duplicates**: Do NOT include entries that were already captured in previous executions of this command
4. **Create summary**: Write structured summary with categorized sections (incremental if previous exists)
5. **Output**: Save to **SUMMARY_OUTPUT**
6. **Display summary** → MANDATORY: Show the complete summary in a copy-pasteable markdown code block with:
    - STRICT 2-space indentation (no tabs, exactly 2 spaces per level)
    - FORMATTING RULES for display block:
        - Make items concise first, use sub-bullets only when conciseness isn't possible
        - Preserve original summary file content unchanged

## Output Format

```markdown
# Testing Summary

## Code Changes Made

### **[Package/Feature Name]**:

- **[Change Category]**:
    - [Specific change 1]
    - [Specific change 2]
- **[Files Modified]**:
    - [File path 1]: [Description of changes]
    - [File path 2]: [Description of changes]

## Test Assumptions Changed

### **[Test Category]**:

- **[Original Assumption]**: [What was assumed before]
- **[New Assumption]**: [What was changed to]
- **[Reason for Change]**: [Why the assumption changed]

## Lessons Learned

### **[Document Category]** (e.g., Testing Strategy, Architecture, SOP):

- **[Lesson Category]**:
    - [Specific lesson 1]
    - [Specific lesson 2]
- **[Action Items]**:
    - [Action item 1]
    - [Action item 2]

## Summary Text

[Timestamp]: Testing summary created covering [X] messages with focus on code changes, test assumptions, and lessons learned.
```

## Usage

Execute when user requests a testing-focused summary of conversation changes and learnings.
