# Create - Change Request Document

## **REFERENCE FILES**

### **Documentation References**

- **ARCHITECTURE_DOCS**: `docs/_Architecture.md`
- **PACKAGE_ARCHETYPES**: `docs/_Package-Archetypes.md`
- **SOP_DOCS**: `docs/_SOP.md`
- **TESTING_STRATEGY**: `docs/testing/_Testing-Strategy.md`
- **ACTIONS_LOG**: `docs/Actions-Log.md`

### **Output File References**

- **CRD_DIRECTORY**: `./docs/crd/{package-alias}-{change-requested}/`
- **HUMAN_CRD**: `./docs/crd/{package-alias}-{change-requested}/human-crd.md`
- **AI_CRD**: `./docs/crd/{package-alias}-{change-requested}/ai-crd.md`
- **TRACKER_ENTRY**: `./docs/crd/{package-alias}-{change-requested}/tracker-entry.md`

---

## **COMMAND PURPOSE**

**Primary Objective**: Create structured change request documents from user input
**Scope**: Generate both human-readable and AI agent execution versions
**Output**: Complete CRD package with both document types

## **USAGE MODES**

### **Mode 1: Full CRD Creation**

```
@Create - Change Request Document.md
```

Creates complete CRD package with all documents.

### **Mode 2: Tracker Entry Only**

```
@Create - Change Request Document.md docs/crd/{existing-crd-path}
```

Creates only the tracker-entry.md for an existing CRD package.

## **EXECUTION PROTOCOL**

### **STEP 1: MODE DETECTION**

**AI TASK**: Determine execution mode based on input

**MODE DETECTION PROCESS**:

1. **Check Input**: Analyze user input for CRD path
2. **Path Validation**: If path provided, validate it exists
3. **Mode Selection**:
    - **Full Mode**: No path provided → Create complete CRD package
    - **Tracker Mode**: Valid CRD path provided → Create tracker entry only

### **STEP 2A: FULL MODE EXECUTION** (if no path provided)

**AI TASK**: Create complete CRD package from user input

#### **STEP 2A.1: INPUT ANALYSIS**

**AI TASK**: Analyze user input to extract change request details

**DATA TO EXTRACT**:

- Package alias and change description
- Non-negotiable requirements
- Terms and definitions
- Detailed change items
- Condensed checklist items
- Pseudo execution examples
- Usage examples and expected behavior

**ANALYSIS PROCESS**:

1. **Extract Package Info**: Identify package alias and change type
2. **Parse Requirements**: Extract non-negotiables and constraints
3. **Identify Terms**: Extract terminology and definitions
4. **Parse Change Items**: Break down into individual change items
5. **Extract Condensed Items**: Capture checklist-style condensed items
6. **Extract Examples**: Capture usage examples and expected outputs

### **STEP 2: DIRECTORY STRUCTURE CREATION**

**AI TASK**: Create CRD directory structure

**DIRECTORY CREATION**:

1. **Create CRD Directory**: `./docs/crd/{package-alias}-{change-requested}/`
2. **Validate Structure**: Ensure directory is properly created
3. **Prepare for Files**: Set up for human and AI CRD creation

### **STEP 3: HUMAN CRD GENERATION**

**AI TASK**: Create human-readable change request document

**HUMAN CRD STRUCTURE**:

```markdown
# Change Request: {Package Alias} - {Change Description}

## **OVERVIEW**

### **Change Summary**

{High-level description of the change}

### **Package Information**

- **Package**: {package-alias}
- **Change Type**: {change-type}
- **Priority**: {priority-level}
- **Estimated Effort**: {effort-estimate}

## **REQUIREMENTS**

### **Non-Negotiables**

{List of non-negotiable requirements}

### **Terms and Definitions**

{Glossary of terms used in the change request}

## **DETAILED CHANGES**

### **Change Item 1: {Item Title}**

{Detailed description of change item 1}

### **Change Item 2: {Item Title}**

{Detailed description of change item 2}

## **CONDENSED**

### **Implementation Checklist**

{Checklist-style condensed items with foldable details}

## **IMPLEMENTATION DETAILS**

### **Pseudo Execution**

{Description of how the change should behave}

### **Examples**

{Usage examples and expected outputs}

## **ACCEPTANCE CRITERIA**

### **Functional Requirements**

- [ ] {Requirement 1}
- [ ] {Requirement 2}
- [ ] {Requirement 3}

### **Testing Requirements**

- [ ] {Test requirement 1}
- [ ] {Test requirement 2}
- [ ] {Test requirement 3}

## **IMPACT ANALYSIS**

### **Affected Components**

- {Component 1}
- {Component 2}
- {Component 3}

### **Dependencies**

- {Dependency 1}
- {Dependency 2}
- {Dependency 3}

## **IMPLEMENTATION PLAN**

### **Phase 1: {Phase Name}**

{Description of phase 1}

### **Phase 2: {Phase Name}**

{Description of phase 2}

## **RISK ASSESSMENT**

### **Technical Risks**

- {Risk 1}: {Mitigation strategy}
- {Risk 2}: {Mitigation strategy}

### **Timeline Risks**

- {Risk 1}: {Mitigation strategy}
- {Risk 2}: {Mitigation strategy}

## **APPROVAL**

### **Stakeholders**

- **Requester**: {Name}
- **Technical Lead**: {Name}
- **Product Owner**: {Name}

### **Approval Status**

- [ ] Technical Review
- [ ] Product Review
- [ ] Implementation Approval
- [ ] Testing Approval
```

### **STEP 2B: TRACKER MODE EXECUTION** (if CRD path provided)

**AI TASK**: Create tracker entry for existing CRD package

#### **STEP 2B.1: CRD PACKAGE ANALYSIS**

**AI TASK**: Read existing CRD documents to extract condensed items

**ANALYSIS PROCESS**:

1. **Read Human CRD**: Extract change items and requirements
2. **Read AI CRD**: Extract implementation steps and patterns
3. **Parse Content**: Identify condensed items for tracker format
4. **Structure Data**: Organize for tracker entry format

#### **STEP 2B.2: TRACKER ENTRY GENERATION**

**AI TASK**: Create tracker-entry.md with condensed checklist format

**TRACKER ENTRY STRUCTURE**:

```markdown
# Tracker Entry: {Package Alias} - {Change Description}

## **IMPLEMENTATION CHECKLIST**

{Checklist-style condensed items with foldable details in exact format provided}

### [ ] {Item Title} <!-- Start Fold -->

{Detailed description of the item}

---

<!-- Close Fold -->

### [ ] {Item Title} <!-- Start Fold -->

{Detailed description of the item}

---

<!-- Close Fold -->
```

#### **STEP 2B.3: VALIDATION**

**AI TASK**: Validate tracker entry creation

**VALIDATION PROCESS**:

1. **Check Structure**: Ensure tracker entry follows proper format
2. **Validate Content**: Verify all condensed items are captured
3. **Test Format**: Ensure foldable format is correct
4. **Check Completeness**: Verify all sections are populated

### **STEP 3: AI CRD GENERATION** (Full Mode Only)

**AI TASK**: Create AI agent execution version

**AI CRD STRUCTURE**:

```markdown
# AI Change Request: {Package Alias} - {Change Description}

## **AI EXECUTION FRAMEWORK**

### **Change Request Identity**

- **Package**: {package-alias}
- **Change Type**: {change-type}
- **AI Task**: {specific AI task description}
- **Execution Priority**: {priority}

### **AI Implementation Strategy**

- **Approach**: {AI implementation approach}
- **Patterns**: {AI patterns to follow}
- **Constraints**: {AI implementation constraints}

## **AI EXECUTION PROTOCOL**

### **STEP 1: REQUIREMENTS ANALYSIS**

**AI TASK**: {Specific AI task for requirements analysis}

**DATA TO EXTRACT**:

- {Data point 1}
- {Data point 2}
- {Data point 3}

**VALIDATION CRITERIA**:

- [ ] {Validation point 1}
- [ ] {Validation point 2}
- [ ] {Validation point 3}

### **STEP 2: IMPLEMENTATION PLANNING**

**AI TASK**: {Specific AI task for implementation planning}

**PLANNING REQUIREMENTS**:

- {Planning requirement 1}
- {Planning requirement 2}
- {Planning requirement 3}

**OUTPUT REQUIREMENTS**:

- {Output requirement 1}
- {Output requirement 2}
- {Output requirement 3}

### **STEP 3: CODE IMPLEMENTATION**

**AI TASK**: {Specific AI task for code implementation}

**IMPLEMENTATION GUIDELINES**:

- {Guideline 1}
- {Guideline 2}
- {Guideline 3}

**CODE PATTERNS**:

- {Pattern 1}
- {Pattern 2}
- {Pattern 3}

### **STEP 4: TESTING AND VALIDATION**

**AI TASK**: {Specific AI task for testing}

**TESTING REQUIREMENTS**:

- {Test requirement 1}
- {Test requirement 2}
- {Test requirement 3}

**VALIDATION CHECKLIST**:

- [ ] {Validation point 1}
- [ ] {Validation point 2}
- [ ] {Validation point 3}

## **AI PATTERN RECOGNITION**

### **Implementation Patterns**

- **Pattern 1**: {Pattern description for AI recognition}
- **Pattern 2**: {Pattern description for AI recognition}
- **Pattern 3**: {Pattern description for AI recognition}

### **Code Patterns**

- **Pattern 1**: {Code pattern for AI to follow}
- **Pattern 2**: {Code pattern for AI to follow}
- **Pattern 3**: {Code pattern for AI to follow}

### **Testing Patterns**

- **Pattern 1**: {Testing pattern for AI to follow}
- **Pattern 2**: {Testing pattern for AI to follow}
- **Pattern 3**: {Testing pattern for AI to follow}

## **AI ACTIONABLE INSIGHTS**

### **Implementation Guidance**

- **How to Approach**: {How AI should approach implementation}
- **Key Decisions**: {Key decisions AI needs to make}
- **Common Pitfalls**: {Common pitfalls AI should avoid}

### **Quality Assurance**

- **Code Quality**: {Code quality standards AI should maintain}
- **Testing Standards**: {Testing standards AI should follow}
- **Documentation Requirements**: {Documentation AI should create}

### **Integration Points**

- **System Integration**: {How AI should integrate with existing system}
- **API Changes**: {API changes AI needs to implement}
- **Configuration Updates**: {Configuration updates AI needs to make}

## **AI EXECUTION CHECKLIST**

### **Pre-Implementation**

- [ ] Requirements fully understood
- [ ] Implementation plan created
- [ ] Dependencies identified
- [ ] Testing strategy defined

### **Implementation**

- [ ] Code changes implemented
- [ ] Configuration updated
- [ ] Tests written and passing
- [ ] Documentation updated

### **Post-Implementation**

- [ ] Integration testing completed
- [ ] Performance validation done
- [ ] User acceptance testing passed
- [ ] Change request marked complete

## **AI TROUBLESHOOTING**

### **Common Issues**

- **Issue 1**: {Problem description and solution}
- **Issue 2**: {Problem description and solution}
- **Issue 3**: {Problem description and solution}

### **Debug Strategies**

- **Strategy 1**: {Debug approach for AI}
- **Strategy 2**: {Debug approach for AI}
- **Strategy 3**: {Debug approach for AI}

### **Recovery Procedures**

- **Procedure 1**: {Recovery steps for AI}
- **Procedure 2**: {Recovery steps for AI}
- **Procedure 3**: {Recovery steps for AI}

## **SUCCESS METRICS**

### **Implementation Success**

- [ ] All requirements implemented
- [ ] All tests passing
- [ ] Performance maintained
- [ ] No regressions introduced

### **Quality Metrics**

- [ ] Code coverage maintained
- [ ] Documentation complete
- [ ] User feedback positive
- [ ] System stability maintained
```

### **STEP 4: VALIDATION AND COMPLETION**

**AI TASK**: Validate generated CRD documents

**VALIDATION PROCESS**:

1. **Check Structure**: Ensure all documents follow proper structure
2. **Validate Content**: Verify all user input is captured
3. **Test Examples**: Ensure examples are accurate and complete
4. **Check Completeness**: Verify all sections are populated

## **OUTPUT FORMAT**

### **CRD PACKAGE STRUCTURE**

**Directory**: **CRD_DIRECTORY**

```
./docs/crd/{package-alias}-{change-requested}/
├── human-crd.md          # Human-readable change request
├── ai-crd.md            # AI agent execution version
├── tracker-entry.md     # Condensed checklist format
└── README.md            # Package overview and usage
```

### **README.md CONTENT**

```markdown
# Change Request Package: {package-alias}-{change-requested}

## **OVERVIEW**

This package contains the change request documentation for {package-alias} - {change-description}.

## **DOCUMENTS**

### **human-crd.md**

Human-readable change request document with:

- Detailed requirements and specifications
- Implementation plan and timeline
- Risk assessment and mitigation strategies
- Approval workflow and stakeholder information

### **ai-crd.md**

AI agent execution version with:

- Step-by-step AI execution protocol
- Pattern recognition frameworks
- Actionable insights and guidance
- Troubleshooting and recovery procedures

### **tracker-entry.md**

Condensed checklist format with:

- Implementation checklist items
- Foldable details for each item
- Progress tracking capabilities
- Quick reference format

## **USAGE**

### **For Humans**

Read `human-crd.md` for complete change request details and approval process.

### **For AI Agents**

Use `ai-crd.md` for systematic implementation of the change request.

### **For Tracking**

Use `tracker-entry.md` for progress monitoring and checklist management.

## **STATUS**

- **Created**: {creation-date}
- **Status**: {current-status}
- **Assignee**: {assignee}
- **Priority**: {priority}

## **LINKS**

- **Related Issues**: {issue-links}
- **Dependencies**: {dependency-links}
- **Implementation**: {implementation-links}
```

## **VALIDATION CHECKLIST**

### **CRD CREATION**

- [ ] Package alias and change description extracted
- [ ] Non-negotiables and requirements captured
- [ ] Terms and definitions documented
- [ ] Change items properly structured
- [ ] Condensed checklist items captured
- [ ] Examples and pseudo execution captured

### **DOCUMENT STRUCTURE**

- [ ] Human CRD follows proper structure
- [ ] AI CRD follows proper structure
- [ ] Both documents are complete and comprehensive
- [ ] README.md provides proper overview

### **CONTENT QUALITY**

- [ ] All user input accurately captured
- [ ] Examples are clear and complete
- [ ] Requirements are specific and actionable
- [ ] AI execution protocol is detailed and clear

### **AI OPTIMIZATION**

- [ ] AI CRD includes pattern recognition frameworks
- [ ] AI CRD includes actionable insights
- [ ] AI CRD includes troubleshooting guidance
- [ ] AI CRD includes success metrics

## **ERROR HANDLING AND RECOVERY**

### **INPUT PARSING FAILURES**

- If input parsing fails, identify specific failure point
- Re-analyze input for missed information
- Validate extracted data for completeness
- Document any parsing issues encountered

### **DOCUMENT GENERATION FAILURES**

- If document generation fails, identify specific failure point
- Re-validate input data and structure
- Check template completeness
- Document any generation issues encountered

### **VALIDATION FAILURES**

- If validation fails, identify specific validation point
- Re-check document structure and content
- Verify all requirements are captured
- Document any validation issues encountered

## **USAGE INSTRUCTIONS**

### **FULL CRD CREATION**

```
@Create - Change Request Document.md
```

Creates complete CRD package with all documents.

### **TRACKER ENTRY ONLY**

```
@Create - Change Request Document.md docs/crd/{existing-crd-path}
```

Creates only the tracker-entry.md for existing CRD package.

### **EXAMPLES**

#### **Full CRD Creation Example**

```
@Create - Change Request Document.md

Package: project-alias-expander
Change: Add echo enhancement with variants
Non-negotiables: Must maintain backward compatibility
Terms: echoX = echo with continue execution
Description: Add --pae-echoX flag with 6 variants
Pseudo execution: Show all variants by default
Examples: --pae-echoX short-in, --pae-echoX global-out
```

#### **Tracker Entry Only Example**

```
@Create - Change Request Document.md docs/crd/pae-echo-enhancement
```

Creates tracker-entry.md from existing CRD documents.

### **Example 1: Basic CRD Creation**

**Input**: User provides change request details
**Output**: Complete CRD package with human and AI versions

### **Example 2: Tracker Entry Only**

**Input**: `@Create - Change Request Document.md docs/crd/pae-echo-enhancement`
**Output**: tracker-entry.md created from existing CRD documents

### **Example 3: Complex Change Request**

**Input**: Multi-item change request with examples and pseudo execution
**Output**: Comprehensive CRD package with detailed implementation guidance

## **BENEFITS**

### **FOR HUMANS**

- **Structured Documentation**: Clear, organized change request format
- **Complete Requirements**: All requirements and constraints captured
- **Implementation Guidance**: Clear implementation plan and timeline
- **Approval Workflow**: Structured approval process and stakeholder management

### **FOR AI AGENTS**

- **Systematic Execution**: Step-by-step implementation protocol
- **Pattern Recognition**: Clear patterns for AI to follow
- **Actionable Insights**: Specific guidance for AI implementation
- **Quality Assurance**: Built-in validation and testing requirements

### **FOR TRACKING**

- **Condensed Format**: Checklist-style items with foldable details
- **Progress Monitoring**: Track implementation progress easily
- **Quick Reference**: Fast access to key implementation items
- **Foldable Details**: Expandable sections for detailed information

### **FOR TEAMS**

- **Consistent Format**: Standardized change request structure
- **Clear Communication**: Both human and AI versions for different audiences
- **Traceability**: Complete documentation of requirements and implementation
- **Quality Control**: Built-in validation and approval processes

## **FUTURE ENHANCEMENTS**

### **AUTOMATED FEATURES**

- Integration with issue tracking systems
- Automated status updates
- Template customization
- Batch CRD creation

### **AI OPTIMIZATION**

- AI-powered requirement analysis
- Automated implementation planning
- Intelligent pattern recognition
- Predictive risk assessment

### **INTEGRATION FEATURES**

- Version control integration
- CI/CD pipeline integration
- Documentation system integration
- Project management tool integration

---

## **CONCLUSION**

This command provides a comprehensive solution for creating structured change request documents that serve both human stakeholders and AI coding agents. The dual-document approach ensures:

- **Human Readability**: Clear, structured documentation for human review and approval
- **AI Usability**: Systematic execution protocol for AI agent implementation
- **Complete Coverage**: All requirements, examples, and implementation details captured
- **Quality Assurance**: Built-in validation and testing requirements
- **Team Collaboration**: Standardized format for consistent communication

The CRD creation process transforms informal change requests into structured, actionable documentation that enables effective implementation by both human developers and AI coding agents.

**Next Steps**:

1. Provide change request input
2. Run CRD creation command
3. Review generated documents
4. Use AI CRD for implementation
5. Track progress using human CRD
