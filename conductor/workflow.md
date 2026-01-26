# Project Workflow: Dev Boards Workspace

## 1. Development Protocol
- **Research-Plan-Implement:** Always analyze the codebase and create a track plan before modifying code.
- **Branching Protocol:** Before starting a new track, create a dedicated branch named `track/<track_id>`. All work for that track must occur on this branch.
- **Resume Branch Protocol:** When resuming work on a track (e.g., via `/conductor:implement`), the agent MUST:
    1. Identify the target branch `track/<track_id>`.
    2. Check the current active branch.
    3. If the current branch is NOT the target branch:
        a. Check for uncommitted changes (`git status --porcelain`).
        b. If the tree is dirty, commit all changes with a checkpoint message (e.g., `chore: checkpoint before switching to track branch`).
        c. Switch to the target branch (`git checkout track/<track_id>`).
    4. If the target branch does not exist, create it from the current branch or `main` as appropriate.
- **Task-Based Execution:** Break work into small, actionable tasks (20-30 minutes each).
- **Atomic Commits:** Commit changes to Git after every successful task completion.
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