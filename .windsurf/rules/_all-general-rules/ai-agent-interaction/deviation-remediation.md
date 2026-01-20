---
trigger: always_on
---

# Deviation Remediation Rules

## **EXECUTIVE SUMMARY GENERATION**

### **1. :: Executive Summary Format**

**✅ CORRECT - Generate executive summary using Lessons Learned format**:

After root cause analysis, generate an executive summary following the format from `SumUp - 3. Lessons Learned.md`:

```markdown
## Deviation Analysis: [Brief Description]

- **Learning**: [What was discovered about the deviation]
- **Pattern**: [The specific deviation pattern that occurred]
- **Root Cause**: [Categorized root cause: Missing Rule | Insufficient Rule | Rule Discovery Failure | Rule Application Failure | Tool Limitation | Process Gap]
- **Evidence**: [Specific evidence supporting the root cause categorization]

- **Not Documented**: [What gaps exist in current rules or documentation]
  - [Specific rule file or section that is missing or insufficient]
  - [Specific process step or quality gate that is missing]

- **Mistake/Assumption**: [What was wrong or incorrectly assumed]
  - [Specific mistake made during execution]
  - [Incorrect assumption that led to the deviation]

- **Correction**: [How the deviation should be prevented in the future]
  - [Specific rule changes needed]
  - [Specific process improvements needed]
  - [Specific tool usage improvements needed]

- **Recommendation**:
    - **AI Agent Rule**: [MUST be the first item if rule changes are needed]
        - **Action**: [ADD to existing rule file | MODIFY existing rule file | CREATE new rule file]
        - **Rule File Path**: [Relative path to rule file, e.g., `.agent/rules/by-language/python/code-quality.mdc`]
        - **Rule Text**: "[Exact rule text that would have prevented this deviation]"
        - **Section**: [If ADD/MODIFY, specify which section in the rule file]
        - **Rationale**: [Explanation of why this rule is needed and how it prevents the deviation from recurring]
    - [Additional remediation recommendations]
    - [Process improvements needed]
    - [Tool usage improvements needed]

- **Response**: ⚠️ {Directive for AI to address}
```

**✅ CORRECT - Executive summary requirements**:

1. **Complete Analysis**: All sections must be filled out
2. **Specific Evidence**: Provide specific evidence, not general statements
3. **Actionable Recommendations**: Recommendations must be specific and actionable
4. **Rule Text**: If rule changes are needed, provide exact rule text
5. **File Paths**: Use relative paths from workspace root
6. **Categorization**: Root cause must be one of the six categories

**❌ INCORRECT - Incomplete executive summary**:

- Missing sections (Learning, Pattern, Root Cause, etc.)
- Vague statements without specific evidence
- Recommendations without actionable steps
- Missing rule text when rule changes are needed
- Incorrect root cause categorization

### **2. :: Rule File Path Discovery**

**✅ CORRECT - Discover all rule files before recommending changes**:

When recommending rule changes, the AI Agent MUST:

1. **Search Entire Workspace**: Scan all `.cursor/` directories for rule files
   - Use `glob_file_search` to find all `.agent/rules/**/*.mdc` files
   - Use `list_dir` to explore `.agent/rules/` directory structure
   - Do NOT rely only on rules the agent is currently aware of

2. **Identify Best Match**: Determine the best rule file for the change
   - **ADD to existing**: If a rule file exists that covers this scenario
   - **MODIFY existing**: If an existing rule needs to be updated
   - **CREATE new**: If no existing rule file is appropriate

3. **Provide Full Path**: Always provide relative path from workspace root
   - Format: `.agent/rules/[category]/[filename].mdc`
   - For language-specific: `.agent/rules/by-language/[language]/[filename].mdc`
   - For tool-specific: `.agent/rules/tool/[tool]/[filename].mdc`
   - For formatting: `.agent/rules/formatting/[filename].mdc`

**❌ INCORRECT - Incomplete rule file discovery**:

- Recommending CREATE when ADD/MODIFY is appropriate
- Not searching entire workspace for rule files
- Using incorrect file paths
- Not identifying the best existing rule file match

## **DEVIATION REMEDIATION**

### **3. :: Remediation Protocol**

**CRITICAL ENFORCEMENT - MANDATORY PAUSE**: After generating a deviation executive summary, the AI agent MUST explicitly pause and state "Waiting for your approval before proceeding with remediation" or equivalent acknowledgment. The agent MUST NOT execute ANY commands (including `cargo build`, `cargo check`, or any terminal commands), make ANY file changes, or take ANY remediation actions until the user explicitly reviews and approves the recommendations. This pause applies to ALL remediation actions, including build verification commands, file modifications, and any other changes.

**✅ CORRECT - Follow remediation protocol**:

After generating executive summary, the AI Agent MUST:

1. **Present Summary**: Display the executive summary for user review
2. **MANDATORY PAUSE**: Explicitly state "Waiting for your approval before proceeding with remediation" or equivalent acknowledgment
3. **Wait for User Input**: Do NOT implement changes, execute commands, or take any actions until user reviews and approves
4. **No Premature Actions**: Do NOT fix issues, modify files, run build commands, or take any remediation actions until approval
5. **Collaborate on Remediation**: Work with user to refine recommendations if needed
6. **Implement Approved Changes**: Only implement changes after explicit user approval

**MANDATORY VERIFICATION CHECKPOINT**: Before implementing any remediation, the AI agent MUST be able to answer:
- "Did the user explicitly approve the remediation recommendations?"
- "Am I about to execute any commands or make changes without approval?"

If the answer to the second question is YES and the first is NO, the agent MUST NOT proceed.

**❌ INCORRECT - Implementing without approval**:

- Implementing rule changes before user review
- Executing commands (cargo build, cargo check, etc.) before user approval
- Making assumptions about what user wants
- Skipping collaboration step
- Not waiting for explicit approval
- Fixing issues immediately after generating summary
- Taking any remediation actions without explicit user approval
- Proceeding with changes "because they seem obvious"

## **QUALITY GATES**

- [ ] **Executive Summary Generated**: Complete executive summary following Lessons Learned format
- [ ] **Rule Files Discovered**: Entire workspace searched for relevant rule files
- [ ] **Specific Recommendations**: Actionable recommendations with exact rule text provided
- [ ] **User Review**: Executive summary presented for user review before implementation

