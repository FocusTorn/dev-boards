---
trigger: always_on
---

# Summary Structure Rules

## **CRITICAL EXECUTION DIRECTIVE**

**AI Agent Directive**: Follow summary structure rules exactly for ALL formatted summaries.

## **SUMMARY STRUCTURE**

### **1. :: Required Summary Format Template**

**✅ CORRECT - Use this exact structure for summaries**:

```markdown
# [TITLE OF SUMMARY]

## [TITLE OF FIRST SECTION]

1. [First Top Level Bullet]
    - [First Detail Bullet]
    - [Second Detail Bullet]

2. [Second Top Level Bullet]
    - [First Detail Bullet]
    - [Second Detail Bullet]

---

## [TITLE OF SECOND SECTION]

1. [First Top Level Bullet]
    - [First Detail Bullet]
    - [Second Detail Bullet]

2. [Second Top Level Bullet]
    - [First Detail Bullet]
    - [Second Detail Bullet]
```

**❌ INCORRECT - Don't use these variations**:

```markdown
# **TITLE OF SUMMARY** <!-- Don't bold titles -->

## TITLE OF FIRST SECTION <!-- Don't skip numbering -->

- First Top Level Bullet <!-- Don't use asterisks for numbered lists -->
    - First Detail Bullet
    - Second Detail Bullet

2. Second Top Level Bullet <!-- Don't skip numbering -->
    - First Detail Bullet
    - Second Detail Bullet
```

**ℹ️ Rationale**:

- Consistent structure ensures readability and scanning
- Numbered sections provide clear hierarchy
- Mixed bullet types (numbered + dash) create visual distinction
- Horizontal rules (`---`) separate major sections clearly
- 3-space indentation for detail bullets maintains visual hierarchy

### **2. :: Summary Content Guidelines**

**✅ CORRECT - Follow these content rules**:

- **Title Brackets**: Use `[TITLE OF SUMMARY]` format for placeholder titles
- **Section Brackets**: Use `[TITLE OF SECTION]` format for placeholder sections
- **Bullet Brackets**: Use `[First Top Level Bullet]` format for placeholder bullets
- **Detail Brackets**: Use `[First Detail Bullet]` format for placeholder details
- **Consistent Indentation**: 3 spaces for detail bullets under numbered items
- **Section Separators**: Use `---` between major sections
- **Sequential Numbering**: Number sections and bullets sequentially

**❌ INCORRECT - Avoid these patterns**:

- Mixed indentation (2 spaces vs 3 spaces)
- Skipping numbers in sequences
- Using asterisks (`*`) for numbered lists
- Missing section separators
- Inconsistent bracket formatting

### **3. :: Nesting Code Blocks Within Summaries**

**✅ CORRECT - Use 4 backticks to nest code blocks within the 3-backtick markdown summary**:

`````markdown
````markdown
# [TITLE OF SUMMARY]

## [TITLE OF SECTION]

1. [First Top Level Bullet]
    - [First Detail Bullet]
    - Add the following configuration:

```ssh-config
Host RPi-Clean
    HostName 192.168.1.159
    User pi
```
````
`````

**❌ INCORRECT - Don't use 3 backticks to nest code (it will close the outer fence)**:

````markdown
# [TITLE OF SUMMARY]

## [TITLE OF SECTION]

1. [First Top Level Bullet]
    - Add the following configuration:

```ssh-config
Host RPi-Clean
    HostName 192.168.1.159
```
````

<!-- This closes the outer markdown fence! -->

```

**ℹ️ Rationale**:

- The outer summary fence uses 3 backticks
- Nested code blocks inside must use 4 backticks
- This prevents the nested code block from closing the outer fence
- Always specify the language type after the opening 4 backticks (e.g., `ssh-config`, `bash`, `powershell`, `python`)
- Close nested code blocks with exactly 4 backticks

### **4. :: Additional Guidelines**

**✅ CORRECT - MANDATORY REQUIREMENTS**:

- **Always wrap in code block**: **MUST** use triple backticks with `markdown` language identifier
- **Maintain placeholder format**: Keep brackets for all placeholder text
- **Preserve structure**: Don't modify the template structure
- **Consistent spacing**: Maintain exact spacing and indentation (3 spaces for detail bullets)
- **Section limits**: Use 2-3 sections maximum for readability
- **Bullet limits**: Use 2-3 bullets per section maximum
- **Detail limits**: Use 2-3 details per bullet maximum
- **Nested code blocks**: Use 4 backticks with language type
- **Plain text format**: Use ASCII box drawing for retro style (see `plaintext-summary.md`)

**❌ INCORRECT - Formatting violations**:

- Missing markdown code block wrapper
- Wrong indentation (not 3 spaces for detail bullets)
- Using asterisks (*) instead of numbered lists
- Missing section separators (`---`)
- Skipping the format entirely

## **ANTI-PATTERNS**

### **❌ Structure Violations**

- ❌ **Missing Code Block Wrapper** - Don't provide summary without markdown code block
- ❌ **Wrong Indentation** - Don't use 2 spaces or 4 spaces (must be 3 spaces for detail bullets)
- ❌ **Using Asterisks** - Don't use `*` for numbered lists (must use `1.`, `2.`, etc.)
- ❌ **Missing Section Separators** - Don't skip `---` between major sections
- ❌ **Inconsistent Formatting** - Don't mix formats within the same summary

## **QUALITY GATES**

- [ ] **Code Block Wrapper**: Summary wrapped in triple backticks with `markdown` identifier
- [ ] **Numbered Lists**: Top-level items use numbered lists (1., 2., etc.)
- [ ] **Dash Bullets**: Detail items use dash bullets (-) with 3-space indentation
- [ ] **Section Separators**: Major sections separated by `---`
- [ ] **Consistent Structure**: All summaries follow the same template structure
