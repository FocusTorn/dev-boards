# ASA "Showcase" Tuning Roadmap: Bambu A1
**Environment:** 43°C Enclosure | **Slicer:** Orca Slicer | **Goal:** Injection-Molded Quality

This directory contains the comprehensive "Showcase" sequence for tuning ASA. For specific brand calibrations (like Overture), see the sub-directories.

## The Showcase Sequence

### Phase 0: Foundations
*   [ ] **0. [Best Practices](0-BestPractices.md):** Mechanical prep, Drying, and Methodology.

### Phase 1: Material & Machine Limits
*   [ ] **1. [Volumetric Flow](1-VolumetricFlow.md):** Find the mechanical limit of the hotend.
*   [ ] **2. [Motion & Acceleration](2-Motion.md):** **CRITICAL.** Set "Showcase" limits (2000-3000 mm/s²).
*   [ ] **3. [Temperature](3-Temperature.md):** Find the "Sweet Spot" for welding vs. stringing.

### Phase 2: Precision Calibration (Perform at Showcase Speeds)
*   [ ] **4. [Flow Rate](4-FlowRate.md):** Fine-tune line width and surface smoothness.
*   [ ] **5. [Pressure Advance](5-PressureAdvance.md):** Sharpen corners and eliminate transition gaps.
*   [ ] **6. [Retraction](6-Retraction.md):** Eliminate stringing while avoiding heat creep.
*   [ ] **7. [VFA (Vertical Fine Artifacts)](7-VFA.md):** identify cleaner wall speeds.

### Phase 3: Feature & Surface Finishing
*   [ ] **8. [Overhangs & Cooling](8-Overhangs.md):** Targeted cooling bursts for steep angles.
*   [ ] **9. [Elephant Foot](9-ElephantFoot.md):** Compensation for high bed-temp squish.
*   [ ] **10. [Ironing](10-Ironing.md):** The final "Mirror Finish" on top surfaces.
*   [ ] **11. [Dimensional Accuracy](11-DimensionalAccuracy.md):** Calibrate shrinkage and XY tolerances.

### Phase 4: Automation & Mapping
*   [ ] **12. [Advanced MQTT Warden](12-Advanced-MQTT-Warden.md):** Using a "Temperature Trap" to automate heatsoak.
*   [ ] **13. [AMS Custom Mapping](13-AMS-Custom-Filament.md):** Pushing custom brands to the A1 firmware.

---

## Brand-Specific Overrides
*   📂 **[Overture ASA](Overture/Overrides.md):** Calibrated values for Overture Brand.

---

## Pro-Tips for Showcase Quality
*   **Methodology:** Calibrate one value at a time.
*   **Cooling:** Always wait 10 mins for the part to shrink before measuring.
*   **Speed:** Cap **Outer Wall Speed** at your VFA "Clean Zone" speed.
*   **Precise Wall:** Keep this enabled in Orca Slicer.
*   **Thermal Stability:** Heatsoak to at least 40°C before starting the mesh leveling.
