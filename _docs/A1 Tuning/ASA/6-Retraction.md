# Step 4: Retraction
**Goal:** Prevent stringing between parts while maintaining the health of the A1 extruder.

## Settings for ASA
ASA is "goopier" than PLA. In a 43°C enclosure, the filament is already pre-softened, so retraction must be precise.

## Procedure
1.  Go to **Calibration** -> **Retraction Test**.
2.  **Distance Range:** 0.4mm to 1.2mm (Step: 0.1).
3.  **Speed:** 30 mm/s to 45 mm/s.

## The "Heat Creep" Risk
If your retraction distance is too high (e.g., >2.0mm), you risk pulling molten ASA into the cold part of the extruder. Inside an enclosure, this cold part isn't very cold, and you will get a permanent jam.

## Target
Find the *lowest* distance that gives no strings. For the A1, this is usually **0.8mm**.
