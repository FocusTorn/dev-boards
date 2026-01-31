# Project Workflow: Dev Boards Workspace

## 1. Development Protocol
- **Research-Plan-Implement:** Always analyze the codebase and create a track plan before modifying code.
- **Branching Protocol:** Before starting a new track:
    1. **Workspace Integrity Check:** Check for uncommitted changes (`git status --porcelain`).
    2. **Handle Dirty Tree:** If the workspace is dirty, inform the user that all changes MUST be committed or stashed before the track can be initialized. **HALT** initialization until the workspace is clean.
    3. **Branch Creation:** Create a dedicated branch named `track/<track_id>` from `main` (or the current clean branch). All work for that track must occur on this branch.
- **Resume Branch Protocol:** When resuming work on a track (e.g., via `/spec:implement`), the agent MUST:
    1. Identify the target branch `track/<track_id>`.
    2. Check the current active branch.
    3. If the current branch is NOT the target branch:
        a. Check for uncommitted changes (`git status --porcelain`).
        b. If the tree is dirty, commit all changes with a checkpoint message (e.g., `chore: checkpoint before switching to track branch`).
        c. Switch to the target branch (`git checkout track/<track_id>`).
    4. If the target branch does not exist, create it from the current branch or `main` as appropriate.
- **Task-Based Execution:** Break work into small, actionable tasks (20-30 minutes each).
- **Plan Synchronization:** Immediately after completing a task, update the track's `plan.md` by marking the task as completed (`[x]`).
- **Atomic Commits:** Commit changes to Git after every successful task completion. This commit MUST include the updated `plan.md`.
- **Task Summaries:** Use **Git Notes** to record a concise summary of what was achieved in each task.

## 2. Testing & Quality Standards
- **Average Coverage:** Maintain a minimum of **80%** code coverage across the entire project.
- **Entry Point Mandate:** All top-level user entry points (CLI commands, TUI actions, API endpoints) must have **100% test coverage**. These must be verified "green" before any phase is considered complete.
- **Regression Testing:** Ensure new features do not break existing hardware monitoring or flashing logic.

## 3. Phase Completion & Checkpointing
- At the end of every Phase defined in a plan.md, a manual verification step is required.
- **Protocol:**
    1. Verify all tasks in the phase are marked [x].
    2. Confirm 100% entry-point coverage for new functionality.
    3. Execute a workspace-wide build to ensure no regressions.
    4. **User Approval:** Present the phase results to the user for explicit verification and approval.
    5. Summarize the phase in the main tracks.md file.
- **Merge Protocol:** A track may only be merged into `main` after all phases are complete and approved by the user.

## 4. Pause Track Protocol
When requested to "pause" a track, the agent MUST:
1. **Sync Plan:** Analyze changes and mark completed tasks in the track's `plan.md` with `[x]`.
2. **Update Metadata:** Update the `updated_at` timestamp in the track's `metadata.json`.
3. **Handover Note:** Update `_spec/tracks/<track_id>/NOTES.md` with:
    - **Current Context:** Exactly what logic was being worked on at the moment of the pause.
    - **Next Action:** The specific first step to take upon resuming.
    - **Open Issues:** Any bugs or blockers identified but not resolved.
4. **Checkpoint Commit:** Commit all changes to the track branch with message: `_spec(pause): <track_id> - <percentage>% complete`.
5. **Final Summary:** Report the pause status and the next task to be performed.

