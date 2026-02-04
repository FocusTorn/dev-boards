# Tasks: Dev-Console V2 System Restructure

## [CODEBASE REUSE ANALYSIS]
Scanned: projects/dev-console2/src/

[EXISTING - REUSE]
- ✅ `InteractiveWidget` trait (`src/widgets/mod.rs`)
- ✅ `WidgetOutcome` enum (`src/widgets/mod.rs`)
- ✅ `executors.rs` logic (to be split)
- ✅ `config.rs` logic (to be split)
- ✅ Modal focus pattern (`src/app/mod.rs:565`)

[NEW - CREATE]
- 🆕 `ComponentManager` (`src/widgets/manager.rs`)
- 🆕 `tui_component!` macro
- 🆕 `src/widgets/elements/` directory
- 🆕 `src/widgets/components/` directory

## [DEPENDENCY GRAPH]
Story completion order:
1. Phase 1: Design Mockups (UI-First)
2. Phase 2: Widget Tiering & Pre-Cleanup
3. Phase 3: Breaking Down God Objects (Blocking Foundation)
4. Phase 4: Component Manager & Registry
5. Phase 5: Encapsulation & Final Migration

## [PARALLEL EXECUTION OPPORTUNITIES]
- Phase 2.2 and Phase 2.3 can be done in parallel (independent file splits).
- Phase 3.1 (Logic) and Phase 3.2 (Macro) can be developed in parallel before integration.

## [IMPLEMENTATION STRATEGY]
**MVP Scope**: Successful modularization of one "Smart" component (e.g., Toast) using the new Registry.
**Testing approach**: TDD (red-green-refactor) for trait implementations and macro generation.

---

## Phase 1: Design Mockups (UI-First)

- [x] T001 [DESIGN] Create terminal mockup navigation hub
  - Output: `projects/dev-console2/_docs/mockups/index.html`
  - Include: Layout for Title Bar, Tabbed Bar, Main View, and Status Bar.
  - From: spec.md [SC-002]

- [x] T002 [DESIGN] Create "Smart Dropdown" component mockup
  - Output: `projects/dev-console2/_docs/mockups/dropdown.html`
  - States: Closed, Open (overlapped), Focused, Selected.
  - Requirement: Accurately represent the "intended look" of the dropdown.

- [x] T003 [DESIGN] Create "File Browser" modal mockup
  - Output: `projects/dev-console2/_docs/mockups/file_browser.html`
  - Goal: Visualize focus delegation (background dimming, modal focus).
  - From: spec.md [Scenario 2]

- [ ] T004 [APPROVAL-GATE] Review and approve TUI visual prototypes
  - Preview: Open `mockups/*.html` in browser.
  - BLOCKS: All implementation tasks.

---

## Phase 2: Widget Tiering & Pre-Cleanup

- [ ] T010 [RED] Create directory structure and move elements
  - Action: Create `src/widgets/elements/` and `src/widgets/components/`.
  - From: plan.md [Task 1.1, 1.2]

- [ ] T011 [GREEN] Update imports and migrate to `ratatui::prelude`
  - Action: Update `ProgressBar`, `StatusBox`, `Dimmer` paths.
  - From: spec.md [SC-004]

- [ ] T012 [REFACTOR] Standardize `InteractiveWidget` trait outcomes
  - From: plan.md [Task 1.4]

---

## Phase 3: Breaking Down God Objects

- [ ] T020 [RED] Split `app/mod.rs` state and actions
  - Output: `src/app/state.rs`, `src/app/actions.rs`.
  - From: plan.md [Task 2.1]

- [ ] T021 [GREEN] Implement `App` routing delegation
  - Output: `src/app/router.rs`.
  - From: spec.md [FR-041]

- [ ] T022 [REFACTOR] Decompose `executors.rs`
  - Output: `arduino.rs`, `serial.rs`, `system.rs`.
  - From: spec.md [FR-042]

- [ ] T023 [US] Modularize Configuration management
  - Output: `src/config/profiles.rs`, `hardware.rs`, `ui.rs`.
  - From: spec.md [FR-043]

---

## Phase 4: Component Manager & Registry

- [ ] T030 [RED] Implement `ComponentManager` focus stack
  - Output: `src/widgets/manager.rs`.
  - From: spec.md [FR-031]

- [ ] T031 [GREEN] Create `tui_component!` macro
  - Action: Generate `ComponentRegistry` enum with static dispatch.
  - From: spec.md [FR-021]

- [ ] T032 [US] Refactor `App` event dispatching
  - Action: Delegate `key` and `mouse` events to `ComponentManager`.
  - From: spec.md [FR-032, FR-033]

---

## Phase 5: Encapsulation & Final Migration

- [ ] T040 [US] Encapsulate "Smart" components (Toast/File Browser)
  - Action: Move logic to `src/widgets/components/<name>/`.
  - From: spec.md [FR-011]

- [ ] T041 [US] Implement local `config.yaml` loading
  - From: spec.md [FR-012, Scenario 3]

- [ ] T042 [REFACTOR] Final cleanup and linting
  - Requirement: SC-001 (File lines < 400).
  - From: spec.md [NFR-001]
