# Bambu A1 Tuning: ASA Elephant Foot
**Enclosure Temperature:** ~43°C
**Slicer:** Orca Slicer

Elephant foot (bulging at the base) is exaggerated in an enclosure because the heat from the 100°C bed radiates into the 43°C air, keeping the bottom layers of ASA in a semi-molten state while the weight of the print presses down.

## 1. Slicer Compensation
The most direct fix in Orca Slicer.
*   **Elephant foot compensation:** `0.15mm` to `0.25mm`.
    *   *Where:* Process -> Quality -> Advanced -> Elephant foot compensation.
    *   *Effect:* Shrinks the first layer slightly to account for the physical "squish."

## 2. Temperature Management
*   **First Layer Bed Temp:** 100°C (For adhesion).
*   **Other Layers Bed Temp:** 90°C - 95°C.
    *   *Why:* Once the first few layers are down, dropping the bed temp slightly prevents the "heat soak" from keeping the base too soft.

## 3. First Layer Flow & Speed
*   **First Layer Flow:** 0.95 (95%).
    *   Reducing the amount of plastic on the first layer gives it room to spread without bulging.
*   **First Layer Speed:** 20 mm/s.
    *   Printing the base slowly ensures it doesn't get dragged or distorted while soft.

## 4. Cooling (First Layer)
*   **Initial Layer Fan:** 0% (Keep off for the first 3 layers).
*   **Regular Fan Speed:** If the bulging persists, enable a tiny amount of fan (10%) starting at layer 4 to help the transition area solidify.
