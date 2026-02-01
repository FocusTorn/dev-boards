# _Spec Context

If a user mentions a "plan" or asks about the plan, and they have used the _spec extension in the current session, they are likely referring to the `_spec/tracks.md` file or one of the track plans (`_spec/tracks/<track_id>/plan.md`).

## Universal File Resolution Protocol

**PROTOCOL: How to locate files.**
To find a file (e.g., "**Product Definition**") within a specific context (Project Root or a specific Track):

1.  **Identify Index:** Determine the relevant index file:
    -   **Project Context:** `_spec/index.md`
    -   **Track Context:**
        a. Resolve and read the **Tracks Registry** (via Project Context).
        b. Find the entry for the specific `<track_id>`.
        c. Follow the link provided in the registry to locate the track's folder. The index file is `<track_folder>/index.md`.
        d. **Fallback:** If the track is not yet registered (e.g., during creation) or the link is broken:
            1. Resolve the **Tracks Directory** (via Project Context).
            2. The index file is `<Tracks Directory>/<track_id>/index.md`.

2.  **Check Index:** Read the index file and look for a link with a matching or semantically similar label.

3.  **Resolve Path:** If a link is found, resolve its path **relative to the directory containing the `index.md` file**.
    -   *Example:* If `_spec/index.md` links to `./workflow.md`, the full path is `_spec/workflow.md`.

4.  **Fallback:** If the index file is missing or the link is absent, use the **Default Path** keys below.

5.  **Verify:** You MUST verify the resolved file actually exists on the disk.

**Standard Default Paths (Workspace):**
- **Product Definition**: `_spec/_meta/workspace/product.md`
- **Tech Stack**: `_spec/_meta/workspace/tech-stack.md`
- **Workflow**: `_spec/_meta/workspace/workflow.md`
- **Product Guidelines**: `_spec/_meta/workspace/product-guidelines.md`
- **Tracks Registry**: `_spec/tracks.md`
- **Tracks Directory**: `_spec/tracks/`
- **Aliases Registry**: `_spec/_meta/aliases.md`

**Standard Default Paths (Project):**
- **North Star (Definition)**: `_spec/_meta/<project_id>/north_star.md`
- **Architecture**: `_spec/_meta/<project_id>/architecture.md`
- **Components**: `_spec/_meta/<project_id>/components.md`
- **Project Backlog**: `_spec/_meta/<project_id>/tracks.md`

**Standard Default Paths (Track):**
- **Specification**: `_spec/tracks/<project_id>/<track_id>/spec.md`
- **Implementation Plan**: `_spec/tracks/<project_id>/<track_id>/plan.md`
- **Metadata**: `_spec/tracks/<project_id>/<track_id>/metadata.json`


