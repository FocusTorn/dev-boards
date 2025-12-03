# SSH Setup Guide: Windows Host to Headless Raspberry Pi 4 (Debian Trixie)

## 1. :: Windows SSH Framework Setup

1. Verify OpenSSH Client Installation
   - Open PowerShell as Administrator
   - Run: `Get-WindowsCapability -Online | Where-Object Name -like 'OpenSSH.Client*'`
   - If not installed, run: `Add-WindowsCapability -Online -Name OpenSSH.Client~~~~0.0.1.0`

2. Create SSH Directory Structure
   - Ensure `C:\Users\<YourUsername>\.ssh` directory exists
   - Run: `New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.ssh"`

---

## 2. :: SSH Key Pair Generation

1. Generate Ed25519 Key Pair (Recommended)
   - Open PowerShell (standard user, not admin)
   - Run: `ssh-keygen -t ed25519 -f "$env:USERPROFILE\.ssh\rpi_clean_ed25519" -C "vscode-rpi-clean"`
   - When prompted for passphrase, press Enter twice for passwordless

---

## 3. :: SSH Config File Creation

1. Create/Edit SSH Config File
   - Open or create: `C:\Users\<YourUsername>\.ssh\config`
   - Add the following configuration:

````ssh-config
Host RPi-Clean
    HostName 192.168.1.159
    User pi
    IdentityFile C:\Users\<YourUsername>\.ssh\rpi_clean_ed25519
    IdentitiesOnly yes
    StrictHostKeyChecking accept-new
````

2. Save and Set Permissions
   - Save the config file
   - Run: `icacls "$env:USERPROFILE\.ssh\config" /inheritance:r /grant:r "$($env:USERNAME):(R)"`

---

## 4. :: Quick Reference Commands

1. Connect via SSH: `ssh RPi-Clean`
2. Copy files to RPi: `scp <file> RPi-Clean:/home/pi/`
3. Test connection: `ssh -v RPi-Clean` (verbose mode for debugging)




















# Add Skeleton Tests

## **COMMAND PURPOSE**

This command generates skeleton test files from natural language descriptions, structured lists, or any number of test requirements. It creates properly structured test files with folding markers, placeholder implementations, and complete adherence to FocusedUX testing guidelines.

**FLEXIBLE INPUT**: Accepts natural language, structured lists, or mixed formats to generate comprehensive skeleton test suites.

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

**AI Agent Directive**: Follow this protocol exactly for all skeleton test generation decisions.

**MANDATORY EXECUTION PROTOCOL**:

1. **NO DEVIATION**: All rules must be followed exactly as written
2. **NO SKIPPING**: No steps may be skipped, abbreviated, or modified
3. **NO SELECTIVE COMPLIANCE**: All rules apply to all actions
4. **FAILURE TO COMPLY**: Violating these rules constitutes a critical protocol violation

## **MANDATORY PRE-RESPONSE VALIDATION FRAMEWORK**

### **CRITICAL EXECUTION CHECKLIST**

**CRITICAL**: Before ANY response involving skeleton test generation operations, execute this validation sequence:

**STEP 1: PAE ALIAS COMPLIANCE**

- [ ] **Alias Discovery**: Will I attempt `pae help` first to discover available aliases?
- [ ] **Workspace Root**: When I need to return to the workspace root, will I use `cd D:/_dev/_Projects/_fux/_FocusedUX` ?
- [ ] **Alias Usage**: Will I use appropriate alias for the operation (e.g., `pae dc b` for dynamicons build)?
- [ ] **Fallback Protocol**: Will I only use direct nx commands if no alias exists or alias fails?

**STEP 2: DOCUMENTATION FIRST COMPLIANCE**

- [ ] **Architecture Check**: Have I checked **ARCHITECTURE_DOCS** for package structure and patterns?
- [ ] **Testing Check**: Have I checked **TESTING_STRATEGY** for testing patterns?
- [ ] **Mocking Check**: Have I checked **MOCKING_STRATEGY** for mocking patterns?

**STEP 3: PACKAGE ANALYSIS COMPLIANCE**

- [ ] **Project Details**: Will I use `nx_project_details` to understand package dependencies?
- [ ] **Package Type Verification**: Will I verify package type (core vs ext vs shared vs tool) and role?
- [ ] **Test Structure Analysis**: Will I analyze existing test directory structure?

**STEP 4: INPUT PROCESSING COMPLIANCE**

- [ ] **Input Parsing**: Will I parse the input to identify test requirements and file structure?
- [ ] **File Organization**: Will I determine appropriate file locations and naming conventions?
- [ ] **Test Structure**: Will I create proper test structure with folding markers?

**STEP 5: SELF-CORRECTION**

- [ ] **Violation Detection**: If I detect any protocol violation, will I acknowledge immediately?

**VIOLATION PENALTY**: Any failure to complete this checklist constitutes a critical failure requiring immediate acknowledgment and correction.

---

## **COMMAND EXECUTION PROTOCOL**

### **Phase 1: Input Analysis and Parsing**

1. **Discover PAE Aliases**: Run `pae help` to identify available commands
2. **Input Processing**: Parse user input to identify test requirements
3. **Package Identification**: Determine target package from context or user input
4. **Test Structure Analysis**: Analyze existing test directory structure
5. **File Organization Planning**: Plan file locations and naming conventions

### **Phase 2: Package Analysis**

1. **Project Analysis**: Use `nx_project_details` to understand package structure
2. **Package Type Verification**: Identify package type (core/ext/shared/tool)
3. **Source File Analysis**: Analyze source files to understand testing targets
4. **Mock Strategy Assessment**: Determine appropriate mock strategy based on package type

### **Phase 3: Skeleton Generation**

1. **Test File Creation**: Create skeleton test files with proper structure
2. **Folding Markers**: Implement proper folding markers for test organization
3. **Placeholder Implementation**: Add placeholder test implementations
4. **Mock Setup**: Include proper mock setup based on package type

### **Phase 4: Validation and Organization**

1. **File Structure Validation**: Ensure proper test file organization
2. **Folding Marker Validation**: Verify folding markers are correctly implemented
3. **Pattern Compliance**: Ensure adherence to testing guidelines
4. **Build Verification**: Ensure package builds successfully

---

## **INPUT PROCESSING PATTERNS**

### **Structured List Processing**

**Input Format**: Numbered lists with categories and sub-items
**Processing**: Parse each category as a test file, sub-items as test groups

```markdown
1. CLI Main Function Testing
    - Missing: main() function execution tests
    - Missing: Argument parsing and validation tests
    - Missing: Error handling and process cleanup tests
    - Missing: Configuration loading integration tests

2. Shell Detection Testing
    - Missing: Cross-platform shell detection tests
    - Missing: Environment variable handling tests
    - Missing: Shell-specific behavior tests
```

**Generated Files**:

- `CLI.test.ts` - CLI Main Function Testing
- `Shell.test.ts` - Shell Detection Testing

### **Natural Language Processing**

**Input Format**: Natural language descriptions
**Processing**: Extract test requirements and organize into logical test files

```markdown
I need tests for the command execution service that handles process management,
output handling, and error scenarios. Also need tests for the configuration
loader that validates YAML files and handles file system errors.
```

**Generated Files**:

- `CommandExecution.service.test.ts` - Command execution service tests
- `ConfigLoader.service.test.ts` - Configuration loader tests

### **Mixed Format Processing**

**Input Format**: Combination of structured lists and natural language
**Processing**: Parse both formats and organize into comprehensive test structure

---

## **SKELETON TEST FILE STRUCTURE**

### **Standard Skeleton Template**

```typescript
// __tests__/functional-tests/{category}/{TestName}.test.ts
import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'
import { setup{Type}TestEnvironment, reset{Type}Mocks } from '@fux/mock-strategy/{type}'

describe('{TestName}', () => {
    // SETUP ----------------->>
    let mocks: Awaited<ReturnType<typeof setup{Type}TestEnvironment>>
    let {testSubject}: any
    //----------------------------------------------------<<

    beforeEach(async () => {
        //>
        mocks = await setup{Type}TestEnvironment()
        await reset{Type}Mocks(mocks)
        {testSubject} = new {TestName}()
    }) //<

    afterEach(() => {
        //>
        vi.clearAllMocks()
    }) //<

    describe('{TestGroup1}', () => {
        it('should {test description}', async () => {
            //>
            // TODO: Implement test
            expect(true).toBe(true)
        }) //<

        it('should handle {error scenario}', async () => {
            //>
            // TODO: Implement error test
            expect(true).toBe(true)
        }) //<
    })

    describe('{TestGroup2}', () => {
        it('should {test description}', async () => {
            //>
            // TODO: Implement test
            expect(true).toBe(true)
        }) //<

        it('should handle {error scenario}', async () => {
            //>
            // TODO: Implement error test
            expect(true).toBe(true)
        }) //<
    })
})
```

---

## **PACKAGE TYPE-SPECIFIC TEMPLATES**

### **Core Packages** (`packages/{feature}/core/`)

**Mock Strategy**: `setupPaeTestEnvironment()` and `resetPaeMocks()`
**Import Path**: `@fux/mock-strategy/pae`
**Test Focus**: Pure business logic testing

```typescript
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
        // TODO: Implement test with scenario builder
        // const scenario = await createPaeMockBuilder(mocks)
        //     .serviceName()
        //     .expectedAction()
        //     .withValidInput()
        //     .build()

        expect(true).toBe(true)
    }) //<
})
```

### **Extension Packages** (`packages/{feature}/ext/`)

**Mock Strategy**: `setupExtTestEnvironment()` and `resetExtMocks()`
**Import Path**: `@fux/mock-strategy/ext`
**Test Focus**: VSCode wrapper testing

```typescript
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
        // TODO: Implement extension activation test
        expect(true).toBe(true)
    }) //<
})
```

### **Shared Packages** (`libs/shared/`)

**Mock Strategy**: `setupLibTestEnvironment()` and `resetLibMocks()`
**Import Path**: `@fux/mock-strategy/lib`
**Test Focus**: In-repo utility testing

```typescript
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
        // TODO: Implement utility test
        expect(true).toBe(true)
    }) //<
})
```

### **Tool Packages** (`libs/tools/{name}/`)

**Mock Strategy**: `setupToolTestEnvironment()` and `resetToolMocks()`
**Import Path**: `@fux/mock-strategy/tool`
**Test Focus**: Standalone utility testing

```typescript
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
        // TODO: Implement tool execution test
        expect(true).toBe(true)
    }) //<
})
```

---

## **FOLDING MARKERS IMPLEMENTATION**

### **MANDATORY Folding Marker Rules**

- **`it` blocks**: Always wrapped with `//>` and `//<` at the end of the line, preceded by a space
- **`beforeEach`/`afterEach`**: Always wrapped with `//>` and `//<` at the end of the line, preceded by a space
- **Setup sections**: Wrapped with `// SETUP ----------------->>` and `//----------------------------------------------------<<` for main setup constants and variables only
- **`describe` blocks**: NO folding markers - they are not wrapped
- **Test-specific constants**: Go inside individual test cases without setup markers
- **Space requirement**: All folding markers must be preceded by a space

### **Folding Marker Examples**

```typescript
describe('TestName', () => {
    // SETUP ----------------->>
    let mocks: any
    let testSubject: any
    //----------------------------------------------------<<

    beforeEach(async () => {
        //>
        mocks = await setupTestEnvironment()
        testSubject = new TestSubject()
    }) //<

    it('should perform action', async () => {
        //>
        // Test implementation
        expect(true).toBe(true)
    }) //<
})
```

---

## **FILE ORGANIZATION PATTERNS**

### **Test File Naming Conventions**

- **Service Tests**: `{ServiceName}.service.test.ts`
- **Component Tests**: `{ComponentName}.test.ts`
- **Utility Tests**: `{UtilityName}.test.ts`
- **CLI Tests**: `{CommandName}.test.ts`
- **Integration Tests**: `{FeatureName}.integration.test.ts`

### **Directory Structure**

```
__tests__/functional-tests/
├── cli/
│   ├── AliasCommand.test.ts
│   ├── HelpCommand.test.ts
│   └── InstallCommand.test.ts
├── core/
│   ├── CLI.test.ts
│   ├── Shell.test.ts
│   └── Architecture.test.ts
├── services/
│   ├── ConfigLoader.service.test.ts
│   ├── CommandExecution.service.test.ts
│   └── AliasManager.service.test.ts
└── utils/
    ├── FileUtils.test.ts
    └── StringUtils.test.ts
```

---

## **VALIDATION AND TESTING**

### **Post-Generation Validation**

1. **File Structure Validation**: Ensure proper test file organization
2. **Folding Marker Validation**: Verify folding markers are correctly implemented
3. **Pattern Compliance**: Ensure adherence to testing guidelines
4. **Build Verification**: Ensure package builds successfully with `pae {alias} b`

### **Success Criteria**

- ✅ **All skeleton test files created** with proper structure
- ✅ **Proper mock strategy** based on package type
- ✅ **Folding markers** properly implemented
- ✅ **Placeholder tests** with TODO comments for implementation
- ✅ **File organization** follows established patterns
- ✅ **Package builds** successfully
- ✅ **Pattern compliance** with testing guidelines

---

## **ANTI-PATTERNS TO AVOID**

### **CRITICAL GENERATION VIOLATIONS**

- ❌ **NEVER CREATE INCOMPLETE SKELETONS** without proper structure
- ❌ **NEVER OMIT FOLDING MARKERS** for test organization
- ❌ **NEVER USE WRONG MOCK STRATEGY** for package type
- ❌ **NEVER CREATE FILES IN WRONG LOCATIONS** outside test directories
- ❌ **NEVER SKIP PACKAGE TYPE VERIFICATION** for mock strategy selection

### **Generation Violations**

- ❌ Incomplete skeleton structure
- ❌ Missing folding markers
- ❌ Wrong mock strategy usage
- ❌ Incorrect file naming
- ❌ Missing placeholder implementations
- ❌ Wrong directory structure

---

## **EXECUTION PRIORITY MATRIX**

### **CRITICAL PRIORITY (Execute immediately)**

- PAE alias compliance verification
- Documentation first verification
- Package analysis execution
- Input processing validation

### **HIGH PRIORITY (Execute before proceeding)**

- Skeleton generation execution
- Folding marker implementation
- File organization validation
- Pattern compliance verification

### **MEDIUM PRIORITY (Execute during normal operation)**

- Build verification
- Structure validation
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

- **MANDATORY**: Missing package type → "Determine package type first"
- **MANDATORY**: Wrong mock strategy → "Use correct strategy for package type"
- **MANDATORY**: Missing folding markers → "Add proper folding markers"
- **MANDATORY**: Wrong file location → "Place files in correct test directory"
- **MANDATORY**: Incomplete skeleton → "Complete skeleton structure"

### **Pattern Recognition**

- Package type → Determines mock strategy and test patterns
- Input format → Determines parsing and organization strategy
- Test complexity → Determines file structure and organization
- File location → Determines naming and directory conventions
- User question type → Determines response strategy

---

## **DYNAMIC MANAGEMENT NOTE**

This document is optimized for AI internal processing and may be updated dynamically based on operational needs and pattern recognition. The structure prioritizes natural compliance over complex enforcement mechanisms.

