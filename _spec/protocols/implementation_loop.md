## IMPLEMENTATION LOOP
**PROTOCOL: Execute tasks sequentially.**

1.  **Iterate:** Loop through each task in the track's **Implementation Plan** one by one.
2.  **Phase Boundary Hard Gate:**
    - **NEVER** initiate a task from Phase N+1 until Phase N has been officially closed.
3.  **Task Execution:**
    - **TDD ENFORCEMENT:** You MUST follow the Red-Green-Refactor pattern. NEVER write implementation code before a corresponding test exists and is failing.
    - Defer to the **Workflow** file for the detailed "Task Workflow" (Test -> Implement -> Refactor -> Update Plan -> Commit).
    - **Update Notes:** Maintain the `NOTES.md` file throughout implementation:
        - **Lessons Learned:** Record insights that can help avoid future pitfalls.
        - **Testing Changes:** Document:
            - Codebase changes made because they didn't match prewritten test expectations.
            - Test changes made because the test itself was incorrectly written.
    - **Update Plan:** Mark the current task as `[x]` in `plan.md` immediately upon completion.
    - **Commit:** Use `git commit` for every task completion, ensuring the updated `plan.md` and `NOTES.md` are included in the commit.
4.  **Completion:** Once all tasks are complete, update the registry status to `[x]` and move the track to `## Closed`.
