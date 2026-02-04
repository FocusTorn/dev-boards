# Implementation Notes: DC2 Restructure

## Key Decisions

### 1. Hybrid Macro Registry
- **Decision**: Use a macro to generate an `enum ComponentRegistry` that wraps all "Smart" components.
- **Rationale**: Provides the performance of static dispatch (no `Box<dyn Component>`) while eliminating the manual boilerplate of adding variants to the enum and match statements in the update loop.

### 2. Local `config.yaml`
- **Decision**: Each component directory in `src/widgets/components/` will contain its own `config.yaml`.
- **Rationale**: Enables ultimate encapsulation. Moving a component folder to a new project should ideally bring all its configuration with it.

### 3. Focus Stack
- **Decision**: Implement a `Vec<ComponentId>` in `App` to manage focus.
- **Rationale**: Simplifies modal interactions (e.g., a File Browser opening over a Command List). The top of the stack always receives input.

## Technical Context
- **Rust Edition**: 2021
- **TUI Framework**: `ratatui`
- **YAML Parser**: `serde-saphyr`

## Research Items
- [ ] Evaluate `proc-macro2` vs declarative `macro_rules!` for the registry generator.
- [ ] Profile the impact of focus-delegated event handling vs monolithic matching.

## Phase 2: Tasks (2026-02-03)

**Summary**:
- Total tasks: 17
- User story tasks: 10
- Parallel opportunities: 4 (Phase 2.2/2.3, Phase 3.1/3.2)
- Task file: projects/dev-console2/_docs/spec-flow/tasks.md

**Checkpoint**:
- ✅ Tasks generated with --ui-first priority
- ✅ User story organization: Complete
- ✅ Dependency graph: Created
- 📋 Ready for: /implement (starting with mockups)
