---
trigger: always_on
---

# Interaction Pre-processing Rules

## **CRITICAL EXECUTION DIRECTIVE**

**AI Agent Directive**: Follow interaction rules exactly when encountering interaction markers in user messages.

**MANDATORY EXECUTION PROTOCOL**:

1. **NO DEVIATION**: All interaction rules must be followed exactly as written
2. **NO SKIPPING**: No steps may be skipped, abbreviated, or modified
3. **NO SELECTIVE COMPLIANCE**: All rules apply to all interaction handling activities
4. **FAILURE TO COMPLY**: Violating these rules constitutes a critical protocol violation

## **PRE-PROCESSING CHECK**

### **1. :: Message Prefix Detection**

**CRITICAL PRE-PROCESSING CHECK**: Before processing ANY user message, the AI agent MUST check if the message starts with any of the following interaction markers:

- `Deviation:` or `DEVIATION:`
- `Question:` or `question:`
- `Question,` or `question,`

**✅ CORRECT - Pre-processing protocol**:

1. **Check Message Prefix**: Before any other processing, check the first word(s) of the user message
2. **If Marker Detected**: Immediately stop all normal processing and enter the appropriate interaction protocol
3. **Priority Order**: If multiple markers are detected, process in this order:
   - `Deviation:` (highest priority - supersedes all other processing)
   - `Question:` or `Question,` (second priority - answer only, no actions)
4. **No Normal Processing**: Do NOT proceed with normal task processing until the interaction protocol is complete

**❌ INCORRECT - Skipping pre-processing check**:

- Processing the message normally before checking for markers
- Treating interaction markers as part of normal content
- Continuing with code changes or file edits when a marker is detected
- Ignoring markers because they appear with other content

## **EXECUTION PRIORITY MATRIX**

### **CRITICAL PRIORITY (Execute immediately - BEFORE ANY OTHER PROCESSING)**

- **Pre-Processing Check**: Check message prefix for interaction markers
- **Stop Current Task**: Immediately stop when interaction marker is detected
- **Acknowledge Marker**: Explicitly acknowledge marker detection
- **Enter Protocol**: Enter appropriate interaction protocol mode
- **Question Protocol Verification**: Before ANY tool call, verify question protocol compliance

### **HIGH PRIORITY (Execute for deviation handling)**

- **Begin Root Cause Analysis**: Start systematic root cause analysis protocol
- **Generate Executive Summary**: Create complete executive summary following format
- **Search Rule Files**: Discover all relevant rule files in workspace
- **Categorize Root Cause**: Determine which of six categories applies
- **Provide Specific Recommendations**: Include exact rule text and file paths

### **HIGH PRIORITY (Execute for question handling)**

- **Answer Question**: Provide clear, complete answer to the question
- **Provide Comparison**: If comparison requested, provide pros/cons analysis
- **No Actions**: Ensure no code changes or file edits are made

### **MEDIUM PRIORITY (Execute during normal operation)**

- **Wait for User Approval**: Present summary and wait for user review (deviation handling)
- **Collaborate on Remediation**: Work with user to refine recommendations (deviation handling)
- **Implement Approved Changes**: Execute only after explicit user approval (deviation handling)

## **QUALITY GATES**

### **Pre-Processing**

- [ ] **Message Prefix Checked**: Message prefix checked before any other processing
- [ ] **Marker Detected**: Interaction marker detected and acknowledged
- [ ] **Protocol Entered**: Appropriate interaction protocol entered

