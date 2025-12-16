# Testing Coverage - Exposed Functionality Analysis

## Command Purpose

Analyze all exposed (public) functionality in the sync-manager library modules and compare with test coverage to identify gaps. Output results in summary format per `.cursor/rules/formatting/summary.mdc`.

## Analysis Protocol

### Step 1: Identify All Public Functions

1. Scan all modules in `___shared/.sync-manager/lib/`
   - Use `grep` to find all function definitions: `^def [^_]`
   - Use `grep` to find all class definitions: `^class [^_]`
   - List all public (non-underscore prefix) functions and classes

2. Categorize by module:
   - `lib/config.py`
   - `lib/conflict_resolver.py`
   - `lib/file_sync.py`
   - `lib/git_ops.py`
   - `lib/output.py`
   - `lib/sync_ops.py`

### Step 2: Identify Test Coverage

1. Scan all test files in `___shared/.sync-manager/tests/`
   - Use `grep` to find imports: `^from lib\.|^import lib\.`
   - Map which functions are imported in each test file
   - Identify which test classes/methods cover which functions

2. Determine coverage type:
   - **Direct coverage**: Function is imported and has dedicated tests
   - **Indirect coverage**: Function is tested through other functions
   - **No coverage**: Function is not tested at all

### Step 3: Compare and Identify Gaps

1. Create coverage matrix:
   - Public function → Test coverage status
   - Note if coverage is direct or indirect
   - Identify functions with no coverage

2. Categorize gaps:
   - **Critical**: Public API functions with no coverage
   - **Medium**: Functions with only indirect coverage
   - **Low**: Private helpers (underscore prefix) with no direct tests

### Step 4: Output Summary

**MANDATORY**: Output MUST follow summary formatting rules from `.cursor/rules/formatting/summary.mdc`:

- Wrap entire summary in markdown code block with triple backticks
- Use numbered lists (1., 2.) for top-level items
- Use dash bullets (-) with 3-space indentation for details
- Separate major sections with `---`
- Structure: Coverage Status → Gaps → Recommendations

## Expected Output Structure

```markdown
# Test Coverage Analysis Summary

## Exposed Functionality Coverage

1. [Module Name]
    - [Total public functions]
    - [Functions with direct tests]
    - [Functions with indirect tests]
    - [Functions with no coverage]

2. [Another Module]
    - [Total public functions]
    - [Functions with direct tests]
    - [Functions with indirect tests]
    - [Functions with no coverage]

---

## Coverage Gaps

1. Critical Gaps
    - [Public API function with no coverage]
    - [Impact and priority]

2. Medium Priority Gaps
    - [Function with only indirect coverage]
    - [Recommendation for direct tests]

---

## Recommendations

1. Immediate Actions
    - [Specific function to test]
    - [Rationale and priority]

2. Future Improvements
    - [Testing strategy recommendation]
    - [Coverage goal]
```

## Implementation Requirements

- **Use grep systematically**: Find all `^def [^_]` and `^class [^_]` patterns
- **Map imports**: Find all `^from lib\.` imports in test files
- **Compare methodically**: Module by module comparison
- **Output format**: MUST use summary format (wrapped in markdown code block)
- **Include context**: Note which functions are tested indirectly through other functions
- **Prioritize gaps**: Focus on public API functions first

## Notes

- Private functions (underscore prefix) are lower priority but should be noted
- Functions tested indirectly are still "covered" but may benefit from direct tests
- Focus analysis on `___shared/.sync-manager/` directory structure

