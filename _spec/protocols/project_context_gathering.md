## PROJECT CONTEXT GATHERING
**PROTOCOL: Build project context interactively.**

1.  **State Goal:** "I'll now guide you through a series of questions to initialize a new project metadata and structure."
2.  **Questioning Phase:**
    - Ask questions sequentially (one by one).
    - **Question 1 (ID):** "What is the unique ID for this project? (e.g., `dev-console2`, `esp32-monitor`)"
    - **Question 2 (Path):** "What is the filesystem path for this project? (Provide relative path from root, or leave blank to create a new one in `projects/`)"
    - **Question 3 (Flavor):** "What is the project flavor? (Select one: `TUI-Ratatui`, `CLI-Python`, `Embedded-Arduino`, `Shared-Lib-Rust`, `Shared-Lib-Python`)"
    - **Question 4 (North Star):** "What is the ultimate end-state goal for this project? (This will be saved to `north_star.md`)"
3.  **Validate:** Ensure the `project_id` does not collide with existing `_spec/_meta/` directories.
4.  **Draft Summary:** Confirm the gathered information with the user before proceeding to file creation.
