# Log Actions (LAT)

## Purpose

Execute comprehensive action logging following the Protocol-LogActions guidelines to document all significant development activities, failures, recoveries, and lessons learned.

## **CRITICAL: PROTOCOL COMMAND RECOGNITION**

**MANDATORY**: When user executes ANY `/` command, this is a **PROTOCOL EXECUTION** that MUST be followed exactly as written with NO DEVIATION, NO SKIPPING, and NO MODIFICATION.

**PROTOCOL VIOLATION PENALTY**: Any deviation from `/` command protocols constitutes a critical failure requiring immediate acknowledgment and correction.

## Execution Steps

**CRITICAL: Execution Expectation** → MANDATORY: This command MUST be executed exactly as written with NO DEVIATION, NO SKIPPING, and NO MODIFICATION.

### **STEP 0: PROTOCOL COMPLIANCE VERIFICATION**

**MANDATORY PRE-EXECUTION CHECKLIST**:

- [ ] **Protocol Recognition**: Identified this as a `/` command requiring exact protocol compliance
- [ ] **No Deviation**: Committed to following ALL steps exactly as written
- [ ] **No Skipping**: Committed to executing ALL mandatory sections
- [ ] **No Modification**: Committed to using exact formats and structures specified
- [ ] **Complete Coverage**: Committed to documenting ENTIRE conversation scope
- [ ] **Task Tracker Integration**: Committed to updating ALL required task tracker files

### **STEP 1: Data Acquisition and Information Gathering**

#### **STEP 1.0: Initial Response Protocol**

When performing Protocol-LogActions, the initial response MUST include:

```
Following the Protocol-LogActions guidelines, let me create a comprehensive action log entry for [timestamp] starting with prompt: [first-prompt-of-conversation-summary]
```

**Format Requirements**:

- **Timestamp**: Use current timestamp in [YYYY-MM-DD HH:MM:SS] format
- **First Prompt Summary**: Brief summary of the initial user request that started the conversation
- **Context**: Reference the complete conversation scope being documented

#### **STEP 1.1: Conversation Scope Requirements**

- **Complete Conversation Coverage**: Action logs MUST reflect the ENTIRE conversation from initial prompt to completion, not just since summary creation
- **Multi-Package Conversations**: If conversation spans multiple packages, each package's feature docs MUST be updated in the same comprehensive manner
- **Chronological Accuracy**: Document events in the order they occurred, including all failed attempts and corrections
- **Task Tracker Synchronization**: Both Actions Log entries and Task Tracker updates MUST reference the complete conversation from the very first prompt, not just recent completions
- **Subsequent LAT Window**: If a LAT was previously performed within the current conversation, set the start of the review window to the first user prompt after the previous LAT's output. Do NOT include the prior LAT's output itself in the new scope.

#### **STEP 1.2: Data Verification Protocol**

- **MANDATORY File Timestamp Verification**: Use PowerShell `Get-FileStats.ps1` to get actual file modification times
- **Specific Files**: `_scripts\ps\Get-FileStats.ps1 -FilePaths @("file1.ts", "file2.js")`
- **Project Directory**: `_scripts\ps\Get-FileStats.ps1 -Directories @("packages/project-name")`
- **Format**: Use `[YYYY-MM-DD HH:MM:SS]` format from verification output
- **NEVER**: Use estimated dates, current system time, or placeholders
- **CRITICAL**: Every action log entry MUST include actual file timestamps from PowerShell verification
- **VIOLATION PENALTY**: Any action log entry without verified timestamps constitutes a critical protocol violation

**Usage Directive (Non-Optional)**:

- Do NOT pipe the script output to `cat`/`Get-Content`/`ConvertTo-Json`. Use the script's default text output or capture the PSObject and access properties directly.
- Canonical usage (copy/paste):

```powershell
# Default text output (no flags)
_scripts\ps\Get-FileStats.ps1 -FilePaths @('path1','path2')
_scripts\ps\Get-FileStats.ps1 -Directories @('dir1','dir2')

# Object mode when you need properties
$s = _scripts\ps\Get-FileStats.ps1 -FilePaths @('path1','path2') -Output object
$s.Maximum.ToString('yyyy-MM-dd HH:mm:ss')  # Latest mod time for log entries

# Per-file verification (optional)
$files = @('path1','path2')
$files | ForEach-Object { '{0}  {1}' -f $_, (Get-Item $_).LastWriteTime.ToString('yyyy-MM-dd HH:mm:ss') }

# Text with per-file list
_scripts\ps\Get-FileStats.ps1 -FilePaths @('path1','path2') -PerFile
```

**Output Interpretation**:

- `Current Time`: The current system time when analysis ran (informational; do not use for log timestamps)
- `Chat Duration`: Time between oldest and newest file modifications among targets (conversation span)
- `Last Change`: Time since the most recent modification (now − Maximum)
- `Targets`: Number of target locations passed (files + directories)
- `Processed`: Total files processed (shown only when directories are used)
- `Maximum` / `Minimum`: Latest / oldest file modification timestamps across all processed files

#### **STEP 1.3: Information Gathering Standards**

- **Explicit User Requests**: Only document when user explicitly requests documentation
- **Completion Confirmation**: User must confirm work is actually complete before documentation
- **Scope Definition**: Clearly define what should be documented based on user direction
- **Success Focus**: Document successful outcomes and conversation elements that directly enabled success
- **Documentation-Only Changes Excluded**: Documentation-only changes (updating docs, comments, README files) should NOT be tracked in completed tasks or actions log
- **Code/Implementation Focus**: Only track changes that involve actual code implementation, bug fixes, feature additions, or architectural changes

#### **STEP 1.4: MANDATORY DECISION LOGIC APPLICATION**

**CRITICAL**: Before proceeding, MUST determine documentation scope using exact decision criteria:

1. **Is this project-targeted change?** (build system, tooling, architecture affecting multiple packages)
    - YES → Write to `docs/FOCUSEDUX-Actions-Log.md` ONLY
    - NO → Continue to step 2

2. **Is this multi-package conversation?** (multiple packages referenced)
    - YES → Update EACH referenced package's logs and task tracker
    - NO → Continue to step 3

3. **Is this single subpackage only?**
    - YES → Update ONLY that package's logs and task tracker

**VIOLATION**: Any failure to apply this decision logic constitutes a critical protocol violation.

#### **STEP 1.5: Multi-Package Documentation Protocol**

- **Project-Level Actions**: For project-wide changes (build system, tooling, architecture), use `docs/FOCUSEDUX-Actions-Log.md`
- **Package-Specific Logs**: `{package-top-level}/_docs/_{subpackage-name}_Actions-Log.md`
- **Package Task Tracker**: `{package-top-level}/_docs/_{subpackage-name}_Task-Tracker.md`
- **Comprehensive Coverage**: Each package log must include the same level of detail as the main Actions Log
- **Cross-Package Dependencies**: Document how changes in one package affected others
- **Consistent Structure**: All package-specific logs must follow the same 12-section structure
- **Main Log Reference**: Package-specific logs should reference the main Actions Log entry for context
- **Examples**:
    - `libs/vsix-packager/_docs/_vsix-packager_Actions-Log.md`
    - `libs/vsix-packager/_docs/_vsix-packager_Task-Tracker.md`
    - `packages/dynamicons/_docs/_assets_Actions-Log.md`
    - `packages/dynamicons/_docs/_assets_Task-Tracker.md`
- **Decision Criteria**:
    - **Project-Level**: Changes affecting multiple packages, build system, tooling, or overall architecture
    - **Package-Level**: Changes specific to a single package's functionality, implementation, or features

#### **STEP 1.6: MANDATORY TASK TRACKER INTEGRATION**

**CRITICAL**: EVERY action log entry MUST update corresponding task tracker files. This is NON-NEGOTIABLE.

**MANDATORY STEPS**:

1. **Read Current Task Tracker**: Load `{package-top-level}/_docs/_{subpackage-name}_Task-Tracker.md`
2. **Move Completed Tasks**: Move ALL completed tasks from "Pending Tasks" to "Completed Tasks" with timestamps
3. **Add New Tasks**: Add ANY new tasks identified during conversation to "Pending Tasks"
4. **Update Future Enhancements**: Add new proposed enhancements to "Future Enhancement Suggestions"
5. **Apply Task Organization Protocol**: Ensure tasks exist in either Current/Pending OR Future Enhancements, not both

**VIOLATION**: Any action log entry without corresponding task tracker updates constitutes a critical protocol violation.

- **Automatic Updates**: Action log entries MUST automatically update the corresponding task tracker file
- **Completed Tasks**: Move completed tasks from "Pending Tasks" to "Completed Tasks" section with timestamp
- **New Tasks**: Add any new tasks identified during the conversation to "Pending Tasks" section
- **Future Enhancements**: Add new proposed enhancements to "Future Enhancement Suggestions" section
- **Conversation Reference**: All task tracker updates MUST reference the complete conversation from first prompt
- **Consolidation**: Maintain consolidated "Future Enhancement Suggestions" section that aggregates all suggestions
- **Deduplication**: When new enhancements are proposed, add them to consolidated list if unique, or add sub-bullets to existing items

#### **STEP 1.7: Default Action Log Structure Protocol**

- **Completed Tasks Placement**: Completed tasks MUST be placed underneath the "Future Enhancement Suggestions" section
- **Combined Format**: Use simple flat list format with `- [x]` checkboxes for completed tasks
- **No Duplicates**: Ensure only one "Completed Tasks" section exists per action log
- **Chronological Order**: List completed tasks in chronological order of completion
- **Comprehensive Coverage**: Include ALL completed tasks from the entire conversation, not just recent ones

#### **STEP 1.8: Conversation Analysis Protocol**

- **Systematic Review**: Review every turn of the conversation from initial user request to completion
- **Self-Critical Focus**: Focus on failures, corrections, and actionable lessons
- **Success Analysis**: Document what core principles or patterns led to efficient and correct outcomes
- **Failure Analysis**: Document where approach failed, root causes, and how user feedback corrected behavior
- **Actionable Lessons**: Extract the most critical, transferable lessons from the interaction

### **STEP 2: Mandatory Task Tracker Updates**

Every action log entry MUST also update the corresponding task tracker file with the following process:

#### **STEP 2.0: MANDATORY TASK TRACKER INTEGRATION**

**CRITICAL**: EVERY action log entry MUST update corresponding task tracker files. This is NON-NEGOTIABLE.

**MANDATORY STEPS**:

1. **Read Current Task Tracker**: Load `{package-top-level}/_docs/_{subpackage-name}_Task-Tracker.md`
2. **Move Completed Tasks**: Move ALL completed tasks from "Pending Tasks" to "Completed Tasks" with timestamps
3. **Add New Tasks**: Add ANY new tasks identified during conversation to "Pending Tasks"
4. **Update Future Enhancements**: Add new proposed enhancements to "Future Enhancement Suggestions"
5. **Apply Task Organization Protocol**: Ensure tasks exist in either Current/Pending OR Future Enhancements, not both

**VIOLATION**: Any action log entry without corresponding task tracker updates constitutes a critical protocol violation.

#### **STEP 2.1: Task Tracker Update Process**

1. **Read Current Task Tracker**: Load the current task tracker file (`{package-top-level}/_docs/_{subpackage-name}_Task-Tracker.md`)
2. **Identify Completed Tasks**: Review the conversation to identify all tasks that were completed (exclude documentation-only changes)
3. **Move Completed Tasks**: Move completed tasks from "Pending Tasks" to "Completed Tasks" section with current timestamp
4. **Add New Tasks**: Add any new tasks identified during the conversation to "Pending Tasks" section (exclude documentation-only tasks)
5. **Update Future Enhancements**: Add new proposed enhancements to "Future Enhancement Suggestions" section
6. **Maintain Consolidation**: Ensure the consolidated "Future Enhancement Suggestions" section aggregates all suggestions
7. **Reference Complete Conversation**: All updates must reference the entire conversation from first prompt
8. **Exclude Documentation Changes**: Do not track documentation-only changes (updating docs, comments, README files) in task tracker

#### **STEP 2.2: Task Tracker Format Requirements**

- **Completed Tasks**: Use format `- [YYYY-MM-DD HH:MM:SS]` with `- [x]` checkboxes for individual tasks
- **Pending Tasks**: Use format `- [ ]` checkboxes for incomplete tasks
- **Future Enhancements**: Use bold categories with sub-bullets for specific suggestions
- **Chronological Order**: List completed tasks in chronological order of completion
- **No Duplicates**: Ensure no duplicate tasks exist across sections

#### **STEP 2.3: What Should Be Tracked**

**Include in Completed Tasks and Actions Log:**

- Code implementation changes (new features, bug fixes, refactoring)
- Architectural changes (package structure, build system modifications)
- Configuration changes (project.json, package.json, build configs)
- Test implementation (unit tests, integration tests, test frameworks)
- Performance improvements (optimizations, caching implementations)
- Error handling improvements (validation, error recovery)
- CLI/API changes (new commands, parameter changes)

**Exclude from Completed Tasks and Actions Log:**

- Documentation updates (README files, comments, doc strings)
- Markdown file changes (guides, tutorials, explanations)
- Comment-only changes (code comments, inline documentation)
- Formatting changes (whitespace, indentation, style)
- Documentation reorganization (moving docs, renaming files)
- Protocol file updates (unless they involve actual implementation changes)

#### **STEP 2.4: Task Duplication Prevention Protocol**

**CRITICAL RULE**: When tasks are completed and added to the completion section, they MUST be removed from the pending or current sections to avoid duplication and maintain clean organization.

**Requirements**:

- **Immediate Removal**: Completed tasks must be removed from pending/current sections immediately when moved to completion
- **Single Location**: Tasks should exist in only one section at a time
- **Clean Organization**: Prevents task duplication and maintains clear task status
- **Consistent Application**: Apply this rule to all task tracker files and action logs

**Violation Prevention**:

- **Before Adding to Completion**: Verify the task is not already in completion section
- **After Adding to Completion**: Immediately remove from pending/current sections
- **Cross-Reference Check**: Ensure no duplicate tasks exist across all sections
- **Status Validation**: Confirm task status is accurately reflected in only one section

#### **STEP 2.5: Task Organization Protocol**

**CRITICAL RULE**: Tasks must be organized so that they exist in either Current/Pending sections OR Future Enhancement Suggestions, but not both. Tasks that are even minorly affected by current work must be moved to Current/Pending with detailed sub-bullets.

**Requirements**:

- **Single Location**: Tasks should exist in only one section at a time (Current/Pending OR Future Enhancements)
- **No Duplicates**: Eliminate any tasks that appear in both Current/Pending and Future Enhancement sections
- **Affected Task Movement**: Tasks in Future Enhancement Suggestions that are even minorly affected by current work must be moved to Current/Pending sections
- **Detailed Sub-bullets**: When moving tasks from Future Enhancements to Current/Pending, expand them with detailed sub-bullets
- **Clear Separation**: Maintain clear distinction between active tasks and future enhancement suggestions

**Implementation Guidelines**:

- **Before Adding Tasks**: Check if task already exists in Future Enhancement Suggestions
- **When Moving Tasks**: Add "(Affected by recent [work type] work)" annotation to show why task was moved
- **Expansion Requirement**: Add detailed sub-bullets when moving tasks from Future Enhancements to Current/Pending
- **Cross-Reference Check**: Ensure no duplicate tasks exist across all sections
- **Organization Note**: Add note to Future Enhancement Suggestions section explaining the organization rules

#### **STEP 2.6: Strengthening Weak/Flaky Implementations**

**Purpose**: Identify and document areas where implementations may be weak, flaky, or prone to failure, requiring future strengthening.

**Task Tracker Section**: Add "Strengthening Weak/Flaky Implementations" section to task tracker files.

**Content Requirements**:

- **Weak Implementation Identification**: Document areas where current implementation may be fragile or unreliable
- **Flaky Behavior Documentation**: Record intermittent failures, timing issues, or environment-dependent problems
- **Strengthening Strategies**: Outline specific approaches to make implementations more robust
- **Priority Assessment**: Categorize by severity and impact on system reliability
- **Future Enhancement Planning**: Connect to Future Enhancement Suggestions for systematic improvement

**Examples**:

- **Timing-Dependent Operations**: Operations that may fail due to race conditions or timing issues
- **Environment-Specific Failures**: Code that works in some environments but fails in others
- **Resource-Intensive Operations**: Operations that may fail under high load or resource constraints
- **External Dependency Failures**: Code that may fail when external services are unavailable
- **Configuration-Dependent Behavior**: Code that behaves differently based on configuration settings

### **STEP 3: Mandatory Sections**

Every action log entry MUST include the following sections in this exact order:

#### **STEP 3.1: Section Template Structure**

```markdown
## **[YYYY-MM-DD HH:MM:SS] [Brief Title]**

### **Summary**

[Single sentence with key metrics]

### **Root Cause Analysis**

- **Problem 1**: [Specific technical issue]
- **Problem 2**: [Architectural misalignment]

### **Key Implementations**

#### **[Component Name]**

- **File**: `path/to/file.ts` - [Purpose and technical details]

### **Key Anti-patterns**

- **[Anti-pattern Name]**: [Description and resolution]

### **Technical Architecture**

- **[Pattern Name]**: [Compliance details and metrics]

### **Performance and Quality Metrics**

- **[Metric Name]**: [Before → After with percentages]

### **What Was Tried and Failed**

- **[Failed Approach]**: [Technical details and why it failed]

### **Critical Failures and Recovery**

1. **[Failure Name]**:
    - **Failure**: [Description]
    - **Root Cause**: [Analysis]
    - **Recovery**: [Solution]
    - **Prevention**: [Strategy]

### **Lessons Learned**

**Correct Methodology**:

- **[Pattern]**: [Best practice]

**Pitfalls and Problems**:

- **[Anti-pattern]**: [What to avoid]

### **Files Created/Modified**

- `path/to/file.ts` - [Purpose and technical details]

### **Protocol Violations**

- **[Violation]**: [Description and correction]

### **Prevention Strategy**

- [Actionable prevention strategies]

### **Future Enhancement Suggestions**

- **[Category]**: [Specific technical enhancements]
```

#### **STEP 3.2: Required Sections (6 Core + 2 Optional)**

1. **Summary** - Concise one-sentence description of the primary accomplishment
2. **Root Cause Analysis** - Detailed analysis of original problems and architectural issues
3. **What Was Tried and Failed** - Comprehensive list of failed approaches with technical details
4. **Critical Failures and Recovery** - Detailed failure analysis with root causes and prevention strategies
5. **Files Created/Modified** - Complete list with specific purposes and technical details
6. **Prevention Strategy** - Actionable prevention strategies for future similar work
7. **Key Implementations** - Organized technical implementations with specific details (OPTIONAL - if complex)
8. **Lessons Learned** - Comprehensive lessons with specific technical insights (OPTIONAL - if non-obvious)

### **STEP 4: Content Requirements**

#### **STEP 4.1: Summary Section Template**

```markdown
### **Summary**

Successfully [ACTION] [TARGET], achieving [METRIC1], [METRIC2], and [OUTCOME].

Example: "Successfully transformed standalone scripts collection into unified Nx core package architecture, achieving 100% type safety, 79% code quality improvement, and complete architectural compliance with established patterns."
```

#### **STEP 4.2: Root Cause Analysis Template**

```markdown
### **Root Cause Analysis**

- **Original Architecture**: [Specific technical description]
- **Architectural Misalignment**: [How it violated standards]
- **Build System Issues**: [Specific problems with tooling]
- **Type Safety Deficiencies**: [Specific type issues]
```

#### **STEP 4.3: Key Implementations Template**

```markdown
### **Key Implementations**

#### **[Component Name]**

- **File**: `path/to/file.ts` - [Purpose and technical details]
- **Key Features**: [Specific functionality implemented]
```

#### **STEP 4.4: Critical Failures Template**

```markdown
### **Critical Failures and Recovery**

1. **[Failure Name]**:
    - **Failure**: [Specific description]
    - **Root Cause**: [Technical analysis]
    - **Recovery**: [Solution implemented]
    - **Prevention**: [Future strategy]
```

#### **STEP 4.5: Lessons Learned Template**

```markdown
### **Lessons Learned**

**Correct Methodology**:

- **[Pattern]**: [Best practice with technical details]

**Pitfalls and Problems**:

- **[Anti-pattern]**: [What to avoid with technical details]
```

#### **STEP 4.6: What Was Tried and Failed Template**

```markdown
### **What Was Tried and Failed**

- **[Failed Approach]**: [Technical details and why it failed]
- **[Misinterpretation]**: [What was misunderstood and why]
- **[Configuration Error]**: [Specific configuration issues]
```

**MANDATORY**: This section is REQUIRED - all failed attempts that led to the successful solution must be documented

### **STEP 5: Quality Standards**

#### **STEP 5.1: Technical Precision Requirements**

- All technical details must be specific and accurate
- File paths must be exact and complete
- Configuration details must include specific values
- Error messages must be quoted exactly

#### **STEP 5.2: Quantified Metrics Requirements**

- All improvements must include specific numbers
- Before/after comparisons must be included
- Percentages must be calculated and included
- Performance metrics must be measurable

#### **STEP 5.3: Comprehensive Coverage Requirements**

- All significant activities must be documented
- All failures must be analyzed with root causes
- All lessons learned must be actionable
- All prevention strategies must be specific

#### **STEP 5.4: Professional Language Requirements**

- Use clear, technical, and professional language
- Avoid vague or ambiguous descriptions
- Use specific technical terminology
- Maintain consistent formatting throughout

### **STEP 6: Formatting Requirements**

#### **STEP 6.1: Section Headers**

- Use `####` for major sections
- Use `-` for bullet points
- Use `**bold**` for emphasis
- Use `code` for file paths and technical terms

#### **STEP 6.2: Technical Details Formatting**

- File paths: `packages/dynamicons/assets/src/orchestrators/asset-orchestrator.ts`
- Configuration: `"executor": "@nx/esbuild:esbuild"`
- Error messages: `"Could not resolve" errors`
- Metrics: `100% elimination of explicit 'any' types (16 → 0)`

#### **STEP 6.3: Hierarchical Organization**

- Major sections use `####` headers
- Sub-sections use bullet points with `-`
- Technical details use nested bullet points
- Specific examples use `code` formatting

### **STEP 7: Validation Checklist**

#### **STEP 7.1: Pre-Execution Checklist**

- [ ] **Conversation Scope**: Complete conversation from first prompt identified
- [ ] **File Verification**: PowerShell Get-FileStats.ps1 command prepared
- [ ] **Timestamp Verification**: Actual file modification times obtained using PowerShell script
- [ ] **Package Analysis**: All affected packages identified
- [ ] **Task Tracker**: Current state loaded and ready for updates

#### **STEP 7.2: Content Validation Checklist**

- [ ] All 6 core sections are present
- [ ] Complete conversation coverage from initial prompt to completion
- [ ] If a prior LAT exists for this conversation, the start boundary is set to the first prompt following that LAT's output
- [ ] **MANDATORY**: File timestamps verified using PowerShell Get-FileStats.ps1
- [ ] **MANDATORY**: Timestamp format is [YYYY-MM-DD HH:MM:SS] from verification output
- [ ] **MANDATORY**: NO estimated dates, current system time, or placeholders used
- [ ] Summary includes key metrics or outcomes
- [ ] Root cause analysis identifies specific technical causes
- [ ] Failed approaches include specific technical details (MANDATORY section)
- [ ] Critical failures include all four sub-sections (failure, root cause, recovery, prevention)
- [ ] Files list includes complete paths and purposes
- [ ] Prevention strategies are actionable and specific
- [ ] Optional sections included only if they add value (Key Implementations if complex, Lessons Learned if non-obvious)

#### **STEP 7.3: Multi-Package Validation Checklist**

- [ ] Multi-package conversations have package-specific logs created
- [ ] Package-specific logs follow same comprehensive structure
- [ ] **Task Tracker Updated**: Corresponding task tracker file has been updated with completed tasks, new tasks, and future enhancements
- [ ] **Completed Tasks Moved**: All completed tasks moved from "Pending Tasks" to "Completed Tasks" with timestamps
- [ ] **New Tasks Added**: Any new tasks identified during conversation added to "Pending Tasks"
- [ ] **Future Enhancements Updated**: New proposed enhancements added to "Future Enhancement Suggestions"
- [ ] **Task Organization Protocol**: Tasks exist in either Current/Pending OR Future Enhancement Suggestions, but not both
- [ ] **Affected Tasks Moved**: Tasks in Future Enhancement Suggestions that are even minorly affected by current work have been moved to Current/Pending with detailed sub-bullets
- [ ] **Strengthening Weak/Flaky Implementations**: New section added to task tracker for identifying fragile areas requiring future strengthening
- [ ] **Conversation Reference**: All task tracker updates reference the complete conversation from first prompt
- [ ] **No Duplicates**: No duplicate tasks exist across sections
- [ ] **Proper Formatting**: Task tracker uses correct checkbox format and chronological ordering
- [ ] **Documentation Changes Excluded**: Documentation-only changes (README, comments, docs) are NOT tracked in completed tasks or actions log
- [ ] **Code/Implementation Focus**: Only actual code implementation, bug fixes, features, and architectural changes are tracked

### **STEP 8: Examples and Templates**

#### **STEP 8.1: Summary Template**

```markdown
### **Summary**

Successfully [ACTION] [TARGET], achieving [METRIC1], [METRIC2], and [OUTCOME].

Example: "Successfully transformed standalone scripts collection into unified Nx core package architecture, achieving 100% type safety, 79% code quality improvement, and complete architectural compliance with established patterns."
```

#### **STEP 8.2: Root Cause Analysis Template**

```markdown
### **Root Cause Analysis**

- **Original Architecture**: [Specific technical description]
- **Architectural Misalignment**: [How it violated standards]
- **Build System Issues**: [Specific problems with tooling]
- **Type Safety Deficiencies**: [Specific type issues]
```

#### **STEP 8.3: Critical Failure Template**

```markdown
### **Critical Failures and Recovery**

1. **[Failure Name]**:
    - **Failure**: [Specific description]
    - **Root Cause**: [Technical analysis]
    - **Recovery**: [Solution implemented]
    - **Prevention**: [Future strategy]
```

#### **STEP 8.4: Lessons Learned Template**

```markdown
### **Lessons Learned**

**Correct Methodology**:

- **[Pattern]**: [Best practice with technical details]

**Pitfalls and Problems**:

- **[Anti-pattern]**: [What to avoid with technical details]
```

#### **STEP 8.5: Task Tracker Template**

```markdown
- **Completed Tasks Section**:
    - [YYYY-MM-DD HH:MM:SS]
        - [x] [Task description]
        - [x] [Task description]

- **Pending Tasks Section**:
    - [ ] [Task description]
    - [ ] [Task description]

- **Future Enhancement Suggestions**:
    - **[Category]**: [Specific technical enhancements]

- **Strengthening Weak/Flaky Implementations**:
    - **[Priority Level]**: [Weak implementation description]
    - **[Flaky Behavior]**: [Intermittent failure description]
    - **[Strengthening Strategy]**: [Specific approach to make more robust]
```

### **STEP 9: Protocol Compliance**

#### **STEP 9.1: Compliance Requirements**

This protocol MUST be followed for all action log entries to ensure:

- **Consistency**: All entries follow the same comprehensive structure
- **Completeness**: All significant activities are documented with appropriate detail
- **Value**: Entries provide actionable insights for future similar work
- **Professional Quality**: Entries meet high standards for technical documentation
- **Traceability**: Complete historical record of all development activities

#### **STEP 9.2: Enforcement Protocol**

- Action log entries that do not meet these requirements must be revised
- Missing sections must be added with appropriate content
- Insufficient detail must be expanded with specific technical information
- Non-compliant formatting must be corrected to match requirements
- **CRITICAL**: Timestamp violations must be corrected immediately using PowerShell verification
- Validation checklist must be completed before finalizing any entry

#### **STEP 9.3: Error Recovery Patterns**

- **Missing Timestamps**: Use `_scripts\ps\Get-FileStats.ps1 -FilePaths @("file.ts")`
- **Estimated Timestamps**: IMMEDIATELY replace with actual PowerShell verification results
- **Wrong Timestamp Format**: Convert to [YYYY-MM-DD HH:MM:SS] format from verification output
- **Incomplete Coverage**: Review conversation from first user prompt
- **Package Confusion**: Verify package type with `nx_project_details` tool
- **Template Violations**: Reference section templates for proper formatting

#### **STEP 9.4: Required Tool Usage**

- **File Reading**: `read_file` tool for all documentation
- **Timestamp Verification**: `run_terminal_cmd` with PowerShell script
- **Package Analysis**: `mcp_nx-mcp_nx_project_details` tool
- **Task Updates**: `search_replace` tool for task tracker modifications

#### **STEP 9.7: PROTOCOL VIOLATION DETECTION**

**MANDATORY**: Before finalizing any entry, verify compliance with ALL protocol requirements:

- [ ] **Protocol Recognition**: Identified this as a `/` command requiring exact protocol compliance
- [ ] **Decision Logic Applied**: Correctly determined documentation scope using exact criteria
- [ ] **Package Logs Created**: Created package-specific logs for all affected packages
- [ ] **Task Tracker Updated**: Updated ALL corresponding task tracker files
- [ ] **Complete Coverage**: Documented ENTIRE conversation from first prompt
- [ ] **Verified Timestamps**: Used actual PowerShell verification results
- [ ] **All Mandatory Sections**: Included all required sections with proper formatting

**CRITICAL**: Any missing element constitutes a protocol violation requiring immediate correction.

## Usage

Execute when user requests comprehensive action logging or when directed to do any definitional variation of "doing a 'LAT'".

## Critical Requirements

- **NO DEVIATION**: All rules must be followed exactly as written
- **NO SKIPPING**: No steps may be skipped, abbreviated, or modified
- **NO SELECTIVE COMPLIANCE**: All rules apply to all actions
- **FAILURE TO COMPLY**: Violating these rules constitutes a critical protocol violation
- **MANDATORY TIMESTAMPS**: Every action log entry MUST include actual file timestamps from PowerShell verification
- **COMPLETE COVERAGE**: Document the ENTIRE conversation from initial prompt to completion
- **TASK TRACKER SYNC**: Both Actions Log entries and Task Tracker updates MUST reference the complete conversation
- **DECISION LOGIC MANDATORY**: MUST apply exact decision criteria to determine documentation scope
- **PACKAGE LOGS MANDATORY**: MUST create package-specific logs for all affected packages
- **PROTOCOL RECOGNITION**: `/` commands are PROTOCOLS that must be followed exactly, not suggestions

---

_This protocol ensures that all action logs provide comprehensive, detailed, and valuable documentation of development activities, failures, recoveries, and lessons learned for future reference and improvement._
