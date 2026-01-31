## PAUSE TRACK
**PROTOCOL: Safely pause an active development track.**

1.  **Sync Plan:**
    - Analyze current changes in the workspace.
    - Mark all completed tasks in the track's `plan.md` with `[x]`.
2.  **Update Metadata:**
    - Update the `updated_at` timestamp in the track's `metadata.json`.
3.  **Handover Note:**
    - Update `_spec/tracks/<track_id>/NOTES.md` with:
        - **Current Context:** Exactly what logic was being worked on at the moment of the pause.
        - **Next Action:** The specific first step to take upon resuming.
        - **Open Issues:** Any bugs or blockers identified but not resolved.
4.  **Checkpoint Commit:**
    - Calculate completion percentage based on `plan.md` tasks.
    - Commit all changes to the track branch with message: `_spec(pause): <track_id> - <percentage>% complete`.
5.  **Final Summary:**
    - Report the pause status and the next task to be performed to the user.
