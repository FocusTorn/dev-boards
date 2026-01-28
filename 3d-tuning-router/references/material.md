# Material Tuning (Extrusion & Thermal)

## Manual Flow Rate (Extrusion Multiplier)
The goal is to find the exact percentage of plastic needed to fill a volume without over-stuffing (blobs) or leaving gaps.

### The "Slab" Test (Manual)
1. Print a 30x30x3mm slab with 100% top infill.
2. Observe the top surface:
   - **Rough/Ridges:** Flow is too high.
   - **Gaps between lines:** Flow is too low.
3. Adjust in 1% increments.

## Pressure Advance (PA) / Linear Advance
Calculates the "springiness" of the filament in the nozzle.

### Tuning Methods
1. **Lines Method:** Printing lines at different speeds to see where the width is most consistent.
2. **Pattern Method (Orca Slicer):** Preferred for high-speed printers. Look for the corner that is neither bulging nor rounded.

## Ironing
Fine-tuning the top surface finish.

### Key Variables
- **Ironing Flow:** Usually 10-20%.
- **Ironing Speed:** 15-30mm/s.
- **Ironing Pattern:** Rectilinear vs concentric (Rectilinear is usually cleaner for large flats).
