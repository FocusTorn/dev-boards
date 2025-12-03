# Create - Phased Slash Command

## **REFERENCE FILES**

### **Documentation References**

- **ARCHITECTURE_DOCS**: `docs/_Architecture.md`
- **PACKAGE_ARCHETYPES**: `docs/_Package-Archetypes.md`
- **SOP_DOCS**: `docs/_SOP.md`
- **TESTING_STRATEGY**: `docs/testing/_Testing-Strategy.md`
- **ACTIONS_LOG**: `docs/Actions-Log.md`

### **Template References**

- **FLUENCY_ORCHESTRATOR**: `.cursor/commands/Deep Dive - Fluency of a package.md`
- **FLUENCY_PHASES**: `.cursor/commands/deep-dive-phases/`

### **Output File References**

- **STAGING_FILE**: `.cursor/ADHOC/{command-name}-output-staging.md`
- **FINAL_OUTPUT**: `.cursor/ADHOC/{command-name}-output-{target}.md`

---

## **COMMAND PURPOSE**

**Primary Objective**: Create a multi-phased command structure following the fluency pattern
**Scope**: Generate orchestrator and phase commands with reference system integration
**Output**: Complete phased command system with staging and synthesis

## **EXECUTION PROTOCOL**

### **STEP 1: INPUT ANALYSIS**

**AI TASK**: Analyze user input to extract command specifications

**DATA TO EXTRACT**:

- Source command or protocol reference
- Orchestrator file name and location
- Phase directory name and location
- Output file patterns (staging and final)
- Command-specific requirements and descriptions

**ANALYSIS PROCESS**:

1. **Extract Source Reference**: Identify source command, protocol, or description
2. **Parse File Structure**: Determine orchestrator and phase directory names
3. **Extract Output Patterns**: Identify staging and final output file patterns
4. **Parse Requirements**: Extract any command-specific requirements
5. **Generate Command Name**: Create appropriate command name if not provided

### **STEP 2: SOURCE ANALYSIS**

**AI TASK**: Analyze the source command or protocol to understand structure

**ANALYSIS PROCESS**:

1. **Read Source File**: Load the referenced command or protocol
2. **Identify Phases**: Extract logical phases from the source
3. **Map Dependencies**: Identify phase dependencies and execution order
4. **Extract Patterns**: Identify recurring patterns and structures
5. **Determine Output Format**: Understand expected output structure

### **STEP 3: ORCHESTRATOR GENERATION**

**AI TASK**: Create the main orchestrator command file

**ORCHESTRATOR STRUCTURE**:

```markdown
# {Command Name}

## **REFERENCE FILES**

### **Documentation References**

- **ARCHITECTURE_DOCS**: `docs/_Architecture.md`
- **PACKAGE_ARCHETYPES**: `docs/_Package-Archetypes.md`
- **SOP_DOCS**: `docs/_SOP.md`
- **TESTING_STRATEGY**: `docs/testing/_Testing-Strategy.md`
- **ACTIONS_LOG**: `docs/Actions-Log.md`

### **Output File References**

- **STAGING_FILE**: `{staging-file-path}`
- **FINAL_OUTPUT**: `{final-output-path}`

### **Phase Command References**

- **PHASE_1_CMD**: `@{command-name}-phase1-{name}.md`
- **PHASE_2_CMD**: `@{command-name}-phase2-{name}.md`
- **PHASE_N_CMD**: `@{command-name}-phaseN-{name}.md`

---

## **COMMAND PURPOSE**

**Primary Objective**: {Command purpose description}
**Scope**: {Command scope description}
**Output**: {Output description}

## **EXECUTION PROTOCOL**

### **STEP 1: PRE-EXECUTION SETUP**

**AI TASK**: Prepare for comprehensive analysis

**CLEANUP PROCESS**:

- [ ] Delete existing staging file: **STAGING_FILE**
- [ ] Delete existing output file: **FINAL_OUTPUT**
- [ ] Ensure clean workspace for new analysis

**REQUIREMENTS**:

- [ ] Target identified and accessible
- [ ] Context window allocated for deep analysis
- [ ] All required files can be read

### **STEP 2: PHASE EXECUTION SEQUENCE**

**AI TASK**: Execute analysis phases in sequence

**PHASE EXECUTION ORDER**:

1. **Phase 1: {Phase Name}** ✅ IMPLEMENTED
    - Execute: **PHASE_1_CMD**
    - Result: Writes to **STAGING_FILE**

2. **Phase 2: {Phase Name}** ✅ IMPLEMENTED
    - Execute: **PHASE_2_CMD**
    - Result: Appends to staging file

3. **Phase N: {Phase Name}** ✅ IMPLEMENTED
    - Execute: **PHASE_N_CMD**
    - Result: Creates **FINAL_OUTPUT**

### **STEP 3: EXECUTION VALIDATION**

**AI TASK**: Validate that all phases executed successfully

**VALIDATION PROCESS**:

- [ ] Check that staging file exists: **STAGING_FILE**
- [ ] Verify all phases are marked as complete (✅)
- [ ] Verify all required sections are present
- [ ] Confirm final output is complete

### **STEP 4: CLEANUP AND COMPLETION**

**AI TASK**: Clean up staging files and complete execution

**CLEANUP PROCESS**:

- [ ] Delete staging file: **STAGING_FILE** (unless otherwise specified)
- [ ] Verify all output files are generated and complete
- [ ] Confirm command execution is fully complete
- [ ] Document any cleanup exceptions or retained files

## **OUTPUT FORMAT**

### **ANALYSIS DOCUMENT LOCATION**

**File**: **FINAL_OUTPUT**

**Document Structure**:

- Phase 1: {Phase Name} ✅ IMPLEMENTED
- Phase 2: {Phase Name} ✅ IMPLEMENTED
- Phase N: {Phase Name} ✅ IMPLEMENTED

## **VALIDATION CHECKLIST**

### **PHASE COMPLETENESS**

- [ ] All phases complete and validated
- [ ] Cross-phase relationships identified
- [ ] Knowledge consistency validated
- [ ] Mental models properly integrated

### **APPLICATION READINESS**

- [ ] Can explain target comprehensively
- [ ] Can identify patterns and decisions
- [ ] Can map workflows and interactions
- [ ] Can analyze quality and structure

## **ERROR HANDLING AND RECOVERY**

### **PHASE FAILURE RECOVERY**

- If a phase fails, identify the specific failure point
- Re-execute the failed phase with additional context
- Validate phase output before proceeding
- Document any phase-specific issues encountered

### **KNOWLEDGE GAP HANDLING**

- Identify missing knowledge areas
- Re-execute relevant phases to fill gaps
- Validate knowledge completeness
- Document any persistent knowledge gaps

## **USAGE INSTRUCTIONS**

### **BASIC USAGE**
```

@{command-name}.md

```

### **TARGETED USAGE**

```

@{command-name}.md {target}

```

### **PHASE-SPECIFIC USAGE**

```

@{command-name}.md {target} --phase={phase-number}

```

```

### **STEP 4: PHASE GENERATION**

**AI TASK**: Create individual phase command files

**PHASE STRUCTURE TEMPLATE**:

````markdown
# {Command Name} Phase {N}: {Phase Name}

## **REFERENCE FILES**

### **Input File References**

- **STAGING_FILE**: `{staging-file-path}`

### **Documentation References**

- **ARCHITECTURE_DOCS**: `docs/_Architecture.md`
- **PACKAGE_ARCHETYPES**: `docs/_Package-Archetypes.md`
- **SOP_DOCS**: `docs/_SOP.md`
- **TESTING_STRATEGY**: `docs/testing/_Testing-Strategy.md`
- **ACTIONS_LOG**: `docs/Actions-Log.md`

### **Command References**

- **MAIN_CMD**: `@{command-name}.md`
- **PHASE_1_CMD**: `@{command-name}-phase1-{name}.md`
- **PHASE_2_CMD**: `@{command-name}-phase2-{name}.md`
- **PHASE_N_CMD**: `@{command-name}-phaseN-{name}.md`

---

## **COMMAND PURPOSE**

**Primary Objective**: {Phase-specific objective}
**Scope**: {Phase-specific scope}
**Output**: {Phase-specific output description}

## **EXECUTION PROTOCOL**

### **STEP 1: {Analysis Step 1}**

**AI TASK**: {Task description}

**DATA TO EXTRACT**:

- {Data point 1}
- {Data point 2}
- {Data point 3}

### **STEP 2: {Analysis Step 2}**

**AI TASK**: {Task description}

**DATA TO EXTRACT**:

- {Data point 1}
- {Data point 2}
- {Data point 3}

### **STEP N: OUTPUT GENERATION AND STORAGE**

**AI TASK**: Generate structured output and append to comprehensive analysis document

**OUTPUT PROCESS**:

1. **Generate Phase N Output**: Create structured {phase name} analysis
2. **Append to Staging File**: Add to existing **STAGING_FILE**
3. **Update Phase Status**: Mark Phase N as complete (✅) and next phase as pending (⏳)
4. **Validate Output Completeness**: Ensure all required sections are present
5. **Prepare for Next Phase**: Mark phase as complete and ready for next phase

## **OUTPUT FORMAT**

### **PHASE N APPEND TO STAGING FILE**

**File**: **STAGING_FILE** (append to existing file)

```markdown
## PHASE N: {PHASE NAME} ✅

### {SECTION 1}

- **{Item 1}**: {Description}
- **{Item 2}**: {Description}
- **{Item 3}**: {Description}

### {SECTION 2}

- **{Item 1}**: {Description}
- **{Item 2}**: {Description}
- **{Item 3}**: {Description}

### AI AGENT PATTERNS

- **{Pattern Type}**: {Pattern description for AI recognition}
- **{Pattern Type}**: {Pattern description for AI recognition}
- **{Pattern Type}**: {Pattern description for AI recognition}

### AI ACTIONABLE INSIGHTS

- **{Insight Type}**: {How AI should use this information}
- **{Insight Type}**: {How AI should use this information}
- **{Insight Type}**: {How AI should use this information}

---
```
````

```

### **STEP 5: VALIDATION AND COMPLETION**

**AI TASK**: Validate generated phased command system

**VALIDATION PROCESS**:

1. **Check Structure**: Ensure orchestrator and phases follow proper structure
2. **Validate References**: Verify all reference tags are properly defined
3. **Test Integration**: Ensure phases integrate with orchestrator
4. **Check Completeness**: Verify all required sections are populated
5. **Verify Cleanup**: Ensure staging file cleanup is properly configured

## **OUTPUT FORMAT**

### **PHASED COMMAND STRUCTURE**

**Orchestrator**: `{orchestrator-file-name}.md`

**Phase Directory**: `{phase-directory-name}/`

**Files Created**:

- **ORCHESTRATOR**: Main command orchestrator
- **PHASE_1**: `{command-name}-phase1-{name}.md`
- **PHASE_2**: `{command-name}-phase2-{name}.md`
- **PHASE_N**: `{command-name}-phaseN-{name}.md`

### **REFERENCE SYSTEM INTEGRATION**

**All files include**:

- **REFERENCE FILES** section at top
- **Semantic tags** for all file paths
- **Consistent naming** across all files
- **Cross-references** between orchestrator and phases

## **VALIDATION CHECKLIST**

### **ORCHESTRATOR VALIDATION**

- [ ] Reference files section complete
- [ ] Phase execution order defined
- [ ] Validation checklist included
- [ ] Error handling procedures defined
- [ ] Usage instructions complete

### **PHASE VALIDATION**

- [ ] All phases have reference files section
- [ ] Phase-specific objectives defined
- [ ] Output format matches staging structure
- [ ] AI agent patterns included
- [ ] Cross-references to other phases

### **SYSTEM INTEGRATION**

- [ ] All reference tags properly defined
- [ ] File paths consistent across all files
- [ ] Phase execution sequence logical
- [ ] Staging file structure compatible
- [ ] Final output format defined
- [ ] Staging file cleanup properly configured

## **ERROR HANDLING AND RECOVERY**

### **GENERATION FAILURES**

- If orchestrator generation fails, identify specific failure point
- Re-generate with corrected specifications
- Validate orchestrator structure before proceeding
- Document any generation issues encountered

### **PHASE GENERATION FAILURES**

- If phase generation fails, identify specific failure point
- Re-generate failed phases with additional context
- Validate phase structure before proceeding
- Document any phase-specific issues encountered

### **INTEGRATION FAILURES**

- If integration fails, identify specific failure point
- Re-check reference system consistency
- Validate file path references
- Document any integration issues encountered

### **CLEANUP FAILURES**

- If staging file cleanup fails, identify specific failure point
- Verify all output files are complete before cleanup
- Document any cleanup exceptions or retained files
- Ensure cleanup process is properly configured

## **USAGE INSTRUCTIONS**

### **BASIC USAGE**

```

@Create - Phased Slash Command.md

```

### **WITH SPECIFICATIONS**

```

@Create - Phased Slash Command.md

Using .cursor/commands/Deep Dive - Fluency of a package.md
put the phases in .cursor/commands/deep-dive-phases
Have the final fluency doc go in .cursor/ADHOC

```

### **EXAMPLES**

#### **Example 1: Fluency Command Recreation**

```

@Create - Phased Slash Command.md

Using .cursor/commands/Deep Dive - Fluency of a package.md
put the phases in .cursor/commands/deep-dive-phases
Have the final fluency doc go in .cursor/ADHOC

```

#### **Example 2: CRD Creation Command**

```

@Create - Phased Slash Command.md

Orchestrator: .cursor/commands/Create - Change Request Document.md
Phases: .cursor/commands/crd-phases

Output:

- Staging: .cursor/ADHOC/crd-output-staging.md
- Output: .cursor/ADHOC/crd-output-{target}.md

```

#### **Example 3: Protocol-Based Command**

```

@Create - Phased Slash Command.md

Based on .cursor/rules/Protocol-Retrospective.mdc
Orchestrator: .cursor/commands/Retrospective - Protocol Analysis.md
Phases: .cursor/commands/retrospective-phases

Output:

- Staging: ./docs/retrospective/retrospective-staging.md
- Output: ./docs/retrospective/retrospective-{target}.md

```

#### **Example 4: Custom Output Locations**

```

@Create - Phased Slash Command.md

Using .cursor/commands/Deep Dive - Fluency of a package.md
put the phases in .cursor/commands/deep-dive-phases

Output:

- Staging: ./docs/fluency/fluency-output-staging.md
- Output:
    - ./docs/fluency/fluency-output-{alias}.md
    - ./path/to/another/output-doc.md

```

#### **Example 5: Retain Staging File**

```

@Create - Phased Slash Command.md

Using .cursor/commands/Deep Dive - Fluency of a package.md
put the phases in .cursor/commands/deep-dive-phases
Have the final fluency doc go in .cursor/ADHOC
Retain staging file for debugging

```

## **BENEFITS**

### **FOR COMMAND CREATION**

- **Modular Structure**: Break complex commands into manageable phases
- **Consistent Pattern**: Follow established fluency pattern
- **Reference System**: Leverage centralized file reference system
- **Maintainability**: Easy to update and modify individual phases

### **FOR AI AGENTS**

- **Systematic Execution**: Step-by-step phase execution
- **Pattern Recognition**: Clear patterns for AI to follow
- **Error Recovery**: Built-in error handling and recovery
- **Knowledge Integration**: Staging system for knowledge accumulation

### **FOR TEAMS**

- **Standardized Approach**: Consistent command structure
- **Clear Documentation**: Well-documented execution process
- **Quality Assurance**: Built-in validation and testing
- **Scalability**: Easy to add new phases or modify existing ones

## **FUTURE ENHANCEMENTS**

### **AUTOMATED FEATURES**

- **Template Generation**: Automated template creation
- **Validation Automation**: Automated structure validation
- **Integration Testing**: Automated integration testing
- **Documentation Generation**: Automated documentation updates

### **ADVANCED PATTERNS**

- **Conditional Phases**: Phases that execute based on conditions
- **Parallel Execution**: Phases that can execute in parallel
- **Dynamic Phase Count**: Variable number of phases
- **Custom Output Formats**: Configurable output formats

### **INTEGRATION FEATURES**

- **IDE Integration**: Direct IDE integration
- **CI/CD Integration**: Continuous integration support
- **Version Control**: Git integration for command versions
- **Collaboration**: Multi-user command development
```
