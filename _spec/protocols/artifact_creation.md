## ARTIFACT CREATION
**PROTOCOL: Create files and branch.**

1.  **Generate ID:** Create unique Track ID (`shortname_YYYYMMDD`). Ensure no collision with existing directories.
2.  **Create Directory:** Create `<Tracks Directory>/<track_id>/`.
3.  **Write Files:**
    - `metadata.json`
    - `spec.md`
    - `plan.md`
    - `tasks.md`
    - `NOTES.md`
    - `index.md`
4.  **Update Registry:** Append new track section to **Tracks Registry**.
5.  **Create Branch:** `git checkout -b track/<track_id>`.
6.  **Announce:** Inform user completion and next steps (`/spec:implement`).
