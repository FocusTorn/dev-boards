# Compare - Doc Alignment - Package

## Command Purpose

Compare a specific package's implementation against project documentation to identify mismatches, gaps, and alignment issues. Provides actionable recommendations for bringing the package into compliance with documented standards and patterns.

## Usage Description

When you receive the output from this command, the **Response** lines indicate how you should interact with each alignment issue. The AI Agent will use these exact icons and descriptions:

### Response Icon Meanings

**✏️ {My description}**
This icon represents that I made the proposed changes, and that the AI should peer review the changes made to ensure it is done so the best way possible and there are no other issues.

**❓ {My question}**
This is a direct question, any other item is not to be addressed, and all questions should be answered one at a time. If there are more than one question in the document, the AI is to fully answer the first question only. I will state something along the lines of, "ok next question", etc. when it is time to move on to the next question.

**❌ {My redirection}**
This is when the AI's understanding is wrong, and I am providing the corrected understanding. The AI is then to re-evaluate what was implemented/changed in regards to this alignment issue, and ensure that changes are correct according to the new understanding.

**⚠️ {Directive for AI to address}**
AI is to implement the changes described - execute the specific implementation instructions provided.

**✅ No action required**
The AI is to do nothing and there is no action that is to be taken for this item.

## Execution Instructions

### When to Use

- After package implementation to verify compliance with documentation
- When reviewing package architecture against documented patterns
- Before package deployment to ensure alignment
- When troubleshooting package issues against documented standards
- During package refactoring to maintain compliance

### Command Format

```
/Compare- Doc Alignment- Package.md

[package-path]

[@doc1.md] [@doc2.md] [@doc3.md] ...
```

### What to Analyze

#### 1. **Architecture Compliance**

For each package, analyze:

- **Package Structure**: Does it follow documented package archetypes?
- **Service Architecture**: Does it use documented service patterns?
- **Dependency Management**: Are dependencies structured according to guidelines?
- **Build Configuration**: Does it follow documented build patterns?
- **Testing Structure**: Does it align with testing strategy documentation?

#### 2. **Implementation Patterns**

Compare against documentation:

- **Code Patterns**: Are implementation patterns consistent with documented approaches?
- **Error Handling**: Does error handling follow documented strategies?
- **Configuration Management**: Does configuration follow documented patterns?
- **Command Execution**: Are execution patterns consistent with documented approaches?
- **Type System Usage**: Does TypeScript usage follow documented patterns?

#### 3. **Documentation Gaps**

Identify missing documentation:

- **Undocumented Patterns**: What patterns exist in the package that aren't documented?
- **Missing Examples**: What examples would help understand the package better?
- **Incomplete Guidelines**: What guidelines need to be added based on package implementation?
- **Process Gaps**: What processes are missing from documentation?

#### 4. **Categories to Focus On**

- **Architecture Violations**: Deviations from documented architectural patterns
- **Testing Misalignments**: Testing approaches that don't follow documented strategies
- **Configuration Issues**: Configuration patterns that don't match documentation
- **Error Handling Deviations**: Error handling that doesn't follow documented patterns
- **Build System Issues**: Build configurations that don't align with documented approaches
- **Documentation Gaps**: Missing documentation for implemented patterns
- **Process Violations**: Workflows that don't follow documented procedures

#### 5. **Format Template**

```markdown
## [Alignment Issue Category]

- **Issue**: [What specific mismatch or gap was identified]
- **Current Implementation**: [How the package currently handles this]
- **Documented Standard**: [What the documentation says should be done]
- **Impact**: [Why this mismatch matters and what problems it could cause]
- **Evidence**: [Specific file paths, line numbers, or code snippets showing the issue]

- **Recommendation**:
    - [First recommended action to align with documentation]
    - [Second recommended action to align with documentation]

- **Response**: ✏️❓❌⚠️✅ No action required

---
```

#### 6. **Context Requirements**

- Reference specific documentation sections
- Include file paths, line numbers, or code snippets when relevant
- Mention the specific documentation that was violated
- Note any tools, commands, or processes that should be used

#### 7. **Quality Criteria**

- Each alignment issue should be actionable and specific
- Focus on patterns that could apply to similar packages
- Include both the "what" and "why" for each issue
- Provide clear evidence of the mismatch

### Execution Notes

- Analyze the package structure against all provided documentation
- Focus on implementation patterns that deviate from documented standards
- Identify both violations and missing documentation
- Include any "aha moments" or significant gaps discovered
- Note any tools or techniques that were particularly effective

### Output Location

- Delete existing `.cursor/ADHOC/_Package-Alignment-Analysis.md` file
- Save analysis to `.cursor/ADHOC/_Package-Alignment-Analysis.md`
- Display the analysis in the chat
- Format for easy copy-paste into documentation or action plans

### Final Display Format

After saving to file, display the analysis in a markdown code block. The AI should output the entire summary wrapped in ```markdown code block tags with the content formatted as:

<!-- MANDATORY: Always start with this usage section -->

## Usage - Response Icon Meanings

**✏️ {My description}**
This icon represents that I made the proposed changes, and that the AI should peer review the changes made to ensure it is done so the best way possible and there are no other issues.

**❓ {My question}**
This is a direct question, any other item is not to be addressed, and all questions should be answered one at a time. If there are more than one question in the document, the AI is to fully answer the first question only. I will state something along the lines of, "ok next question", etc. when it is time to move on to the next question.

**❌ {My redirection}**
This is when the AI's understanding is wrong, and I am providing the corrected understanding. The AI is then to re-evaluate what was implemented/changed in regards to this alignment issue, and ensure that changes are correct according to the new understanding.

**⚠️ {Directive for AI to address}**
AI is to implement the changes described - execute the specific implementation instructions provided.

**✅ No action required**
The AI is to do nothing and there is no action that is to be taken for this item.

---

# [Alignment Issue Category Name]

- **Issue**: [What specific mismatch or gap was identified]
- **Current Implementation**: [How the package currently handles this]
- **Documented Standard**: [What the documentation says should be done]
- **Impact**: [Why this mismatch matters and what problems it could cause]
- **Evidence**: [Specific file paths, line numbers, or code snippets showing the issue]

- **Recommendation**:
    - [AI Agent provides specific remediation recommendations to align with documentation]
    - [AI Agent provides specific remediation recommendations to align with documentation]

- **Response**: ✏️❓❌⚠️✅ No action required

---

# [Next Alignment Issue Category Name]

- **Issue**: [What specific mismatch or gap was identified]
- **Current Implementation**: [How the package currently handles this]
- **Documented Standard**: [What the documentation says should be done]
- **Impact**: [Why this mismatch matters and what problems it could cause]
- **Evidence**: [Specific file paths, line numbers, or code snippets showing the issue]

- **Recommendation**:
    - [AI Agent provides specific remediation recommendations to align with documentation]
    - [AI Agent provides specific remediation recommendations to align with documentation]

- **Response**: ✏️❓❌⚠️✅ No action required

---

````

**CRITICAL OUTPUT REQUIREMENT**: The AI must wrap the entire output above in ```markdown code block tags so it can be copy-pasted directly. The output should start with ```markdown and end with ```.

**Note**: The response lines with icons are only for the summary display block, NOT for the saved markdown file. The saved file should contain only the analysis content without response lines.

**CRITICAL**: The **Response** line MUST be EXACTLY `- **Response**: ✏️❓❌⚠️✅ No action required`

### Recommendation Section

The **Recommendation** section is provided by the AI Agent executing the command and should contain specific remediation recommendations to align the package with documented standards. This is where the AI Agent suggests concrete actions, code changes, or documentation updates.

### Response Icon System

Each alignment issue in the summary display block includes a response line with icons that represent different types of feedback:

- ✅ **No action required**: Alignment issue is minor or already addressed, no further action needed
- ✏️ **Change made by user**: Description of change made by the user to address the alignment issue
- ❓ **Question asked**: Question asked that requires an answer from the AI agent
- ⚠️ **Action required by AI Agent**: Specific action that needs to be taken by the AI agent to address the alignment issue
- ❌ **Problem with implementation**: Description of problem with alignment correction that needs to be reviewed

### Execution Steps

1. **Clear existing file**: Delete `.cursor/ADHOC/_Package-Alignment-Analysis.md` file
2. **Analyze package**: Review package structure, implementation, and patterns
3. **Compare against docs**: Check implementation against provided documentation
4. **Identify mismatches**: Find deviations, gaps, and alignment issues
5. **Format analysis**: Use the template format for each alignment issue
6. **Save to file**: Write to `.cursor/ADHOC/_Package-Alignment-Analysis.md`
7. **Display summary**: Show formatted analysis in markdown code block

````

--- End Command ---

