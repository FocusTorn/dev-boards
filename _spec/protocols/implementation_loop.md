## IMPLEMENTATION LOOP
**PROTOCOL: Execute tasks sequentially.**

1.  **Iterate:** Loop through each task in the track's **Implementation Plan** one by one.
2.  **Phase Boundary Hard Gate:**
    - **NEVER** initiate a task from Phase N+1 until Phase N is complete.
    - **MANDATORY PAUSE:** When the last task of a Phase is finished, execute the 'Phase Completion & Checkpointing' protocol.
    - **USER APPROVAL:** Wait for user approval before starting the next Phase.
3.  **Task Execution:**
    - Defer to the **Workflow** file for the "Task Workflow" (Implement -> Test -> Update Plan -> Commit).
    - **Update Plan:** Mark the current task as `[x]` in `plan.md` immediately upon completion.
    - **Commit:** Use `git commit` for every task completion, ensuring the updated `plan.md` is included in the commit.
4.  **Completion:** Once all tasks are complete, update the registry status to `[x]` and move the track to `## Closed`.
