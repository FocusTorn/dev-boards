# SF-GENERATE-TASKS
**PROTOCOL: Generate TDD task breakdown using Spec-Flow patterns tailored for _spec.**

1.  **State Goal:** "I will now generate a TDD task breakdown (tasks.md) using Spec-Flow standards within the _spec architecture."
2.  **Context Loading:**
    - Read `_spec/tracks/<track_id>/spec.md` and `plan.md`.
    - Reference [Rust Style Guide](../code_styleguides/rust.md) (or relevant language guide).
3.  **Generation (Internal Logic):**
    - Apply **Spec-Flow Task Standards**:
      - **TDD Cycles**: Break work into Red-Green-Refactor cycles.
      - **Sequential IDs**: Use unique sequential IDs (**T001**, **T002**, etc.).
      - **Domain Markers**: Label tasks with `[BACKEND]`, `[FRONTEND]`, `[TESTS]`.
      - **Priority Markers**: Use `[P]` for high-priority/critical-path tasks.
      - **Phase Labels**: Map tasks to the Phases defined in `plan.md`.
      - **Verification Steps**: Include "Verify Success (Green)" checkpoints.
4.  **Artifact Creation:**
    - Write to `_spec/tracks/<track_id>/tasks.md`.
    - Update `_spec/tracks/<track_id>/NOTES.md` with task-level decisions.
5.  **Quality Gate:**
    - Verify task count matches expected complexity (typically 15-30 tasks).
    - Ensure every `[RED]` task is followed by a `[GREEN]` task.
    - Check that all Phase boundaries from `plan.md` are respected.
6.  **Confirm:** Present the summary (Total tasks, Backend/Frontend breakdown, TDD cycle count) to the user.
