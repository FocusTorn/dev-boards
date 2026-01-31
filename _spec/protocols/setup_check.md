## SETUP CHECK
**PROTOCOL: Verify that the _spec environment is properly set up.**

1.  **Branch Integrity Hard Gate:** Your first tool call MUST be `git branch --show-current` to verify context.
2.  **Verify Core Context:** Using the **Universal File Resolution Protocol**, resolve and verify the existence of:
    -   **Product Definition**: `_spec/_meta/workspace/product.md`
    -   **Tech Stack**: `_spec/_meta/workspace/tech-stack.md`
    -   **Workflow**: `_spec/_meta/workspace/workflow.md`

3.  **Handle Failure:** If ANY of these are missing (or their resolved paths do not exist), Announce: "_spec is not set up. Please run `/spec:setup`." and HALT.

