# Sumarize: Granular

## MANDATORY EXECUTION PROTOCOL

1. **CRITICAL: Execution Expectation** → MANDATORY: This command MUST be executed exactly as written with NO DEVIATION, NO SKIPPING, and NO MODIFICATION.

### MANDATORY REQUIREMENTS

1. **COMPLETE CONVERSATION SCOPE**: Summary MUST reflect the ENTIRE conversation from initial prompt to completion, NOT just since "Summarizing Chat Context" creation
2. **OUTPUT LOCATION**: MUST output to `.cursor/ADHOC/Summary.md`
3. **MEETING MINUTES FORMAT**: Detailed discussion points with outcomes and context
4. **CODE EXAMPLES**: Include ONLY when necessary for understanding context
5. **OUTCOMES DOCUMENTATION**: Record both positive and negative results

### MANDATORY EXECUTION STEPS

1. **ANALYZE ENTIRE CONVERSATION**: Review ALL messages from conversation start to current point
2. **EXTRACT DETAILED DISCUSSIONS**: Identify specific topics, decisions, and outcomes
3. **DOCUMENT OUTCOMES**: Record both successful and failed results
4. **INCLUDE RELEVANT CODE**: Add code examples ONLY when essential for context understanding
5. **CREATE MEETING MINUTES**: Generate detailed summary with timestamps and outcomes
6. **OUTPUT TO SPECIFIED LOCATION**: Write summary to `.cursor/ADHOC/Summary.md`
7. **VERIFY COMPLETENESS**: Ensure summary covers entire conversation scope with details

### MANDATORY OUTPUT FORMAT

```markdown
# Conversation Summary - Granular (Meeting Minutes)

## Discussion Timeline

- **[Timestamp]**: [Topic/Issue discussed]
    - **Context**: [Background information]
    - **Discussion**: [Key points raised]
    - **Outcome**: [Resolution/Decision/Status]
    - **Code Examples**: [Only if essential for understanding]

## Key Decisions Made

- **[Decision 1]**: [Description and rationale]
- **[Decision 2]**: [Description and rationale]
- **[Additional decisions as applicable]**

## Outcomes Achieved

### Successful Outcomes

- [Outcome 1]: [Description]
- [Outcome 2]: [Description]
- [Additional successes as applicable]

### Issues Encountered

- [Issue 1]: [Description and resolution attempt]
- [Issue 2]: [Description and resolution attempt]
- [Additional issues as applicable]

## Technical Details

- **[Technical Topic 1]**: [Relevant technical information]
- **[Technical Topic 2]**: [Relevant technical information]
- [Additional technical details as applicable]

## Summary Generated

[Timestamp]: Complete granular conversation summary created covering [X] messages from initial prompt to completion with detailed meeting minutes format.
```

### MANDATORY COMPLIANCE VERIFICATION

- [ ] **ENTIRE CONVERSATION ANALYZED**: All messages from start to finish included
- [ ] **DETAILED DISCUSSIONS EXTRACTED**: Specific topics and decisions documented
- [ ] **OUTCOMES DOCUMENTED**: Both positive and negative results recorded
- [ ] **CODE EXAMPLES INCLUDED**: Only when essential for context understanding
- [ ] **MEETING MINUTES FORMAT**: Detailed summary with timestamps and outcomes
- [ ] **OUTPUT LOCATION**: Summary written to `.cursor/ADHOC/Summary.md`
- [ ] **COMPLETENESS VERIFIED**: Summary reflects full conversation scope with granular details

**VIOLATION PENALTY**: Any failure to complete these steps constitutes a critical protocol violation requiring immediate acknowledgment and correction.

## EXECUTION TRIGGER

Execute this command when user requests detailed conversation summary, meeting minutes, or comprehensive discussion documentation.
