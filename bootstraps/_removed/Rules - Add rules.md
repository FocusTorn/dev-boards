# Rules - Add rules

This command helps you add rules to any rule file in the workspace, following established patterns and formatting requirements.

## Usage

```
@Rules - Add rules
```

When prompted, specify the target rule file using one of these formats:

- **Path format**: `@.cursor/rules/formatting/AI-Agent-Document.mdc`
- **Subject matter**: `AI-Agent-Document` (will resolve to the appropriate path)

## What it does

1. **Resolves the target rule file** from the provided subject matter or path:
    - If path format (`@.cursor/rules/...`), uses the exact path
    - If subject matter, searches for matching rule files in `.cursor/rules/`
    - Validates that the target file exists or can be created

2. **Analyzes the target rule file** to understand its:
    - Current structure and sections
    - Formatting patterns and requirements
    - Existing rules and anti-patterns
    - Reference files and dependencies
    - Quality gates and compliance checks

3. **Adds new rules** following the file's established patterns:
    - Maintains consistent formatting and structure
    - Follows the document's section organization
    - Preserves existing rules and anti-patterns
    - Integrates new rules seamlessly with existing content
    - Ensures compliance with document-specific formatting requirements

4. **Follows established patterns** from:
    - `.cursor/rules/formatting/AI-Agent-Document.mdc` (for AI agent documents)
    - `.cursor/rules/_workspace.mdc` (for workspace rules)
    - Other rule files in `.cursor/rules/` (for domain-specific rules)

## Rule File Types Supported

- **Formatting rules** (`.cursor/rules/formatting/`)
- **Workspace rules** (`.cursor/rules/`)
- **Package rules** (`.cursor/rules/` in packages)
- **Domain-specific rules** (any `.mdc` file in `.cursor/rules/`)

## Rule File Structure

Each rule file may have different structures, but common patterns include:

### Header Metadata (Frontmatter)

```yaml
---
alwaysApply: true
---

# or

---
globs: ['**/*']
alwaysApply: false
---
```

### Common Sections

- **REFERENCE FILES** - Documentation and related rule references
- **CRITICAL EXECUTION DIRECTIVE** - Mandatory protocols
- **RULES** - Specific rules and requirements
- **PATTERNS** - Implementation patterns and examples
- **ANTI-PATTERNS** - Violations to avoid
- **QUALITY GATES** - Compliance validation checkpoints
- **VIOLATION PREVENTION** - Natural stops and pattern recognition

## Example Usage

### Adding rules to a formatting document:

```
@Rules - Add rules
@.cursor/rules/formatting/AI-Agent-Document.mdc
```

### Adding rules by subject matter:

```
@Rules - Add rules
AI-Agent-Document
```

## Implementation

The command will:

1. **Resolve target file** from subject matter or path
2. **Read existing rule file** to understand structure and patterns
3. **Analyze formatting requirements** from the file's structure
4. **Prompt for new rules** or accept rules from context
5. **Integrate new rules** following the file's established patterns
6. **Maintain formatting compliance** with document-specific requirements
7. **Preserve existing content** while adding new rules appropriately
8. **Validate structure** to ensure compliance with formatting rules

## Benefits

- **Consistency**: Ensures all rules follow established patterns
- **Integration**: Seamlessly adds rules to existing rule files
- **Compliance**: Maintains formatting and structure requirements
- **Context Awareness**: Understands existing rules to avoid conflicts
- **Pattern Preservation**: Follows the document's established organization

This ensures rules are added in a way that maintains consistency with existing rule files while following the specific formatting and structural requirements of each document type.

