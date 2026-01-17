# Rule Hierarchy & Scope Policy

This document defines the hierarchy and placement policies for AI agent rules.

## **HIERARCHY**

### **1. Universal Rules**
**Path**: `.agent/rules/*.md` or `.agent/rules/by-language/<language>/*.md`
**Scope**: Apply to the entire workspace and all projects.
**Content**:
- General coding standards (formatting, naming conventions).
- Language-specific idioms and best practices (e.g., Rust idioms, Python patterns).
- Development workflows (git usage, PR process).
- Architectural principles (Clean Architecture, error handling patterns).

### **2. Project-Specific Rules**
**Path**: `projects/<project_name>/.agent/rules/*.md`
**Scope**: Apply ONLY when working within the specific project directory.
**Content**:
- Project-specific business logic references.
- Infrastructure and dependency injection specifics.
- Configuration schemas unique to the project.
- Deviations from universal rules (explicitly documented as overrides).

## **PLACEMENT GUIDELINES**

### **When to Move a Rule to Universal**
If a rule meets **ANY** of the following criteria, it belongs in the **Universal** rules:
1.  **Generic Pattern**: It describes a coding pattern applicable to any project in that language (e.g., "Use Result Enums for state transitions").
2.  **Tool Usage**: It describes how to use a standard tool or library used across projects (e.g., "TUI Layout Caching" using `ratatui`).
3.  **Process**: It describes a general refactoring or verification process.

### **When to Keep a Rule in Project-Specific**
If a rule meets **ALL** of the following criteria, it belongs in **Project-Specific** rules:
1.  **Specific Types**: It references specific domain types, structs, or enums unique to that project (e.g., `SettingsField` enum in `dev-console`).
2.  **Strictly Local**: The pattern solves a problem unique to that project's specific architecture or constraints.
3.  **Non-Portable**: The code or pattern would strictly fail to compile or make sense in another project context.

## **MIGRATION PROTOCOL**

When identifying general patterns in project rules:
1.  **Extract**: Generalize the pattern (remove specific type names if possible, or use them as generic examples).
2.  **Move**: Place the generalized rule in the appropriate `.agent/rules/by-language/<language>/` file.
3.  **Reference**: In the project rule file, remove the detailed explanation and refer to the universal rule, noting only project-specific overrides or specific implementations.
