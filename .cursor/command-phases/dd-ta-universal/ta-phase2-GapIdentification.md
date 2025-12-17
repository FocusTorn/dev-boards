# Testing Analysis Phase 2: Gap Identification

## 🚧 PHASE GATE ENFORCEMENT

**CRITICAL**: This is Phase 2 of 5. 
- **PREREQUISITE**: Phase 1 must be complete before executing this phase
- Complete ALL steps in this phase before proceeding
- Verify all completion requirements are met
- Then automatically proceed to Phase 3

**DO NOT** skip to Phase 3 until Phase 2 is fully complete.

---

## Output File References

- **STAGING_FILE**: `.cursor/command-phases/dd-ta-universal/testing-analysis-staging.md`

## Command References

- **PHASE_1_CMD**: `ta-phase1-BaselineAssessment.md`
- **PHASE_2_CMD**: `ta-phase2-GapIdentification.md` (this file)
- **PHASE_3_CMD**: `ta-phase3-AntiPatternDetection.md`
- **PHASE_4_CMD**: `ta-phase4-ImplementationStrategy.md`
- **PHASE_5_CMD**: `ta-phase5-FinalSynthesis.md`

---

## Command Purpose

**Primary Objective**: Identify testing gaps including function-level, code-path-level, and structural gaps
**Scope**: Function coverage gaps, branch coverage gaps, parameter variation gaps, mode coverage gaps
**Output**: Comprehensive gap analysis with specific missing test types at all coverage levels

---

## Execution Protocol

### Step 1: Function-Level Gap Analysis

**AI TASK**: Identify functions that lack test coverage

**GAP IDENTIFICATION**:

Using Phase 1 coverage mapping, identify:

1. **No Coverage Functions**: Functions with zero test references
2. **Indirect-Only Coverage**: Functions only tested through other functions
3. **Low Coverage Functions**: Functions with <50% of code paths tested

**DATA TO EXTRACT**:

- Function name
- Source module
- Coverage status (none/indirect/partial)
- Criticality assessment (public API vs internal)
- Lines of code (complexity indicator)

### Step 2: Code-Path Coverage Gap Analysis

**AI TASK**: Identify untested code paths within functions

**CRITICAL ANALYSIS**: This is the key differentiator for universal analysis

#### 2.1 Branch Coverage Gaps

**DETECTION METHOD**:

For each function with branches:
1. Count total branches (if/elif/else)
2. Search test files for assertions covering each branch
3. Identify branches without test coverage

**GREP PATTERNS**:

```
# Find branches in function
if .+:
elif .+:
else:

# Search tests for branch-specific assertions
assert.*{branch_condition}
expect.*{branch_condition}
```

**GAP CLASSIFICATION**:

| Function | Total Branches | Tested Branches | Gap |
|----------|---------------|-----------------|-----|
| {name} | {count} | {count} | {count} |

#### 2.2 Parameter Variation Gaps

**DETECTION METHOD**:

For each function with enum/string parameters:
1. Identify all valid parameter values from source code
2. Search test files for each parameter value usage
3. Identify parameter values without test coverage

**GREP PATTERNS**:

```
# Find parameter variations in function signature/body
direction='from'|'to'|'both'
mode=['read', 'write', 'append']
if param == 'value1':
elif param == 'value2':
```

**GAP CLASSIFICATION**:

| Function | Parameter | Valid Values | Tested Values | Missing |
|----------|-----------|--------------|---------------|---------|
| {name} | {param} | {values} | {tested} | {missing} |

#### 2.3 Mode Coverage Gaps

**DETECTION METHOD**:

For each function with operational modes:
1. Identify all modes from conditional logic
2. Search test files for mode-specific test cases
3. Identify modes without dedicated tests

**EXAMPLE**:

```python
# Source: sync_package_mapping()
if direction == 'both':    # Mode 1
    ...
elif direction == 'from':  # Mode 2
    ...
elif direction == 'to':    # Mode 3
    ...
```

**GAP CLASSIFICATION**:

| Function | Total Modes | Tested Modes | Untested Modes |
|----------|-------------|--------------|----------------|
| {name} | {count} | {list} | {list} |

#### 2.4 Within-Mode Behavior Coverage Gaps

**CRITICAL ANALYSIS**: Modes that perform multiple operations may have sub-behaviors that aren't tested even when the mode itself is "covered".

**DETECTION METHOD**:

For each mode that performs multiple operations:
1. Identify all sub-behaviors within the mode
2. Check if tests assert on each sub-behavior
3. Check if mocks are hiding testable behavior
4. Identify sub-behaviors without test assertions

**EXAMPLE**:

```python
# Source: sync_package_mapping() with direction='both'
if direction == 'both':
    # Sub-behavior 1: Git pull from remote
    if is_repo and remote_exists:
        pull_shared_repo(...)  # Is this tested?
    
    # Sub-behavior 2: Bidirectional sync
    from_synced, to_synced = sync_directory_bidirectional(...)
    # Are BOTH from_synced AND to_synced behaviors tested?
    
    # Sub-behavior 3: Git commit
    if has_changes:
        commit_to_shared_repo(...)  # Is this tested?
    
    # Sub-behavior 4: Git push
    if commit_success:
        push_to_remote(...)  # Is this tested?
```

**ANALYSIS CRITERIA**:

| Criterion | Question |
|-----------|----------|
| **Sub-Behavior Identification** | What distinct operations does this mode perform? |
| **Assertion Coverage** | Do tests assert on the outcome of each sub-behavior? |
| **Mock Transparency** | Are mocks hiding behavior that should be tested? |
| **Return Value Testing** | If function returns multiple values, are all tested? |

**GAP CLASSIFICATION**:

| Function | Mode | Sub-Behaviors | Tested | Untested | Mock-Hidden |
|----------|------|---------------|--------|----------|-------------|
| {name} | both | pull, from-sync, to-sync, commit, push | to-sync | from-sync, commit, push | pull (mocked) |

**MOCK-HIDING DETECTION**:

```
# Signs of mock-hiding behavior:
- Mock returns static values without testing variations
- Mock called but return values not asserted
- Mock hides complex logic that has its own branches
- Integration behavior replaced entirely with mock

# Example of problematic mock:
mock_sync.return_value = (['file1'], ['file2'])  # Both directions "work"
# But no test verifies from-direction actually syncs correctly
```

### Step 3: Terminal Output Coverage Gap Analysis

**AI TASK**: Identify gaps in terminal/console output testing

**REFERENCE**: See `.cursor/rules/formatting/terminal-output.mdc` for complete formatting specifications

**DETECTION METHOD**:

First, determine the language/platform of the code under test:

#### 3.1 Python Terminal Output (outerm Package)

**DETECTION**: Check for `outerm` imports or terminal output patterns

```
# Find outerm usage
from outerm import
import outerm

# Find direct print statements that should use outerm
print(.*\\x1B\[  # Raw ANSI codes (should use outerm)
print(.*COLOR_  # Legacy constants (should use outerm)
```

**GAP CLASSIFICATION**:

| Function | Output Type | Uses outerm | Gap |
|----------|-------------|-------------|-----|
| {name} | status message | no - uses raw print | Should use outerm.success/error/warning/info |
| {name} | header | no - uses manual formatting | Should use outerm.write_header |
| {name} | indented region | no - uses manual indent | Should use outerm.start_region |

**outerm API COVERAGE CHECK**:

| outerm Function | Expected Usage | Test Exists | Gap |
|-----------------|----------------|-------------|-----|
| error() | Error messages | {yes/no} | {description} |
| warning() | Warning messages | {yes/no} | {description} |
| info() | Info messages | {yes/no} | {description} |
| success() | Success messages | {yes/no} | {description} |
| action() | Action messages | {yes/no} | {description} |
| write_header() | Section headers | {yes/no} | {description} |
| write_header_fat() | Main section headers | {yes/no} | {description} |
| start_region() | Indented regions | {yes/no} | {description} |
| Palette.* | Color access | {yes/no} | {description} |

#### 3.2 Non-Python Terminal Output (ANSI Codes)

**DETECTION**: For non-Python CLI tools, verify output matches patterns in `.cursor/rules/formatting/terminal-output.mdc`

**KEY PATTERNS TO VERIFY** (see rules file for complete specs):

- Error: Red (code 196), ✗ icon, space after
- Warning: Yellow (code 220), ⚠ icon, space after, blank line above, full-line coloring
- Info: Blue (code 39), ｉ icon (bold), **NO space after**
- Success: Green (code 46), ✔ icon, space after
- All output must reset color with `\x1B[0m`

**GAP CLASSIFICATION**:

| Output Type | Matches Spec | Gap |
|-------------|--------------|-----|
| Error msg | {yes/no} | {mismatch description} |
| Warning msg | {yes/no} | {mismatch description} |
| Info msg | {yes/no} | {mismatch description} |
| Success msg | {yes/no} | {mismatch description} |

#### 3.3 Output Capture Testing

**DETECTION METHOD**:

```
# Check for stdout/stderr capture in tests
capsys
capfd
StringIO
redirect_stdout
redirect_stderr
mock.patch('sys.stdout')
```

**GAP CLASSIFICATION**:

| Function | Has Output | Capture Test | Gap |
|----------|------------|--------------|-----|
| {name} | stdout | no capture test | Missing output verification |
| {name} | stderr | no capture test | Missing error output test |
| {name} | both | partial | Only stdout tested |

#### 3.4 Interactive Output Testing

**DETECTION METHOD**:

```
# Find interactive prompts
input(
prompt_toolkit
questionary
click.confirm
click.prompt
```

**GAP CLASSIFICATION**:

| Function | Interactive Type | Test Exists | Gap |
|----------|-----------------|-------------|-----|
| {name} | confirmation prompt | no | Missing prompt test |
| {name} | menu selection | no | Missing menu test |
| {name} | text input | no | Missing input test |

### Step 4: Structural Completeness Gap Analysis

**AI TASK**: Verify no missing test blocks within skeleton structures

**ANALYSIS FOCUS**:

1. **Missing Test Blocks**: Test files exist but missing test cases for functions
2. **Missing Test Files**: Source modules without corresponding test files
3. **Incomplete Test Suites**: Test classes missing expected test methods

**DETECTION METHOD**:

```
# Check if test file exists for module
test_{module}.py exists?
{module}.test.ts exists?

# Check if test class exists for each class
class Test{ClassName} exists?

# Check if test method exists for each public method
def test_{method_name} exists?
```

### Step 5: Integration Gap Analysis

**AI TASK**: Identify gaps in integration testing

**GAP IDENTIFICATION**:

1. **Cross-Module Integration**: Functions that call other modules
2. **External Dependencies**: Functions that use external libraries/APIs
3. **Configuration Integration**: Functions that read/write configuration
4. **I/O Operations**: Functions that perform file/network operations

**DETECTION METHOD**:

```
# Find integration points
from {other_module} import
import subprocess
import requests
open(
Path(
```

### Step 6: Bug Discovery Gap Analysis

**AI TASK**: Identify gaps that hide bugs (tests pass but code is broken)

#### 6.1 Assertion Quality Gaps

**DETECTION METHOD**:

Tests might "cover" code but with weak assertions that don't catch bugs:

```
# Weak assertions (GAP indicators)
assert result is not None      # Doesn't verify WHAT result is
assert result                  # Truthy check only
assert len(result) > 0         # Doesn't verify content
assert isinstance(result, X)   # Doesn't verify values
expect(result).toBeDefined()   # JS: existence only

# Strong assertions (expected)
assert result == expected_value
assert result.field == 'expected'
expect(result).toEqual(expected)
```

**GAP CLASSIFICATION**:

| Function | Test | Assertion Type | Gap |
|----------|------|----------------|-----|
| {name} | {test} | weak (not None) | Should assert specific value |
| {name} | {test} | weak (truthy) | Should assert expected result |

#### 6.2 Return Value Coverage Gaps

**DETECTION METHOD**:

For functions returning multiple values or complex objects:

```python
# Function returns multiple values
def sync_directory_bidirectional(...) -> Tuple[List[str], List[str]]:
    return from_synced, to_synced

# Test should assert BOTH values
assert from_synced == [...]  # ✓
assert to_synced == [...]    # Often missing!
```

**GAP CLASSIFICATION**:

| Function | Returns | Values Asserted | Missing |
|----------|---------|-----------------|---------|
| {name} | Tuple[X, Y] | X only | Y not asserted |
| {name} | Dict | keys a, b | key c not checked |

#### 6.3 Boundary Condition Gaps

**DETECTION METHOD**:

Using Phase 1 boundary-sensitive parameters, check test coverage:

| Parameter Type | Expected Boundary Tests | Found in Tests |
|----------------|------------------------|----------------|
| numeric | 0, -1, MAX | {found values} |
| string | empty, None, unicode | {found values} |
| collection | empty, single, many | {found values} |
| path | not exists, no perms | {found values} |
| boolean | True AND False | {found values} |

**GAP CLASSIFICATION**:

| Function | Parameter | Boundary | Tested | Gap |
|----------|-----------|----------|--------|-----|
| {name} | {param} | empty string | no | Missing empty string test |
| {name} | {param} | 0 value | no | Missing zero test |

#### 6.4 Negative Path Gaps

**DETECTION METHOD**:

For each function with error handling, check if failure paths are tested:

```
# Find error paths in source
try:
    ...
except SomeError:        # Is this tested?
    return error_result

if condition_fails:      # Is this tested?
    raise ValueError(...)

if dependency_call():    # What if this fails?
    ...
```

**GAP CLASSIFICATION**:

| Function | Error Path | Test Exists | Gap |
|----------|------------|-------------|-----|
| {name} | except NetworkError | no | Network failure not tested |
| {name} | validation fails | no | Invalid input not tested |
| {name} | dependency returns None | no | Dependency failure not tested |

#### 6.5 Data Flow Gaps

**DETECTION METHOD**:

Using Phase 1 data flow chains, check if transformations are tested:

```
Input → [Transform A] → [Transform B] → [Transform C] → Output
        ↑ tested?       ↑ tested?       ↑ tested?
```

**GAP CLASSIFICATION**:

| Function | Transformation | Tested | Gap |
|----------|---------------|--------|-----|
| {name} | input validation | no | Input transformation not tested |
| {name} | data conversion | no | Conversion logic not tested |
| {name} | output formatting | no | Output formatting not tested |

### Step 7: Symptom Correlation (Bug Discovery Mode)

**AI TASK**: If symptom provided, correlate gaps with likely bug locations

**CORRELATION METHOD**:

1. **Match Symptom to Gaps**: Cross-reference Phase 1 symptom analysis with identified gaps
2. **Rank by Likelihood**: Score each gap by probability of containing the bug
3. **Generate Hypotheses**: Produce ranked list of likely bug locations

**LIKELIHOOD SCORING**:

| Factor | Score |
|--------|-------|
| Gap in symptom-relevant function | +5 |
| Untested branch matches symptom condition | +4 |
| Weak assertion on symptom-related value | +3 |
| Missing boundary test for symptom context | +2 |
| Untested error path for symptom failure | +2 |
| Gap in function called by relevant function | +1 |

**GAP CLASSIFICATION**:

| Gap | Symptom Relevance | Likelihood Score | Hypothesis |
|-----|-------------------|------------------|------------|
| {gap description} | {how relates to symptom} | {score} | {bug hypothesis} |

### Step 8: Error Handling Gap Analysis

**AI TASK**: Identify untested error conditions

**GAP IDENTIFICATION**:

1. **Exception Handling**: try/except blocks without error tests
2. **Error Returns**: Functions that return error states
3. **Validation Errors**: Input validation that can fail
4. **Edge Cases**: Boundary conditions not tested

**DETECTION METHOD**:

```
# Find error handling in source
try:
except
raise
return None
return False
if not .+:
    raise
```

### Step 9: Output Generation

**AI TASK**: Generate structured gap analysis and append to staging document

**OUTPUT PROCESS**:

1. Generate Phase 2 output following template below
2. Append to **STAGING_FILE**
3. Validate output completeness
4. Mark phase as complete

---

## Output Format

### Staging File Output

**File**: **STAGING_FILE** (append)

```markdown
## PHASE 2: GAP IDENTIFICATION ✅

### FUNCTION-LEVEL COVERAGE GAPS

#### No Coverage (Critical)

| Function | Module | Lines | Criticality |
|----------|--------|-------|-------------|
| {name} | {module} | {loc} | Public API / Internal |

**Total Functions with No Coverage**: {count} / {total} ({percentage}%)

#### Indirect-Only Coverage

| Function | Module | Tested Via |
|----------|--------|------------|
| {name} | {module} | {other_function} |

**Total Functions with Indirect-Only Coverage**: {count}

---

### CODE-PATH COVERAGE GAPS

#### Branch Coverage Gaps

| Function | Module | Branches | Tested | Untested Branches |
|----------|--------|----------|--------|-------------------|
| {name} | {module} | {total} | {tested} | {list of untested} |

**Functions with Untested Branches**: {count}
**Total Untested Branches**: {count}

#### Parameter Variation Gaps

| Function | Parameter | All Values | Tested | Missing |
|----------|-----------|------------|--------|---------|
| {name} | direction | from, to, both | both | from, to |

**Functions with Untested Parameter Values**: {count}
**Total Missing Parameter Tests**: {count}

#### Mode Coverage Gaps

| Function | All Modes | Tested Modes | Untested Modes |
|----------|-----------|--------------|----------------|
| {name} | {list} | {list} | {list} |

**Functions with Untested Modes**: {count}
**Total Untested Modes**: {count}

#### Within-Mode Behavior Coverage Gaps

| Function | Mode | Sub-Behaviors | Tested | Untested | Mock-Hidden |
|----------|------|---------------|--------|----------|-------------|
| {name} | {mode} | {list} | {list} | {list} | {list} |

**Modes with Untested Sub-Behaviors**: {count}
**Total Untested Sub-Behaviors**: {count}
**Mock-Hidden Behaviors**: {count}

##### Mock-Hiding Issues

| Function | Mode | Mocked Function | Hidden Behavior | Risk |
|----------|------|-----------------|-----------------|------|
| {name} | {mode} | {mock} | {what's hidden} | {high/medium/low} |

---

### TERMINAL OUTPUT COVERAGE GAPS

**Reference**: `.cursor/rules/formatting/terminal-output.mdc`

#### Python (outerm Package Usage)

| Function | Output Type | Uses outerm | Gap |
|----------|-------------|-------------|-----|
| {name} | {status/header/region} | {yes/no} | {description} |

**outerm Function Coverage**:

| Function | In Use | Tested | Gap |
|----------|--------|--------|-----|
| error() | {yes/no} | {yes/no} | {description} |
| warning() | {yes/no} | {yes/no} | {description} |
| info() | {yes/no} | {yes/no} | {description} |
| success() | {yes/no} | {yes/no} | {description} |
| write_header() | {yes/no} | {yes/no} | {description} |
| start_region() | {yes/no} | {yes/no} | {description} |

#### Non-Python (ANSI Code Compliance)

| Output Type | Matches Spec | Gap |
|-------------|--------------|-----|
| {type} | {yes/no} | {mismatch description} |

#### Output Capture Testing

| Function | Has Output | Capture Test | Gap |
|----------|------------|--------------|-----|
| {name} | {stdout/stderr/both} | {yes/no/partial} | {description} |

#### Interactive Output Testing

| Function | Interactive Type | Test Exists | Gap |
|----------|-----------------|-------------|-----|
| {name} | {prompt/menu/input} | {yes/no} | {description} |

**Functions with Terminal Output Gaps**: {count}
**Missing Output Capture Tests**: {count}
**Interactive Output Untested**: {count}

---

### STRUCTURAL COMPLETENESS GAPS

#### Missing Test Files

| Source Module | Expected Test File | Status |
|---------------|-------------------|--------|
| {module} | test_{module}.py | Missing |

**Modules Without Test Files**: {count}

#### Missing Test Classes/Methods

| Test File | Missing For |
|-----------|-------------|
| {test_file} | {class or method} |

---

### INTEGRATION GAPS

#### Cross-Module Integration

| Function | Calls To | Integration Tests |
|----------|----------|-------------------|
| {name} | {other_modules} | {none/partial/full} |

#### External Dependencies

| Function | Dependency | Mocked | Integration Test |
|----------|------------|--------|------------------|
| {name} | {dep} | {yes/no} | {yes/no} |

---

### BUG DISCOVERY GAPS

#### Assertion Quality Gaps (Weak Assertions)

| Function | Test | Assertion | Issue |
|----------|------|-----------|-------|
| {name} | {test} | `assert result is not None` | Doesn't verify value |
| {name} | {test} | `assert result` | Truthy only |

**Tests with Weak Assertions**: {count}

#### Return Value Coverage Gaps

| Function | Returns | Asserted | Not Asserted |
|----------|---------|----------|--------------|
| {name} | Tuple[X, Y] | X | Y |
| {name} | Dict{a, b, c} | a, b | c |

**Functions with Incomplete Return Assertions**: {count}

#### Boundary Condition Gaps

| Function | Parameter | Type | Missing Boundaries |
|----------|-----------|------|-------------------|
| {name} | {param} | numeric | 0, -1 |
| {name} | {param} | string | empty, None |
| {name} | {param} | collection | empty |

**Parameters Missing Boundary Tests**: {count}

#### Negative Path Gaps

| Function | Failure Scenario | Tested |
|----------|-----------------|--------|
| {name} | Network error | No |
| {name} | Invalid input | No |
| {name} | Dependency failure | No |

**Untested Failure Paths**: {count}

#### Data Flow Gaps

| Function | Transformation | Tested |
|----------|---------------|--------|
| {name} | input validation | No |
| {name} | data conversion | No |

**Untested Data Transformations**: {count}

---

### SYMPTOM CORRELATION (if symptom provided)

**Symptom**: "{symptom text}"

#### Bug Location Hypotheses (Ranked by Likelihood)

| Rank | Gap | Likelihood | Hypothesis |
|------|-----|------------|------------|
| 1 | {gap in relevant function} | HIGH | {description of likely bug} |
| 2 | {untested branch} | MEDIUM | {description} |
| 3 | {weak assertion} | MEDIUM | {description} |

#### Recommended Tests to Expose Bug

1. **{Test Name}**: {What to test and why it would expose the bug}
2. **{Test Name}**: {What to test and why}
3. **{Test Name}**: {What to test and why}

---

### ERROR HANDLING GAPS

#### Untested Exception Paths

| Function | Exception Type | Test Exists |
|----------|---------------|-------------|
| {name} | {exception} | No |

#### Untested Error Returns

| Function | Error Condition | Test Exists |
|----------|-----------------|-------------|
| {name} | {condition} | No |

---

### GAP SUMMARY

| Gap Category | Count | Percentage |
|--------------|-------|------------|
| Functions with No Coverage | {count} | {%} |
| Functions with Untested Branches | {count} | {%} |
| Functions with Untested Parameters | {count} | {%} |
| Functions with Untested Modes | {count} | {%} |
| Modes with Untested Sub-Behaviors | {count} | {%} |
| Mock-Hidden Behaviors | {count} | {%} |
| **Weak Assertions (Bug Hiding)** | {count} | {%} |
| **Missing Return Value Assertions** | {count} | {%} |
| **Missing Boundary Tests** | {count} | {%} |
| **Untested Failure Paths** | {count} | {%} |
| **Untested Data Transformations** | {count} | {%} |
| Terminal Output Gaps | {count} | {%} |
| Missing Output Capture Tests | {count} | {%} |
| Missing Test Files | {count} | {%} |
| Untested Error Paths | {count} | {%} |

**Overall Code-Path Coverage Estimate**: {percentage}%
**Adjusted for Mock-Hidden Gaps**: {adjusted_percentage}%
**Bug Discovery Score**: {score} (lower = more likely to hide bugs)

---
```

---

## Validation Checklist

- [ ] Function-level gaps identified and categorized
- [ ] Branch coverage gaps identified with specific branches
- [ ] Parameter variation gaps identified with specific values
- [ ] Mode coverage gaps identified with specific modes
- [ ] Within-mode sub-behavior gaps identified
- [ ] Mock-hiding issues identified and flagged
- [ ] Terminal output coverage assessed (outerm for Python, ANSI compliance for non-Python)
- [ ] Output capture testing gaps identified
- [ ] Interactive output testing gaps identified
- [ ] **Assertion quality analyzed (weak vs strong)**
- [ ] **Return value coverage checked (all parts asserted)**
- [ ] **Boundary conditions identified and gap checked**
- [ ] **Negative/failure paths identified and gap checked**
- [ ] **Data flow transformations tracked and gap checked**
- [ ] **Symptom correlation performed (if symptom provided)**
- [ ] Structural completeness gaps documented
- [ ] Integration gaps assessed
- [ ] Error handling gaps identified
- [ ] Gap summary with metrics generated
- [ ] Output appended to staging file

---

## Knowledge Retention Strategy

**Mental Model Structure**:

- Store as structured gap inventory with priorities
- Link gaps to specific functions and code paths
- Cross-reference with baseline for impact assessment
- Map to implementation priorities for resolution

**Cross-Reference Points**:

- Link function gaps to Phase 1 inventory
- Connect code-path gaps to specific source lines
- Map integration gaps to dependency analysis
- Associate error gaps to exception handling patterns

---

## Next Phase Requirements

**Output for Phase 3**:

- Complete gap inventory at all levels
- Code-path coverage gaps with specific details
- Priority assessment for each gap category
- Structural completeness evaluation

**Phase 3 Will Analyze**:

- Anti-pattern test files to remove
- Redundant tests that inflate coverage
- Non-best-practice tests
- Tests that should be refactored

---

## ✅ PHASE 2 COMPLETION - THEN PROCEED

**After completing all steps above:**

1. **VERIFY** all completion requirements are met
2. **CONFIRM** staging file contains Phase 2 output marked as ✅
3. **PROCEED** automatically to Phase 3

**Completion Verification**:
- [ ] Function-level gaps identified
- [ ] Code-path coverage gaps identified
- [ ] Bug discovery gaps analyzed
- [ ] Terminal output gaps assessed
- [ ] Symptom correlation complete (if applicable)
- [ ] Gap summary generated
- [ ] Output appended to staging file

**→ All requirements met? Proceed to Phase 3**
