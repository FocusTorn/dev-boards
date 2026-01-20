---
trigger: always_on
---

# Rule Files Structure and Format Rules

## CRITICAL EXECUTION DIRECTIVE

**AI Agent Directive**: Follow rule files structure and format rules exactly for all rule file creation, organization, and maintenance tasks.

**MANDATORY EXECUTION PROTOCOL**:

1. **NO DEVIATION**: All rule file structure and format rules must be followed exactly as written
2. **NO SKIPPING**: No steps may be skipped, abbreviated, or modified
3. **NO SELECTIVE COMPLIANCE**: All rules apply to all rule file activities
4. **FAILURE TO COMPLY**: Violating these rules constitutes a critical protocol violation

## RULES DIRECTORY STRUCTURE

### 1. :: Directory Organization

**✅ CORRECT - Rules directory structure**:

```
.agent/rules/
├── [universal-rule].mdc              # Universal rules (root level)
├── by-language/
│   └── [language]/
│       └── [language-specific-rule].mdc
├── tool/
│   └── [tool-name]/
│       └── [tool-specific-rule].mdc
├── formatting/
│   └── [formatting-rule].mdc
├── dev-boards/
│   └── [project-specific-rule].mdc
└── [other-category]/
    └── [category-rule].mdc
```

**✅ CORRECT - Universal rules (root level)**:

```
.agent/rules/
├── code-maintenance.mdc               # Universal code maintenance rules
├── documentation.mdc                  # Universal documentation rules
└── [other-universal-rule].mdc         # Other universal rules
```

**✅ CORRECT - Language-specific rules**:

```
.agent/rules/by-language/
└── python/
    ├── code-organization.mdc          # Python code organization
    ├── code-structure.mdc             # Python code structure
    └── script-to-package.mdc          # Python script to package migration
```

**✅ CORRECT - Tool-specific rules**:

```
.agent/rules/tool/
└── uv/
    └── workspace.mdc                  # UV workspace configuration
```

**✅ CORRECT - Formatting rules**:

```
.agent/rules/formatting/
├── AI-Agent-Document.mdc              # AI agent document formatting
├── markdown.mdc                       # Markdown formatting
├── summary.mdc                        # Summary formatting
├── terminal-output.mdc                # Terminal output formatting
└── test.mdc                           # Test file formatting
```

**✅ CORRECT - Project-specific rules**:

```
.agent/rules/dev-boards/
└── project-level-workspace.mdc        # Dev-boards project-specific rules
```

**❌ INCORRECT - Wrong rule placement**:

```
# Wrong: Language-specific rule at root level
.agent/rules/python-code-organization.mdc

# Wrong: Universal rule in language directory
.agent/rules/by-language/python/code-maintenance.mdc

# Wrong: Tool-specific rule at root level
.agent/rules/uv-workspace.mdc

# Wrong: Project-specific rule at root level
.agent/rules/dev-boards-workspace.mdc
```

### 2. :: Universal vs Project-Level Rules

**✅ CORRECT - Universal rules characteristics**:

- **Location**: Root level of `.agent/rules/` directory
- **Scope**: Apply to all projects and workspaces
- **Content**: Generic patterns, best practices, universal conventions
- **Examples**: Use generic placeholders (e.g., `workspace-member`, `some-package`)
- **Configuration**: Typically `trigger: always_on` (applies to all files)

**✅ CORRECT - Project-level rules characteristics**:

- **Location**: Project-specific directory (e.g., `.agent/rules/dev-boards/`)
- **Scope**: Apply only to specific project/workspace
- **Content**: Project-specific configurations, preferences, examples
- **Examples**: Can reference actual project paths and packages
- **Configuration**: Typically `trigger: always_on` (applies within project context)

**✅ CORRECT - Universal rule example**:

```markdown
# code-maintenance.mdc (universal)
## BACKWARD COMPATIBILITY REMOVAL

**✅ CORRECT - Remove all backward compatibility code immediately**:

```python
# Generic example - no project-specific references
from ..core.prompts import confirm, HAS_PROMPT_TOOLKIT
proceed = confirm("Proceed?", default=True, indent="")
```
```

**✅ CORRECT - Project-level rule example**:

```markdown
# dev-boards/project-level-workspace.mdc (project-specific)
## PACKAGE MANAGER PREFERENCE

**✅ CORRECT - UV is the preferred Python package manager for dev-boards projects**:

- UV is the preferred package manager for all Python projects in the dev-boards workspace
- Reference: See `tool/uv/workspace.mdc` for complete UV workspace configuration rules
```

**❌ INCORRECT - Mixing universal and project-specific content**:

```markdown
# Wrong: Universal rule with project-specific examples
# code-maintenance.mdc
```python
# Wrong: Using project-specific path
from bootstraps.git_py.core.prompts import confirm
```

# Wrong: Project-level rule with generic patterns that should be universal
# dev-boards/project-level-workspace.mdc
## UV WORKSPACE CONFIGURATION
# This should be in tool/uv/workspace.mdc (universal)
```

## RULE FILE FORMAT

### 1. :: File Header Structure

**✅ CORRECT - Standard rule file header**:

```markdown
---
*.py

---

# [Rule Category] Rules

## CRITICAL EXECUTION DIRECTIVE

**AI Agent Directive**: Follow [rule category] rules exactly for all [activity description] tasks.

**MANDATORY EXECUTION PROTOCOL**:

1. **NO DEVIATION**: All [rule category] rules must be followed exactly as written
2. **NO SKIPPING**: No steps may be skipped, abbreviated, or modified
3. **NO SELECTIVE COMPLIANCE**: All rules apply to all [activity] activities
4. **FAILURE TO COMPLY**: Violating these rules constitutes a critical protocol violation
```

**✅ CORRECT - Universal rule header**:

```markdown
---
trigger: always_on
---

# [Universal Rule Category] Rules
```

**✅ CORRECT - Language-specific rule header**:

```markdown
---
*.py

---

# Python [Rule Category] Rules
```

**✅ CORRECT - Tool-specific rule header**:

```markdown
---
trigger: always_on
---

# [Tool Name] [Rule Category] Rules
```

**❌ INCORRECT - Missing or incorrect header**:

```markdown
# Wrong: Missing frontmatter
# Code Maintenance Rules

# Wrong: Missing globs for language-specific rules
---

---

# Wrong: Using trigger: always_on for language-specific rules
---
*.py
trigger: always_on  # Should be false for language-specific
---
```

### 2. :: Section Structure

#### Header Formatting Rule

**CRITICAL**: Headers (`#`, `##`, `###`, `####`, etc.) must NEVER be bolded in the source markdown.

**✅ CORRECT - Plain text headers**:

```markdown
## PRIMARY SECTION

### 1. :: Subsection with Numbering

#### Sub-subsection (if needed)

**✅ CORRECT - Correct pattern**:

[Correct examples and patterns]

**❌ INCORRECT - Incorrect pattern**:

[Incorrect examples and anti-patterns]
```

**✅ CORRECT - Required sections**:

```markdown
## CRITICAL EXECUTION DIRECTIVE
[Directive and protocol]

## [TOPIC] RULES
[Main rule content with subsections]

## ANTI-PATTERNS
[Violations and incorrect patterns]

## QUALITY GATES
[Checklist items]

## SUCCESS METRICS
[Success criteria]
```

**❌ INCORRECT - Missing required sections**:

```markdown
# Wrong: Missing CRITICAL EXECUTION DIRECTIVE
## RULES

# Wrong: Missing ANTI-PATTERNS
## RULES
## QUALITY GATES

# Wrong: Missing SUCCESS METRICS
## RULES
## ANTI-PATTERNS
```

### 3. :: Content Formatting

**✅ CORRECT - Code example format**:

```markdown
**✅ CORRECT - Description**:

```python
# Code example
def example():
    pass
```

**❌ INCORRECT - Description**:

```python
# Wrong code example
def bad_example():
    pass
```
```

**✅ CORRECT - Anti-pattern format**:

```markdown
## ANTI-PATTERNS

### ❌ [Category] Violations

- ❌ **Violation Name** - Description of violation
- ❌ **Another Violation** - Description of another violation
```

**✅ CORRECT - Quality gates format**:

```markdown
## QUALITY GATES

- [ ] **Checkpoint Name**: Description of checkpoint
- [ ] **Another Checkpoint**: Description of another checkpoint
```

**✅ CORRECT - Success metrics format**:

```markdown
## SUCCESS METRICS

After implementing proper [topic]:

- ✅ **Metric Name** - Description of metric
- ✅ **Another Metric** - Description of another metric
```

**❌ INCORRECT - Inconsistent formatting**:

```markdown
# Wrong: Missing checkmarks in success metrics
## SUCCESS METRICS
- Metric Name - Description

# Wrong: Missing checkboxes in quality gates
## QUALITY GATES
- Checkpoint Name

# Wrong: Missing ❌ in anti-patterns
## ANTI-PATTERNS
- Violation Name - Description

# Wrong: Bolded headers
## **SUCCESS METRICS**  <!-- Headers should not be bolded -->
```

## STYLE AND TONE

### 1. :: Writing Style

**✅ CORRECT - Directive and authoritative tone**:

```markdown
**AI Agent Directive**: Follow [rule category] rules exactly for all [activity] tasks.

**MANDATORY EXECUTION PROTOCOL**:

1. **NO DEVIATION**: All rules must be followed exactly as written
2. **NO SKIPPING**: No steps may be skipped, abbreviated, or modified
```

**✅ CORRECT - Clear, actionable language**:

```markdown
**✅ CORRECT - Remove all backward compatibility code immediately**:

```python
# Clear action: Remove legacy code
# Clear pattern: Use new API
```

**❌ INCORRECT - Vague or passive language**:

```markdown
# Wrong: Vague directive
**AI Agent Directive**: Consider following some rules when possible.

# Wrong: Passive language
**MANDATORY EXECUTION PROTOCOL**:
1. Rules should probably be followed
2. It might be good to not skip steps

# Wrong: Unclear action
**✅ CORRECT - Maybe remove compatibility code**:
```

### 2. :: Example Quality

**✅ CORRECT - Complete, executable examples**:

```markdown
**✅ CORRECT - Complete example**:

```python
#!/usr/bin/env python3
"""Complete example with all necessary imports."""
import sys
from pathlib import Path

def example_function():
    """Complete function implementation."""
    return True

if __name__ == "__main__":
    sys.exit(0)
```
```

**✅ CORRECT - Generic placeholders in universal rules**:

```markdown
**✅ CORRECT - Generic example**:

```python
# Use generic placeholders
package_name = "workspace-member"
dependency = "some-package"
```
```

**✅ CORRECT - Project-specific examples in project-level rules**:

```markdown
**✅ CORRECT - Project-specific example**:

```python
# Can use actual project paths in project-level rules
package_name = "___shared/shared-python"
```
```

**❌ INCORRECT - Incomplete or project-specific in universal rules**:

```markdown
# Wrong: Incomplete example
**✅ CORRECT - Example**:
```python
def example():
    # Missing imports, incomplete implementation
    pass
```

# Wrong: Project-specific in universal rule
**✅ CORRECT - Example**:
```python
# Wrong: Using actual project path in universal rule
from bootstraps.git_py.core import something
```
```

### 3. :: Consistency Requirements

**✅ CORRECT - Consistent terminology**:

```markdown
# Use consistent terms throughout
- "workspace member" (not "project" or "module")
- "package manager" (not "package installer" or "dependency tool")
- "locally installable package" (not "editable package" or "development package")
```

**✅ CORRECT - Consistent formatting**:

```markdown
# Use consistent markdown formatting
## SECTION NAME  # Headers are NOT bolded - they're already bold when rendered
### 1. :: Subsection  # Headers are NOT bolded, numbered subsections
**✅ CORRECT**  # Always bold correct examples
**❌ INCORRECT**  # Always bold incorrect examples
```

**❌ INCORRECT - Inconsistent terminology**:

```markdown
# Wrong: Mixing terms
- "workspace member" in one place
- "project" in another place
- "module" in yet another place

# Wrong: Inconsistent formatting
## **Section Name**  # Wrong: Headers should NOT be bolded
### **Subsection**  # Wrong: Headers should NOT be bolded
✅ CORRECT  # Wrong: Should be bolded
```

## FILE NAMING CONVENTIONS

### 1. :: Naming Patterns

**✅ CORRECT - Descriptive, kebab-case names**:

```markdown
# Universal rules
code-maintenance.mdc
documentation.mdc

# Language-specific rules
code-organization.mdc
code-structure.mdc
script-to-package.mdc

# Tool-specific rules
workspace.mdc  # In tool/uv/ directory

# Formatting rules
terminal-output.mdc
AI-Agent-Document.mdc

# Project-specific rules
project-level-workspace.mdc  # In dev-boards/ directory
```

**✅ CORRECT - Clear, specific names**:

```markdown
# Good: Clear purpose
script-to-package.mdc          # Migration from script to package
code-organization.mdc           # Code organization patterns
terminal-output.mdc            # Terminal output formatting
```

**❌ INCORRECT - Vague or unclear names**:

```markdown
# Wrong: Too vague
rules.mdc
python.mdc
formatting.mdc

# Wrong: Unclear purpose
package.mdc
workspace.mdc  # At root level (should be in tool/uv/)

# Wrong: Wrong naming convention
CodeOrganization.mdc          # Should be kebab-case
code_organization.mdc         # Should use hyphens, not underscores
```

## CONFIGURATION OPTIONS

### 1. :: Glob Patterns

**✅ CORRECT - Language-specific globs**:

```markdown
---
*.py

---
```

**✅ CORRECT - File-type-specific globs**:

```markdown
---
*.mdc

---
```

**✅ CORRECT - Multiple file types**:

```markdown
---
*.{py,pyi}

---
```

**❌ INCORRECT - Wrong glob usage**:

```markdown
# Wrong: Using globs with trigger: always_on
---
*.py
trigger: always_on  # Should be false when using globs
---

# Wrong: Missing globs for language-specific rules
---
  # Should have globs for language-specific
---
```

### 2. :: Always Apply Configuration

**✅ CORRECT - Universal rules**:

```markdown
---
trigger: always_on
---
```

**✅ CORRECT - Tool-specific rules**:

```markdown
---
trigger: always_on
---
```

**✅ CORRECT - Project-level rules**:

```markdown
---
trigger: always_on
---
```

**✅ CORRECT - Language-specific rules**:

```markdown
---
*.py

---
```

**❌ INCORRECT - Wrong alwaysApply usage**:

```markdown
# Wrong: trigger: always_on with globs
---
*.py
trigger: always_on  # Should be false
---

# Wrong:  for universal rules
---
  # Universal rules should be true
---
```

## ANTI-PATTERNS

### ❌ Rule File Structure Violations

- ❌ **Wrong Rule Placement** - Don't put language-specific rules at root level
- ❌ **Wrong Rule Placement** - Don't put tool-specific rules at root level
- ❌ **Wrong Rule Placement** - Don't put project-specific rules at root level
- ❌ **Mixing Universal and Project-Specific** - Don't put project-specific examples in universal rules
- ❌ **Missing Required Sections** - Don't create rule files without CRITICAL EXECUTION DIRECTIVE, ANTI-PATTERNS, QUALITY GATES, SUCCESS METRICS
- ❌ **Inconsistent Formatting** - Don't use inconsistent markdown formatting or terminology
- ❌ **Bolded Headers** - Don't use bold formatting (`**`) on headers - headers are already bold when rendered
- ❌ **Vague File Names** - Don't use vague or unclear file names
- ❌ **Wrong Configuration** - Don't use `trigger: always_on` with globs
- ❌ **Missing Globs** - Don't create language-specific rules without glob patterns

## QUALITY GATES

- [ ] **Correct Directory**: Rule file is in appropriate directory (root, by-language, tool, formatting, or project-specific)
- [ ] **Correct Header**: Rule file has proper frontmatter with appropriate globs and alwaysApply settings
- [ ] **Required Sections**: Rule file includes CRITICAL EXECUTION DIRECTIVE, main rules section, ANTI-PATTERNS, QUALITY GATES, SUCCESS METRICS
- [ ] **Consistent Formatting**: All sections use consistent markdown formatting (no bolded headers, checkmarks, etc.)
- [ ] **No Bolded Headers**: Headers are NOT bolded - they're already bold when rendered
- [ ] **Clear Examples**: All examples are complete, executable, and use appropriate placeholders
- [ ] **Appropriate Scope**: Universal rules use generic examples, project-level rules can use project-specific examples
- [ ] **Descriptive Name**: File name clearly describes the rule category and purpose
- [ ] **Proper Configuration**: Language-specific rules use globs with ``, universal rules use `trigger: always_on`

## SUCCESS METRICS

After implementing proper rule file structure and format:

- ✅ **Clear Organization** - Rules are organized in logical directory structure
- ✅ **Proper Separation** - Universal and project-specific rules are clearly separated
- ✅ **Consistent Format** - All rule files follow the same structure and formatting
- ✅ **No Bolded Headers** - Headers are not bolded in source, keeping markdown clean and readable
- ✅ **Easy Discovery** - Rules are easy to find based on their category and location
- ✅ **Maintainable Structure** - Rule files can be easily updated and maintained
- ✅ **Clear Scope** - Universal vs project-specific rules are clearly differentiated
- ✅ **Complete Documentation** - All rule files include required sections and examples

