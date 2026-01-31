## PROJECT SCAFFOLDING
**PROTOCOL: Initialize project filesystem structure.**

1.  **Check Path:** If the path (from Q2) already exists and is not empty:
    - **Analyze**: Identify the language and build system (e.g., look for `Cargo.toml`, `pyproject.toml`, or `.ino` files).
    - **Report**: "Existing project detected. Mapping structure to metadata."
2.  **Initialize New:** If the path does not exist or is empty:
    - **Create Directory**: `mkdir -p <path>`.
    - **Flavor-Specific Init**:
        - `TUI-Ratatui` / `Shared-Lib-Rust`: Execute `cargo init --bin` or `--lib`.
        - `CLI-Python`: Execute `uv init`.
        - `Embedded-Arduino`: Create a `.ino` file with standard `setup()` and `loop()` boilerplate.
    - **Verify**: Confirm the entry point is created.
3.  **Finalize**: Announce that the project is now ready for its first track (`/spec:newTrack`).
