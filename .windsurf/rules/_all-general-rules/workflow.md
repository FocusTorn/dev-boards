---
trigger: always_on
---

# Workflow Rules

## CRITICAL EXECUTION DIRECTIVE

**AI Agent Directive**: Follow workflow rules exactly for all task execution and skill usage patterns.

**MANDATORY EXECUTION PROTOCOL**:

1. **NO DEVIATION**: All workflow rules must be followed exactly as written
2. **NO SKIPPING**: No steps may be skipped, abbreviated, or modified
3. **NO SELECTIVE COMPLIANCE**: All rules apply to all workflow activities
4. **FAILURE TO COMPLY**: Violating these rules constitutes a critical protocol violation

## SKILL USAGE RULES

### 1. :: Language-Specific Task Skill Invocation

**✅ CORRECT - Always invoke skills for language-specific tasks**:

When working on language-specific tasks (Rust, Python, JavaScript, etc.), ALWAYS check for and invoke the corresponding skill before proceeding with implementation or specification. Skills contain current best practices, dependency versions, and idiomatic patterns.

**✅ CORRECT - Skill invocation protocol**:

1. **Check for Skills**: Before starting any language-specific task, check if a corresponding skill exists
2. **Invoke Skill**: Use the skill tool to access specialized knowledge and current best practices
3. **Apply Knowledge**: Use the skill's guidance to inform implementation decisions
4. **Verify Current**: Skills provide up-to-date information on dependencies and patterns

**✅ CORRECT - Examples of tasks requiring skill invocation**:

- Creating specifications for Rust applications
- Implementing Python packages or scripts
- Writing JavaScript/TypeScript code
- Setting up project configurations
- Choosing dependency versions
- Following language-specific architectural patterns

**❌ INCORRECT - Skipping skill invocation**:

- Proceeding with Rust tasks without using the rust skill
- Creating Python specifications without checking current best practices
- Implementing solutions without verifying dependency versions
- Assuming knowledge without consulting specialized resources

## TASK EXECUTION PATTERNS

### 1. :: Proactive Rule Discovery

**✅ CORRECT - Check rules before starting tasks**:

Before beginning any task, always check for relevant rules in the `.agent/rules/` directory structure:

1. **Universal Rules**: Check root-level rules for general patterns
2. **Language-Specific Rules**: Check `by-language/[language]/` for language guidance
3. **Tool-Specific Rules**: Check `tool/[tool]/` for tool-specific patterns
4. **Project-Specific Rules**: Check project directories for local requirements

**✅ CORRECT - Rule discovery process**:

```bash
# Check for relevant rules before starting
find .agent/rules -name "*.mdc" | grep -E "(rust|workflow|general)"
```

**❌ INCORRECT - Proceeding without rule discovery**:

- Starting tasks without checking for relevant rules
- Assuming no rules exist without verification
- Missing critical guidance that affects implementation

### 2. :: Task Planning and Verification

**✅ CORRECT - Plan before execution**:

1. **Research Requirements**: Understand task requirements thoroughly
2. **Check Dependencies**: Verify current versions and best practices
3. **Create Plan**: Develop step-by-step implementation plan
4. **Verify Resources**: Ensure all necessary tools and skills are available

**✅ CORRECT - Use todo lists for complex tasks**:

```markdown
- [ ] Step 1: Research and verify requirements
- [ ] Step 2: Check for relevant rules and skills
- [ ] Step 3: Invoke appropriate skills
- [ ] Step 4: Create implementation plan
- [ ] Step 5: Execute implementation
- [ ] Step 6: Verify and test results
```

## ANTI-PATTERNS

### ❌ Workflow Violations

- ❌ **Skipping Skill Invocation** - Don't proceed with language-specific tasks without invoking corresponding skills
- ❌ **Ignoring Rule Discovery** - Don't start tasks without checking for relevant rules
- ❌ **Assuming Current Knowledge** - Don't assume your knowledge is current without verification
- ❌ **Proceeding Without Planning** - Don't start implementation without proper research and planning
- ❌ **Missing Dependency Verification** - Don't use dependency versions without verifying they're current
- ❌ **Incomplete Task Execution** - Don't abandon tasks before completion and verification

## QUALITY GATES

- [ ] **Skill Checked**: Relevant skills have been invoked for language-specific tasks
- [ ] **Rules Discovered**: All relevant rules have been checked before task execution
- [ ] **Current Knowledge**: Dependency versions and best practices are verified as current
- [ ] **Plan Created**: Clear implementation plan exists before starting work
- [ ] **Resources Verified**: All necessary tools and resources are available
- [ ] **Task Completed**: Tasks are fully completed with verification and testing

## SUCCESS METRICS

After implementing proper workflow patterns:

- ✅ **Consistent Quality** - All work follows current best practices and patterns
- ✅ **Current Dependencies** - All dependency versions are verified as current and appropriate
- ✅ **Complete Research** - Tasks are thoroughly researched before implementation
- ✅ **Rule Compliance** - All relevant rules are discovered and followed
- ✅ **Skill Utilization** - Specialized knowledge is leveraged through skill invocation
- ✅ **Systematic Approach** - All tasks follow consistent planning and execution patterns
- ✅ **Verified Results** - All implementations are tested and verified as correct
- ✅ **No Assumptions** - Knowledge is verified rather than assumed
