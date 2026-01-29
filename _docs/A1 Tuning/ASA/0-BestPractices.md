# Step 0: The "Showcase" Foundation
**Goal:** Ensure the machine and material are in peak condition before calibration begins.

Professional results require a stable baseline. Don't waste time calibrating a machine with loose belts or wet filament.

## 1. Mechanical Preparation
*   **Belt Tensioning:** Check the X and Y belts. They should be "taut but not singing." The A1 has specific rear-access tensioning logic. Refer to the Bambu Wiki for the frequency tuning procedure.
*   **Build Plate Hygiene:** ASA is extremely sensitive to oils. Wash your PEI plate with dish soap and warm water, then wipe with 99% IPA. Never touch the print area with your bare hands.
*   **Nozzle Inspection:** Ensure the nozzle wipe area (the silicone tab) is clean and not torn. Debris here ruins the A1's auto Z-offset.

## 2. Material Preparation (Drying)
*   **ASA is Hygroscopic:** Even a "fresh" roll of Overture ASA can be wet from the factory.
*   **Procedure:** Dry at **70°C for 6-8 hours** in a dedicated filament dryer or oven.
*   **Signs of Moisture:** Popping sounds, steam from the nozzle, or a "fuzzy" surface finish on the walls.

## 3. Testing Methodology
To achieve showcase quality, follow the **"One at a Time"** rule:
*   **Isolate Variables:** Never change speed and temperature at the same time.
*   **Cooling Period:** Wait 5-10 minutes after a test print finishes before measuring. ASA shrinks as it cools; measuring a hot part leads to incorrect flow/compensation values.
*   **Document:** Keep a log of which values produced which artifacts.

## 4. Adhesion Helpers (Warping Prevention)
*   **Brims:** For parts with sharp corners or large footprints, use a **5mm Outer Brim**. 
*   **Brim-Object Gap:** Set to **0.1mm - 0.15mm** for easy removal without leaving a white stress mark on the ASA.
*   **Draft Shield:** If your enclosure isn't hitting 43°C yet, enable a "Draft Shield" in Orca Slicer to keep a cocoon of warm air around the part.
