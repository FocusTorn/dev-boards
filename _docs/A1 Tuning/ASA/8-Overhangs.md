# Bambu A1 Tuning: ASA Cooling & Overhangs
**Enclosure Temperature:** ~43°C (Passively Heated)
**Slicer:** Orca Slicer

ASA in an enclosure requires a delicate balance between high ambient heat (to prevent warping) and targeted cooling (to "freeze" overhangs and bridges).

## 1. Overhang Speed Overrides
Slow speed is mandatory for ASA when there is no supporting structure. These settings allow the plastic to cool slightly and bond to the previous layer without drooping.

| Overhang Percentage | Speed (mm/s) |
| :--- | :--- |
| 10% - 25% | 60 |
| 25% - 50% | 30 |
| 50% - 75% | 10 |
| 75% - 100% | 5 |

## 2. Advanced Cooling Logic
In a 43°C environment, low fan speeds (10-20%) are ineffective for small features because the air is too warm. Use targeted bursts instead.

*   **Min fan speed:** 0% (Maintain layer adhesion for structural parts)
*   **Max fan speed:** 20-30% (Only for very small layers < 5s)
*   **Overhang fan speed:** 50% - 80% (The "Cold Snap" strategy)
*   **Internal bridges:** 80% (Prevents infill sagging)
*   **Support interface:** 100% (**Mandatory** for easy, scar-free support removal)
*   **Ironing fan speed:** 25% (Prevents the nozzle from "plowing" or burning the surface)

## 3. Wall & Pathing Strategy
*   **Wall Ordering:** `Inner/Outer`
    *   *Why:* Inner walls provide a physical ledge for the outer (overhanging) wall to grab onto.
*   **Precise Wall:** Enabled (Orca Slicer specific).
*   **Layer Height:** 0.16mm or lower. Thinner layers reduce the "step" distance of the overhang.

## 4. Hardware Safety (A1 Specific)
The Bambu A1 is not officially enclosure-rated. At 43°C ambient:
*   The motherboard and PSU (located in the base) are under increased thermal stress.
*   **Recommendation:** For prints exceeding 4 hours, ensure the base of the printer has access to cooler air or slightly crack the enclosure door to prevent electronics overheating.