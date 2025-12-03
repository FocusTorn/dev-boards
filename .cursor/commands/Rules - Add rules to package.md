# Rules - Add rules to package

This command helps you add package-level rules to any package in the workspace, following the same pattern used for PAE.

## Usage

```
@Rules - Add rules to package
```

## What it does

1. **Analyzes the target package** to understand its:
    - Architecture patterns
    - Build configuration
    - Testing strategy
    - Dependencies and externalization
    - Current anti-patterns and violations

2. **Creates comprehensive rule file** at `{package-path}/.cursor/rules/{package-name}.mdc` using **Auto Attached** frontmatter containing:
    - Package classification and purpose
    - Architectural principles and patterns
    - Directory structure requirements
    - Configuration system rules
    - Build configuration patterns
    - Testing strategy requirements
    - Critical anti-patterns to avoid
    - Development workflow guidelines
    - Quality gates and compliance triggers
    - Violation prevention patterns

3. **Follows established patterns** from:
    - `libs/project-alias-expander/.cursor/rules/pae.mdc`
    - `libs/.cursor/rules/lib.mdc`
    - `.cursor/rules/_workspace.mdc`

## Package Types Supported

- **Core packages** (`packages/{feature}/core/`)
- **Extension packages** (`packages/{feature}/ext/`)
- **Shared libraries** (`libs/`)
- **Direct exec tools** (`libs/direct-exec-tools/{name}/`)
- **CLI tools** (like PAE)

## Rule File Structure

Each package rule file includes:

### Header Metadata (Frontmatter)

**Recommended Approach - Auto Attached Rules:**

```yaml
---
globs: ['**/*']
alwaysApply: false
---
```

**Why Auto Attached is Better:**

- **Automatic**: Rules automatically attach when files in the package directory are referenced
- **Scoped**: Only applies when working within that specific package
- **Efficient**: No need to specify complex glob patterns
- **Nested**: Follows Cursor's recommended nested rules pattern

**Directory Structure:**

```
project/
  .cursor/rules/                    # Project-wide rules
  libs/
    project-alias-expander/
      .cursor/rules/                # Package-specific rules (Auto Attached)
        pae.mdc
    mock-strategy/
      .cursor/rules/                # Package-specific rules (Auto Attached)
        mock-strategy.mdc
  packages/
    dynamicons/
      core/
        .cursor/rules/              # Core-specific rules (Auto Attached)
          dynamicons-core.mdc
      ext/
        .cursor/rules/              # Extension-specific rules (Auto Attached)
          dynamicons-ext.mdc
```

**Examples:**

For PAE package (Auto Attached):

```yaml
---
globs: ['**/*']
alwaysApply: false
---
```

For a core package (Auto Attached):

```yaml
---
globs: ['**/*']
alwaysApply: false
---
```

**Alternative - Manual Rules (if needed):**

```yaml
---
description: Rules for {package-name} package
globs: ['{package-path}/**/*']
alwaysApply: false
---
```

### Core Sections

- **Package Classification**: Type, purpose, architecture
- **Architectural Principles**: Service patterns, directory structure
- **Configuration System**: Config files, loading rules, validation
- **Build Configuration**: Package.json, project.json patterns
- **Testing Strategy**: Test organization, coverage requirements
- **Critical Anti-Patterns**: Architecture, configuration, execution violations
- **Development Workflow**: Pre-development, development, post-development checklists
- **Quality Gates**: Pre-release checklist and compliance verification
- **Natural Compliance Triggers**: Mental models for different concepts
- **Violation Prevention**: Natural stops and pattern recognition

## Example Output

For a package like `@fux/mock-strategy`, it would create:

```
libs/mock-strategy/.cursor/rules/mock-strategy.mdc
```

With rules covering:

- Mock strategy patterns and best practices
- Service-based architecture requirements
- Configuration loading and validation
- Testing patterns with proper mocking
- Build configuration for shared libraries
- Anti-patterns specific to mocking utilities

## Benefits

- **Consistency**: Ensures all packages follow established patterns
- **Documentation**: Captures lessons learned and architectural decisions
- **Guidance**: Provides clear rules for development and maintenance
- **Violation Prevention**: Identifies common mistakes before they happen
- **Quality Assurance**: Establishes quality gates and compliance checks

## Implementation

The command will:

1. **Analyze package structure** and configuration files
2. **Identify package type** and architectural patterns
3. **Extract lessons learned** from existing implementations
4. **Generate comprehensive rules** following established templates
5. **Create directory structure** if needed (`.cursor/rules/`)
6. **Write rule file** with **Auto Attached** frontmatter (`globs: ['**/*']`, `alwaysApply: false`)
7. **Ensure proper nesting** following Cursor's recommended patterns

This ensures every package has its own governing rules that automatically attach when working within that package, aligning with workspace standards while capturing package-specific patterns and requirements.

## Benefits of Auto Attached Rules

- **Automatic Activation**: Rules automatically apply when files in the package are referenced
- **Scoped Application**: Only affects work within that specific package
- **Clean Frontmatter**: Simple `globs: ['**/*']` instead of complex path patterns
- **Nested Structure**: Follows Cursor's recommended nested rules architecture
- **Maintainable**: Easier to manage and update package-specific rules
