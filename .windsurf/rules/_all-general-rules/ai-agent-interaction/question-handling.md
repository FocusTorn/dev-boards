---
trigger: always_on
---

# Question Handling Rules

## **QUESTION HANDLING PROTOCOL**

### **1. :: Question Marker Recognition**

**✅ CORRECT - Recognize question markers**:

When the user provides a message starting with `Question:` or `question:` or `Question,` or `question,` followed by a question, the AI Agent MUST:

1. **Immediately Stop**: Stop any current task or response - question handling protocol supersedes normal task flow
2. **Acknowledge Question**: Explicitly acknowledge that a question marker has been detected
3. **Answer Only**: Provide ONLY an answer to the question - do NOT take any actions, make changes, or execute tasks
4. **No Code Changes**: Do NOT edit files, create files, or modify code
5. **No Tool Calls for Actions**: Do NOT call tools that would make changes (file edits, deletions, etc.)
6. **No Tool Calls for Information Gathering**: Do NOT call tools to gather information unless explicitly needed to answer the question (and even then, prefer using already-loaded context)
7. **Comparison Requests**: If the question asks to compare solutions (e.g., "why did we...", "should it..."), provide a comparison with pros/cons list

**CRITICAL ENFORCEMENT**: When ANY message contains `Question:` or `question:` or `Question,` or `question,` marker at the start, immediately STOP all current processing, acknowledge the question marker explicitly, and enter question handling protocol mode. Do NOT proceed with any actions, code changes, or task execution. Only provide answers, explanations, or comparisons as requested.

**MANDATORY VERIFICATION CHECKPOINT**: Before making ANY tool calls or taking ANY actions, the AI agent MUST explicitly verify:
- "Is this a question that requires only an answer?"
- "Am I about to make any code changes or file edits?"
- "Am I about to call any tools that modify files or execute commands?"

If the answer to the second or third question is YES, the agent MUST NOT proceed and MUST only provide an answer.

**✅ CORRECT - Question marker format**:

```
Question: [The question being asked]

[Additional context or details]
```

or

```
Question, [The question being asked]

[Additional context or details]
```

**CRITICAL**: The marker MUST be at the very start of the message. Any message starting with these markers triggers question protocol, regardless of what follows.

**✅ CORRECT - Question types and responses**:

1. **Comparison Questions** (e.g., "why did we...", "should it...", "is it better to..."):
   - Provide comparison between proposed solution and current implementation
   - Include pros/cons list for each approach
   - Explain which is better and why
   - Do NOT implement changes

2. **Explanation Questions** (e.g., "how does...", "what is...", "explain..."):
   - Provide clear explanation
   - Reference relevant code or documentation
   - Do NOT make changes

3. **Analysis Questions** (e.g., "what are the...", "list the...", "show me..."):
   - Provide analysis or list as requested
   - Use appropriate formatting (summary format if applicable)
   - Do NOT make changes

**❌ INCORRECT - Ignoring question markers**:

- Taking actions when question marker is detected
- Making code changes when only an answer is requested
- Implementing solutions when comparison is requested
- Continuing with normal task processing before answering
- Treating question as a task request rather than information request
- Calling file editing tools (search_replace, write, etc.) when question marker is present
- Calling terminal commands when question marker is present
- Gathering information with tools when the answer can be provided from existing context
- Proceeding with any tool calls before explicitly verifying the question protocol

**CRITICAL VIOLATION**: Making ANY code changes, file edits, or tool calls that modify the codebase when a question marker is detected constitutes a CRITICAL PROTOCOL VIOLATION.

## **NATURAL STOPS FOR QUESTION HANDLING**

**MANDATORY PAUSE POINTS**: The AI agent MUST pause and verify question protocol at these natural stops:

1. **Before ANY Tool Call**: Before calling ANY tool, verify if question marker is present
2. **Before File Edits**: Before calling search_replace, write, delete_file, or any file modification tool, verify question protocol
3. **Before Terminal Commands**: Before calling run_terminal_cmd, verify question protocol
4. **Before Code Changes**: Before making any code changes, verify question protocol

**VERIFICATION REQUIRED**: At each natural stop, the AI agent MUST explicitly verify:
- "Did the user's message start with Question: or question: or Question, or question,?"
- "If yes, am I about to make any changes or call any tools?"
- "If yes to both, I MUST NOT proceed - answer only"

If a question marker is detected and the agent is about to make changes, the agent MUST NOT proceed and MUST only provide an answer.

**CRITICAL**: These natural stops apply BEFORE any tool calls are made. The question protocol check must happen at the tool call decision point, not after tools have been called.

## **QUALITY GATES**

### **Question Handling**

- [ ] **Pre-Processing Check**: Message prefix checked for question markers BEFORE any processing
- [ ] **Question Acknowledged**: Question marker was recognized and explicitly acknowledged
- [ ] **Verification Checkpoint Passed**: Verified "answer only" mode before any tool calls
- [ ] **Answer Only**: Only answer provided, no actions taken
- [ ] **No Code Changes**: No files edited, created, or modified
- [ ] **No Tool Calls**: No tool calls made (except read-only if absolutely necessary for answer)
- [ ] **No File Edits**: No search_replace, write, delete_file, or other file modification tools called
- [ ] **No Terminal Commands**: No run_terminal_cmd calls made
- [ ] **Comparison Provided**: If comparison requested, pros/cons list provided

