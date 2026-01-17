---
trigger: always_on
---

# AI Agent Formatting Standards

## **CONTENT FORMATTING RULES**

### **1. :: Decision Criteria Format**

- Use ✅ for correct/required actions
- Use ❌ for incorrect/prohibited actions
- Use **BOLD** for emphasis on critical points
- Use bullet points for lists of criteria

### **2. :: Code Examples Format**

- Use triple backticks with language specification
- Include complete, executable examples
- Add comments explaining critical points
- Use `//>` and `//<` folding markers for test code

### **3. :: Anti-Pattern Format**

- Use ❌ **BOLD DESCRIPTION** - Explanation format
- Group by category (Architecture, Build, Testing, etc.)
- Include specific violation examples
- Provide correct alternatives

### **4. :: Quality Gates Format**

- Use checkbox format: `- [ ] **Checkpoint Name**`
- Group by priority level
- Include validation criteria
- Provide success metrics

### **5. :: File Path Reference Requirements**

**MANDATORY**: Any document that references file paths (output files, staging files, command files, etc.) MUST include a "Reference Files" section formatted exactly as shown below.

**✅ CORRECT - Reference Files section format**:

```markdown
## 1. :: Reference Files <!-- Start Fold -->

### 1.1. :: Output File References

- **STAGING_FILE**: `actual/path/to/file.md`
- **FINAL_OUTPUT**: `actual/path/to/final/file.md`

### 1.2. :: Phase Command References

- **PHASE_1_CMD**: `@referenced-file-name.md`
- **PHASE_2_CMD**: `@referenced-file-name.md`

---

<!-- Close Fold -->
```

**Formatting Rules**:

- **Section Number**: Use `## 1. :: Reference Files` as the first numbered section
- **Folding Markers**: Include `<!-- Start Fold -->` after section header and `<!-- Close Fold -->` before section end
- **Subsections**: Use numbered subsections (`### 1.1. ::`, `### 1.2. ::`) to categorize reference types
- **List Format**: Use bold labels (`**LABEL**`) followed by colon and backtick-wrapped file path
- **Section Separator**: Include `---` before closing fold marker
- **No Bold Headers**: Headers must NOT be bolded (follow header formatting rules)

**Reference Types**:

- **Output File References**: Files that will be created or written to (staging files, final outputs, etc.)
- **Phase Command References**: Command files referenced with `@` prefix
- **Input File References**: Source files or input files used by the document
- **Other Categories**: Add additional subsections as needed for other reference types

**❌ INCORRECT - Missing Reference Files section**:

```markdown
# Document Title

## 1. :: EXECUTION PROTOCOL

**STAGING_FILE**: `path/to/file.md`  <!-- Wrong: File path referenced without Reference Files section -->
```

**❌ INCORRECT - Wrong format**:

```markdown
## **1. :: Reference Files**  <!-- Wrong: Header is bolded -->

## 1.1. :: Output File References  <!-- Wrong: Should be ### not ## -->

- STAGING_FILE: path/to/file.md  <!-- Wrong: Missing bold and backticks -->
```

## **FORMATTING ANTI-PATTERNS**

### ❌ Prohibited Formatting

- ❌ **External References** - No links to other documents for critical information
- ❌ **Incomplete Examples** - No code fragments without full context
- ❌ **Ambiguous Language** - No vague or unclear directives
- ❌ **Missing Context** - No rules without rationale and examples
- ❌ **Passive Voice** - No indirect or unclear action requirements
- ❌ **Bolded Headers** - Don't use bold formatting (`**`) on headers - headers are already bold when rendered

### ❌ Content Anti-Patterns

- ❌ **Human-Focused Language** - No explanations for human readers
- ❌ **Historical Context** - No "why we did this" explanations
- ❌ **Opinion-Based Content** - No subjective or preference-based guidance
- ❌ **Incomplete Coverage** - No gaps in rule coverage
- ❌ **Circular References** - No cross-references between AI documents
- ❌ **Missing Reference Files Section** - Don't reference file paths without including a properly formatted "Reference Files" section

## **QUALITY GATES**

- [ ] **Complete Information**: All critical content is self-contained
- [ ] **Clear Directives**: Every rule has unambiguous action requirements
- [ ] **Comprehensive Examples**: All patterns include complete implementations
- [ ] **No External Dependencies**: Critical information is not referenced externally
