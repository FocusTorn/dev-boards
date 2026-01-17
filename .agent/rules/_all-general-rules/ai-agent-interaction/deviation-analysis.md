---
trigger: always_on
*
---

# Deviation Analysis Rules

## **DEVIATION DETECTION AND ANALYSIS**

### **1. :: Deviation Marker Recognition**

**✅ CORRECT - Recognize deviation markers**:

When the user provides a message starting with `Deviation:` followed by error information, the AI Agent MUST:

1. **Immediately Stop**: Stop any current task or response - deviation handling protocol supersedes all normal task flow
2. **Acknowledge Deviation**: Explicitly acknowledge that a deviation has been detected
3. **Begin Root Cause Analysis**: Start the deviation analysis protocol (see section 2)
4. **Do NOT Defend**: Do not explain why the deviation occurred until root cause analysis is complete
5. **Do NOT Make Excuses**: Focus on identifying the systemic cause, not justifying actions
6. **Do NOT Fix**: Do NOT proceed with any fixes, changes, or normal task processing until deviation analysis is complete and user approves remediation

**CRITICAL ENFORCEMENT**: When ANY message contains `Deviation:` marker at the start, immediately STOP all current processing, acknowledge the deviation marker explicitly, and enter deviation handling protocol mode. Do NOT proceed with any fixes, changes, or normal task processing until deviation analysis is complete. The deviation handling protocol takes precedence over all other tasks and must be completed before returning to normal operation.

**CRITICAL ENFORCEMENT - MANDATORY PAUSE**: After a deviation is detected and acknowledged, the AI agent MUST explicitly pause and verify completion of ALL deviation protocol steps before taking ANY other actions. The agent MUST NOT proceed with new feature requests, bug fixes, or any other task until: (1) Root cause analysis is complete, (2) Executive summary is generated, (3) User has reviewed and approved remediation. If a new request arrives during deviation analysis, the agent MUST explicitly state: "Deviation analysis in progress - completing protocol before proceeding with new request."

**✅ CORRECT - Deviation marker format**:

```
Deviation: [Brief description of what was not followed]

[Error details, linting errors, or rule violation information]
```

**❌ INCORRECT - Ignoring deviation markers**:

- Continuing with current task after seeing `Deviation:`
- Fixing issues immediately without following deviation protocol
- Providing explanations before root cause analysis
- Defending actions instead of analyzing systemic causes
- Treating deviation as a one-time error rather than a systemic issue
- Proceeding with normal task processing before completing deviation analysis

### **2. :: Root Cause Analysis Protocol**

**✅ CORRECT - Systematic root cause analysis**:

When a deviation is detected, the AI Agent MUST perform the following analysis in order:

#### **2.1. :: Rule Discovery Analysis**

1. **Check Rule Discovery**: Determine if relevant rules were checked before the task
   - Was the workflow rule for proactive rule discovery followed?
   - Were relevant rule files read before starting the task?
   - Did the agent check `.agent/rules/` directory structure?

2. **Identify Missing Rule Checks**: List all rule files that should have been checked but weren't
   - Language-specific rules (`.agent/rules/by-language/[language]/`)
   - Tool-specific rules (`.agent/rules/tool/[tool]/`)
   - Formatting rules (`.agent/rules/formatting/`)
   - Universal rules (workflow, code-maintenance, documentation, etc.)

#### **2.2. :: Rule Sufficiency Analysis**

1. **Evaluate Existing Rules**: Determine if existing rules cover the deviation
   - Do existing rules explicitly address this scenario?
   - Are the rules clear and unambiguous?
   - Do the rules provide sufficient guidance to prevent the deviation?

2. **Identify Rule Gaps**: Determine what's missing from existing rules
   - Missing coverage: No rule exists for this scenario
   - Insufficient clarity: Rule exists but is unclear or ambiguous
   - Missing enforcement: Rule exists but lacks enforcement mechanisms
   - Incomplete scope: Rule covers part but not all of the scenario

#### **2.3. :: Execution Analysis**

1. **Check Execution Steps**: Determine if rules were followed during execution
   - Were all required steps from the rule executed?
   - Were steps executed in the correct order?
   - Were quality gates checked?

2. **Identify Execution Gaps**: Determine what execution steps were missed
   - Skipped steps: Required steps that were not executed
   - Wrong order: Steps executed in incorrect sequence
   - Missing validation: Quality gates or checks that were skipped

#### **2.4. :: Tool and Process Analysis**

1. **Evaluate Tool Usage**: Determine if tools were used correctly
   - Were appropriate tools selected for the task?
   - Were tools used with correct parameters?
   - Did tool outputs provide expected information?

2. **Identify Tool Gaps**: Determine if tool limitations contributed to deviation
   - Tool limitations: Tools that don't catch certain errors
   - Incomplete tool usage: Tools that should have been used but weren't
   - Tool output misinterpretation: Tool outputs that were misunderstood

### **3. :: Root Cause Categorization**

**✅ CORRECT - Categorize root cause**:

After completing root cause analysis, categorize the deviation as one of:

1. **Missing Rule**: No rule exists that covers this scenario
   - **Evidence**: No rule file or rule section addresses this scenario
   - **Remediation**: Create new rule or add to existing rule file

2. **Insufficient Rule**: Rule exists but is inadequate
   - **Evidence**: Rule exists but is unclear, ambiguous, or incomplete
   - **Remediation**: Modify existing rule to be more explicit and comprehensive

3. **Rule Discovery Failure**: Rules exist but weren't checked
   - **Evidence**: Relevant rules exist but weren't read before task
   - **Remediation**: Strengthen workflow rules for rule discovery

4. **Rule Application Failure**: Rules were checked but not applied
   - **Evidence**: Rules were read but not followed during execution
   - **Remediation**: Strengthen rule enforcement mechanisms

5. **Tool Limitation**: Tools used don't catch the error
   - **Evidence**: Tools were used correctly but didn't detect the issue
   - **Remediation**: Add additional validation steps or use additional tools

6. **Process Gap**: Missing process steps or quality gates
   - **Evidence**: Required process steps or quality gates are missing
   - **Remediation**: Add missing process steps or quality gates to rules

## **QUALITY GATES**

- [ ] **Deviation Acknowledged**: Deviation marker was recognized and acknowledged
- [ ] **Root Cause Analysis Complete**: All four analysis steps completed
- [ ] **Root Cause Categorized**: Deviation categorized as one of six root cause types

