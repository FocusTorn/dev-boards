# Bambu A1 3D Printer Complete Calibration Guide - Executive Summary

This guide assumes OrcaSlicer is being used.

---

## Testing Methodology: One Value at a Time

**CRITICAL**: When testing multiple parameter values (e.g., acceleration: 2000, 3000, 4000, 5000 mm/s²), always test **one value at a time** with observation periods between tests.

**✅ CORRECT - Testing Process**:

1. **Print First Test Value**: Print the test model with the first parameter value (e.g., acceleration = 2000 mm/s²)
2. **Wait for Completion**: Let the print finish completely
3. **Observation Period**: Wait 5-10 minutes after print completion to:
   - Allow print to cool (especially important for materials like ASA/ABS)
   - Visually inspect the print thoroughly
   - Take measurements if needed
   - Document results (take photos, note observations)
4. **Analyze Results**: Determine if this value is acceptable or needs adjustment
5. **Print Next Test Value**: Only after completing observation, print the next test value (e.g., acceleration = 3000 mm/s²)
6. **Repeat**: Continue this process for all test values

**❌ INCORRECT - Testing Multiple Values Simultaneously**:

- Printing all test values in one multi-part print without observation periods
- Starting the next test immediately after the previous one finishes
- Not allowing cooling time for materials that need it
- Not documenting results before moving to the next value

**Why This Matters**:

- **Cooling Time**: Materials like ASA/ABS need time to cool and stabilize before accurate measurement
- **Clear Attribution**: You can clearly identify which parameter value produced which result
- **Prevent Confusion**: Avoids mixing up results from different test values
- **Better Decision Making**: Observation periods allow you to make informed decisions about which value worked best
- **Documentation**: Time between tests allows proper documentation of results

**Note**: Some OrcaSlicer built-in tests (like Temperature Tower, Max Flow Rate) automatically print multiple values in one model. For these, still allow an observation period after the complete test finishes before analyzing results.

---

## Phase 1: Mechanical & On-Machine Calibration (Do First, Do Infrequently)

1. Mechanical Inspection <!-- Start Fold -->
   - Manually check and tension belts (refer to Bambu Lab Wiki for frequency tuning if desired)
   - Verify build plate is clean and free of oils or dust

<!-- Close Fold -->

2. On-Machine Calibration Suite <!-- Start Fold -->
   - Navigate to Settings > Maintenance > Calibration on A1 screen
   - Run full suite: Vibration Compensation (Input Shaper to prevent ghosting) and Motor Noise Cancellation (for quieter operation)

<!-- Close Fold -->

3. Manual Bed Leveling/Z-Offset <!-- Start Fold -->
   - While A1 has auto-leveling, ensure good starting point
   - Manually adjust bed height using paper test or feeler gauge if needed

<!-- Close Fold -->

4. First Layer Height/Squish Calibration <!-- Start Fold -->
   - Print first layer test pattern (single layer square/circle)
   - Adjust Z-offset to achieve proper squish (lines merge without gaps, no elephant's foot)
   - Action: Fine-tune Z-offset in 0.01mm increments until perfect first layer

<!-- Close Fold -->

5. Brim/Skirt/Raft Settings <!-- Start Fold -->
   - Test different adhesion helpers for warping-prone materials
   - Brim: For edge adhesion (ASA/ABS, typically 3-5mm width)
   - Skirt: For priming nozzle
   - Raft: For difficult prints
   - Action: Determine optimal brim width or when to use raft based on material and print geometry

<!-- Close Fold -->

---

## Phase 2: Material & Flow Calibration (Do Per New Spool)

1. Temperature Tower <!-- Start Fold -->
   - **How to Test:**
     - Open OrcaSlicer → Calibration tab → Temperature Tower
     - Set temperature range (typically 240-260°C for ASA, 190-220°C for PLA)
     - Set increment (5°C recommended)
     - **Important:** Before generating, set a conservative Max Volumetric Speed in Filament Settings:
       - **PLA**: 12-15 mm³/s (conservative default)
       - **ASA/ABS**: 10-12 mm³/s (conservative default)
       - **PETG**: 10-12 mm³/s (conservative default)
     - This prevents under-extrusion that could be mistaken for temperature issues
     - You'll calibrate max flow properly after determining optimal temperature
     - Generate and print tower (30-60 minutes)
   - **What to Look For:**
     - Surface quality: Smoothest surface texture (no blobs, zits, or roughness)
     - Layer adhesion: Try to break tower - strongest section indicates best adhesion
     - Stringing: Least stringing between tower sections
     - Bridging: Best bridging quality (if tower has bridges)
     - Overhangs: Sharpest overhang angles without drooping
   - **How to Judge:**
     - Best temperature balances all factors: smooth surface + strong adhesion + minimal stringing
     - If multiple sections look good, choose middle temperature for safety margin
     - Avoid temperatures with visible defects (blobs, stringing, poor adhesion)
     - **Note:** If you see consistent under-extrusion across all temperatures, the max flow rate may be too low - increase it slightly and re-test, or proceed to max flow calibration after selecting temperature
   - **Action:** Lock in best temperature in filament profile

<!-- Close Fold -->

2. Max Volumetric Flow Rate <!-- Start Fold -->
   - **How to Test:**
     - Open OrcaSlicer → Calibration tab → Max Flowrate test
     - When prompted for "Step File Import Parameters":
       - **Linear Deflection**: Default 0.003 mm (range: 0.001-0.1 mm)
         - Default is excellent for calibration tests (lower = more accurate mesh)
         - Can use default or go slightly lower (0.001-0.003 mm) for maximum accuracy
       - **Angular Deflection**: Default 0.5° (range: 0.01-1.0°)
         - Default is excellent for calibration tests (lower = more accurate mesh)
         - Can use default or go slightly lower (0.01-0.5°) for maximum accuracy
       - **Recommendation**: Use defaults (0.003 mm and 0.5°) - they provide excellent accuracy for calibration tests
       - Lower values = more detail but larger file size (defaults are already well-optimized)
     - Test will print increasing flow rates (mm³/s) at different heights
     - Print completes in 20-40 minutes
   - **What to Look For:**
     - Under-extrusion: Gaps, missing lines, or thin walls indicate flow rate too high
     - Surface quality: Rough texture or inconsistent extrusion at high flow rates
     - Failure point: Highest flow rate where print still looks acceptable
     - Layer consistency: Consistent layer thickness throughout test
   - **How to Judge:**
     - Find the highest flow rate section that still has good quality (no gaps, consistent walls)
     - Set max volumetric speed to 80-90% of failure point for safety margin
     - Example: If failure occurs at 20 mm³/s, set to 16-18 mm³/s
     - This value limits all print speeds automatically
   - **Action:** Set Max volumetric speed in filament profile to value slightly below failure point (e.g., 18 mm³/s for Generic PLA)
   - **Critical for overall speed limits**

<!-- Close Fold -->

3. Pressure Advance (PA) / Flow Dynamics <!-- Start Fold -->
   - **How to Test:**
     - Open OrcaSlicer → Calibration tab → Pressure Advance (Line or Tower method)
     - Line method: Prints single line with varying K-values
     - Tower method: Prints tower with K-values changing by height
     - Print completes in 15-30 minutes
   - **What to Look For:**
     - Corner sharpness: Sharp, clean corners without bulging or rounding
     - Under-extrusion: Gaps or thin spots at corners (K-value too high)
     - Over-extrusion: Bulging or rounded corners (K-value too low)
     - Surface quality: Smooth walls without artifacts at corners
   - **How to Judge:**
     - Find K-value where corners are sharpest without gaps
     - If corners bulge: Increase K-value slightly (e.g., 0.02 → 0.025)
     - If corners have gaps: Decrease K-value slightly (e.g., 0.02 → 0.015)
     - Typical range: 0.01-0.05 for most materials
   - **Action:** Enter K-value in Pressure advance field in filament settings

<!-- Close Fold -->

4. Flow Rate Ratio (Extrusion Multiplier) <!-- Start Fold -->
   - **How to Test:**
     - **Recommended**: Use OrcaSlicer's built-in Flow Rate Calibration test
       - Open OrcaSlicer → Calibration tab → Flow Rate Calibration
       - Built-in test automatically handles speed limits to avoid max flow rate capping
     - **Alternative (Custom Test File)**: If using a custom test file instead:
       - **How OrcaSlicer handles MVS**: OrcaSlicer automatically caps print speeds to respect Max Volumetric Speed limits
         - Formula: Max Speed = MVS / (Layer Height × Line Width)
         - Example: MVS 15 mm³/s, 0.2mm layer, 0.4mm width → Max speed ≈ 187 mm/s
         - Slicer automatically reduces speeds if they would exceed MVS
       - **Dynamically Calculate Safe Speeds** (same as OrcaSlicer's built-in test):
         - **Formula**: Use these formulas to automatically calculate speeds based on your current settings:
           - **Base Max Speed** = MVS / (Layer Height × Line Width)
           - **Outer Wall Speed** = Base Max Speed × 0.5
           - **Inner Wall Speed** = Base Max Speed × 0.6
           - **Top/Bottom Speed** = Base Max Speed × 0.5
           - **Infill Speed** = Base Max Speed × 0.7
           - **Small Perimeter Speed** = Base Max Speed × 0.5
           - **First Layer Speed** = Base Max Speed × 0.3 (slower for adhesion)
         - **Example Calculation**:
           - MVS: 15 mm³/s, Layer Height: 0.2mm, Line Width: 0.4mm
           - Base Max Speed = 15 / (0.2 × 0.4) = 187.5 mm/s
           - Outer Wall = 187.5 × 0.5 = **94 mm/s**
           - Inner Wall = 187.5 × 0.6 = **112 mm/s**
           - Top/Bottom = 187.5 × 0.5 = **94 mm/s**
           - Infill = 187.5 × 0.7 = **131 mm/s**
           - Small Perimeter = 187.5 × 0.5 = **94 mm/s**
           - First Layer = 187.5 × 0.3 = **56 mm/s**
         - **Quick Calculator Method**:
           - Get your MVS, Layer Height, and Line Width from OrcaSlicer
           - Calculate: `Base = MVS / (Layer Height × Line Width)`
           - Multiply Base by percentages above to get each speed
           - Or use spreadsheet: `=MVS/(LayerHeight*LineWidth)*0.5` for Outer Wall (change 0.5 to 0.6 for Inner, etc.)
         - **Why 50-70%**: Ensures you're well below MVS limit, testing flow ratio accuracy, not flow limits
         - **Quick Reference**: For typical 0.2mm layer, 0.4mm width:
           - MVS 12 mm³/s → Base 150 mm/s → Outer Wall: 75 mm/s, Inner: 90 mm/s, Infill: 105 mm/s
           - MVS 15 mm³/s → Base 187 mm/s → Outer Wall: 94 mm/s, Inner: 112 mm/s, Infill: 131 mm/s
           - MVS 18 mm³/s → Base 225 mm/s → Outer Wall: 112 mm/s, Inner: 135 mm/s, Infill: 157 mm/s
       - **After calibration**: Restore print speeds and MVS to normal values
     - Print Flow Rate Pass 1 (grid of squares with different flow modifiers)
     - Optionally print Pass 2 for fine-tuning
     - Print on smooth surface plate (PEI/textured plate)
   - **What to Look For:**
     - Top surface smoothness: Smoothest top surface without gaps or blobs
     - Gap detection: Gaps between perimeters indicate under-extrusion (flow too low)
     - Blob detection: Raised lines or blobs indicate over-extrusion (flow too high)
     - Layer consistency: Uniform layer thickness across entire square
   - **How to Judge:**
     - Select square with smoothest top surface (no gaps, no blobs, uniform texture)
     - Note the modifier value on that square (e.g., -5%, 0%, +5%)
     - Calculate: New Flow = Old Flow × (100 + modifier) / 100
     - Example: If 100% flow with -3% modifier looks best: New Flow = 100 × (100 - 3) / 100 = 97%
     - If Pass 1 results are close, run Pass 2 for finer increments (±1%)
   - **Action:** Calculate new flow ratio using formula: New Flow = Old Flow * (100 + modifier) / 100 and save in filament profile

<!-- Close Fold -->

5. Retraction Tower <!-- Start Fold -->
   - **How to Test:**
     - Open OrcaSlicer → Calibration tab → Retraction test
     - Test prints tower with varying retraction distances (and optionally speeds)
     - Print completes in 20-40 minutes
   - **What to Look For:**
     - Stringing: Thin filament strings between tower sections
     - Oozing: Blobs or material buildup at layer changes
     - Clean sections: Sections with no stringing or oozing
     - Surface quality: Clean surface without artifacts from retraction
   - **How to Judge:**
     - Find lowest retraction distance that eliminates stringing
     - If stringing persists: Increase retraction distance (e.g., 0.5mm → 0.8mm)
     - If no stringing at low values: Use lowest value to minimize risk of clogs
     - Typical range: 0.5-1.0mm for direct drive (A1), 3-7mm for Bowden
     - For retraction speed: Start with 30-50 mm/s, adjust if needed
   - **Action:** Adjust retraction distance and speed in filament profile until stringing disappears

<!-- Close Fold -->

6. Nozzle Wipe/Prime Settings <!-- Start Fold -->
   - **How to Test:**
     - Print test model with multiple perimeters and infill sections
     - Test different wipe distances: 0mm, 0.2mm, 0.5mm, 1.0mm
     - Test different wipe patterns: before outer wall, before infill, both
     - Print same model multiple times with different settings
   - **What to Look For:**
     - Oozing: Material oozing at start of new sections
     - Blobs: Small blobs at layer start points
     - Clean starts: Clean layer starts without artifacts
     - Surface quality: Smooth outer walls without defects
   - **How to Judge:**
     - Find wipe distance that eliminates oozing without causing under-extrusion
     - If oozing occurs: Increase wipe distance (e.g., 0.2mm → 0.5mm)
     - If gaps appear: Decrease wipe distance or increase prime amount
     - Test "wipe before outer wall" for best surface quality
     - Typical range: 0.2-0.5mm wipe distance
   - **Action:** Set optimal wipe distance (typically 0.2-0.5mm) and prime amount in Print Settings

<!-- Close Fold -->

7. Travel Speed & Acceleration <!-- Start Fold -->
   - **How to Test:**
     - Print test model with many travel moves (e.g., stringing test, multiple small parts)
     - Test different travel speeds: 200, 300, 400, 500 mm/s
     - Test different travel accelerations: 1000, 2000, 3000, 5000 mm/s²
     - Print same model multiple times with different settings
   - **What to Look For:**
     - Stringing: Filament strings between parts during travel moves
     - Layer shifts: Misaligned layers from excessive acceleration
     - Print time: Faster travel = shorter print times
     - Artifacts: Defects caused by too-fast travel moves
   - **How to Judge:**
     - Find highest travel speed without stringing or artifacts
     - If stringing occurs: Increase travel speed (faster = less time for oozing)
     - If layer shifts occur: Reduce travel acceleration
     - Balance: Maximum speed without quality degradation
     - Typical range: 200-500 mm/s travel speed, 2000-5000 mm/s² acceleration
   - **Action:** Set travel speed and acceleration in Print Settings

<!-- Close Fold -->

8. Combing Mode <!-- Start Fold -->
   - **How to Test:**
     - Print stringing test model (multiple small towers or parts)
     - Test each combing mode: Off, All, Within Infill, Not in Skin
     - Print same model with each setting
   - **What to Look For:**
     - Stringing: Filament strings between parts
     - Surface marks: Scratches or marks from combing over printed areas
     - Print quality: Overall surface quality of printed parts
     - Travel paths: Visualize travel paths in slicer preview
   - **How to Judge:**
     - "All": Best stringing reduction but may mark surfaces
     - "Within Infill": Good stringing reduction, avoids visible surfaces
     - "Not in Skin": Prevents surface marks, moderate stringing reduction
     - "Off": Fastest but most stringing
     - Choose mode that eliminates stringing without surface marks
     - For visible parts: Use "Not in Skin" or "Within Infill"
     - For functional parts: "All" is acceptable
   - **Action:** Enable optimal combing mode based on stringing test results in Print Settings → Travel

<!-- Close Fold -->

9. Filament Diameter Verification <!-- Start Fold -->
   - **How to Test:**
     - Use digital calipers to measure filament diameter
     - Measure at 10+ points along filament (every 50-100cm)
     - Measure at different angles (rotate filament 90° between measurements)
     - Record all measurements and calculate average
   - **What to Look For:**
     - Diameter consistency: Variation should be ±0.05mm or less
     - Average diameter: Should be close to 1.75mm (or 2.85mm for 3mm filament)
     - Ovality: Filament should be round, not oval
     - Out-of-spec: Diameter outside 1.70-1.80mm range (for 1.75mm filament)
   - **How to Judge:**
     - Calculate average of all measurements
     - If average differs from profile value by >0.02mm: Update profile
     - Example: If measured average is 1.73mm but profile says 1.75mm, update to 1.73mm
     - If variation is >0.1mm: Consider different spool (poor quality control)
     - Accurate diameter is critical for flow calculations
   - **Action:** Set actual diameter in Filament Settings (affects flow calculations)

<!-- Close Fold -->

10. Nozzle Size Verification <!-- Start Fold -->
    - **How to Test:**
      - Check printer profile in OrcaSlicer: Printer Settings → Nozzle Diameter
      - Physically verify installed nozzle (check printer or nozzle package)
      - Print single-wall calibration cube and measure wall thickness
      - Wall thickness should equal nozzle diameter (0.4mm nozzle = 0.4mm wall)
    - **What to Look For:**
      - Profile mismatch: Profile says 0.4mm but 0.6mm nozzle installed
      - Wall thickness: Measured wall thickness matches nozzle diameter
      - Extrusion width: Should be 100-120% of nozzle diameter
      - Print quality: Incorrect nozzle size causes poor quality
    - **How to Judge:**
      - If wall thickness ≠ nozzle diameter: Profile nozzle size is wrong
      - If wall is too thick: Profile nozzle size is larger than actual
      - If wall is too thin: Profile nozzle size is smaller than actual
      - Update profile to match actual installed nozzle
      - Critical: Wrong nozzle size causes all flow calculations to be incorrect
    - **Action:** Ensure nozzle diameter matches actual installed nozzle in Printer Settings

<!-- Close Fold -->

---

## Phase 3: Advanced Geometry & Speed Tuning (Fine-Tuning)

1. Acceleration <!-- Start Fold -->
   - **How to Test:**
     - **Note**: OrcaSlicer does not have a separate built-in acceleration test
     - **Method**: Use the L-shaped model from Junction Deviation test (OrcaSlicer → Calibration tab → Cornering test), but keep Junction Deviation constant and vary acceleration values
     - Print the same model multiple times at different acceleration values (2000, 3000, 4000, 5000 mm/s²)
     - Set acceleration in Print Settings → Speed → Acceleration Control for each test print
     - Observe for ringing/ghosting on the vertical walls at each acceleration level
     - The L-shaped model is ideal because it has vertical walls that show ringing artifacts clearly
     - A1's Input Shaping (vibration compensation) helps reduce ringing, but acceleration limits should still be tested
     - **Alternative**: Use Vibration/VFA test (item 4) which indirectly tests acceleration effects through speed variation
   - **What to Look For:**
     - Ringing/Ghosting: Repeating patterns or shadows on vertical surfaces
     - Layer shifts: Misaligned layers from excessive acceleration
     - Surface quality: Smoothness of vertical walls
     - Vibration artifacts: Visible vibration patterns on print surface
   - **How to Judge:**
     - Find highest acceleration where ringing is minimal or acceptable
     - If ringing is visible: Decrease acceleration (e.g., 4000 → 3000 mm/s²)
     - If no ringing at low values: Increase acceleration gradually to find maximum
     - Typical range: 2000-5000 mm/s² acceleration
     - With A1's Input Shaping: Higher accelerations may be usable (test to find limits)
     - Balance: Maximum acceleration without quality degradation
   - **Action:** Set optimal acceleration in Print Settings → Speed → Acceleration Control
   - **Note**: Calibrate acceleration BEFORE Junction Deviation/Jerk, as cornering parameters depend on acceleration values

<!-- Close Fold -->

2. Junction Deviation (Marlin 2) or Classic Jerk (Legacy Firmware) <!-- Start Fold -->
   - **How to Test:**
     - **For Marlin 2 (Junction Deviation)**: Open OrcaSlicer → Calibration tab → Cornering test
       - This tests Junction Deviation (cornering behavior)
       - Tests different Junction Deviation values to find optimal corner sharpness
       - Uses an L-shaped model with corners to test cornering quality
       - Print completes in 15-30 minutes
     - **For Legacy Firmware (Classic Jerk)**: Same test, but firmware uses Classic Jerk instead
       - OrcaSlicer will use Classic Jerk if Junction Deviation is disabled in firmware
       - Test method is the same, but parameter name differs
   - **What to Look For:**
     - Corner quality: Sharpness of corners (bulging or rounding indicates issues)
     - Corner bulging: Corners bulge outward (Junction Deviation/Jerk too low)
     - Corner gaps: Gaps or under-extrusion at corners (Junction Deviation/Jerk too high)
     - **Wavy walls: Undulating/wavy pattern on vertical walls (like a sheet hanging in a breeze) - Junction Deviation/Jerk too HIGH**
     - Surface quality: Smoothness of corner transitions
     - Over-extrusion at corners: Material buildup at corner points
   - **How to Judge:**
     - Find value where corners are sharpest without bulging, gaps, or wavy walls
     - If corners bulge: Increase Junction Deviation (e.g., 0.02 → 0.03) or increase Jerk (e.g., 8 → 10 mm/s)
     - If corners have gaps: Decrease Junction Deviation (e.g., 0.02 → 0.015) or decrease Jerk (e.g., 8 → 6 mm/s)
     - **If walls are wavy/undulating: Junction Deviation/Jerk is too HIGH - decrease Junction Deviation (e.g., 0.02 → 0.015 → 0.01) or decrease Jerk (e.g., 8 → 6 → 4 mm/s)**
     - Typical range: 0.01-0.05 Junction Deviation (Marlin 2), or 1-20 mm/s Classic Jerk (Legacy)
     - Best value: Sharp corners without defects, smooth walls without waviness
   - **Action:** Set Junction Deviation in Print Settings → Speed → Junction Deviation (or Advanced → Junction Deviation), or Classic Jerk in Print Settings → Speed → Jerk XY (for legacy firmware)
   - **Note**: Calibrate AFTER acceleration, as Junction Deviation/Jerk calculations depend on acceleration values

<!-- Close Fold -->

3. VFA (Vertical Fine Artifacts) Tower <!-- Start Fold -->
   - **How to Test:**
     - Open OrcaSlicer → Calibration tab → VFA test
     - Test prints tower with varying outer wall speeds at different heights
     - Print completes in 20-40 minutes
   - **What to Look For:**
     - Vertical lines: Fine vertical lines on print surface caused by motor harmonics
     - Clean zones: Speed ranges where vertical lines are minimal or absent
     - Surface quality: Smoothness of outer walls at different speeds
   - **How to Judge:**
     - Identify speed ranges (clean zones) where vertical lines are minimal
     - Avoid speeds where vertical lines are prominent
     - Choose a speed within the clean zone for outer walls
     - Typical clean zones: Often between 40-80 mm/s or 120-160 mm/s (varies by printer)
   - **Action:** Set Outer Wall Speed in Print Settings → Speed tab to VFA "clean zone" speed

<!-- Close Fold -->

4. XY Dimensional Accuracy & Skew <!-- Start Fold -->
   - **How to Test:**
     - Open OrcaSlicer → Calibration tab → XY Size Compensation (or download CaliFlower/20mm cube from Printables)
     - Print dimensional test object (like CaliFlower or 20mm cube)
     - Measure with calipers: X, Y, Z dimensions
     - **Important**: Test AFTER acceleration and junction deviation are calibrated, as cornering settings affect dimensional accuracy
   - **What to Look For:**
     - Dimension errors: Measured dimensions differ from designed dimensions
     - Skew: Parts appear rotated or parallelogram-shaped instead of square
     - Consistency: Same error across multiple prints
   - **How to Judge:**
     - If dimensions are consistently off: Use XY compensation in OrcaSlicer
     - Calculate compensation: (Designed - Measured) / Designed × 100
     - Example: 20mm designed, 19.8mm measured = (20-19.8)/20 × 100 = 1% compensation needed
     - For skew: Use OrcaSlicer's Skew Calibration feature to measure and input compensation values
   - **Action:** If dimensions are off, use Skew Calibration feature in OrcaSlicer (Calibration tab → Skew Calibration) to measure and input compensation values into machine G-code
   - **Note**: Calibrate AFTER acceleration and junction deviation, as cornering behavior affects how edges and corners are printed, which impacts dimensional measurements

<!-- Close Fold -->

5. Infill/Wall Overlap <!-- Start Fold -->
   - **How to Test:**
     - Print test piece with visible infill (low infill density, 10-15%)
     - Inspect transition from infill to walls in sliced preview and printed part
     - Test different overlap percentages: 5%, 10%, 15%, 20%
   - **What to Look For:**
     - Gaps: Visible gaps between infill and walls (overlap too low)
     - Bulging: Infill lines visible through walls or bulging (overlap too high)
     - Clean transition: Smooth transition from infill to walls
   - **How to Judge:**
     - Find overlap percentage where infill and walls bond without gaps or bulging
     - If gaps visible: Increase overlap (e.g., 10% → 15%)
     - If bulging visible: Decrease overlap (e.g., 15% → 10%)
     - Typical range: 10-15% for most materials
   - **Action:** Adjust Infill/wall overlap percentage in Print Settings → Strength tab (usually 10-15%) to prevent gaps or bulging internal lines

<!-- Close Fold -->

6. Cooling & Overhangs <!-- Start Fold -->
   - **How to Test:**
     - Open OrcaSlicer → Calibration tab → Overhang test (or Bridging test)
     - Print Overhang and Bridging test with varying fan speeds
     - Test different fan speeds: 0%, 25%, 50%, 75%, 100%
   - **What to Look For:**
     - Overhang quality: Sharpest overhang angles without drooping
     - Bridging quality: Longest bridges without sagging
     - Layer separation: Layers separating due to too much cooling
     - Surface quality: Smooth surfaces without warping
   - **How to Judge:**
     - Find fan speed that produces best overhangs without layer separation
     - For ASA/ABS: Lower cooling (0-20%) to prevent warping
     - For PLA: Higher cooling (50-100%) for sharp overhangs
     - Balance: Maximum cooling without compromising layer adhesion
   - **Action:** Tune part cooling fan speeds (especially Aux fan in Bambu A1) in Filament Settings → Cooling tab to get sharp angles without layer separation

<!-- Close Fold -->

7. Seam Placement & Alignment <!-- Start Fold -->
   - **How to Test:**
     - Print test cube (20mm cube or calibration cube)
     - Test different seam positions: Aligned, Random, Nearest, Rear
     - Print same model with each seam position setting
   - **What to Look For:**
     - Seam visibility: How visible the Z-seam is on the print
     - Surface quality: Impact of seam position on overall surface appearance
     - Part geometry: How seam position affects different part shapes
   - **How to Judge:**
     - "Aligned": Seam in same location each layer (visible line, but predictable)
     - "Random": Seam in different location each layer (less visible, but rougher surface)
     - "Nearest": Seam at nearest point to previous layer (good for round parts)
     - "Rear": Seam at back of part (hides seam, good for visible parts)
     - Choose based on part visibility and geometry
   - **Action:** Set optimal seam position in Print Settings → Layers → Seam position

<!-- Close Fold -->

8. Layer Height Optimization <!-- Start Fold -->
   - **How to Test:**
     - Print same model with different layer heights: 0.1mm, 0.2mm, 0.3mm
     - Compare print time, quality, and surface smoothness
     - Test different layer heights for different features (walls, top/bottom, infill)
   - **What to Look For:**
     - Surface smoothness: Smoother surfaces with smaller layer heights
     - Print time: Faster prints with larger layer heights
     - Detail quality: Fine details visible with smaller layer heights
     - Layer lines: Visibility of layer lines on surfaces
   - **How to Judge:**
     - Balance between speed and quality for your needs
     - Fine details: Use 0.1mm layer height
     - Standard quality: Use 0.2mm layer height (good balance)
     - Fast prints: Use 0.3mm layer height (faster, but less detail)
     - Can use different heights for different features (e.g., 0.2mm walls, 0.3mm infill)
   - **Action:** Set layer heights in Print Settings → Quality tab (typically 0.2mm for ASA, 0.1mm for fine details)

<!-- Close Fold -->

9. Top/Bottom Layer Settings <!-- Start Fold -->
   - **How to Test:**
     - Print test piece with visible top/bottom layers
     - Test different top/bottom layer counts: 2, 3, 4, 5 layers
     - Test different solid infill patterns: rectilinear, monotonic, monotonic lines
   - **What to Look For:**
     - Top surface quality: Smoothness and appearance of top surface
     - Bottom surface quality: Adhesion and appearance of bottom surface
     - Infill visibility: Whether infill shows through top/bottom layers
     - Strength: Structural integrity of top/bottom layers
   - **How to Judge:**
     - Find minimum layer count where infill doesn't show through
     - More layers = smoother surface but slower print
     - Typical: 3-5 top layers, 2-4 bottom layers
     - Monotonic pattern: Best for top surface appearance
   - **Action:** Set top/bottom layer count and solid infill pattern in Print Settings → Strength tab

<!-- Close Fold -->

10. Infill Pattern Selection <!-- Start Fold -->
   - **How to Test:**
     - Print test pieces with different infill patterns: Grid, Gyroid, Cubic, Triangles, etc.
     - Test at same infill density (e.g., 20%) for fair comparison
     - Compare print time, strength, and appearance
   - **What to Look For:**
     - Strength: Structural strength of different patterns
     - Print time: Speed of different patterns
     - Material usage: Amount of material used
     - Appearance: Visual appearance when visible
   - **How to Judge:**
     - Grid: Fast, good strength, visible pattern
     - Gyroid: Strong in all directions, smooth pattern, slower
     - Cubic: Very strong, fast, good for functional parts
     - Triangles: Strong, fast, good for structural parts
     - Choose based on part requirements (strength vs. speed vs. appearance)
   - **Action:** Set infill pattern and density based on part requirements in Print Settings → Strength tab

<!-- Close Fold -->

11. Ironing Settings <!-- Start Fold -->
    - **How to Test:**
      - Print test piece with large flat top surface
      - Enable ironing in Print Settings → Quality tab
      - Test different ironing flow rates: 5%, 10%, 15%, 20%
      - Test different ironing speeds: 50, 100, 150 mm/s
    - **What to Look For:**
      - Surface smoothness: Smoothness of top surface
      - Gloss: Glossy appearance from ironing
      - Over-extrusion: Raised lines or blobs from too much flow
      - Under-extrusion: Gaps or rough surface from too little flow
    - **How to Judge:**
      - Find flow rate that produces smoothest surface without defects
      - If surface is rough: Increase flow rate (e.g., 10% → 15%)
      - If raised lines appear: Decrease flow rate (e.g., 15% → 10%)
      - Slower speed = smoother surface but slower print
      - Typical range: 5-15% flow rate, 50-150 mm/s speed
    - **Action:** Enable ironing and set flow rate (typically 5-15%) for top surfaces in Print Settings → Quality tab

<!-- Close Fold -->

12. Bridge Settings <!-- Start Fold -->
    - **How to Test:**
      - Open OrcaSlicer → Calibration tab → Bridging test
      - Test different bridge flow rates: 80%, 90%, 100%, 110%
      - Test different bridge speeds: 20, 30, 40, 50 mm/s
      - Test different bridge fan speeds: 50%, 75%, 100%
    - **What to Look For:**
      - Bridge sagging: How much bridges sag
      - Bridge quality: Smoothness and consistency of bridges
      - Stringing: Filament strings between bridge supports
      - Strength: Structural strength of bridges
    - **How to Judge:**
      - Find flow rate that produces best bridges without sagging
      - Lower flow (80-90%) often works better for bridges
      - Slower speed = better bridges but slower print
      - Higher fan speed = better bridges (100% recommended)
      - Typical: 80-100% flow rate, 20-40 mm/s speed, 100% fan
    - **Action:** Set bridge flow rate (typically 80-100%) and bridge speed in Print Settings → Speed tab

<!-- Close Fold -->

13. Hole/Pin Tolerance Calibration <!-- Start Fold -->
    - **How to Test:**
      - Print tolerance test (download from Printables: "Hole Tolerance Test" or "Pin Tolerance Test")
      - Measure actual vs. designed dimensions with calipers
      - Test different XY compensation values: -0.1mm, 0mm, +0.1mm
    - **What to Look For:**
      - Hole size: Actual hole diameter vs. designed diameter
      - Pin size: Actual pin diameter vs. designed diameter
      - Fit: How well pins fit in holes
      - Consistency: Same error across multiple holes/pins
    - **How to Judge:**
      - If holes are too small: Increase XY compensation (positive value)
      - If holes are too large: Decrease XY compensation (negative value)
      - If pins are too large: Decrease XY compensation
      - If pins are too small: Increase XY compensation
      - Typical range: -0.1mm to +0.1mm compensation
    - **Action:** Apply XY compensation or horizontal expansion in Print Settings → Advanced → XY Size Compensation to achieve correct hole/pin sizes

<!-- Close Fold -->

14. Fan Speed Profiles <!-- Start Fold -->
    - **How to Test:**
      - Print test piece with overhangs, bridges, small perimeters, and external perimeters
      - Test different fan speeds for each feature type
      - Test in Print Settings → Cooling tab
    - **What to Look For:**
      - Overhang quality: Quality of overhangs at different fan speeds
      - Bridge quality: Quality of bridges at different fan speeds
      - Small perimeter quality: Quality of small details at different fan speeds
      - External perimeter quality: Surface quality of outer walls
    - **How to Judge:**
      - Overhangs: Higher fan speed = better overhangs (50-100%)
      - Bridges: Maximum fan speed = best bridges (100%)
      - Small perimeters: Moderate fan speed = good detail (50-75%)
      - External perimeters: Lower fan speed = smoother surface (25-50%)
      - Balance: Maximum cooling where needed without compromising layer adhesion
    - **Action:** Set feature-specific fan speeds in Print Settings → Cooling tab

<!-- Close Fold -->

---

## Phase 4: Support Settings (Critical for Overhangs)

1. Support Pattern & Density <!-- Start Fold -->
   - **How to Test:**
     - Print test model with overhangs (download "Overhang Test" from Printables)
     - Test different support patterns: Grid, Tree, Organic (Snug)
     - Test different support densities: 5%, 10%, 15%, 20%
     - Configure in Print Settings → Support tab
   - **What to Look For:**
     - Support quality: How well supports hold up overhangs
     - Material usage: Amount of support material used
     - Removal difficulty: How easy supports are to remove
     - Surface quality: Quality of supported surfaces
   - **How to Judge:**
     - Grid: Good support, easy to remove, uses more material
     - Tree: Less material, good for complex overhangs, may be harder to remove
     - Organic (Snug): Minimal material, follows part contour, good for complex parts
     - Density: Higher density = better support but more material and harder removal
     - Typical: 10-20% density for most materials
   - **Action:** Set support pattern and density in Print Settings → Support tab

<!-- Close Fold -->

2. Support Interface Settings <!-- Start Fold -->
   - **How to Test:**
     - Print test model with supports
     - Test different interface layer counts: 1, 2, 3, 4 layers
     - Test different interface patterns: Rectilinear, Grid
     - Test different interface spacing: 0mm (solid), 0.2mm, 0.4mm
   - **What to Look For:**
     - Surface quality: Quality of surface where support contacts part
     - Removal difficulty: How easy interface is to remove
     - Support effectiveness: How well interface supports the part
   - **How to Judge:**
     - More layers = better surface but harder removal
     - Solid interface (0mm spacing) = best surface but hardest removal
     - Spaced interface (0.2-0.4mm) = easier removal but may leave marks
     - Typical: 2-3 interface layers, 0.2mm spacing for balance
   - **Action:** Set interface layers (2-3 typical) and pattern for easy removal in Print Settings → Support tab

<!-- Close Fold -->

3. Support Z-Distance <!-- Start Fold -->
   - **How to Test:**
     - Print test model with supports
     - Test different Z-distances: 0.1mm, 0.2mm, 0.3mm, 0.4mm
     - Configure in Print Settings → Support tab → Support on Build Plate Only / Everywhere
   - **What to Look For:**
     - Support gap: Gap between support and part
     - Removal difficulty: How easy supports are to remove
     - Surface quality: Quality of supported surface
     - Support effectiveness: How well supports hold the part
   - **How to Judge:**
     - Too small (0.1mm): Hard to remove, may fuse to part
     - Too large (0.4mm+): Poor support, part may sag
     - Optimal: Balance between support quality and removal ease
     - Typical: 0.2-0.3mm for most materials (0.2mm for ASA/ABS, 0.3mm for PLA)
   - **Action:** Set optimal Z-distance (typically 0.2-0.3mm for ASA) in Print Settings → Support tab

<!-- Close Fold -->

4. Tree Support vs. Regular Support <!-- Start Fold -->
   - **How to Test:**
     - Print same model with Tree Support enabled and disabled
     - Compare material usage, print time, and removal difficulty
     - Configure in Print Settings → Support tab → Support Type
   - **What to Look For:**
     - Material usage: Amount of support material used
     - Print time: Time to print with each support type
     - Removal difficulty: How easy supports are to remove
     - Support quality: How well each type supports overhangs
   - **How to Judge:**
     - Tree Support: Less material, faster prints, good for complex overhangs, may be harder to remove
     - Regular Support: More material, slower prints, easier to remove, good for simple overhangs
     - Use Tree Support for: Complex geometries, minimal material usage
     - Use Regular Support for: Simple overhangs, easy removal priority
   - **Action:** Determine optimal support type based on model geometry in Print Settings → Support tab → Support Type

<!-- Close Fold -->

---

## Phase 5: Advanced Features & Material-Specific

1. Multi-Material/AMS Settings (if using AMS Lite)
   - **How to Test:**
     - Print multi-color test model using AMS Lite
     - Test different purge volumes: 100, 200, 300, 400 mm³
     - Test wipe tower settings: Enable/disable, tower size
     - Configure in OrcaSlicer → Printer Settings → Multi-Material
   - **What to Look For:**
     - Color bleeding: Previous color visible in new color (purge too low)
     - Material waste: Excessive purge material (purge too high)
     - Print quality: Quality of color transitions
     - Wipe tower effectiveness: How well wipe tower cleans nozzle
   - **How to Judge:**
     - Find minimum purge volume that eliminates color bleeding
     - If color bleeding: Increase purge volume (e.g., 200 → 300 mm³)
     - If excessive waste: Decrease purge volume if quality allows
     - Wipe tower: Enable for better color transitions, disable to save material
     - Typical: 200-400 mm³ purge volume depending on material compatibility
   - **Action:** Set purge volumes and wipe tower settings in OrcaSlicer → Printer Settings → Multi-Material

2. Heatsoak Settings (ASA/ABS Specific)
   - **How to Test:**
     - Monitor bed temperature during heatsoak period
     - Test different heatsoak durations: 0min, 5min, 10min, 15min
     - Test different heatsoak temperatures: 90°C, 100°C, 110°C
     - Configure in OrcaSlicer → Printer Settings → Custom G-code → Start G-code
   - **What to Look For:**
     - Temperature stability: Consistent bed temperature across entire bed
     - Heatsoak time: Time for bed to reach stable temperature
     - Warping: Reduction in warping with proper heatsoak
     - Print quality: Quality of first layers with heatsoak
   - **How to Judge:**
     - Longer heatsoak = more stable temperature but longer wait time
     - Higher temperature = better adhesion but longer heatsoak time
     - Find balance: Minimum heatsoak time for stable temperature
     - Typical: 5-10 minutes heatsoak at 100°C for ASA/ABS
   - **Action:** Set heatsoak temperature and duration in OrcaSlicer → Printer Settings → Custom G-code → Start G-code (if using custom start script)

3. PID Tuning (if needed)
   - **How to Test:**
     - Monitor nozzle temperature during prints (use printer display or OrcaSlicer)
     - Check for temperature fluctuations: ±2°C or more indicates instability
     - Run PID autotune if fluctuations detected
     - Configure via printer firmware or OrcaSlicer → Printer Settings → Machine Limits
   - **What to Look For:**
     - Temperature stability: Consistent temperature during prints
     - Temperature fluctuations: Variations in setpoint vs. actual temperature
     - Print quality: Quality issues related to temperature instability
   - **How to Judge:**
     - If temperature fluctuates ±2°C or more: PID tuning needed
     - If temperature is stable: No PID tuning needed (A1 usually auto-tunes)
     - PID autotune: Run via printer menu or G-code command
     - Note: A1 typically auto-tunes PID, manual tuning rarely needed
   - **Action:** Run PID autotune if temperature instability detected (usually not needed on A1) via printer menu or G-code

4. Speed/Quality/Strength Mode Calibration
   - **How to Test:**
     - Print same model in Speed, Quality, and Strength modes
     - Compare print time, surface quality, and structural strength
     - Configure in OrcaSlicer → Print Settings → Mode dropdown
   - **What to Look For:**
     - Print time: Time to complete print in each mode
     - Surface quality: Smoothness and appearance of surfaces
     - Structural strength: Strength of printed parts
     - Layer quality: Quality of individual layers
   - **How to Judge:**
     - Speed mode: Fastest prints, acceptable quality, moderate strength
     - Quality mode: Best surface quality, slower prints, good strength
     - Strength mode: Maximum strength, slower prints, good quality
     - Create custom profiles: Optimize settings for specific use cases
   - **Action:** Create custom print profiles optimized for each use case in OrcaSlicer → Print Settings → Save Profile

5. Surface Mode Settings
   - **How to Test:**
     - Print same model on different build plates: Smooth PEI, Textured PEI, Engineering Plate
     - Test different first layer settings for each surface type
     - Configure in OrcaSlicer → Printer Settings → Build Plate Type
   - **What to Look For:**
     - First layer adhesion: How well first layer sticks to each surface
     - Surface finish: Appearance of bottom surface (smooth vs. textured)
     - Removal difficulty: How easy parts are to remove from each surface
     - Print quality: Overall quality on each surface type
   - **How to Judge:**
     - Smooth PEI: Smooth bottom surface, good adhesion, easy removal
     - Textured PEI: Textured bottom surface, excellent adhesion, easy removal
     - Engineering Plate: Smooth surface, good for high-temp materials, may need glue
     - Adjust first layer settings (speed, temperature, Z-offset) for each surface
   - **Action:** Create surface-specific profiles if using different build plates in OrcaSlicer → Printer Settings → Build Plate Type

---

## Calibration Priority Guide

**Must Do (Every New Spool):**
- Phase 2: Temperature Tower, Max Volumetric Flow, Pressure Advance, Flow Rate Ratio, Retraction Tower

**Should Do (Initial Setup):**
- Phase 1: All mechanical and first layer calibrations
- Phase 2: Travel Speed, Combing Mode, Filament Diameter
- Phase 3: XY Accuracy, Acceleration/Cornering, VFA, Seam Placement

**Fine-Tuning (As Needed):**
- Phase 3: Infill/Wall Overlap, Cooling, Layer Heights, Ironing, Bridge Settings
- Phase 4: All Support Settings
- Phase 5: Advanced features based on use case

**Material-Specific:**
- ASA/ABS: Heatsoak Settings, Brim Settings, Lower Cooling
- PLA: Higher Cooling, Faster Speeds
- PETG: Moderate Cooling, Slower Speeds
