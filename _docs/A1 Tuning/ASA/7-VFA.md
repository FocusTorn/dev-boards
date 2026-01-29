# Step 7: VFA (Vertical Fine Artifacts)
**Goal:** Identify the "Clean Zone" speeds to eliminate motor harmonics patterns on walls.

## What are VFAs?
VFA appears as fine, vertical ridges on your print walls. They are caused by the resonance of the stepper motors and belts at specific speeds. Every printer has "Noisy" speeds and "Clean" speeds.

## Procedure (Orca Slicer)
1.  Go to **Calibration** -> **VFA Test**.
2.  **Range:** 40 mm/s to 200 mm/s.
3.  Print the tower.
4.  Examine the walls under a harsh, raking light.
5.  Find the speed range where the vertical lines disappear or become minimal.

## Showcase Tuning
*   **Typical Clean Zone:** Often between **60-80 mm/s** (Slow) or **140-160 mm/s** (Fast).
*   **Action:** Set your **Outer Wall Speed** in Orca Slicer to match your most stable "Clean Zone" speed. This ensures your visible surfaces are perfectly smooth.
