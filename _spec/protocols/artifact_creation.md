## ARTIFACT CREATION
**PROTOCOL: Create files and branch.**

1.  **Resolve Project Context:** Identify the target project from the track description (e.g., `dev-console2(dc2)` -> `dev-console2`).
    -   Check `_spec/_meta/aliases.md` to resolve aliases to formal `project_id`.
    -   Verify the project metadata exists in `_spec/_meta/<project_id>/`.
    -   **CRITICAL:** A track MUST never be created directly in the tracks directory. If no project is specified, it MUST default to the `workspace` project.
2.  **Generate ID:** Create unique Track ID (`shortname_YYYYMMDD`). Ensure no collision with existing directories in `_spec/tracks/<project_id>/`.
3.  **Create Directory:** Create `<Tracks Directory>/<project_id>/<track_id>/`.
4.  **Create Branch:** `git checkout -b track/<track_id>`.
5.  **Write Files:**
    - `metadata.json` (Ensure `project_id` is included)
    - `spec.md`
    - `plan.md`
    - `tasks.md`
    - `NOTES.md`
    - `index.md`
6.  **Update Registry:** Append new track section to **Tracks Registry**. The link MUST reflect the project-specific path: `[./tracks/<project_id>/<track_id>/](./tracks/<project_id>/<track_id>/)`.
7.  **Announce:** Inform user completion and next steps (`/spec:implement`).
