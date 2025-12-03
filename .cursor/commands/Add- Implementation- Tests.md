# Add Implementation Tests

## **COMMAND PURPOSE**

This command implements full test implementations for a **single skeleton testing file**, maintaining complete adherence to the FocusedUX testing guidelines and architecture patterns.

**CRITICAL**: This command works on **ONE FILE AT A TIME** - specify the exact test file to implement.

## **REFERENCE FILES**

### **Documentation References**

- **ARCHITECTURE_DOCS**: `docs/_Architecture.md`
- **TESTING_STRATEGY**: `docs/testing/(AI) _Strategy- Base- Testing.md`
- **MOCKING_STRATEGY**: `docs/testing/(AI) _Strategy- Base- Mocking.md`
- **TEST_TROUBLESHOOTING**: `docs/testing/(AI) _Troubleshooting- Tests.md`

### **Targeted Testing Strategies**

- **EXT_STRATEGY**: `docs/testing/(AI) _Strategy- Specific- Ext.md`
- **EXT_CONSUMED_STRATEGY**: `docs/testing/(AI) _Strategy- Specific- ExtConsumed.md`
- **LIBS_STRATEGY**: `docs/testing/(AI) _Strategy- Specific- Libs.md`
- **PLUGINS_STRATEGY**: `docs/testing/(AI) _Strategy- Specific- Plugins.md`
- **UTIL_STRATEGY**: `docs/testing/(AI) _Strategy- Specific- Utilities.md`

---

## **CRITICAL EXECUTION DIRECTIVE**

**AI Agent Directive**: Follow this protocol exactly for all test implementation decisions.

**MANDATORY EXECUTION PROTOCOL**:

1. **NO DEVIATION**: All rules must be followed exactly as written
2. **NO SKIPPING**: No steps may be skipped, abbreviated, or modified
3. **NO SELECTIVE COMPLIANCE**: All rules apply to all actions
4. **FAILURE TO COMPLY**: Violating these rules constitutes a critical protocol violation

## **MANDATORY PRE-RESPONSE VALIDATION FRAMEWORK**

### **CRITICAL EXECUTION CHECKLIST**

**CRITICAL**: Before ANY response involving test implementation operations, execute this validation sequence:

**STEP 1: PAE ALIAS COMPLIANCE**

- [ ] **Alias Discovery**: Will I attempt `pae help` first to discover available aliases?
- [ ] **Workspace Root**: When I need to return to the workspace root, will I use `cd D:/_dev/_Projects/_fux/_FocusedUX` ?
- [ ] **Alias Usage**: Will I use appropriate alias for the operation (e.g., `pae dc b` for dynamicons build)?
- [ ] **Fallback Protocol**: Will I only use direct nx commands if no alias exists or alias fails?

**STEP 2: DOCUMENTATION FIRST COMPLIANCE**

- [ ] **Architecture Check**: Have I checked **ARCHITECTURE_DOCS** for package structure and patterns?
- [ ] **Testing Check**: Have I checked **TESTING_STRATEGY** (`docs/testing/(AI) _Strategy- Base- Testing.md`) for testing patterns?
- [ ] **Mocking Check**: Have I checked **MOCKING_STRATEGY** (`docs/testing/(AI) _Strategy- Base- Mocking.md`) for mocking patterns?
- [ ] **Troubleshooting Check**: Have I checked **TEST_TROUBLESHOOTING** (`docs/testing/(AI) _Troubleshooting- Tests.md`) for error handling?
- [ ] **Package-Specific Strategy**: Have I checked the appropriate package-specific strategy document?
    - [ ] **Core Packages**: `docs/testing/(AI) _Strategy- Specific- Core.md` (if exists)
    - [ ] **Extension Packages**: `docs/testing/(AI) _Strategy- Specific- Ext.md`
    - [ ] **Shared Packages**: `docs/testing/(AI) _Strategy- Specific- Libs.md`
    - [ ] **Tool Packages**: `docs/testing/(AI) _Strategy- Specific- Utilities.md`
    - [ ] **Plugin Packages**: `docs/testing/(AI) _Strategy- Specific- Plugins.md`

**STEP 3: PACKAGE ANALYSIS COMPLIANCE**

- [ ] **Project Details**: Will I use `nx_project_details` to understand package dependencies?
- [ ] **Package Type Verification**: Will I verify package type (core vs ext vs shared vs tool) and role?
- [ ] **Test File Analysis**: Will I analyze the skeleton test file structure and patterns?

**STEP 4: IMPLEMENTATION STRATEGY COMPLIANCE**

- [ ] **Mock Strategy**: Will I use the correct mock strategy based on package type?
- [ ] **Test Patterns**: Will I follow the established test patterns for the package type?
- [ ] **Folding Markers**: Will I implement proper folding markers for test organization?
- [ ] **Folding Marker Validation**: Will I validate that `//>` and `//<` are placed at the END of lines, not on separate lines?
- [ ] **Mock Hierarchy Compliance**: Will I check for existing package-level mocks before creating direct `vi.mock` calls?
- [ ] **Global Mocks Check**: Will I verify that `packages/{package}/__tests__/__mocks__/globals.ts` exists and contains required mocks?

**STEP 5: SELF-CORRECTION**

- [ ] **Violation Detection**: If I detect any protocol violation, will I acknowledge immediately?

**VIOLATION PENALTY**: Any failure to complete this checklist constitutes a critical failure requiring immediate acknowledgment and correction.

### **CRITICAL DOCUMENTATION ENFORCEMENT**

**MANDATORY**: Before implementing ANY test, you MUST read ALL applicable AI-specific testing strategy documents:

1. **ALWAYS READ**: `docs/testing/(AI) _Strategy- Base- Testing.md`
2. **ALWAYS READ**: `docs/testing/(AI) _Strategy- Base- Mocking.md`
3. **ALWAYS READ**: `docs/testing/(AI) _Troubleshooting- Tests.md`
4. **READ BASED ON PACKAGE TYPE**:
    - **Tool Packages** (`libs/tools/` or `libs/project-alias-expander`): `docs/testing/(AI) _Strategy- Specific- Utilities.md`
    - **Core Packages** (`packages/{feature}/core/`): Check for `docs/testing/(AI) _Strategy- Specific- Core.md`
    - **Extension Packages** (`packages/{feature}/ext/`): `docs/testing/(AI) _Strategy- Specific- Ext.md`
    - **Shared Packages** (`libs/shared/`): `docs/testing/(AI) _Strategy- Specific- Libs.md`
    - **Plugin Packages** (`plugins/`): `docs/testing/(AI) _Strategy- Specific- Plugins.md`

**CRITICAL FAILURE**: Reading general testing documents (`docs/testing/_Testing-Strategy.md`) instead of AI-specific documents (`docs/testing/(AI) _Strategy- Base- Testing.md`) constitutes a **CRITICAL PROTOCOL VIOLATION** requiring immediate acknowledgment and correction.

---

## **COMMAND EXECUTION PROTOCOL**

### **Phase 1: Single File Analysis**

1. **Discover PAE Aliases**: Run `pae help` to identify available commands
2. **Single File Identification**: Determine the exact test file to implement from user input
3. **File Validation**: Verify the specified file exists and contains skeleton tests
4. **Package Analysis**: Use `nx_project_details` to understand the package structure
5. **Package Type Verification**: Identify package type (core/ext/shared/tool) for the file's package

### **Phase 2: Single File Test Analysis**

1. **Skeleton Test Identification**: Identify placeholder test implementations in the specified file
2. **Test Structure Analysis**: Analyze existing test structure and patterns in the file
3. **Mock Strategy Assessment**: Determine appropriate mock strategy based on package type
4. **Dependency Analysis**: Identify dependencies and external modules to mock for this file

### **Phase 3: Single File Implementation**

1. **Mock Setup**: Implement proper mock setup based on package type for this file
2. **Test Implementation**: Implement full test cases following established patterns for this file
3. **Folding Markers**: Add proper folding markers for test organization in this file
4. **Folding Marker Validation**: Ensure `//>` and `//<` are placed at the END of lines, not on separate lines
5. **Error Handling**: Implement comprehensive error scenario testing for this file

### **Phase 4: Validation and Testing**

1. **Development Build**: Rebuild using development build if it exists
2. **Single File Testing**: Run tests for the specific file being worked on
3. **Full Test Suite**: Run complete test suite once single file is green
4. **Coverage Validation**: Verify test coverage meets requirements
5. **Pattern Compliance**: Ensure adherence to testing guidelines

---

## **MOCK HIERARCHY VALIDATION PROTOCOL**

### **CRITICAL MOCK HIERARCHY RULES**

**MANDATORY**: Before creating ANY direct `vi.mock` calls, execute this validation sequence:

#### **Step 1: Check Package-Level Global Mocks**

```bash
# Check if globals.ts exists
ls packages/{package}/__tests__/__mocks__/globals.ts

# Check if it contains the required mocks
grep -E "(vi\.mock|fs|path|node:)" packages/{package}/__tests__/__mocks__/globals.ts
```

#### **Step 2: Verify Mock Coverage**

- [ ] **Node.js Built-ins**: `fs`, `path`, `os`, `child_process` → MUST be in `globals.ts`
- [ ] **Package Services**: `ConfigLoader`, `shell`, etc. → MUST be in `globals.ts` or package mocks
- [ ] **External Dependencies**: Third-party packages → Check if already mocked

#### **Step 3: Mock Hierarchy Decision Tree**

```
Do I need to mock Node.js built-in modules (fs, path, os, etc.)?
├─ YES → Check globals.ts first
│   ├─ EXISTS → Use existing mocks (NO direct vi.mock)
│   └─ MISSING → Add to globals.ts (NOT in test file)
└─ NO → Do I need to mock package-specific services?
    ├─ YES → Check package __mocks__/ directory
    │   ├─ EXISTS → Use existing mocks
    │   └─ MISSING → Create package-level mock
    └─ NO → Direct vi.mock in test file (ONLY for test-specific mocks)
```

### **MOCK HIERARCHY VIOLATION DETECTION**

**MANDATORY**: Before implementing any test, validate:

1. **No Direct Node.js Module Mocking**: Never use `vi.mock('fs')` or `vi.mock('path')` in test files
2. **No Duplicate Mocking**: Never recreate mocks that already exist in `globals.ts`
3. **Proper Mock Location**: Use the correct mock hierarchy level
4. **Existing Infrastructure**: Leverage existing package-level mock infrastructure

### **CORRECT MOCK PATTERNS**

#### **✅ CORRECT: Using Existing Global Mocks**

```typescript
import { describe, it, expect, beforeEach } from 'vitest'
import { AliasManagerService } from '../../../src/services/AliasManager.service.js'
// No direct vi.mock calls - globals.ts handles all Node.js modules

describe('AliasManagerService', () => {
    // SETUP ----------------->>
    let aliasManager: AliasManagerService

    beforeEach(() => {
        //>
        // globals.ts already mocks fs, path, ConfigLoader, shell
        aliasManager = new AliasManagerService()
    }) //<
    //----------------------------------------------------<<

    // Tests go here...
})
```

#### **❌ INCORRECT: Direct Node.js Module Mocking**

```typescript
// ❌ FORBIDDEN: Direct mocking of Node.js modules
vi.mock('fs', () => ({
    existsSync: vi.fn(),
    readFileSync: vi.fn(),
    // ... etc
}))

vi.mock('path', () => ({
    join: vi.fn(),
    resolve: vi.fn(),
    // ... etc
}))
```

#### **✅ CORRECT: Test-Specific Mocking Only**

```typescript
// ✅ ALLOWED: Only for test-specific scenarios not covered by globals.ts
vi.mock('some-specific-test-dependency', () => ({
    specificTestFunction: vi.fn(),
}))
```

---

## **FOLDING MARKER VALIDATION PATTERNS**

### **CRITICAL FOLDING MARKER RULES**

**MANDATORY**: All folding markers must follow these exact patterns:

#### **Correct Pattern (REQUIRED)**

```typescript
it('should do something', () => {
    //>
    // test code here
}) //<

beforeEach(() => {
    //>
    // setup code here
}) //<

afterEach(() => {
    //>
    // cleanup code here
}) //<
```

#### **Incorrect Pattern (FORBIDDEN)**

```typescript
it('should do something', () => {
    //>
    // test code here
}) //<

beforeEach(() => {
    //>
    // setup code here
}) //<
```

### **FOLDING MARKER VALIDATION CHECKLIST**

- [ ] **`it` blocks**: `//>` at END of opening line, `//<` at END of closing line
- [ ] **`beforeEach` blocks**: `//>` at END of opening line, `//<` at END of closing line
- [ ] **`afterEach` blocks**: `//>` at END of opening line, `//<` at END of closing line
- [ ] **Setup sections**: `// SETUP ----------------->>` and `//----------------------------------------------------<<` for main setup constants
- [ ] **Setup section content**: All `beforeEach`/`afterEach` blocks MUST be inside the setup section
- [ ] **NO separate lines**: Folding markers must NOT be on their own lines
- [ ] **Space requirement**: All folding markers must be preceded by a space

### **CRITICAL FOLDING MARKER VALIDATION RULES**

**MANDATORY**: Before completing any test implementation, verify these exact patterns:

#### **✅ CORRECT: Folding markers at END of lines**

```typescript
it('test name', () => {
    //>
    // test content
}) //<

beforeEach(() => {
    //>
    // setup content
}) //<

afterEach(() => {
    //>
    // cleanup content
}) //<
```

#### **❌ FORBIDDEN: Folding markers on separate lines**

```typescript
it('test name', () => {
    //>
    // test content
}) //<

beforeEach(() => {
    //>
    // setup content
}) //<
```

#### **✅ CORRECT: Setup section markers**

```typescript
describe('TestSuite', () => {
    // SETUP ----------------->>
    let variable1: Type
    let variable2: Type
    //----------------------------------------------------<<

    beforeEach(() => {
        //>
        // setup code
    }) //<

    // Tests go here...
})
```

#### **❌ FORBIDDEN: Setup blocks outside setup section**

```typescript
describe('TestSuite', () => {
    let variable1: Type

    beforeEach(() => {
        //>
        // setup code - WRONG: outside setup section
    }) //<
})
```

### **AUTOMATIC VALIDATION**

**MANDATORY**: Before completing any test implementation, validate that:

1. **No folding markers on separate lines** - All `//>` and `//<` must be at the end of their respective lines
2. **Proper spacing** - All folding markers must be preceded by a space
3. **Complete wrapping** - All `it`, `beforeEach`, and `afterEach` blocks must be properly wrapped
4. **Setup sections** - Main setup constants must use `// SETUP ----------------->>` and `//----------------------------------------------------<<`
5. **Setup section content** - All `beforeEach`/`afterEach` blocks must be inside the setup section

### **MANDATORY PRE-COMPLETION VALIDATION**

**CRITICAL**: Before marking any test implementation as complete, execute this validation:

#### **Step 1: Folding Marker Placement Check**

- [ ] **Scan all `it` blocks**: Verify `//>` is at the END of the opening line, not on a separate line
- [ ] **Scan all `beforeEach` blocks**: Verify `//>` is at the END of the opening line, not on a separate line
- [ ] **Scan all `afterEach` blocks**: Verify `//>` is at the END of the opening line, not on a separate line
- [ ] **Scan all closing lines**: Verify `//<` is at the END of the closing line, not on a separate line

#### **Step 2: Setup Section Validation**

- [ ] **Setup constants**: Verify they are between `// SETUP ----------------->>` and `//----------------------------------------------------<<`
- [ ] **Setup blocks**: Verify all `beforeEach`/`afterEach` are INSIDE the setup section
- [ ] **Setup markers**: Verify setup markers are properly positioned

#### **Step 3: Pattern Compliance Check**

- [ ] **No separate line markers**: Confirm no folding markers are on their own lines
- [ ] **Proper spacing**: Confirm all folding markers are preceded by a space
- [ ] **Complete wrapping**: Confirm all test blocks are properly wrapped

**VIOLATION DETECTION**: If ANY folding marker is found on a separate line, STOP and fix immediately before proceeding.

---

## **IMPLEMENTATION PATTERNS BY PACKAGE TYPE**

### **Core Packages** (`packages/{feature}/core/`)

**Mock Strategy**: Use `setupPaeTestEnvironment()` and `resetPaeMocks()`
**Test Pattern**: Pure business logic testing with comprehensive mocking

```typescript
import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setupPaeTestEnvironment, resetPaeMocks } from '@fux/mock-strategy/pae'

describe('ServiceName', () => {
    // SETUP ----------------->>
    let mocks: Awaited<ReturnType<typeof setupPaeTestEnvironment>>
    let service: any
    //----------------------------------------------------<<

    beforeEach(async () => {
        //>
        mocks = await setupPaeTestEnvironment()
        await resetPaeMocks(mocks)
        service = new ServiceName()
    }) //<

    it('should perform expected action', async () => {
        //>
        const scenario = await createPaeMockBuilder(mocks)
            .serviceName()
            .expectedAction()
            .withValidInput()
            .build()

        const result = await service.performAction(scenario.input)

        expect(result).toBeDefined()
        expect(mocks.mockFunction).toHaveBeenCalledWith(scenario.input)
    }) //<

    it('should handle error scenarios', async () => {
        //>
        const scenario = await createPaeMockBuilder(mocks)
            .serviceName()
            .expectedAction()
            .withErrorHandling('specific-error')
            .build()

        await expect(service.performAction(scenario.input)).rejects.toThrow('Expected error')
    }) //<
})
```

### **Extension Packages** (`packages/{feature}/ext/`)

**Mock Strategy**: Use `setupExtTestEnvironment()` and `resetExtMocks()`
**Test Pattern**: VSCode wrapper testing with extension lifecycle testing

```typescript
import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setupExtTestEnvironment, resetExtMocks } from '@fux/mock-strategy/ext'

describe('ExtensionName', () => {
    // SETUP ----------------->>
    let mocks: Awaited<ReturnType<typeof setupExtTestEnvironment>>
    let extension: any
    //----------------------------------------------------<<

    beforeEach(async () => {
        //>
        mocks = await setupExtTestEnvironment()
        await resetExtMocks(mocks)
        extension = new ExtensionName()
    }) //<

    it('should activate extension correctly', async () => {
        //>
        const scenario = await createExtMockBuilder(mocks)
            .extension()
            .activation()
            .withValidContext()
            .build()

        const result = await extension.activate(scenario.context)

        expect(result).toBeDefined()
        expect(mocks.vscode.window.showInformationMessage).toHaveBeenCalled()
    }) //<

    it('should handle activation errors', async () => {
        //>
        const scenario = await createExtMockBuilder(mocks)
            .extension()
            .activation()
            .withErrorHandling('activation-failed')
            .build()

        await expect(extension.activate(scenario.context)).rejects.toThrow('Activation failed')
    }) //<
})
```

### **Shared Packages** (`libs/shared/`)

**Mock Strategy**: Use `setupLibTestEnvironment()` and `resetLibMocks()`
**Test Pattern**: In-repo utility testing with cross-package functionality

```typescript
import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setupLibTestEnvironment, resetLibMocks } from '@fux/mock-strategy/lib'

describe('SharedUtility', () => {
    // SETUP ----------------->>
    let mocks: Awaited<ReturnType<typeof setupLibTestEnvironment>>
    let utility: any
    //----------------------------------------------------<<

    beforeEach(async () => {
        //>
        mocks = await setupLibTestEnvironment()
        await resetLibMocks(mocks)
        utility = new SharedUtility()
    }) //<

    it('should perform utility function', async () => {
        //>
        const scenario = await createLibMockBuilder(mocks)
            .utility()
            .performFunction()
            .withValidInput()
            .build()

        const result = await utility.performFunction(scenario.input)

        expect(result).toBeDefined()
        expect(result).toEqual(scenario.expectedOutput)
    }) //<

    it('should handle utility errors', async () => {
        //>
        const scenario = await createLibMockBuilder(mocks)
            .utility()
            .performFunction()
            .withErrorHandling('utility-error')
            .build()

        await expect(utility.performFunction(scenario.input)).rejects.toThrow('Utility error')
    }) //<
})
```

### **Tool Packages** (`libs/tools/{name}/`)

**Mock Strategy**: Use `setupToolTestEnvironment()` and `resetToolMocks()`
**Test Pattern**: Standalone utility testing with command-line interface testing

```typescript
import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setupToolTestEnvironment, resetToolMocks } from '@fux/mock-strategy/tool'

describe('ToolName', () => {
    // SETUP ----------------->>
    let mocks: Awaited<ReturnType<typeof setupToolTestEnvironment>>
    let tool: any
    //----------------------------------------------------<<

    beforeEach(async () => {
        //>
        mocks = await setupToolTestEnvironment()
        await resetToolMocks(mocks)
        tool = new ToolName()
    }) //<

    it('should execute tool command', async () => {
        //>
        const scenario = await createToolMockBuilder(mocks).tool().execute().withValidArgs().build()

        const result = await tool.execute(scenario.args)

        expect(result).toBeDefined()
        expect(result.exitCode).toBe(0)
    }) //<

    it('should handle command errors', async () => {
        //>
        const scenario = await createToolMockBuilder(mocks)
            .tool()
            .execute()
            .withErrorHandling('command-failed')
            .build()

        const result = await tool.execute(scenario.args)

        expect(result.exitCode).not.toBe(0)
        expect(result.error).toBeDefined()
    }) //<
})
```

---

## **MOCK STRATEGY IMPLEMENTATION**

### **Scenario Builder Usage**

**MANDATORY**: Use scenario builders for complex mocking scenarios (3+ mocks, related behavior, stateful interactions)

```typescript
// Example: Complex service testing with scenario builder
const scenario = await createPaeMockBuilder(mocks)
    .configLoader()
    .loadConfig()
    .withValidYaml(validConfig)
    .withFileModificationDetection()
    .withErrorHandling('permission-denied')
    .build()

// Use scenario in test
const result = await configLoader.loadConfig(scenario.configPath)
expect(result).toEqual(scenario.expectedResult)
```

### **Mock Extension Protocol**

**CRITICAL RULE**: When scenario builder methods don't exist, you MUST extend the package-level scenario builder, NOT revert to direct mocking.

1. **Identify Missing Method**: Determine what scenario builder method is needed
2. **Locate Package Scenario Builder**: Find `packages/{package}/__tests__/__mocks__/mock-scenario-builder.ts`
3. **Add Method to Builder**: Extend the builder class with the new method
4. **Implement Scenario Logic**: Add the scenario implementation
5. **Use Extended Builder**: Use the new method in tests

---

## **FOLDING MARKERS IMPLEMENTATION**

### **MANDATORY Folding Marker Rules**

- **`it` blocks**: Always wrapped with `//>` at the END of the opening line and `//<` at the END of the closing line, preceded by a space
- **`beforeEach`/`afterEach`**: Always wrapped with `//>` at the END of the opening line and `//<` at the END of the closing line, preceded by a space
- **Setup sections**: Wrapped with `// SETUP ----------------->>` and `//----------------------------------------------------<<` for main setup constants and variables only
- **`describe` blocks**: NO folding markers - they are not wrapped
- **Test-specific constants**: Go inside individual test cases without setup markers
- **Space requirement**: All folding markers must be preceded by a space

---

## **ERROR HANDLING IMPLEMENTATION**

### **Comprehensive Error Testing**

Every test implementation must include:

1. **Success Scenarios**: Test normal operation with valid inputs
2. **Error Scenarios**: Test error handling with invalid inputs
3. **Edge Cases**: Test boundary conditions and edge cases
4. **Mock Verification**: Verify mocks are called with expected parameters

### **Error Scenario Patterns**

```typescript
it('should handle specific error condition', async () => {
    //>
    const scenario = await createMockBuilder(mocks)
        .service()
        .method()
        .withErrorHandling('specific-error-type')
        .build()

    await expect(service.method(scenario.input)).rejects.toThrow('Expected error message')
    expect(mocks.errorLogger).toHaveBeenCalledWith('Expected error message')
}) //<
```

---

## **VALIDATION AND TESTING**

### **Post-Implementation Validation**

1. **Development Build**: Rebuild using development build if it exists:

    ```bash
    nx run @fux/project-alias-expander:build:dev
    ```

2. **Single File Testing**: Run tests for the specific file being worked on:

    ```bash
    nx run {project.json name}:{project.json target} --testFile {file path relative to workspace root}
    ```

3. **Full Test Suite**: Once single file is green, run complete test suite:

    ```bash
    nx run {project.json name}:{project.json target}
    ```

4. **Coverage Validation**: Run coverage tests to verify coverage meets requirements
5. **Pattern Compliance**: Verify adherence to testing guidelines

### **Success Criteria**

- ✅ **Single file skeleton tests implemented** with full functionality
- ✅ **Proper mock strategy** based on package type for this file
- ✅ **Comprehensive error handling** for all scenarios in this file
- ✅ **Folding markers** properly implemented in this file
- ✅ **Development build** completes successfully (if it exists)
- ✅ **Single file tests pass** without failures
- ✅ **Full test suite passes** without failures
- ✅ **Coverage meets** requirements for this file
- ✅ **Pattern compliance** with testing guidelines for this file

---

## **ANTI-PATTERNS TO AVOID**

### **CRITICAL IMPLEMENTATION VIOLATIONS**

- ❌ **NEVER WORK ON MULTIPLE FILES** - focus on single file only
- ❌ **NEVER SIMPLIFY TESTS** to make them pass quickly
- ❌ **NEVER SKIP ERROR SCENARIOS** for faster implementation
- ❌ **NEVER OMIT FOLDING MARKERS** for test organization
- ❌ **NEVER PLACE FOLDING MARKERS ON SEPARATE LINES** - must be at END of lines, not on their own lines
- ❌ **NEVER USE DIRECT NODE.JS MODULE MOCKING** - use existing globals.ts mocks
- ❌ **NEVER DUPLICATE EXISTING MOCKS** - check globals.ts first
- ❌ **NEVER USE DIRECT MOCKING** when scenario builders exist
- ❌ **NEVER IGNORE PACKAGE TYPE** for mock strategy selection

### **Implementation Violations**

- ❌ Incomplete test implementations
- ❌ Missing error scenario testing
- ❌ Incorrect mock strategy usage
- ❌ Missing folding markers
- ❌ **Folding markers on separate lines** (must be at END of lines, not on their own lines)
- ❌ **Missing space before folding markers** (must be preceded by space)
- ❌ **Direct Node.js module mocking** (must use existing globals.ts)
- ❌ **Duplicate mock implementations** (check existing mocks first)
- ❌ **Ignoring package-level mock infrastructure** (use existing mocks)
- ❌ Non-deterministic test data
- ❌ Hardcoded mock values
- ❌ Missing mock verification

---

## **EXECUTION PRIORITY MATRIX**

### **CRITICAL PRIORITY (Execute immediately)**

- PAE alias compliance verification
- Documentation first verification
- Package analysis execution
- Mock strategy validation

### **HIGH PRIORITY (Execute before proceeding)**

- Test implementation execution
- Development build verification
- Single file test execution
- Folding marker implementation
- Error scenario testing
- Pattern compliance verification

### **MEDIUM PRIORITY (Execute during normal operation)**

- Full test suite execution
- Coverage validation
- Documentation updates
- Status reporting

### **LOW PRIORITY (Execute when time permits)**

- Performance optimization
- Pattern documentation
- Lesson sharing
- Future planning

---

## **VIOLATION PREVENTION**

### **Natural Stops**

- **MANDATORY**: Multiple files specified → "STOP! Work on single file only"
- **MANDATORY**: Missing scenario builder method → "Extend scenario builder"
- **MANDATORY**: Direct mocking for complex scenarios → "Use scenario builder"
- **MANDATORY**: Missing error scenarios → "Implement comprehensive error testing"
- **MANDATORY**: Folding markers on separate lines → "STOP! Move folding markers to END of lines, not on separate lines"
- **MANDATORY**: Missing space before folding markers → "STOP! Add space before folding markers"
- **MANDATORY**: beforeEach/afterEach outside setup section → "STOP! Move setup blocks inside setup section"
- **MANDATORY**: Direct Node.js module mocking → "STOP! Check globals.ts for existing mocks"
- **MANDATORY**: Duplicate mock implementations → "STOP! Use existing package-level mocks"
- **MANDATORY**: Missing folding markers → "Add proper folding markers"
- **MANDATORY**: Wrong mock strategy → "Use correct strategy for package type"

### **Pattern Recognition**

- Package type → Determines mock strategy and test patterns
- Test complexity → Determines scenario builder usage
- Error context → Determines error scenario implementation
- File structure → Determines folding marker placement
- User question type → Determines response strategy

---

## **DYNAMIC MANAGEMENT NOTE**

This document is optimized for AI internal processing and may be updated dynamically based on operational needs and pattern recognition. The structure prioritizes natural compliance over complex enforcement mechanisms.
