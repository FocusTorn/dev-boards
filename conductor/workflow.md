# Project Workflow: Dev Boards Workspace

## 1. Development Protocol
- **Research-Plan-Implement:** Always analyze the codebase and create a track plan before modifying code.
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
    4. Summarize the phase in the main 	racks.md file.
