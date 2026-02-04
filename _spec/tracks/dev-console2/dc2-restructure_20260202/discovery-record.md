# Discovery Record: dc2-restructure_20260202

## 2026-02-02: Architectural Brainstorming Transcript

> Create a new track based on D:/_dev/_Projects/dev-boards/_spec/tracks/dev-console2/dc2-restructure_20260202/2026-02-02-system-restructure-plan.md

**Orchestrator**: The restructure plan is ambitious... which area should we prioritize for the first phase?
- **A) Widget Tiering First**
- **B) App Decomposition First**
- **C) Configuration & Cleanup First**

> a

**Orchestrator**: [Proposed moving component-specific config/state into component directories] Should we:
- **A) Fully Encapsulated** (Move State/Config into component folder)
- **B) Split View/State**
- **C) Hybrid**

> I think I would prefer to have the smart ones fully encapsulated.. should make portability easier, esqpecially when we start writing the macros for their usage

**Orchestrator**: [Proposed Registry Macro] Should we use Dynamic Dispatch or a Static Enum-based Registry?
- **A) Dynamic Dispatch**
- **B) Static Enum**

> how much better performance and what would updating the enum look like

**Orchestrator**: [Explained performance is negligible; explained Enum boilerplate vs Dynamic Dispatch; recommended "Hybrid" Macro-generated Enum Registry]

> Love it

**Orchestrator**: [Presented Section 1: Trait & Macro; Section 2: Internal Structure & `on_tick`; Section 3: Data Flow & Focus Stack]

> I would prefer that each component has its config.yaml in its directory

**Orchestrator**: [Updated design to prioritize local `config.yaml` for ultimate encapsulation]

> Yes

---

## Finalized Conceptual Design
- **Tiered Structure**: `elements/` (dumb) vs `components/` (smart).
- **Encapsulation**: Each smart component directory owns its `mod.rs`, `state.rs`, `view.rs`, and `config.yaml`.
- **Registry**: Macro-generated Enum Registry for static dispatch without boilerplate.
- **Routing**: `ComponentManager` with a focus stack for input delegation.
- **Update**: `on_tick` method added to `Component` trait for time-based logic.