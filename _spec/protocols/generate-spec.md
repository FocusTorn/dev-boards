# SF-GENERATE-SPEC
**PROTOCOL: Generate feature specification.**

1.  **State Goal:** "I will now generate a technical specification (spec.md) using Spec-Flow standards within the _spec architecture."
2.  **Context Loading:**
    - Identify the active track: `_spec/tracks/<track_id>/`.
    - Read existing `state.yaml` and `domain-memory.yaml` if they exist.
    - Reference [Product Definition](../product.md) and [Tech Stack](../tech-stack.md).
3.  **Generation (Internal Logic):**
    - Apply **Spec-Flow Specification Standards**:
      - **Problem Statement**: Clear pain point/opportunity.
      - **Goals & Success Criteria**: Measurable outcomes (G1, G2, etc.).
      - **User Scenarios**: Gherkin format (Given/When/Then).
      - **Functional Requirements**: Unique sequential IDs (**FR-001**, **FR-002**).
      - **Non-Functional Requirements**: Unique sequential IDs (**NFR-001**).
      - **Out of Scope**: Explicit exclusions.
      - **Open Questions**: Flag blockers with `[NEEDS CLARIFICATION]`.
4.  **Artifact Creation:**
    - Write to `_spec/tracks/<track_id>/spec.md`.
    - Initialize or update `_spec/tracks/<track_id>/NOTES.md` with key decisions and assumptions.
5.  **Quality Gate:**
    - Verify FR/NFR sequentiality and uniqueness.
    - Ensure at least 3 user scenarios are present.
    - Limit `[NEEDS CLARIFICATION]` markers to max 3; move others to `NOTES.md`.
6.  **Confirm:** Present the summary (Story count, FR count, NFR count, Clarifications) to the user.
