# Testing Coverage - Exposed Functionality Analysis

## Command Purpose

Analyze all exposed (public) functionality in a package/module and its dependencies, comparing with test coverage to identify gaps. Output results in summary format per `.cursor/rules/formatting/summary.mdc`.

## Usage

```
@Testing- Coverage- Exposed.md {path/to/package-or-file}
```

**Examples**:
```
@Testing- Coverage- Exposed.md ___shared/.sync-manager
@Testing- Coverage- Exposed.md ___shared/shared-python/outerm
@Testing- Coverage- Exposed.md scripts/my-script.py
```

## Input Parameter

- **`{path}`**: Path to a Python package directory or file (relative to workspace root)
  - If directory: Analyze all `.py` files in `lib/`, `src/`, or root
  - If file: Analyze the file and its local imports
  - Automatically detects `tests/` directory for coverage analysis

---

## Analysis Protocol

### Step 1: Discover Package Structure

1. **Determine package type from provided path**:
   - If path contains `lib/` → Use `{path}/lib/` as source, `{path}/tests/` for tests
   - If path contains `src/` → Use `{path}/src/` as source, `{path}/tests/` for tests
   - If path is a file → Analyze file and its local imports, find adjacent `tests/`
   - Otherwise → Scan `{path}/` for `.py` files, `{path}/tests/` for tests

2. **List all source files**:
   - Use `glob` to find all `*.py` files in source directory
   - Exclude `__init__.py`, `__pycache__`, test files

3. **List all test files**:
   - Use `glob` to find all `test_*.py` files in tests directory
   - Map test files to source modules

### Step 2: Identify All Public Functions

1. **For each source file**, use `grep` to find:
   - Public function definitions: `^def [^_]`
   - Public class definitions: `^class [^_]`
   - Exported names in `__all__` if present

2. **Categorize by module**:
   - Group functions/classes by their source file
   - Note which are in `__all__` (explicitly exported)

3. **Identify dependencies**:
   - Find local imports: `from \. import`, `from \.\.`, `from {package_name}`
   - Include imported module's public functions in analysis

### Step 3: Identify Test Coverage

1. **For each test file**, use `grep` to find:
   - Imports from source: `^from {module}|^import {module}`
   - Test class names: `^class Test`
   - Test function names: `def test_`

2. **Map coverage**:
   - **Direct coverage**: Function is imported AND has dedicated test(s)
   - **Indirect coverage**: Function is called by tested code but no dedicated test
   - **No coverage**: Function not tested at all

3. **Detect coverage type**:
   - Search test files for function name usage
   - Check if function appears in test assertions or calls

### Step 4: Compare and Identify Gaps

1. **Create coverage matrix**:
   - Source file → Functions → Coverage status (direct/indirect/none)
   - Include line count and complexity indicators

2. **Categorize gaps by priority**:
   - **Critical**: Public API functions (in `__all__` or main exports) with no coverage
   - **High**: Public functions with no coverage
   - **Medium**: Functions with only indirect coverage
   - **Low**: Private helpers with no direct tests

3. **Identify dependency gaps**:
   - Functions from imported local modules that lack tests
   - Shared utilities used but not tested

### Step 5: Output Summary

**MANDATORY**: Output MUST follow summary formatting rules from `.cursor/rules/formatting/summary.mdc`:

- Wrap entire summary in markdown code block with triple backticks
- Use numbered lists (1., 2.) for top-level items  
- Use dash bullets (-) with 3-space indentation for details
- Separate major sections with `---`
- Structure: Package Info → Coverage Status → Gaps → Recommendations

---

## Expected Output Structure

```markdown
# Test Coverage Analysis: {Package Name}

## Package Structure

1. Source Location
   - Path: {detected source path}
   - Modules: {count} files analyzed
   - Dependencies: {list of local imports}

2. Test Location
   - Path: {detected test path}
   - Test Files: {count} files found

---

## Exposed Functionality Coverage

1. {module_name.py}
   - Total public functions: {count}
   - Direct coverage: {count} ({percentage}%)
   - Indirect coverage: {count}
   - No coverage: {count}
   - Functions: {list of function names with status}

2. {another_module.py}
   - Total public functions: {count}
   - Direct coverage: {count} ({percentage}%)
   - Indirect coverage: {count}
   - No coverage: {count}
   - Functions: {list of function names with status}

---

## Coverage Gaps

1. Critical Gaps (Public API, No Coverage)
   - {function_name} in {module} - {reason/impact}
   - {function_name} in {module} - {reason/impact}

2. High Priority (Public Functions, No Coverage)
   - {function_name} in {module}
   - {function_name} in {module}

3. Medium Priority (Indirect Coverage Only)
   - {function_name} in {module} - tested via {other_function}
   - {function_name} in {module} - tested via {other_function}

---

## Dependency Coverage

1. Local Dependencies Analyzed
   - {dependency_module}: {coverage_status}
   - {dependency_module}: {coverage_status}

2. Dependency Gaps
   - {dependency_function} used but not tested
   - {dependency_function} used but not tested

---

## Recommendations

1. Immediate Actions
   - Add tests for {critical_function} - {rationale}
   - Add tests for {critical_function} - {rationale}

2. Test Structure Improvements
   - {recommendation}
   - {recommendation}

3. Coverage Goals
   - Target: {percentage}% direct coverage for public API
   - Current: {percentage}%
```

---

## Implementation Requirements

### Discovery Phase
- **Detect package structure**: Auto-detect `lib/`, `src/`, or flat structure
- **Find test directory**: Look for `tests/`, `test/`, or `*_test.py` patterns
- **Identify dependencies**: Parse imports to find local module dependencies

### Analysis Phase
- **Use grep systematically**: Find all `^def [^_]` and `^class [^_]` patterns
- **Map imports in tests**: Find all imports from source modules
- **Search for function usage**: Grep test files for each public function name
- **Compare methodically**: Module by module comparison

### Output Phase
- **Output format**: MUST use summary format (wrapped in markdown code block)
- **Include context**: Note which functions are tested indirectly
- **Prioritize gaps**: Focus on public API functions first
- **Include dependencies**: Show coverage status of imported local modules

---

## Notes

- Private functions (underscore prefix) are lower priority but should be noted
- Functions tested indirectly are still "covered" but may benefit from direct tests
- Include dependency analysis for comprehensive coverage view
- Adapt to different package structures (lib/, src/, flat)
- If no tests directory found, report that explicitly

