## PROJECT META CREATION
**PROTOCOL: Create project-specific metadata files.**

1.  **Create Directory:** Create `_spec/_meta/<project_id>/`.
2.  **Initialize Files:**
    - **`architecture.md`**: Initialized with basic module structure and design patterns based on the project flavor.
    - **`north_star.md`**: Populated with the end-state goal from the gathering phase and a "Future Tracks / Wishlist" section.
    - **`components.md`**: Initialized as a registry for agnostic components (widgets, drivers, or modules).
    - **`tracks.md`**: Initialized as the project-specific backlog.
3.  **Cross-Linking:** Add a `GEMINI.md` file to the actual project directory (from Q2) that links back to these `_spec/_meta/` documents.
