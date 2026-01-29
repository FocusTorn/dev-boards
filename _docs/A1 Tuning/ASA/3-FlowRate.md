# Step 2: Flow Rate (Extrusion Multiplier)
**Goal:** Match the actual width of a printed line to the width requested by the slicer.

## Procedure (Orca Slicer)
1.  Go to **Calibration** -> **Flow Rate**.
2.  **Pass 1:** Prints 9 chips. Pick the smoothest one (no gaps, no "plowing" zits).
3.  Update the **Flow Ratio** in your profile based on the chip value (e.g., if +5 is best, new flow = current flow * 1.05).
4.  **Pass 2:** (Optional but recommended) Run Pass 2 for finer adjustment.

## Why this matters for ASA
ASA shrinks. If your flow rate is too low, the shrinkage will pull layers apart. If it's too high, you'll get excessive "zits" that catch the nozzle and cause layer shifts.

## Target
Look for a surface that feels like a "smooth sheet of paper" when you run your fingernail across the top layers.
