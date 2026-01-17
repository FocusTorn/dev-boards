# Modifying OrcaSlicer Source Code to Add Checkboxes

## Overview

Modifying OrcaSlicer's source code to add checkboxes to the "Send to Printer" popup is a **significant undertaking** that requires:

- **C++ development experience**
- **wxWidgets GUI framework knowledge**
- **CMake build system familiarity**
- **Understanding of OrcaSlicer's architecture**
- **Time investment: 20-40+ hours** (depending on experience)

## Technical Requirements

### 1. Development Environment Setup

**Required Tools:**
- **C++ Compiler**: MSVC (Windows), GCC/Clang (Linux/Mac)
- **CMake**: Build system (version 3.16+)
- **wxWidgets**: GUI framework (version 3.1+)
- **Git**: Version control
- **IDE**: Visual Studio (Windows), Qt Creator, or similar

**Dependencies:**
- Boost libraries
- OpenGL
- Various other libraries (see OrcaSlicer build docs)

**Build Time**: First build can take 1-2 hours on a modern machine

### 2. Codebase Structure

OrcaSlicer is a fork of PrusaSlicer, which is based on Slic3r. The codebase is large (~500K+ lines of C++).

**Key Directories:**
```
OrcaSlicer/
├── src/                    # Main source code
│   ├── gui/               # UI components
│   │   ├── GUI_App.cpp    # Main application
│   │   ├── GUI_ObjectList.cpp
│   │   └── ...            # Dialog classes
│   ├── libslic3r/         # Core slicing engine
│   └── ...
├── resources/             # UI resources, icons
└── CMakeLists.txt        # Build configuration
```

### 3. Files You'd Need to Modify

#### A. Dialog Class (Send to Printer Popup)

**Likely Location**: `src/gui/GUI_App.cpp` or similar dialog class

**What to Find:**
- Dialog class for "Send to Printer" (likely `SendToPrinterDialog` or similar)
- Dialog creation code
- Event handlers for dialog buttons

**What to Add:**
```cpp
// Add checkbox controls
wxCheckBox* m_checkbox_bed_leveling;
wxCheckBox* m_checkbox_flow_calibration;
wxCheckBox* m_checkbox_mech_mode;

// In dialog constructor
m_checkbox_bed_leveling = new wxCheckBox(this, wxID_ANY, 
    _("Auto Bed Leveling (G29)"));
m_checkbox_flow_calibration = new wxCheckBox(this, wxID_ANY, 
    _("Flow Calibration"));
m_checkbox_mech_mode = new wxCheckBox(this, wxID_ANY, 
    _("Mech Mode / Resonance Testing"));

// Add to sizer (layout)
sizer->Add(m_checkbox_bed_leveling, 0, wxALL, 5);
sizer->Add(m_checkbox_flow_calibration, 0, wxALL, 5);
sizer->Add(m_checkbox_mech_mode, 0, wxALL, 5);
```

#### B. Gcode Processing

**Location**: `src/libslic3r/GCode.cpp` or similar

**What to Add:**
- Logic to inject `M1002 set_flag` commands based on checkbox states
- Flag injection before start gcode
- Integration with existing gcode processing pipeline

```cpp
// Pseudo-code
if (send_dialog->m_checkbox_bed_leveling->GetValue()) {
    gcode += "M1002 set_flag g29_before_print_flag=1\n";
}
if (send_dialog->m_checkbox_flow_calibration->GetValue()) {
    gcode += "M1002 set_flag extrude_cali_flag=1\n";
}
if (send_dialog->m_checkbox_mech_mode->GetValue()) {
    gcode += "M1002 set_flag mech_mode_flag=1\n";
}
```

#### C. Dialog Data Structure

**Location**: Dialog header file (`.h`)

**What to Add:**
- Member variables for checkboxes
- Getter methods to retrieve checkbox states
- Integration with existing dialog data structure

#### D. Internationalization (i18n)

**Location**: `resources/localization/`

**What to Add:**
- Translation strings for checkbox labels
- Support for multiple languages

### 4. Build and Testing Process

**Steps:**
1. Clone OrcaSlicer repository
2. Set up build environment
3. Make code changes
4. Build project (can take 30-60 minutes)
5. Test changes
6. Debug issues
7. Rebuild and retest
8. Repeat until working

**Testing Requirements:**
- Test on Windows (your platform)
- Verify checkboxes appear correctly
- Verify flags are injected into gcode
- Test with different printer profiles
- Ensure no regressions in existing functionality

### 5. Maintenance Burden

**Ongoing Issues:**
- **Version Updates**: OrcaSlicer updates frequently - your changes may break
- **Merge Conflicts**: Upstream changes may conflict with your modifications
- **Rebuilding**: Every OrcaSlicer update requires rebuilding your custom version
- **Distribution**: You'd need to distribute your custom build to use it

## Estimated Effort

| Task | Time Estimate |
|------|---------------|
| Environment Setup | 2-4 hours |
| Finding Relevant Code | 4-8 hours |
| Implementing Changes | 8-16 hours |
| Testing & Debugging | 4-8 hours |
| Documentation | 2-4 hours |
| **Total** | **20-40+ hours** |

**For Experienced C++/wxWidgets Developer**: 20-30 hours
**For Developer New to Codebase**: 40-60+ hours

## Alternative Solutions (Recommended)

### Option 1: Post-Processing Script (Current Approach)
- ✅ **Pros**: No source code modification, works immediately
- ✅ **Pros**: Easy to update/maintain
- ✅ **Pros**: Portable across OrcaSlicer versions
- ❌ **Cons**: Runs during slicing, not in "Send to Printer" popup

### Option 2: Manual Flag Control in Start Gcode
- ✅ **Pros**: Simplest solution, no scripts needed
- ✅ **Pros**: Full control over flags
- ❌ **Cons**: Requires manual editing

### Option 3: OrcaSlicer Plugin System (If Available)
- ✅ **Pros**: Official extension mechanism
- ❌ **Cons**: May not exist or may be limited

### Option 4: Feature Request to OrcaSlicer
- ✅ **Pros**: Official implementation if accepted
- ✅ **Pros**: Benefits entire community
- ❌ **Cons**: No guarantee of implementation
- ❌ **Cons**: May take months/years

## Recommendation

**Don't modify the source code** unless:
1. You're an experienced C++ developer
2. You're comfortable maintaining a custom fork
3. You have 20-40+ hours to invest
4. The post-processing script approach doesn't meet your needs

**Instead, use the post-processing script approach** - it's:
- Much faster to implement (already done!)
- Easier to maintain
- Works with any OrcaSlicer version
- Can be shared with others easily

## If You Still Want to Proceed

### Getting Started

1. **Clone Repository**:
   ```bash
   git clone https://github.com/SoftFever/OrcaSlicer.git
   cd OrcaSlicer
   ```

2. **Read Build Instructions**:
   - https://github.com/SoftFever/OrcaSlicer/wiki/How-to-build

3. **Search for Dialog Code**:
   ```bash
   # Search for "Send to Printer" or similar
   grep -r "Send.*Printer" src/
   grep -r "send.*printer" src/ -i
   ```

4. **Find Dialog Class**:
   - Look in `src/gui/` directory
   - Search for dialog creation code
   - Find where "Send" button is handled

5. **Make Changes**:
   - Add checkbox controls
   - Add flag injection logic
   - Test thoroughly

6. **Build and Test**:
   - Follow build instructions
   - Test your changes
   - Debug issues

### Resources

- **OrcaSlicer GitHub**: https://github.com/SoftFever/OrcaSlicer
- **OrcaSlicer Wiki**: https://github.com/SoftFever/OrcaSlicer/wiki
- **wxWidgets Documentation**: https://docs.wxwidgets.org/
- **PrusaSlicer (Parent Project)**: https://github.com/prusa3d/PrusaSlicer

## Conclusion

Modifying OrcaSlicer source code is **technically feasible** but **not recommended** for this use case. The post-processing script approach provides 90% of the functionality with 5% of the effort.

Consider modifying source code only if:
- You need the checkboxes in the exact "Send to Printer" popup
- You're willing to maintain a custom fork
- You have significant C++ development experience
- The time investment is acceptable

Otherwise, stick with the post-processing script - it's the pragmatic solution.
