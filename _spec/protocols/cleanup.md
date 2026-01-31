## CLEANUP
**PROTOCOL: Archive or delete track.**

1.  **Ask:** Prompt user:
    1)  **Archive:** Move to `_spec/tracks/<project_id>/closed/`, update status to `[x]`.
    2)  **Delete:** Permanently remove folder and registry entry.
    3)  **Skip:** Leave in registry as `[x]`.
2.  **Execute:**
    - If Archive: Move folder, update registry link, commit `chore(_spec): Archive track '<track_description>' to tracks/<project_id>/closed`.
    - If Delete: Final confirmation, delete folder and registry entry, commit `chore(_spec): Delete track '<track_description>'`.
