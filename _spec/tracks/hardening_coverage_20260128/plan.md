# Implementation Plan: `hardening_coverage_20260128`

## Phase 1: Command Logic Hardening

- [ ] Task: Fix `Generating` Stage Detection
    - [ ] Analyze `detect_stage_change` logic for regex/substring mismatches.
    - [ ] Update `tests.rs` with correct expectations.
    - [ ] Verify transition from `Linking` to `Generating`.
- [ ] Task: 100% Coverage for `compile_parser.rs`
    - [ ] Add tests for all error detection branches.
    - [ ] Add tests for every stage transition variant.
- [ ] Task: 100% Coverage for `compile_state.rs`
    - [ ] Add tests for `update_stage_progress` boundary conditions.
    - [ ] Add tests for `check_for_missing_markers` warning logic.
- [ ] Task: 100% Coverage for `history.rs`
    - [ ] Add tests for invalid JSON loading recovery.
    - [ ] Add tests for normalization edge cases (zero weights).

## Phase 2: Widget & UI Hardening

- [ ] Task: 100% Coverage for `smooth_scrollbar.rs`
    - [ ] Implement rendering tests for all scroll states.
    - [ ] Verify interaction logic for mouse clicks and dragging.
- [ ] Task: 100% Coverage for `output_box.rs`
    - [ ] Add tests for max line limits and auto-scroll toggling.
    - [ ] Verify input handling when active vs inactive.
- [ ] Task: 100% Coverage for `tab_bar.rs`
    - [ ] Refactor and simplify existing tests.
    - [ ] Add coverage for all color parsing variants.

## Phase 3: Final Audit & Documentation

- [ ] Task: Global Coverage Verification
    - [ ] Run final `llvm-cov` report and fix any remaining 1-2% gaps.
- [ ] Task: Documentation Update
    - [ ] Document testing strategy and GitBash requirement in `CONTRIBUTING.md` or `README.md`.
    - [ ] Clean up `NOTES.md` and archive the track.
- [ ] Task: _spec - Final Verification (Protocol in workflow.md)
