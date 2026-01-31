## ARTIFACT CREATION
**PROTOCOL: Create files and branch.**

1.  **Generate ID:** Create unique Track ID (`shortname_YYYYMMDD`). Ensure no collision with existing directories.
2.  **Create Directory:** Create `<Tracks Directory>/<track_id>/`.
3.  **Create Branch:** `git checkout -b track/<track_id>`.
4.  **Write Files:**
    - `metadata.json`
    - `spec.md`
    - `plan.md`
    - `tasks.md`
    - `NOTES.md`
    - `index.md`
5.  **Update Registry:** Append new track section to **Tracks Registry**.
6.  **Announce:** Inform user completion and next steps (`/spec:implement`).
