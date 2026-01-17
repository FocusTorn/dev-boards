---
trigger: always_on
---

# Plain Text Summary Rules

## **PLAIN TEXT / RETRO STYLE FORMAT**

### **1. :: Plain Text Summary Format**

**When user requests a plain text/ASCII summary format, use this retro style**:

**✅ CORRECT - Plain text summary with ASCII styling**:

```text
╔═══════════════════════════════════════════════════════════════════════════╗
║                          [TITLE OF SUMMARY]                               ║
╚═══════════════════════════════════════════════════════════════════════════╝

┌───────────────────────────────────────────────────────────────────────────┐
│ [SECTION 1 TITLE]                                                         │
└───────────────────────────────────────────────────────────────────────────┘

  1. [First Top Level Bullet]
     • [First Detail Bullet]
     • [Second Detail Bullet]

  2. [Second Top Level Bullet]
     • [First Detail Bullet]
     • [Second Detail Bullet]

═══════════════════════════════════════════════════════════════════════════

┌───────────────────────────────────────────────────────────────────────────┐
│ [SECTION 2 TITLE]                                                         │
└───────────────────────────────────────────────────────────────────────────┘

  1. [First Top Level Bullet]
     • [First Detail Bullet]
     • [Second Detail Bullet]

  2. [Second Top Level Bullet]
     • [First Detail Bullet]
     • [Second Detail Bullet]

═══════════════════════════════════════════════════════════════════════════
```

### **2. :: Nesting Code Blocks in Plain Text Summaries**

```text
┌───────────────────────────────────────────────────────────────────────────┐
│ [SECTION TITLE]                                                           │
└───────────────────────────────────────────────────────────────────────────┘

  1. [First Top Level Bullet]
     • [First Detail Bullet]
     • [Second Detail Bullet with code example]:

       ┌─────────────────────────────────────────────────────────────────┐
       │ Host RPi-Clean                                                  │
       │     HostName 192.168.1.159                                      │
       │     User pi                                                     │
       │     IdentityFile C:\Users\<YourUsername>\.ssh\rpi_clean        │
       └─────────────────────────────────────────────────────────────────┘

  2. [Second Top Level Bullet]
     • [First Detail Bullet]
```

### **3. :: Formatting Rules**

- **Title Box**: Use double-line box (╔═╗╚═╝) for main title
- **Section Box**: Use single-line box (┌─┐└─┘) for section headers
- **Major Separator**: Use double-line (═══) between major sections
- **Bullet Points**: Use • (bullet) for detail items
- **Code Boxes**: Use single-line box for nested code/config examples
- **Indentation**: 2 spaces for numbered items, 5 spaces for bullets
- **Consistent Width**: Keep boxes at consistent width (75 chars recommended)

### **4. :: Trigger Phrases**

- "plain text summary"
- "ASCII summary"
- "text file summary"
- "retro style summary"
- "no markdown summary"
