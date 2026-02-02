## BRANCH MANAGEMENT
**PROTOCOL: Ensure the workspace is on the correct branch.**

1.  **Workspace Integrity Check:** Check for uncommitted changes (`git status --porcelain`).
2.  **Handle Dirty Tree:** 
    -   **CRITICAL (New Track):** If the command is to create a new track (e.g., `/spec:track`) and the workspace is dirty:
        -   Inform the user: "The workspace has uncommitted changes. You MUST commit or stash these changes prior to beginning a new track."
        -   **HALT:** You are NOT permitted to commit or stash these changes yourself.
    -   **Existing Track:** If the workspace is dirty and you are NOT on the target `track/<track_id>` branch: Commit a checkpoint (`chore: checkpoint before switching to track branch`).
    -   **Resuming/Implementing:** Auto-commit checkpoint if necessary.
3.  **Switch Branch:**
    -   Identify target branch `track/<track_id>`.
    -   If current branch is different, checkout the target branch.
    -   If target branch does not exist, create it from `main`.
4.  **Status Update (Registry):**
    -   If the command is `implement` or `resume`, update the track's status in the **Tracks Registry** from `[ ]` to `[~]`.

