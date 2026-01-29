# Step 8: Ironing (The "Mirror" Finish)
**Goal:** Eliminate layer lines on flat top surfaces for a injection-molded look.

## Why this matters for ASA
ASA Irons beautifully because it has a broad melting range. However, in an enclosure, it can "heat soak" and turn into a sticky mess if the ironing flow is too high.

## The "Pro" Ironing Logic
*   **Ironing Type:** `Topmost surface only`
*   **Ironing Speed:** `30 - 45 mm/s`
*   **Ironing Flow:** `8 % - 15 %` (Requires brand-specific calibration)
*   **Ironing Spacing:** `0.15 mm`

---
*For brand-specific values, see [Overture Overrides](Overture/Overrides.md).*

## Best Practices & Tips
*   **Ironing Fan Speed:** Set to **20% - 30%** (as discussed in Cooling). Without this, the nozzle will "plow" and leave a yellow/burnt tint on the ASA.
*   **Top Layers:** Ensure you have at least **4 or 5 top layers**. Ironing puts pressure on the surface; if the "roof" is too thin, the nozzle will fall through into the infill.
*   **Monotonic Top Surface:** Always use "Monotonic" or "Monotonic Line" for the layer *under* the ironing pass. This ensures the grain of the plastic all goes the same way before the iron hits it.
*   **One-Way Ironing:** If Orca Slicer allows, use one-way ironing to keep the visual "sheen" consistent across the part.
