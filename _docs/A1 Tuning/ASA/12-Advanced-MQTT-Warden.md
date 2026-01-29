# Step 9: Advanced MQTT "Temperature Trap"
**Goal:** Prevent printer deadlock and automate the heatsoak process.

## The Problem: The M400 U1 Deadlock
The Bambu A1 firmware has a limitation: when the printer is in a "User Pause" state (`M400 U1`), sending G-code commands (like `M104`) via MQTT can cause the firmware to deadlock. This makes it impossible to maintain nozzle temperature during a pause without breaking the "Resume" functionality.

## The Solution: The "Temperature Trap"
Instead of pausing, we use a "Wait for Temperature" command (`M109`) as a logic gate. This keeps the printer in a "Running" state (responsive to MQTT) but prevents it from starting the print until the `monitor_heatsoak.py` script "trips" the trap.

### 1. The G-Code Implementation
In the optimized start G-code, the heatsoak section is structured as follows:

```gcode
M104 S180 ; Maintain nozzle at 180°C (Responsive state)
M190 S100 ; (Wait) Confirm bed at 100°C for enclosure heating
M1002 gcode_claim_action : 16 ; Message: [HEATSOAKING] Waiting for Chamber...

; THE TRAP
; We tell the printer to wait for 181°C. 
; Since the heater is set to 180, it will wait here indefinitely.
M109 S181 
```

### 2. The Automation Script
The `monitor_heatsoak.py` script handles the "Release" automatically:
1.  **Detection:** It monitors the SHT21 sensor for a temperature plateau (~40°C+).
2.  **Verification:** It confirms the printer is "Trapped" (Running at ~180°C nozzle).
3.  **The Trip:** It sends `M104 S181` via MQTT.
4.  **Result:** The printer immediately completes the `M109` command and moves to **Bed Leveling**.

## Why this is "Showcase" Grade
1.  **Thermal Perfection:** Bed Leveling happens *after* the chamber and bed are fully expanded and stable.
2.  **No Deadlocks:** The printer remains 100% responsive to remote commands.
3.  **Zero Ooze:** By maintaining exactly 180°C, the ASA stays ready but doesn't "cook" or drip excessively like it would at 260°C.