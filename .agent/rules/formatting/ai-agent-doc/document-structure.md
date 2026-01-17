---
trigger: always_on
---

# AI Agent Document Structure Rules

## **CRITICAL EXECUTION DIRECTIVE**

**AI Agent Directive**: Follow this structure exactly for all AI agent documentation.

## **AI AGENT DOCUMENT STRUCTURE**

### **1. :: Required Document Header**

```markdown
# [Document Title]

## REFERENCE FILES

### Documentation References

- **SOP_DOCS**: `docs/_SOP.md`
- **ARCHITECTURE_DOCS**: `docs/_Architecture.md`
- **PACKAGE_ARCHETYPES**: `docs/_Package-Archetypes.md`

### AI Testing Documentation References

- **AI_TESTING_BASE**: `docs/testing/(AI) _Strategy- Base- Testing.md`
- **AI_MOCKING_BASE**: `docs/testing/(AI) _Strategy- Base- Mocking.md`
- **AI_TROUBLESHOOTING**: `docs/testing/(AI) _Troubleshooting- Base.md`

---

## CRITICAL EXECUTION DIRECTIVE

**AI Agent Directive**: [Specific directive for document purpose]

**MANDATORY EXECUTION PROTOCOL**:

1. **NO DEVIATION**: All rules must be followed exactly as written
2. **NO SKIPPING**: No steps may be skipped, abbreviated, or modified
3. **NO SELECTIVE COMPLIANCE**: All rules apply to all actions
4. **FAILURE TO COMPLY**: Violating these rules constitutes a critical protocol violation
```

### **2. :: Section Structure Requirements**

#### **2.1. :: Header Formatting Rule**

**CRITICAL**: Headers (`#`, `##`, `###`, `####`, etc.) must NEVER be bolded in the source markdown.

**✅ CORRECT - Plain text headers**:

```markdown
## Section Name

### 1. :: Subsection Name

#### 1.1. :: Sub-subsection Name
```

**❌ INCORRECT - Bolded headers**:

```markdown
## **Section Name**

### **1. :: Subsection Name**

#### **1.1. :: Sub-subsection Name**
```

**Rationale**: 
- Markdown headers are already bold when rendered
- Adding bold formatting (`**`) creates visual redundancy and clutters the source
- Plain text headers are cleaner, more readable, and follow standard markdown conventions

#### **2.2. :: Primary Sections (##)**

- **DO NOT BOLD HEADERS** - Headers are already bold when rendered
- Format: `## SECTION NAME` (no bold formatting)
- No numbering in primary sections
- Each section must contain complete, self-contained information

#### **2.3. :: Subsections (###)**

- **DO NOT BOLD HEADERS** - Headers are already bold when rendered
- Format: `### SUBSECTION NAME` (no bold formatting)
- Can use numbering for logical grouping: `### 1. :: Subsection Name`

#### **2.4. :: Sub-subsections (####)**

- **DO NOT BOLD HEADERS** - Headers are already bold when rendered
- Format: `#### SUB-SUBSECTION NAME` (no bold formatting)
- Can use numbering for logical grouping: `#### 1.1. :: Sub-subsection Name`

### **3. :: Mandatory Sections for AI Agent Documents**

#### **3.1. :: Required Sections**

1. **REFERENCE FILES** - Minimal context only
2. **CRITICAL EXECUTION DIRECTIVE** - Mandatory protocol
3. **[TOPIC] SYSTEM** - Complete system definition
4. **[TOPIC] RULES** - All applicable rules with full context
5. **[TOPIC] PATTERNS** - Implementation patterns with examples
6. **[TOPIC] ANTI-PATTERNS** - Violation prevention
7. **[TOPIC] QUALITY GATES** - Compliance validation
8. **[TOPIC] SUCCESS METRICS** - Performance indicators
9. **[TOPIC] VIOLATION PREVENTION** - Natural stops and pattern recognition
10. **EXECUTION PRIORITY MATRIX** - Decision-making hierarchy
11. **DYNAMIC MANAGEMENT NOTE** - Document evolution statement

#### **3.2. :: Section Content Requirements**

- **Complete Information**: No external references for critical content
- **Actionable Directives**: Every section must drive specific AI actions
- **Compliance Enforcement**: Built-in violation detection and prevention
- **Measurable Outcomes**: Clear success criteria for AI agent performance

## **ANTI-PATTERNS**

### ❌ Structure Violations

- ❌ **Inconsistent Structure** - No deviation from required section order
- ❌ **Bolded Headers** - Don't use bold formatting (`**`) on headers - headers are already bold when rendered
- ❌ **Missing Sections** - Don't omit mandatory sections

## **QUALITY GATES**

- [ ] **Document Header**: Follows exact header template
- [ ] **Plain Text Headers**: No headers use bold formatting
- [ ] **Required Sections**: All 11 mandatory sections are present
- [ ] **Structure Enforcement**: Sections follow specified hierarchy and formatting
