# Step 11: Dimensional Accuracy & Skew
**Goal:** Ensure a 100mm part is exactly 100mm.

## Why this matters
ASA shrinks roughly 0.5% to 1% as it cools. If you are printing interlocking parts, they will not fit unless you calibrate for this shrinkage.

## 1. Linear Shrinkage (Shrinkage Factor)
*   **Test:** Print a 100mm "Calibration Cross" or "CaliFlower."
*   **Formula:** `(Expected / Measured) * 100 = Shrinkage Factor %`
*   **Action:** Enter this value in **Filament Settings** -> **Shrinkage Factor**.

## 2. XY Hole/Pin Tolerance
*   **Test:** Print a "Hole Tolerance Test" model.
*   **Action:** Use **XY Size Compensation** (Process -> Quality -> Advanced) to adjust.
    *   If holes are too small: Positive value (e.g., `0.05`).
    *   If pins are too tight: Negative value (e.g., `-0.05`).

## 3. Skew Calibration
If your squares are slightly parallelogram-shaped:
*   Use Orca Slicer's **Skew Calibration** tool.
*   Measure the diagonals of a printed square and enter them into the machine G-code or slicer settings as prompted.
