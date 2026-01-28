# Orca Slicer & Bambu A1 Manual Workflow

## 1. Disabling Auto-Calibrations
To use manual tuning values, you must ensure the printer doesn't overwrite them:
- **Flow Dynamics:** Uncheck "Flow Dynamics Calibration" in the "Send to Printer" dialog.
- **Flow Rate:** Ensure the "Flow Ratio" in the filament profile matches your manual test result.

## 2. Multi-Material (AMS Lite) Tuning
### Purge / Flush Reduction
- **Bambu's default purge is conservative.**
- For PLA to PLA (similar colors), you can often reduce flush volumes by 30-50%.
- **Test:** Print a "Purge Calibration Tower" to find the minimum transition volume for your specific colors.

## 3. A1 Maintenance for Precision
- **Heatbed Tramming:** Even with ABL (Auto Bed Leveling), the bed should be manually trammed if the ABL mesh shows a deviation > 0.5mm.
- **PTFE Tube Friction:** Ensure the AMS Lite tubes have large, smooth curves. Tight bends = underextrusion during high-speed moves.
- **Camera Monitoring:** Setting up G-code for time-lapses and AI detection (if using external OctoPrint/Home Assistant integrations).
