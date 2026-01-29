# Step 7: Advanced Motion (Acceleration & Jerk)
**Goal:** Reach "Professional Grade" surface finish by reducing machine-induced artifacts.

## Why this matters for ASA
ASA is a "stiff" plastic with high thermal stress. High-speed direction changes (Jerk/Acceleration) can cause "ringing" (ghosting) that is more visible on ASA than on matte PLA.

## 1. Acceleration Tuning
The A1 defaults are aggressive (often 5000+ mm/s²). For high-quality ASA:
*   **Outer Wall Acceleration:** Reduce to **2000 - 3000 mm/s²**.
    *   *Effect:* Smoother surface, eliminates the "echo" of sharp corners.
*   **Inner Wall Acceleration:** Reduce to **4000 mm/s²**.
*   **Top Surface Acceleration:** Reduce to **2000 mm/s²**.

## 2. Jerk (Junction Deviation)
Modern Bambu firmware uses Input Shaping, which handles most "Jerk" issues. However:
*   **Travel Jerk:** If you hear the printer "thump" during travel, reduce Travel Jerk by 20%.
*   **Inner Wall Jerk:** Keep low (7-9 mm/s) to ensure the inner wall is a stable foundation for the outer wall.

## 3. Best Practices (The "Pro" Tips)
*   **Slow Down for Quality:** For "Showcase" parts, set your **Outer Wall Speed** to 60 mm/s, regardless of what the "Max Volumetric Speed" allows.
*   **Precise Wall:** Always enable in Orca Slicer. It improves the mathematical consistency of the toolpath.
*   **Vibration Calibration:** Every time you move the A1 or change the surface it sits on, re-run the **Vibration Compensation** from the printer menu. ASA is very sensitive to resonance.
