# Fluency Phase 2: Architecture Pattern Analysis

## **REFERENCE FILES**

### **Output File References**

- **STAGING_FILE**: `.cursor/ADHOC/fluency-output-staging.md`

### **Documentation References**

- **ARCHITECTURE_DOCS**: `docs/_Architecture.md`
- **PACKAGE_ARCHETYPES**: `docs/_Package-Archetypes.md`
- **SOP_DOCS**: `docs/_SOP.md`
- **TESTING_STRATEGY**: `docs/testing/_Testing-Strategy.md`

### **Command References**

- **FLUENCY_CMD**: `@Deep Dive - Fluency of a package.md`
- **FLUENCY_PHASE_1**: `@fluency-phase1-Identity.md`
- **FLUENCY_PHASE_2**: `@fluency-phase2-Architecture.md`
- **FLUENCY_PHASE_3**: `@fluency-phase3-Functionality.md`
- **FLUENCY_PHASE_4**: `@fluency-phase4-Implementation.md`
- **FLUENCY_PHASE_5**: `@fluency-phase5-Integration.md`
- **FLUENCY_PHASE_6**: `@fluency-phase6-Synthesis.md`

---

## **COMMAND PURPOSE**

**Primary Objective**: Catalog architectural patterns, design decisions, and implementation strategies
**Scope**: Design patterns, service architecture, interface organization, dependency management
**Output**: Pattern catalog with implementation examples and relationships

## **EXECUTION PROTOCOL**

### **STEP 1: PACKAGE TYPE PATTERN RECOGNITION**

**AI TASK**: Identify and analyze package type patterns

**DATA TO EXTRACT**:

- Package type classification (core/ext/shared/tool)
- Type-specific architectural patterns
- Type-specific design constraints
- Type-specific integration patterns
- Type-specific build and deployment patterns

**PATTERN CATALOGING**:

- Core package: Business logic isolation, self-contained architecture
- Extension package: VSCode integration, adapter patterns
- Shared package: Utility functions, cross-package consumption
- Tool package: Standalone execution, minimal dependencies

### **STEP 2: DESIGN PATTERN RECOGNITION**

**AI TASK**: Identify and catalog design patterns used

**DATA TO EXTRACT**:

- Service architecture patterns (singleton, factory, etc.)
- Data flow patterns (pipeline, event-driven, etc.)
- Error handling patterns (try-catch, result types, etc.)
- Configuration management patterns
- State management patterns
- Interface design patterns

### **STEP 3: SERVICE ARCHITECTURE ANALYSIS**

**AI TASK**: Understand service organization and boundaries

**DATA TO EXTRACT**:

- Service responsibilities and boundaries
- Service interaction patterns
- Service lifecycle management
- Service dependency relationships
- Service communication patterns
- Service testing strategies

### **STEP 4: INTERFACE ORGANIZATION STRATEGY**

**AI TASK**: Analyze interface design and organization

**DATA TO EXTRACT**:

- Public API design patterns
- Interface abstraction levels
- Interface versioning strategies
- Interface documentation patterns
- Interface testing approaches
- Interface evolution patterns

### **STEP 5: DEPENDENCY MANAGEMENT PATTERNS**

**AI TASK**: Understand dependency architecture and management

**DATA TO EXTRACT**:

- Dependency injection patterns
- Dependency resolution strategies
- Dependency lifecycle management
- Dependency testing approaches
- Dependency optimization patterns
- Dependency security considerations

### **STEP 6: CODE PATTERN EXAMPLES ANALYSIS**

**AI TASK**: Extract and analyze actual code examples showing architectural patterns

**DATA TO EXTRACT**:

- Facade pattern implementations
- Dependency injection setup examples
- Service boundary implementations
- Interface design examples
- Configuration pattern examples
- Error handling implementations

### **STEP 7: ANTI-PATTERNS AND COMMON MISTAKES ANALYSIS**

**AI TASK**: Identify architectural anti-patterns and common mistakes to avoid

**DATA TO EXTRACT**:

- Architectural mistakes to avoid
- Design pattern misapplications
- Service boundary violations
- Dependency management mistakes
- Interface design anti-patterns
- Configuration mistakes

### **STEP 8: OUTPUT GENERATION AND STORAGE**

**AI TASK**: Generate structured output and append to comprehensive analysis document

**OUTPUT PROCESS**:

1. **Generate Phase 2 Output**: Create structured architecture pattern catalog with examples
2. **Append to Staging File**: Add to existing **STAGING_FILE**
3. **Update Phase Status**: Mark Phase 2 as complete (✅) and Phase 3 as pending (⏳)
4. **Validate Output Completeness**: Ensure all required sections are present
5. **Prepare for Next Phase**: Mark phase as complete and ready for Phase 3

## **OUTPUT FORMAT**

### **PHASE 2 APPEND TO STAGING FILE**

**File**: **STAGING_FILE** (append to existing file)

```markdown
## PHASE 2: ARCHITECTURE PATTERN ANALYSIS ✅

### PACKAGE TYPE ANALYSIS

- **Type**: {core/ext/shared/tool}
- **Type Patterns**: {Type-specific architectural patterns}
- **Type Constraints**: {Design constraints for this type}
- **Type Integration**: {Integration patterns for this type}

### DESIGN PATTERNS

- **Service Patterns**: {Service architecture patterns used}
- **Data Flow Patterns**: {Data processing and flow patterns}
- **Error Handling**: {Error handling and recovery patterns}
- **Configuration**: {Configuration management patterns}
- **State Management**: {State handling and persistence patterns}

### SERVICE ARCHITECTURE

- **Service Boundaries**: {Service responsibilities and boundaries}
- **Service Interactions**: {How services communicate}
- **Service Lifecycle**: {Service initialization and cleanup}
- **Service Dependencies**: {Service dependency relationships}
- **Service Testing**: {Service testing strategies}

### INTERFACE DESIGN

- **API Patterns**: {Public API design patterns}
- **Abstraction Levels**: {Interface abstraction strategies}
- **Versioning**: {Interface versioning approaches}
- **Documentation**: {Interface documentation patterns}
- **Evolution**: {Interface evolution strategies}

### DEPENDENCY ARCHITECTURE

- **Injection Patterns**: {Dependency injection strategies}
- **Resolution**: {Dependency resolution approaches}
- **Lifecycle**: {Dependency lifecycle management}
- **Testing**: {Dependency testing strategies}
- **Optimization**: {Dependency optimization patterns}

### CODE PATTERN EXAMPLES

- **Facade Implementation**: {Actual facade pattern code examples}
- **Dependency Injection**: {DI container setup and usage examples}
- **Service Boundaries**: {Service interface and implementation examples}
- **Configuration Patterns**: {Config.json examples with explanations}
- **Error Handling**: {Error handling implementation examples}
- **Interface Design**: {API design and implementation examples}

### ANTI-PATTERNS AND COMMON MISTAKES

- **Architectural Mistakes**: {Common architectural mistakes to avoid}
- **Design Pattern Misuse**: {Design pattern misapplications}
- **Service Boundary Violations**: {Service boundary mistakes}
- **Dependency Mistakes**: {Dependency management errors}
- **Interface Anti-Patterns**: {Interface design mistakes}
- **Configuration Mistakes**: {Configuration errors and consequences}

### AI AGENT PATTERNS

- **Architecture Pattern Recognition**: {Patterns for AI to identify architectural decisions}
- **Service Pattern Mapping**: {Patterns for AI to understand service organization}
- **Interface Pattern Analysis**: {Patterns for AI to recognize interface design}
- **Dependency Pattern Recognition**: {Patterns for AI to understand dependency management}
- **Code Example Recognition**: {Patterns for AI to identify code patterns}
- **Anti-Pattern Detection**: {Patterns for AI to avoid common mistakes}

### AI ACTIONABLE INSIGHTS

- **Architecture Implementation**: {How AI should implement similar architectures}
- **Service Design Patterns**: {Service patterns AI should follow}
- **Interface Design Strategies**: {Interface design approaches AI should use}
- **Dependency Management**: {Dependency strategies AI should apply}
- **Code Pattern Application**: {How AI should apply code patterns}
- **Mistake Avoidance**: {How AI should avoid common mistakes}

---

## **VALIDATION CHECKLIST**

- [ ] Package type correctly identified and analyzed
- [ ] Design patterns cataloged with examples
- [ ] Service architecture patterns identified
- [ ] Interface organization strategy understood
- [ ] Dependency management patterns analyzed
- [ ] Pattern relationships mapped
- [ ] Code pattern examples extracted and analyzed
- [ ] Anti-patterns identified and documented
- [ ] Implementation examples identified
- [ ] Common mistakes cataloged

## **KNOWLEDGE RETENTION STRATEGY**

**Pattern Catalog Structure**:

- Store patterns with implementation examples
- Link patterns to architectural principles
- Map pattern relationships and dependencies
- Associate patterns with quality outcomes

**Cross-Reference Points**:

- Link architectural patterns to implementation choices
- Connect service patterns to functionality design
- Map interface patterns to user experience
- Associate dependency patterns to maintainability

## **NEXT PHASE REQUIREMENTS**

**Output for Phase 3**:

- Complete architecture pattern catalog
- Service architecture analysis
- Interface design patterns
- Dependency management patterns
- Pattern relationship mapping

**Phase 3 Input Requirements**:

- Package identity model (Phase 1 output)
- Architecture pattern catalog (this output)
- Package source code access
- Service implementation details
- Interface usage patterns
```

