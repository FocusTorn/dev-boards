# ASA "Showcase" Tuning Roadmap: Bambu A1
**Environment:** 43°C Enclosure | **Slicer:** Orca Slicer | **Goal:** Injection-Molded Quality

To achieve "Showcase" quality, we prioritize machine stability **before** fine-tuning material flow. This ensures your Flow and Pressure Advance calibrations are performed at the same stable accelerations used for the final part.

## The Showcase Sequence

### Phase 1: Material & Machine Limits
*   [ ] **1. Volumetric Flow:** Find the mechanical limit of the hotend (The "Never Cross" line).
*   [ ] **2. Motion & Acceleration:** **CRITICAL.** Set your "Showcase" limits (2000-3000 mm/s²) now.

### Phase 2: Precision Calibration (Perform at Showcase Speeds)
*   [ ] **3. Flow Rate:** Fine-tune line width and top-surface smoothness.
*   [ ] **4. Pressure Advance:** Sharpen corners and eliminate transition gaps.
*   [ ] **5. Retraction:** Eliminate stringing while avoiding enclosure-induced heat creep.

### Phase 3: Feature & Surface Finishing
*   [ ] **6. Overhangs & Bridges:** Targeted cooling bursts for steep angles.
*   [ ] **7. Elephant Foot:** Compensation for high bed-temp squish.
### Phase 4: Professional Finishing
*   [ ] **7. Elephant Foot:** Fix the "squish" caused by the 100°C bed and 43°C ambient air.
*   [ ] **8. Ironing:** Achieve a smooth, injection-molded top surface.
*   [ ] **9. Advanced MQTT Warden:** Using a "Temperature Trap" to avoid printer deadlock.

## Pro-Tips for Showcase Quality
*   **Slow Down:** For showcase parts, cap your **Outer Wall Speed** at 60 mm/s and **Small Perimeter Speed** at 25 mm/s.
*   **Inner/Outer Walls:** Always print Inner walls first to provide a foundation for the outer surface.
*   **Vibration Compensation:** Re-run the printer's auto-calibration whenever the enclosure is moved or the table surface changes.