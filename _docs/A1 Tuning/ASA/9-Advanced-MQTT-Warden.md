# Step 9: Advanced MQTT "Temperature Trap"
**Goal:** Prevent printer deadlock when using external automation scripts.

## The Problem: The M400 U1 Deadlock
The Bambu A1 firmware has a specific limitation: when the printer is in a "User Pause" state (`M400 U1`), the G-code executor is partially locked. 
*   If an external script (like `monitor_heatsoak.py`) sends an `M104` or `M105` command via MQTT while the printer is paused, the printer often deadlocks. 
*   The "Resume" button on the screen may become unresponsive, or the printer may simply ignore the resume command.

## The Solution: The "Temperature Trap"
Instead of using a formal Pause command, we use a "Wait for Temperature" command (`M109`) as a gate. This keeps the printer in a "Running" state (and thus responsive to MQTT) but prevents it from starting the print until the script "trips" the trap.

### 1. The G-Code Implementation
In your start G-code, replace the Pause section with this:

```gcode
M104 S180 ; Maintain nozzle at 180 (Responsive state)
M190 S100 ; Wait for bed 100
M1002 gcode_claim_action : 16 ; Message: [HEATSOAKING] Waiting for Chamber...

; THE TRAP
; We tell the printer to wait for 181°C. 
; Since the heater is set to 180, it will stay here forever.
M109 S181 
```

### 2. The Script Implementation
Your `monitor_heatsoak.py` script should be updated to "trip" the trap instead of sending a resume command.
*   **Action:** When the SHT21 sensor hits the target plateau, the script sends: `M104 S181`.
*   **Result:** The printer sees the target is met, finishes the `M109` command, and immediately moves to the next line of G-code.

## Benefits for "Showcase" Quality
1.  **Zero Nozzle Drop:** The nozzle never drops to the 90°C "Safety" temp, so there is no reheating delay or oozing issues.
2.  **Responsive Controls:** You can still use the printer screen or Bambu Handy to adjust settings while it's "trapped" in the soak.
3.  **Perfect Timing:** The print starts the exact second the chamber is ready.
