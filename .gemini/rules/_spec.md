# Spec Process Rules

Standards for managing tracks and session transitions using the Spec  methodology.

## Track Management

- **Mandatory Branching**: Before starting a new track, ALWAYS create a dedicated branch named `track/<track_id>`. All implementation tasks must occur on this branch.
- **Pause Track Protocol**: When pausing work on a track, MUST follow the 'Pause Track Protocol' defined in `_spec/workflow.md`. This includes:
    - Synchronizing the implementation plan (`plan.md`).
    - Updating metadata timestamps.
    - Creating a Handover Note (`NOTES.md`) with explicit 'Next Action' context.
    - Performing a Checkpoint Commit on the track branch.
- **Resume Branch Protocol**: When resuming work, verify the current active branch matches `track/<track_id>`. If the tree is dirty on the wrong branch, commit it as a checkpoint before switching.

### Branch Integrity Hard Gate
- **SESSION START**: Whenever resuming or starting a track, the first tool call MUST be `git branch --show-current` to verify context.
- **ABORT PATH**: If the current branch is `main` or does not match `track/<track_id>`, you MUST NOT modify any code or execute any commits until you have successfully switched to the correct branch using the 'Resume Branch Protocol' in `workflow.md`.
- **PRE-COMMIT CHECK**: Every `git commit` attempt MUST be preceded by an explicit branch verification in the same turn or the immediate previous turn.

### Phase Boundary Hard Gate
- **NEVER** initiate a task from Phase N+1 until Phase N has been officially closed.
- **MANDATORY PAUSE**: When the last functional task of a Phase is marked complete, you MUST stop and execute the 'Phase Completion & Checkpointing' protocol from `workflow.md`.
- **USER APPROVAL**: You MUST explicitly present the Phase results and wait for user approval before modifying the `plan.md` to start Phase N+1.





## Governance & State Management

- PHASE TRANSITION BLOCKING GATE: Every phase transition within a `plan.md` MUST be treated as a strict, non-negotiable blocking event.
- Upon completing the final task of Phase N, the agent MUST explicitly state 'Phase N is complete' and halt all execution immediately.
- You are strictly FORBIDDEN from reading, parsing, planning, or mentioning any tasks from Phase N+1 until the user provides an explicit 'Proceed' message or equivalent approval.
- This blocking gate is absolute and applies regardless of whether specific verification tasks are listed; the boundary between markdown headers (e.g., Phase 1 to Phase 2) is the primary trigger for this lock.
