# Extract - Lessons Learned

The purpose of this command is to extract key learning patterns, solutions, insights and error occurances from the current conversation and format them as actionable lessons for future reference.

## 1. :: Instructions

### 1.1. :: Initial Execution <!-- Start Fold -->

1. **Clear existing file**: Delete `.cursor/ADHOC/_Extracted-Lessons.md` file
2. **Extract lessons**: Review entire conversation for patterns and discoveries
3. **Format lessons**: Use the template format for each lesson
4. **Evaluate for AI Agent Rules**: For EACH lesson, determine if an AI Agent rule is needed to prevent the pitfall from recurring. Add AI Agent rules to recommendations where applicable.
5. **Save to file**: Write to `.cursor/ADHOC/_Extracted-Lessons.md`
6. **Display summary**: Show formatted lessons in markdown code block as explained in `### Final Display Format`

### 1.3. :: Validation Before Saving <!-- Start Fold -->

Before saving the file, verify:
- Each lesson has been evaluated for AI Agent rule necessity
- All applicable lessons have AI Agent rules as the first item in their Recommendation section
- AI Agent rules are formatted correctly with exact, enforceable language using imperative terms (ALWAYS, NEVER, MUST)
- AI Agent rules are specific and actionable

<!-- Close Fold -->

**NOTES**:

- Review the entire conversation for patterns and discoveries
- Focus on solutions that required multiple iterations to solve and direct input from user correcting an action.
- Extract both successful patterns and failed approaches that provided learning
- Include any "aha moments" or breakthrough insights
- Note any tools or techniques that were particularly effective

<!-- Close Fold -->

### 1.2. :: Processing Responses <!-- Start Fold -->

When you receive the output from this command, the **Response** lines indicate how you should interact with each lesson. The AI Agent will use these exact icons and descriptions:

#### 1.2.1. :: Response Icon Meanings

- There is no need to ever update the \_Extracted-Lessons after it is generated and the summary is displayed.

**✏️ {My description}**
This icon represents that I made the proposed changes, and that the AI should peer review the changes made to ensure it is done so in the best way possible and there are no other issues.

**❓ {My question}**
This is a direct question, any other item is not to be addressed, and all questions should be answered one at a time. If there are more than one question in the document, the AI is to fully answer the first question only. I will state something along the lines of, "ok next question", etc. when it is time to move on to the next question.

**❌ {My redirection}**
This is when the AI's understanding is wrong, and I am providing the corrected understanding. The AI is then to re-evaluate what was implemented/changed in regards to this lesson, and ensure that changes are correct according to the new understanding. Then update what ever documentation is either incorrect or in need of modicfication

**⚠️ {Directive for AI to address}**
AI is to implement the changes described - execute the specific implementation instructions provided.

**✅ No action required**
The AI is to do nothing and there is no action that is to be taken for this item.

<!-- Close Fold -->

## 2. :: What to Extract

### 2.1. :: Data Points <!-- Start Fold -->

For each significant discovery, extract:

- **Learning**: [What was discovered or learned] Note: MANDATORY - Describe the key insight or discovery made during the work session
- **Pattern**: [The specific technique, approach, or solution] Note: MANDATORY - Document the exact method, strategy, or approach that was used to solve the problem
- **Implementation**: [How it was implemented (if applicable else "None")] Note: Include this section when specific implementation details are relevant to the lesson
- **Benefit**: [Why this approach is better] Note: MANDATORY - Explain the specific advantages, improvements, or positive outcomes achieved
- **Not documented**: [What gaps exist in current documentation (if applicable else "None")] Note: MANDATORY - Identify what knowledge, patterns, or processes are missing from current documentation in ./docs and well as package level. If documentation needs to be added, note that in the recommendations as well as the document.
- **Mistake/Assumption**: [What was wrong or incorrectly assumed] _Note: MANDATORY - Include this section for every lesson where the AI made errors, wrong assumptions, or incorrect implementations during the work session_
- **Correction**: [How it was fixed] _Note: MANDATORY - Include this section whenever Mistake/Assumption is included - describe exactly how the mistake was corrected_

<!-- Close Fold -->

### 2.2. :: Categories to Focus On <!-- Start Fold -->

- **Testing Patterns**: New testing techniques, mocking strategies, coverage approaches
- **Architecture Decisions**: Package structure, build configurations, dependency management
- **Troubleshooting Solutions**: Error resolution, debugging techniques, common pitfalls
- **Documentation Gaps**: Missing patterns, undocumented behaviors, knowledge gaps
- **Tool Integration**: New tool usage patterns, configuration discoveries
- **Migration Strategies**: Legacy code removal, format transitions, architectural refactoring patterns
- **Error Correction Patterns**: Mistakes made, incorrect assumptions, failed approaches, and how they were corrected

<!-- Close Fold -->

### 2.3. :: Recommendation Section <!-- Start Fold -->

The **Recommendation** section is provided by the AI Agent executing the command and should contain specific remediation recommendations to prevent the documented pitfall from occurring in the future. This is where the AI Agent suggests proactive measures, process improvements, or documentation (workspace or package level) updates.

**CRITICAL**: For EACH lesson, you MUST evaluate if an AI Agent rule is needed. An AI Agent rule is required when:
- The lesson describes a pattern that should ALWAYS be followed (not optional)
- The lesson describes a mistake that could be prevented by an explicit rule
- The lesson describes a technical requirement that must be enforced (like encoding, invocation patterns, etc.)
- The lesson describes output formatting or behavior standards that must be consistent

If an AI Agent rule is needed, it MUST be the FIRST item in the Recommendation section, formatted as:

```
- **AI Agent Rule**: 
    - **Action**: [ADD to existing rule file | MODIFY existing rule file | CREATE new rule file]
    - **Rule File Path**: [Relative path to rule file, e.g., `.cursor/rules/formatting/terminal-output.mdc`]
    - **Rule Text**: "[Exact rule text that enforces the pattern or prevents the mistake]"
    - **Section**: [If ADD/MODIFY, specify which section in the rule file, e.g., "Terminal Output Formatting Rules > Region Indentation System"]
```

**Rule File Path Guidelines**:
- Use relative paths from workspace root
- For formatting rules: `.cursor/rules/formatting/[filename].mdc`
- For workspace rules: `.cursor/rules/[filename].mdc`
- For package-specific rules: `packages/[package-name]/.cursor/rules/[filename].mdc`

**Action Determination**:
- **ADD to existing rule file**: When the rule fits into an existing rule file's scope and structure
- **MODIFY existing rule file**: When an existing rule needs to be updated or corrected
- **CREATE new rule file**: When the rule doesn't fit into any existing rule file and requires a new category

**Note**: If an AI Agent rule should be added in addition to human-readable documentation to strengthen understanding of expectations of output, that needs to be added to the recommendations as well as the document.

<!-- Close Fold -->

### 2.4. :: Context Requirements <!-- Start Fold -->

- Reference the specific conversation context
- Mention the problem that led to the discovery
- Note any tools, commands, or processes used
- Keep implementation details concise - avoid code blocks and granular specifics in the summary

<!-- Close Fold -->

### 2.5. :: Quality Criteria <!-- Start Fold -->

- Each lesson should be actionable and reusable
- Focus on patterns that could apply to similar situations
- Include both the "what" and the "why"
- Keep summaries concise and high-level - detailed implementation specifics should be in the full extracted document

<!-- Close Fold -->

### 2.6. :: AI Agent Rule Evaluation <!-- Start Fold -->

**MANDATORY**: For each lesson extracted, you MUST evaluate whether an AI Agent rule is needed. An AI Agent rule is required when:
- The lesson describes a pattern that should ALWAYS be followed (not optional)
- The lesson describes a mistake that could be prevented by an explicit rule
- The lesson describes a technical requirement that must be enforced (like encoding, invocation patterns, etc.)
- The lesson describes output formatting or behavior standards that must be consistent
- The lesson describes a type handling or parsing pattern that must be followed consistently

**Evaluation Process**:
1. Review each lesson's Pattern, Mistake/Assumption, and Not Documented sections
2. Determine if the lesson describes something that should be enforced automatically
3. If yes, create an AI Agent rule with exact, enforceable language
4. The AI Agent rule MUST be the FIRST item in the Recommendation section
5. Determine the appropriate action (ADD/MODIFY/CREATE) and identify the rule file path
6. Format with Action, Rule File Path, Rule Text, and Section (if ADD/MODIFY)

**Rule Format Guidelines**:
- Use imperative language (ALWAYS, NEVER, MUST)
- Be specific and actionable
- Include the exact pattern or requirement
- Make it enforceable (clear what to do/not do)

**Rule File Path and Action Guidelines**:
- **Determine Action Type**: Evaluate whether the rule fits an existing file (ADD/MODIFY) or needs a new file (CREATE)
- **Identify Rule File**: **CRITICAL** - The AI Agent MUST scan the entire workspace for all instances of `.cursor/` directories to locate existing rule files, not just rely on rule files the agent is aware of at the time of execution. Search all `.cursor/rules/` directories and subdirectories across the entire workspace to find the best match.
- **Specify Section**: If ADD/MODIFY, identify the specific section where the rule belongs
- **Provide Full Path**: Always provide the relative path from workspace root
- **Include Rule Text**: Provide the exact rule text that should be added/modified

<!-- Close Fold -->

## **Format Template**

```markdown
## [Lesson Category Name]

Learning: [What was discovered]
Pattern: [The specific technique or approach]
Implementation: [Concise text summary without code blocks or detailed implementation specifics]

Benefit: [Why this approach is better]

**Not Documented**: [What gaps exist in current documentation]

**Mistake/Assumption**: [What was wrong or incorrectly assumed]
**Correction**: [How it was fixed] (if mistake/assumption exists)

- **Recommendation**:
    - **AI Agent Rule**: [If applicable, this MUST be the first item]
        - **Action**: [ADD to existing rule file | MODIFY existing rule file | CREATE new rule file]
        - **Rule File Path**: [Relative path to rule file]
        - **Rule Text**: "[Exact rule text that enforces the pattern or prevents the mistake]"
        - **Section**: [If ADD/MODIFY, specify which section in the rule file]
    - [Other recommended actions to be taken to prevent this issue in the future]

- **Response**: ✏️❓❌⚠️✅ No action required

---
```

## Final Display Format

After saving to file, display the lessons in a markdown code block.

**CRITICAL OUTPUT REQUIREMENT**: The AI must wrap the entire summary output in markdown code block tags so it can be copy-pasted directly. The output should start with ` ```markdown` and end with ` ``` `  with the content formatted as:

```markdown

# [Concise title referencing the content the lessons were derived from]

## 1. :: Shorthand reference Mapping and Explanations <!-- Start Fold -->

### 1.1. :: Alias Mapping

- **\_strat**: `docs/testing/_Testing-Strategy.md`
- **\_ts**: `docs/testing/_Troubleshooting - Tests.md`
- **lib guide**: `docs/testing/Library-Testing-AI-Guide.md`

### 1.2. :: Details for shorthand execution details:

#### Add to strat

You will understand that _add to strat_ means to do the following:

1. Add the needed documentation to **\_strat**
2. Ensure there is a `### **Documentation References**` to **\_strat** within **guide**
3. Add or modify a concise section with a pointer to the main file for more detail to **guide**

#### Add to trouble

You will understand that _add to trouble_ means to do the following:

1. Add the needed documentation to **\_ts**
2. Ensure there is a `### **Documentation References**` to **\_strat** within **guide**
3. Add or modify a concise section with a pointer to the main file for more detail to **guide**

---

<!-- Close Fold -->

## 2.0 :: [Lesson Category Name] <!-- Start Fold -->

- Learning: [What was discovered]
- Pattern: [The specific technique or approach]
- Implementation: [Concise text summary without code blocks or detailed implementation specifics] (if applicable)
- Benefit: [Why this approach is better]

- **Not documented**: [What gaps exist in current documentation]

- **Mistake/Assumption**: [What was wrong or incorrectly assumed]
- **Correction**: [How it was fixed] _Note: Only if Mistake/Assumption is displayed_

- **Recommendation**:
    - **AI Agent Rule**: [If applicable, this MUST be the first item]
        - **Action**: [ADD to existing rule file | MODIFY existing rule file | CREATE new rule file]
        - **Rule File Path**: [Relative path to rule file]
        - **Rule Text**: "[Exact rule text that enforces the pattern or prevents the mistake]"
        - **Section**: [If ADD/MODIFY, specify which section in the rule file]
    - [AI Agent provides specific remediation recommendations to prevent this pitfall in the future]

- **Response**: ✏️❓❌⚠️✅ No action required


---

<!-- Close Fold -->

## 2.0 :: [Next Lesson Category Name] <!-- Start Fold -->

- Learning: [What was discovered]
- Pattern: [The specific technique or approach]
- Implementation: [Concise text summary without code blocks or detailed implementation specifics] (if applicable)
- Benefit: [Why this approach is better]
- **Not documented**: [What gaps exist in current documentation]

- **Mistake/Assumption**: [What was wrong or incorrectly assumed]
- **Correction**: [How it was fixed] _Note: Only if Mistake/Assumption is displayed_

- **Recommendation**:
    - **AI Agent Rule**: [If applicable, this MUST be the first item]
        - **Action**: [ADD to existing rule file | MODIFY existing rule file | CREATE new rule file]
        - **Rule File Path**: [Relative path to rule file]
        - **Rule Text**: "[Exact rule text that enforces the pattern or prevents the mistake]"
        - **Section**: [If ADD/MODIFY, specify which section in the rule file]
    - [AI Agent provides specific remediation recommendations to prevent this pitfall in the future]

- **Response**: ✏️❓❌⚠️✅ No action required



---

<!-- Close Fold -->

```

**Note**: The response lines with icons are only for the summary display block, NOT for the saved markdown file. The saved file should contain only the lesson content without response lines.

**CRITICAL**: The **Response** line MUST be EXACTLY `- **Response**: ✏️❓❌⚠️✅ No action required`

