# SF-GENERATE-PLAN
**PROTOCOL: Generate architectural implementation plan using Spec-Flow patterns tailored for _spec.**

1.  **State Goal:** "I will now generate an architectural implementation plan (plan.md) using Spec-Flow standards within the _spec architecture."
2.  **Context Loading:**
    - Read `_spec/tracks/<track_id>/spec.md`.
    - Reference [Tech Stack](../tech-stack.md) and [Workflow](../workflow.md).
    - Scan codebase for **REUSE** opportunities (existing components, patterns).
3.  **Generation (Internal Logic):**
    - Apply **Spec-Flow Planning Standards**:
      - **Architecture Overview**: Define patterns (Layered, Repository, etc.).
      - **Component Design**: Identify new vs reusable components.
      - **Data Flow**: Describe interactions and state transitions.
      - **Reuse Analysis**: Explicitly mark `REUSE: <component>` where applicable.
      - **Phase Breakdown**: Group work into logical hierarchical phases.
      - **Verification Tasks**: Append manual/automated verification steps for each phase.
4.  **Artifact Creation:**
    - Write to `_spec/tracks/<track_id>/plan.md`.
    - Write technical rationale to `_spec/tracks/<track_id>/research.md`.
    - Update `_spec/tracks/<track_id>/NOTES.md` with architectural trade-offs and decisions.
5.  **Quality Gate:**
    - Verify at least 1 reuse opportunity OR justification for 0 reuse.
    - Ensure all Functional Requirements from `spec.md` are mapped to tasks.
    - Check for `BLOCKER` markers for technical constraints.
6.  **Confirm:** Present the summary (Architecture pattern, Reuse count, Major phases) to the user.
