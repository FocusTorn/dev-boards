# Dynamics & Speed Tuning

## Resonance Compensation (Input Shaper)
Mitigates ringing/ghosting caused by vibrations at high speeds.

### Tuning Workflow
1. **Manual Frequency Test:** Print a ringing tower. Measure the distance between peaks (d) and the speed (v). Frequency = v/d.
2. **Shaper Types:** MZV (Standard), EI (Robust), 2HUMP_EI (Aggressive).
3. **Hardware Check:** Always check belt tension before tuning software shaping.

## Cornering Logic
- **Jerk / Square Corner Velocity:** Controls the minimum speed in a corner. Too high = ringing; too low = blobs.
- **Wall Ordering:** Inner/Outer (Better dimensional accuracy) vs Outer/Inner (Better surface finish).
