# Deep Dive - Testing Analysis (Universal)

## Output File References

- **STAGING_FILE**: `.cursor/command-phases/dd-ta-universal/testing-analysis-staging.md`
- **FINAL_OUTPUT**: `.cursor/ADHOC/DD Testing Analysis: {package-name}.md`

## Phase Command References

- **PHASE_1_CMD**: `.cursor/command-phases/dd-ta-universal/ta-phase1-BaselineAssessment.md`
- **PHASE_2_CMD**: `.cursor/command-phases/dd-ta-universal/ta-phase2-GapIdentification.md`
- **PHASE_3_CMD**: `.cursor/command-phases/dd-ta-universal/ta-phase3-AntiPatternDetection.md`
- **PHASE_4_CMD**: `.cursor/command-phases/dd-ta-universal/ta-phase4-ImplementationStrategy.md`
- **PHASE_5_CMD**: `.cursor/command-phases/dd-ta-universal/ta-phase5-FinalSynthesis.md`

---

## Command Purpose

**Primary Objective**: Comprehensive testing analysis using modular phase approach
**Scope**: Execute phases sequentially, combine outputs, validate completeness
**Output**: Comprehensive testing analysis with structured recommendations

**Bug Discovery Mode**: When a known symptom is provided, performs targeted analysis to locate untested code paths that likely contain the bug

## Core Analysis Principles

### 1. :: Code-Path Coverage Analysis

**CRITICAL**: This framework performs **code-path level analysis**, not just function-level analysis:

- **Branch Coverage**: Enumerate all `if`/`elif`/`else` branches within functions and verify tests exist for each branch
- **Parameter Variation**: Identify functions with enum/string parameters and verify tests for each valid value
- **Mode Coverage**: For functions with multiple operational modes, verify tests cover all modes
- **Within-Mode Behavior**: For modes that perform multiple operations (e.g., `both` does from + to sync), verify all sub-behaviors are tested
- **Mock-Hiding Detection**: Identify where mocks hide behavior that should be tested (e.g., mock returns success but doesn't test actual behavior)
- **Conditional Path Enumeration**: Map execution paths and verify test coverage for each path

### 2. :: Bug Discovery Analysis

When tests pass but code is broken, the bug lives in **untested territory**:

- **Assertion Quality**: Detect weak assertions that don't verify actual behavior (e.g., `assert result is not None` vs `assert result == expected`)
- **Return Value Coverage**: For functions returning multiple values/complex objects, verify ALL parts are asserted
- **Boundary Conditions**: Systematically check edge cases (empty, null, 0, -1, MAX, unicode, permissions)
- **Negative Path Testing**: Verify failure cases are tested (what happens when dependencies fail?)
- **Data Flow Gaps**: Track data transformations and identify untested mutations
- **State Mutation Verification**: For stateful operations, verify post-state is asserted

### 3. :: Bug Localization Mode (Symptom-Driven)

When a **known symptom** is provided, perform targeted analysis:

1. **Symptom Parsing**: Extract keywords and identify relevant code areas
2. **Code Path Tracing**: Map all paths that could produce the symptom
3. **Gap Correlation**: Match symptom-relevant paths against coverage gaps
4. **Likelihood Ranking**: Rank untested paths by probability of containing the bug
5. **Targeted Recommendations**: Prioritize tests that would expose the specific bug

**Example Flow**:

```
Symptom: "shared→project sync not happening in 'both' mode"
    ↓
Keywords: "sync", "shared", "project", "both", "mode"
    ↓
Relevant Functions: sync_package_mapping(), sync_directory_bidirectional()
    ↓
Relevant Paths: direction='both' branch, from_synced handling, to_synced handling
    ↓
Untested Paths: from_synced return value not asserted
    ↓
Likely Bug Location: sync_directory_bidirectional() from-direction logic
    ↓
Recommended Test: Assert from_synced contains expected files when direction='both'
```

### 4. :: Placeholder Test Treatment

Placeholder tests are treated as correctly implemented and passing:

- **Automatic Green Status**: Placeholder tests are treated as passing by default
- **Structural Analysis Focus**: Focus on structural completeness rather than implementation details
- **Best Practice Validation**: Validate test structure and organization against best practices

### 5. :: Universal Discovery Protocol

Analysis is performed through direct code inspection:

- **Source Discovery**: Use `grep` and `glob` to find all source files
- **Test Discovery**: Use `grep` and `glob` to find all test files
- **Function Enumeration**: Use `grep` patterns to find all public functions
- **Branch Detection**: Use `grep` to find conditional statements within functions
- **Coverage Mapping**: Match test files to source files and verify coverage

---

## Execution Protocol

### Step 1: Pre-Execution Setup

**AI TASK**: Prepare for comprehensive testing analysis

**CLEANUP PROCESS**:

- [ ] Delete existing staging file if present
- [ ] Delete existing output file if present
- [ ] Ensure clean workspace for new analysis

**REQUIREMENTS**:

- [ ] Target package path identified and accessible
- [ ] Test directory structure accessible
- [ ] Source code access verified

**VALIDATION**:

- [ ] Package exists and is accessible
- [ ] Source files can be discovered and read
- [ ] Test files can be discovered and read
- [ ] Analysis scope confirmed

### Step 2: Package Structure Discovery

**AI TASK**: Discover package structure through direct inspection

**DISCOVERY PROTOCOL**:

1. **Detect Package Type**:
   - If path contains `lib/` → Use `{path}/lib/` as source
   - If path contains `src/` → Use `{path}/src/` as source
   - Otherwise → Scan `{path}/` for source files

2. **Find Test Directory**:
   - Look for `tests/`, `test/`, `__tests__/` directories
   - Look for `*_test.py`, `*.test.ts`, `*.spec.ts` patterns

3. **Enumerate Source Files**:
   - Use `glob` to find all source files (`.py`, `.ts`, `.js`, etc.)
   - Exclude `__init__.py`, `__pycache__`, test files

4. **Enumerate Test Files**:
   - Use `glob` to find all test files
   - Map test files to source modules

### Step 3: Sequential Phase Execution

**CRITICAL**: Each phase MUST be fully completed before proceeding to the next phase. Phases execute automatically in sequence - no user confirmation required between phases.

**EXECUTION RULE**: Read phase → Execute completely → Verify output → Proceed to next phase

---

#### Phase 1 → Phase 2 → Phase 3 → Phase 4 → Phase 5

---

#### 🚧 GATE 1: Phase 1 Execution

**Execute**: Read and complete **PHASE_1_CMD** (`ta-phase1-BaselineAssessment.md`)

**COMPLETION REQUIREMENTS** (all must be true before proceeding):
- [ ] Staging file created with Phase 1 header
- [ ] Package structure documented
- [ ] All public functions enumerated
- [ ] Code paths identified
- [ ] Boundary-sensitive parameters identified
- [ ] Data flow chains mapped
- [ ] Symptom analysis complete (if symptom provided)
- [ ] Phase 1 marked as ✅ in staging file

**→ Once complete, automatically proceed to Phase 2**

---

#### 🚧 GATE 2: Phase 2 Execution

**Execute**: Read and complete **PHASE_2_CMD** (`ta-phase2-GapIdentification.md`)

**COMPLETION REQUIREMENTS** (all must be true before proceeding):
- [ ] Function-level gaps identified
- [ ] Code-path coverage gaps identified
- [ ] Bug discovery gaps analyzed (assertions, boundaries, negative paths)
- [ ] Terminal output gaps assessed
- [ ] Symptom correlation complete (if symptom provided)
- [ ] Gap summary with metrics generated
- [ ] Phase 2 marked as ✅ in staging file

**→ Once complete, automatically proceed to Phase 3**

---

#### 🚧 GATE 3: Phase 3 Execution

**Execute**: Read and complete **PHASE_3_CMD** (`ta-phase3-AntiPatternDetection.md`)

**COMPLETION REQUIREMENTS** (all must be true before proceeding):
- [ ] Anti-pattern tests identified
- [ ] Files to remove listed
- [ ] Files to refactor listed
- [ ] Phase 3 marked as ✅ in staging file

**→ Once complete, automatically proceed to Phase 4**

---

#### 🚧 GATE 4: Phase 4 Execution

**Execute**: Read and complete **PHASE_4_CMD** (`ta-phase4-ImplementationStrategy.md`)

**COMPLETION REQUIREMENTS** (all must be true before proceeding):
- [ ] Priority matrix generated
- [ ] Immediate action items listed
- [ ] Implementation guidance provided
- [ ] Bug localization strategy complete (if symptom provided)
- [ ] Success criteria defined
- [ ] Phase 4 marked as ✅ in staging file

**→ Once complete, automatically proceed to Phase 5**

---

#### 🚧 GATE 5: Phase 5 Execution (Final)

**Execute**: Read and complete **PHASE_5_CMD** (`ta-phase5-FinalSynthesis.md`)

**COMPLETION REQUIREMENTS**:
- [ ] All phase outputs validated
- [ ] Comprehensive document generated
- [ ] Final output saved to **FINAL_OUTPUT**
- [ ] Executive summary displayed
- [ ] Phase 5 marked as ✅ in staging file

**→ Analysis complete - display executive summary**

---

### Step 4: Execution Validation

**AI TASK**: Validate that all phases executed successfully

**PHASE COMPLETION TRACKING**:

| Phase | Gate | Status |
|-------|------|--------|
| Phase 1: Baseline | 🚧 GATE 1 | ⬜ → ✅ |
| Phase 2: Gaps | 🚧 GATE 2 | ⬜ → ✅ |
| Phase 3: Anti-Patterns | 🚧 GATE 3 | ⬜ → ✅ |
| Phase 4: Strategy | 🚧 GATE 4 | ⬜ → ✅ |
| Phase 5: Synthesis | 🚧 GATE 5 | ⬜ → ✅ |

**VALIDATION PROCESS**:

- [ ] Staging file exists with all phase outputs
- [ ] Phase 1 marked as ✅
- [ ] Phase 2 marked as ✅
- [ ] Phase 3 marked as ✅
- [ ] Phase 4 marked as ✅
- [ ] Phase 5 marked as ✅
- [ ] Final output document created

**GATE ENFORCEMENT**:

Each phase must be fully complete before proceeding to the next. If a phase fails:
1. Complete the failed phase
2. Verify all requirements met
3. Then proceed to next phase

---

## Output Format

### Analysis Document Location

**File**: **STAGING_FILE**

**Document Structure**:

- Phase 1: Baseline Testing Assessment
- Phase 2: Gap Identification (including Code-Path Coverage)
- Phase 3: Anti-Pattern Detection
- Phase 4: Implementation Strategy
- Phase 5: Final Synthesis

### Executive Summary Format

The final executive summary follows this structure:

```markdown
# Test Coverage Analysis: {Package Name}

## 1. :: Package Structure

1. Source Location
   - Path: {detected source path}
   - Modules: {count} files analyzed

2. Test Location
   - Path: {detected test path}
   - Test Files: {count} files found

---

## 2. :: Function-Level Coverage

{For each source module:}
1. {module_name}
   - Total public functions: {count}
   - Direct coverage: {count} ({percentage}%)
   - No coverage: {count}

---

## 3. :: Code-Path Coverage Gaps

### 3.1. :: Branch Coverage Gaps
{Functions with untested branches}

### 3.2. :: Parameter Variation Gaps
{Functions with untested parameter values}

### 3.3. :: Mode Coverage Gaps
{Functions with untested operational modes}

---

## 4. :: Anti-Pattern Files

### 4.1. :: Files to Remove
{List of anti-pattern files}

---

## 5. :: Priority Matrix

### 5.1. :: Immediate Action Required
{Critical gaps}

### 5.2. :: Future Improvements
{Non-critical improvements}
```

---

## Validation Checklist

### Phase Completeness

- [ ] Phase 1: Baseline assessment complete
- [ ] Phase 2: Gap identification complete (including code-path analysis)
- [ ] Phase 3: Anti-pattern detection complete
- [ ] Phase 4: Implementation strategy complete
- [ ] Phase 5: Final synthesis complete

### Analysis Completeness

- [ ] All source files enumerated
- [ ] All test files enumerated
- [ ] All public functions identified
- [ ] Code-path coverage analyzed (branches, parameters, modes)
- [ ] Anti-patterns identified
- [ ] Priority matrix generated

---

## Error Handling and Recovery

### Phase Failure Recovery

- If a phase fails, identify the specific failure point
- Re-execute the failed phase with additional context
- Validate phase output before proceeding
- Document any phase-specific issues encountered

### Discovery Failure Recovery

- If source discovery fails, prompt for package structure clarification
- If test discovery fails, report missing test directory
- If file reading fails, report access issues

---

## Usage Instructions

### Basic Usage

```
@Deep Dive- Testing Analysis- Universal (dd-ta-universal).md {path/to/package}
```

### Bug Discovery Mode (Known Symptom)

When you know something is broken but tests pass, provide the symptom:

```
@dd-ta-universal.md {path/to/package} --symptom "{description of what's not working}"
```

**Symptom Format Examples**:

```
--symptom "shared→project sync not happening in 'both' mode"
--symptom "files are copied but not committed"
--symptom "error messages not displayed when connection fails"
--symptom "menu selection returns wrong option"
```

**What Bug Discovery Mode Does**:

1. **Symptom Analysis**: Parses the symptom to identify relevant code areas
2. **Path Tracing**: Traces code paths that could produce the symptom
3. **Gap Correlation**: Correlates symptom with untested code paths
4. **Bug Localization**: Ranks untested paths by likelihood of containing the bug
5. **Targeted Recommendations**: Prioritizes tests that would expose the bug

### Examples

```
# Standard analysis
@dd-ta-universal.md ___shared/.sync-manager
@dd-ta-universal.md src/components
@dd-ta-universal.md lib/services

# Bug discovery mode
@dd-ta-universal.md ___shared/.sync-manager --symptom "both direction not syncing from shared"
@dd-ta-universal.md lib/auth --symptom "login fails silently on network error"
```

---

## Key Differentiators from Standard Analysis

| Standard Analysis | Universal Analysis |
|-------------------|-------------------|
| Function-level coverage only | Code-path level coverage |
| Requires workspace docs | Self-standing, no docs required |
| Static test mapping | Dynamic discovery via grep/glob |
| Misses parameter variations | Explicit parameter coverage |
| Misses branch coverage | Explicit branch coverage |
| Misses mode coverage | Explicit mode coverage |
| Ignores within-mode behavior | Explicit sub-behavior coverage |
| Doesn't detect mock-hiding | Identifies mock-hidden gaps |
| No assertion quality check | Weak assertion detection |
| No boundary analysis | Systematic boundary condition check |
| No negative path analysis | Failure path coverage gaps |
| No return value analysis | Multi-value return coverage |
| No data flow tracking | Data transformation gap detection |
| No bug localization | **Symptom-driven bug discovery** |
