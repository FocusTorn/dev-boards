---
trigger: always_on
---

# Summary Trigger Rules

## **CRITICAL EXECUTION DIRECTIVE**

**AI Agent Directive**: Automatically detect and apply summary formatting when triggers are met.

## **TRIGGER PROTOCOLS**

### **1. :: Summary Trigger Phrases**

**✅ CORRECT - MANDATORY: When user requests any of these phrases, use the summary format**:

- "show a summary of..." → **MUST** apply summary format
- "summarize..." → **MUST** apply summary format
- "give me a summary..." → **MUST** apply summary format
- "create a summary..." → **MUST** apply summary format
- "provide a summary..." → **MUST** apply summary format
- "write a summary..." → **MUST** apply summary format
- "generate a summary..." → **MUST** apply summary format
- "show me a summary" → **MUST** apply summary format
- "what's the summary" → **MUST** apply summary format
- Any phrase containing "summary" → **MUST** apply summary format

**❌ INCORRECT - Don't ignore trigger phrases**:

- Providing plain text response when user asks for "summary"
- Using different formatting when summary is requested
- Skipping the markdown code block wrapper

**MANDATORY Implementation**: Always wrap the summary in a markdown code block as defined in `summary-structure.md`.

### **2. :: Automatic High-Level Summary Detection**

**✅ CORRECT - MANDATORY ENFORCEMENT**: Apply summary format when providing ANY of these content types:

- **Analysis Results**: When presenting findings from analysis tasks → **MUST** use summary format
- **Issue Lists**: When listing problems, issues, or inconsistencies found → **MUST** use summary format
- **Overview Content**: When giving high-level explanations of complex topics → **MUST** use summary format
- **Findings Reports**: When reporting on investigations or audits → **MUST** use summary format
- **Multi-Point Responses**: When providing structured information across multiple categories → **MUST** use summary format
- **Comprehensive Reviews**: When reviewing documents, code, or systems → **MUST** use summary format

**✅ CORRECT - Detection Triggers (AUTOMATIC APPLICATION REQUIRED)**:

- Presenting multiple categories of information → **AUTOMATICALLY** apply summary format
- Using phrases like "issues found," "problems identified," "analysis shows" → **AUTOMATICALLY** apply summary format
- Providing structured lists across different topics → **AUTOMATICALLY** apply summary format
- Giving overviews of complex subjects → **AUTOMATICALLY** apply summary format
- Reporting findings from investigations → **AUTOMATICALLY** apply summary format

**MANDATORY Implementation**: Automatically wrap in summary format template when these patterns are detected. **NO EXCEPTIONS**.

### **3. :: Post-File-Generation Display Requirements**

**✅ CORRECT - MANDATORY ENFORCEMENT**: When generating analysis output files (fluency outputs, testing analyses, optimization reports, etc.) that include executive summaries:

- After successfully writing the analysis file, the AI agent **MUST** also display the executive summary content to the user
- The displayed executive summary **MUST** be wrapped in a markdown code block using the summary format structure
- The displayed summary **MUST** follow the exact summary format requirements (numbered lists with 3-space indented dash bullets)
- This applies even if the command doesn't explicitly require displaying the summary
- The summary formatting rules have `trigger: always_on` and analysis results automatically trigger summary format application

**Rationale**: The `trigger: always_on` flag means summary formatting rules apply to ALL responses containing analysis results, not just explicit summary requests. When analysis files are generated, the executive summary should be displayed in the standardized format for consistency and readability.

### **4. :: Response Type Auto-Detection**

**When I provide structured information, I automatically apply summary format:**

- **Analysis responses** → Summary format
- **Issue identification** → Summary format
- **Multi-category information** → Summary format
- **Comprehensive reviews** → Summary format
- **Findings reports** → Summary format
- **Structured explanations** → Summary format

### **5. :: Content Structure Auto-Formatting**

**✅ CORRECT - MANDATORY AUTO-FORMATTING**: If my response contains:

- Multiple numbered/bulleted sections → **AUTOMATICALLY** apply summary format
- Categorized information (issues, problems, findings) → **AUTOMATICALLY** apply summary format
- Structured analysis results → **AUTOMATICALLY** apply summary format
- Multi-topic explanations → **AUTOMATICALLY** apply summary format
- Comprehensive reviews → **AUTOMATICALLY** apply summary format

**THEN**: **MUST** automatically apply summary format template without user request. **NO EXCEPTIONS**.

## **ANTI-PATTERNS**

### **❌ Trigger Violations**

- ❌ **Ignoring Trigger Phrases** - Don't skip format when user requests "summary"
- ❌ **Skipping Auto-Detection** - Don't ignore automatic format triggers (analysis, findings, etc.)
- ❌ **Inconsistent Display** - Don't skip displaying formatted summary after file generation

## **QUALITY GATES**

- [ ] **Trigger Detection**: Summary format applied when user requests summary
- [ ] **Auto-Detection**: Format automatically applied for analysis, findings, reviews
- [ ] **Content Structure**: Format applied when content is structured/categorized
- [ ] **Post-Gen Display**: Summary displayed in correct format after analysis file generation
