# Step 3: Pressure Advance (PA)
**Goal:** Compensate for the "springiness" of molten ASA inside the hotend.

## The A1 "Auto-Flow" vs. Manual
While the A1 has "Auto-Flow Dynamics," manual calibration is often better for ASA because the enclosure heat can mess with the sensor's baseline.

## Procedure (Orca Slicer)
1.  Go to **Calibration** -> **Pressure Advance**.
2.  **Test Type:** Pattern (preferred for A1) or Tower.
3.  Examine the corners of the printed lines.
    *   **Rounded/Bulging Corner:** PA is too low.
    *   **Gaps/Sharp Thinning:** PA is too high.
4.  Pick the line with the most consistent width across the turn.

## Typical ASA Values
*   **A1 (Direct Drive):** Usually between `0.02` and `0.05`.

## Update Profile
*   **Filament Settings** -> **Enable Pressure Advance** -> [Your Result]
*   **IMPORTANT:** Uncheck "Use Pressure Advance from Printer" in the print dialog to ensure your manual value is used.
