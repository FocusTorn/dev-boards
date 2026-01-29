# Step 1: Max Volumetric Flow
**Goal:** Determine how fast the A1 can melt this specific ASA before it under-extrudes.

## Why this matters
The A1's hotend has a physical limit. If you try to print at 200mm/s but your volumetric flow is only 8mm³/s, you will get "matte" patches, gaps, and weak parts.

## Procedure (Orca Slicer)
1.  Go to **Calibration** -> **Max Flow Rate**.
2.  **Start:** 5 mm³/s | **End:** 20 mm³/s | **Step:** 0.5.
3.  Print the test.
4.  Examine the surface. Look for where the finish goes from "Shiny" to "Matte" or where gaps appear.
5.  Measure the height of the failure and use the provided formula in the slicer to find your Max Flow.

## Typical ASA Values for A1
*   **Standard ASA:** 10-12 mm³/s
*   **High-Speed ASA:** 15-18 mm³/s

## Update Profile
*   **Filament Settings** -> **Volumetric speed limit** -> [Your Result]
