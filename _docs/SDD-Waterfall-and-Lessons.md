# SDD Logic Waterfall & Session Lessons Learned

## 1. :: The SDD Logic Waterfall (Dry Run)
_A comparison of the three primary workflows in the Spec-Driven Development ecosystem._

| Feature | `/brainstorm` | `spec-flow` (Spec > Plan > Task) | `/spec:newProject` |
| :--- | :--- | :--- | :--- |
| **Primary Goal** | **Divergent Thinking**: Explore possibilities, edge cases, and architectures. | **Convergent Execution**: Narrow down a specific set of changes into code. | **Structural Foundation**: Create a new identity and "home" for code. |
| **Response Style** | Expansive, investigative, option-heavy. Uses "What if?" | Strict, procedural, gate-heavy. Uses "We will do X." | Scaffold-oriented, configuration-driven. Uses "Setting up Y." |
| **Artifacts** | None (unless summary requested). | `spec.md`, `plan.md`, `tasks.md`. | `_spec/_meta/`, `architecture.md`, `north_star.md`. |
| **Focus** | Requirements & Concepts. | Implementation & Verification. | Long-term roadmap & Tooling. |

### Case Study: "Add an MQTT Dashboard"

#### FLOW A: `/brainstorm`
- **Focus**: Requirements & Design.
- **AI Response**: "Let's explore architectural paths: Integrated Tab vs Standalone Tool. What happens if the broker is offline? Should we support SSL? Should the UI be a grid of gauges or a scrolling log?"

#### FLOW B: `/spec:newProject`
- **Focus**: Metadata & Scaffolding.
- **AI Response**: "Initializing `mqtt-ui` project. ID: `mqtt-ui`. Path: `projects/mqtt-ui`. Flavor: `TUI-Ratatui`. North Star: High-speed metrics visualizer. Action: `cargo init`. Registering in `_spec/index.md`."

#### FLOW C: `spec-flow`
- **Focus**: Implementation & TDD.
- **AI Response**: "Track: `mqtt_ui_dashboard_20260131`. Phase 1 RED: Writing test for `MqttDashboardWidget`. FAILURE: Widget not found. IMPLEMENT: Creating `src/widgets/mqtt_dashboard.rs`. GREEN: Test passed."

---

## 2. :: Lessons Learned (Session: device_mgmt_20260127)
_Key insights extracted from the Device Management refactor._

### TDD Integrity: The "Helpful Test" Trap
- **Learning**: A test that passes does not guarantee a correct implementation if the test setup manually performs logic that the production code is responsible for.
- **Pattern**: Zero-Assistance Verification.
- **Correction**: Reverted implementation, removed manual layout calls from tests, verified failure (RED), then implemented automatic recalculation in executors.
- **Rule Proposal**: ALWAYS verify that tests fail when automatic logic is missing. NEVER perform manual state calculations in a test that the production code handles automatically.

### Production Data Protection in Testing
- **Learning**: Automated tests must be strictly isolated from production configuration files to prevent accidental data loss.
- **Pattern**: Configurable Path Injection.
- **Correction**: Added `profile_config_path` to `App` struct; updated tests to use `test_config.yaml`.
- **Rule Proposal**: NEVER allow tests to perform write operations on production configuration files. ALWAYS implement a path-override mechanism.

### Event-Driven Layout Synchronization
- **Learning**: In complex TUI applications, layout geometry is not static; it must be recalculated as a side-effect of structural state changes.
- **Pattern**: Post-Transition Recalculation.
- **Correction**: Updated `exec_next_tab`, `exec_prev_tab`, and mouse-based tab detection to call `self.calculate_layout` after updating the active tab.
- **Rule Proposal**: ALWAYS recalculate the `AppLayout` immediately following a tab switch or transition that changes structural geometry.

---

## 3. :: Workspace Deviations

### Commit and Fix Confirmation Gates
- **Gap**: Fixes identified during Phase 5 (Verification) were committed before user approval.
- **Correction**: Strengthen protocol to ensure user explicitly confirms a fix is complete before the history is made permanent.

### Keyboard Binding Robustness
- **Gap**: String-based key matching was too strict on formatting (e.g., `PgDown` vs `pgdown`).
- **Correction**: Normalize key-binding strings to lowercase and trim surrounding brackets. Support common aliases (pgup, pgdn, page_up).
