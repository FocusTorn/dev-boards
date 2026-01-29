;┌──────────────────────────────────────────┐
;│               Machine: A1                │
;│         Showcase Optimized (ASA)         │
;└──────────────────────────────────────────┘

; NOTE: M1002 only recognizes predefined flags (g29_before_print_flag, extrude_cali_flag, build_plate_detect_flag)
; Custom flags are not supported. Mech mode testing is controlled by commenting/uncommenting the section.

; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                   INITIAL SETUP AND HEATING                                    │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; Starts heating nozzle and bed in parallel (non-blocking commands) to maximize efficiency. Sets filament type
; and initializes system configuration. Material-aware: ASA/ABS materials heat bed to 100°C for heatsoak;
; other materials heat to initial temp directly.
;<

M1002 set_filament_type:{filament_type[initial_no_support_extruder]} ; Set filament type for the slicer logic
M104 S140 ; (No Wait) Start heating nozzle to 140°C standby temperature
M1002 gcode_claim_action : 2 ; Message: Heatbed preheating notification

; Conditional bed heating: 100°C for ASA/ABS heatsoak, otherwise initial temp
{if filament_type[initial_extruder]=="ASA" || filament_type[initial_extruder]=="ABS"} ; Check for high-temp materials
    M140 S100 ; (No Wait) Set bed to 100°C for ASA/ABS heatsoak phase
{else} ; Otherwise use standard temp
    M140 S[bed_temperature_initial_layer_single] ; (No Wait) Set bed to initial layer temp for other materials
{endif} ; End material condition

M960 S5 P1  ; Turn on logo lamp for better camera/physical visibility
G392 S0     ; Disable nozzle clog detection during startup to avoid false positives
M9833.2     ; Mandatory internal system command for proper machine initialization


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                         AVOID END STOP                                         │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; Safely moves Z-axis using sensorless end stop detection to prevent collisions during initialization.
;<

G91                ; Use incremental (relative) positioning mode
G380 S2 Z40 F1200  ; Safely move Z up 40mm using sensorless end stop detection
G380 S3 Z-15 F1200 ; Safely move Z down 15mm using sensorless end stop detection
G90                ; Restore absolute positioning mode


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                      RESET MACHINE STATUS                                      │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; Ensures all machine parameters are at known defaults for consistent startup state.
;<

G91                 ; Use incremental (relative) positioning mode
M220 S100           ; Reset feedrate override to 100%
M221 S100           ; Reset flowrate override to 100%
M204 S6000          ; Reset global acceleration to default 6000 mm/s²
M630 S0 P0          ; Clear any existing bed leveling/mesh data from memory

M17 Z0.3            ; Lower Z motor current before reset to prevent excessive homing force
M17 X0.65 Y1.2 Z0.6 ; Reset motor currents to factory default values for X, Y, and Z

M73.2 R1.0          ; Reset remaining time magnitude calculation to 100%
G90                 ; Restore absolute positioning mode
M982.2 S1           ; Enable motor cog noise reduction for quieter operation


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                        HOMING TOOLHEAD                                         │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; Homes X and prepares for Z homing with a quick nozzle check to ensure precision.
;<

M1002 gcode_claim_action : 13 ; Message: Homing toolhead notification
G28 X ; Home the X-axis (Lateral)

G91 ; Use incremental positioning
G1 Z5 F1200 ; Lift nozzle 5mm for clearance
G90 ; Restore absolute positioning

G0 X128 F30000 ; Move toolhead to center position on X
G0 Y254 F3000  ; Move toolhead to the rear position on Y

G91 ; Use incremental positioning
G1 Z-5 F1200 ; Lower nozzle back 5mm
G90 ; Restore absolute positioning

M109 S25 H140 ; (Wait) Stabilize nozzle at 165°C using 25°C hysteresis above 140°C

M17 E0.3     ; Lower extruder motor current for safe priming
M83          ; Set extruder to relative movement mode
G1 E10 F1200 ; Prime the extruder with 10mm of filament
G1 E-0.5 F30 ; Retract 0.5mm at slow speed to prevent oozing
M17 D        ; Reset extruder motor current to default strength

; Quick nozzle wipe before Z homing to prevent debris from affecting Z-offset accuracy
M211 X0 Y0 Z0       ; Disable software endstops for purge area access
G90                 ; Use absolute positioning
G1 X-28.5 F30000    ; Move to the purge area boundary
G1 X-48.2 F3000     ; Move slowly into the purge bucket position
G1 X-28.5 F30000    ; Rapid shake/wipe move to clear loose material
G1 X-48.2 F3000     ; Return to slow wipe position
G1 X-28.5 F30000    ; Final rapid shake/wipe move
M211 S              ; Restore software endstop safety

G0 X128 Y254 F30000 ; Return toolhead to center for precise Z homing
G28 Z P0 T140       ; Home Z-axis with low precision (safe at 140°C nozzle temp)

M104 S{nozzle_temperature_initial_layer[initial_extruder]} ; (No Wait) Start heating nozzle to print temp

M1002 judge_flag build_plate_detect_flag ; Check if build plate detection is requested
M622 S1 ; If build plate detection flag is set
    G39.4 ; Execute the automatic build plate type detection
    G90   ; Ensure absolute positioning is set
    G1 Z5 F1200 ; Lift nozzle 5mm after detection for clearance
M623 ; End of build plate detection block


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                PREPARE PRINT TEMPERATURE AND MATERIAL                          │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; Switches material if using AMS and performs a thorough dual-flush cleaning.
;<

M1002 gcode_claim_action : 24 ; Message: Prepare print temperature and material notification
M400                ; Wait for all previous movement commands to finish
M211 X0 Y0 Z0       ; Disable software endstops for purge bucket access
M975 S1             ; Enable vibration suppression logic

G90                 ; Ensure absolute positioning is set
G1 X-28.5 F30000    ; Move rapidly to the purge area
G1 X-48.2 F3000     ; Move toolhead into the purge bucket

M620 M ; Enable AMS material remapping logic
M620 S[initial_no_support_extruder]A ; Start material switch sequence if using AMS
    M1002 gcode_claim_action : 4 ; Message: Changing filament notification
    M1002 set_filament_type:UNKNOWN ; Set material to unknown during the swap
    M109 S[nozzle_temperature_initial_layer] ; Wait for nozzle to reach target print temp
    M104 S250 ; (No Wait) Set common flush temperature to 250°C
    T[initial_no_support_extruder] ; Physically switch to the requested AMS extruder
    G1 X-48.2 F3000 ; Ensure toolhead is correctly positioned in the bucket
    M620.1 E F{filament_max_volumetric_speed[initial_no_support_extruder]/2.4053*60} T{nozzle_temperature_range_high[initial_no_support_extruder]} ; Load filament
    M109 S250 ; (Wait) Confirm nozzle has reached 250°C for the flush
    M106 P1 S0 ; Turn off the part cooling fan during the flush
    G92 E0 ; Reset the extruder position counter
    G1 E50 F200 ; Flush 50mm of filament at common temp to clear the hotend
    M1002 set_filament_type:{filament_type[initial_no_support_extruder]} ; Update material type in system
M621 S[initial_no_support_extruder]A ; Finalize AMS material selection

M109 S{nozzle_temperature_range_high[initial_extruder]} H300 ; (Wait) Confirm high temperature for the second flush
G92 E0 ; Reset the extruder position counter
G1 E50 F200 ; Flush an additional 50mm at high temperature for thorough cleaning
M106 P1 S178 ; Turn on part cooling fan at ~70% to assist cooling
G92 E0 ; Reset the extruder position counter
G1 E5 F200 ; Perform a small final flush to ensure pressure
M104 S{nozzle_temperature_initial_layer[initial_extruder]} ; (No Wait) Set nozzle back to initial layer temp
G1 E-0.5 F300 ; Retract 0.5mm to prevent oozing during movement

; Multi-cycle wipe and shake sequence to clear all waste from the nozzle tip
G1 X-28.5 F30000 ; Rapid move to shake off waste
G1 X-48.2 F3000  ; Slow move to wipe against the bucket edge
G1 X-28.5 F30000 ; Rapid move to shake off waste
G1 X-48.2 F3000  ; Slow move to wipe against the bucket edge
G1 X-28.5 F30000 ; Rapid move to shake off waste
G1 X-48.2 F3000  ; Slow move to wipe against the bucket edge
G1 X-28.5 F30000 ; Final rapid shake to ensure a clean tip

M106 P1 S0 ; Turn off the part cooling fan


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                  AUTO EXTRUSION CALIBRATION                                    │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;<
M975 S1             ; Enable vibration suppression logic
G90                 ; Ensure absolute positioning is set
M83                 ; Set extruder to relative movement mode
T1000               ; Reset active tool to default
G1 X-48.2 Y0 Z10 F10000 ; Move toolhead to the designated calibration zone
M412 S1             ; Enable filament runout detection sensor
M620.3 W1           ; Enable filament tangle detection logic

M1002 judge_flag extrude_cali_flag ; Check if automatic flow calibration is enabled
M622 J1 ; If extrusion calibration flag is set
    M1002 gcode_claim_action : 8 ; Message: Calibrating extrusion (Flow Dynamics) notification
    M109 S{nozzle_temperature[initial_extruder]} ; (Wait) Ensure nozzle is at target print temperature
    G1 E10 F{outer_wall_volumetric_speed/2.4*60} ; Prime the nozzle before starting the test
    M983 F{outer_wall_volumetric_speed/2.4} A0.3 H[nozzle_diameter] ; Execute the dynamic flow calibration
    M106 P1 S255 ; Turn fan to 100% to clean debris after the test
    G1 X-28.5 F18000 ; Rapid move to clear calibration debris
    G1 X-48.2 F3000 ; Slow move to wipe
    G1 X-28.5 F12000 ; Final rapid wipe move
    M106 P1 S0 ; Turn off the part cooling fan
    M984 A0.1 E1 S1 F{outer_wall_volumetric_speed/2.4} H[nozzle_diameter] ; Execute final calibration line painting
M623 ; End of extrusion calibration block


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                         WIPE NOZZLE                                            │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; Deep clean of the nozzle tip before the heatsoak and leveling process to ensure precision.
;<

M1002 gcode_claim_action : 14 ; Message: Cleaning nozzle tip notification
M104 S170 ; (No Wait) Set nozzle to 170°C for the mechanical wipe phase
M106 S255 ; Turn part cooling fan to 100% to reach wipe temp quickly
M211 S    ; Push current soft endstop status to the stack
M211 X0 Y0 Z0 ; Disable software endstops for manual zone access
G1 E-1 F500 ; Retract filament 1mm to prevent oozing during the wipe
M109 S170 ; (Wait) Confirm nozzle has reached the 170°C wipe temperature

; Mechanical wipe cycles on the front build plate cleaning tab
G0 X108 Y-0.5 F30000 ; Move toolhead to start of the cleaning tab
G380 S3 Z-5 F1200 ; Probe nozzle downward against the tab to scrub debris
G1 Z2 F1200 ; Lift nozzle 2mm for lateral movement
G1 X120 F10000 ; Move toolhead 12mm along the tab
G380 S3 Z-5 F1200 ; Probe nozzle downward against the tab to scrub debris
G1 Z2 F1200 ; Lift nozzle 2mm for lateral movement
G1 X138 F10000 ; Move toolhead to end of the cleaning tab
G380 S3 Z-5 F1200 ; Final downward scrub probe against the tab
G1 Z5 F30000 ; Rapidly lift toolhead 5mm for clearance

; Remove sticky waste via cooled-plastic-adhesion on the rear exposed steel plate
G0 X118 Y261 F30000 ; Move toolhead to the rear waste area
M109 S{nozzle_temperature_initial_layer[initial_extruder]-30} ; (Wait) Cool nozzle 30°C to "grab" material
G28 Z P0 T300 ; Perform a Z homing move to establish position
G29.2 S0 ; Temporarily disable automatic bed leveling for this move
G0 Z-1.01 F1200 ; Press nozzle firmly against the steel plate to flatten waste

; Alternating circular scrub patterns to dislodge stubborn nozzle tip debris
G91 ; Switch to incremental positioning mode
G2 I1 J0 X2 Y0 F2000 ; Clockwise scrub arc
G2 I-0.75 J0 X-1.5 ; Counter-clockwise scrub arc
G3 I1 J0 X2 ; Counter-clockwise scrub arc (Reverse logic)
G3 I-0.75 J0 X-1.5 ; Clockwise scrub arc (Reverse logic)
G90 ; Restore absolute positioning mode
G1 Z10 F1200 ; Lift nozzle 10mm after scrub for clearance

; Final high-precision brush wipes on the PEI surface area
G1 Y250 F30000 ; Move to start of the brush zone
G1 X55 ; Position X correctly for the brush
G1 Z1.300 F1200 ; Lower nozzle to brush height
G1 Y262.5 F6000 ; Wipe nozzle across the brush
G91 ; Switch to incremental positioning mode
G1 X-35 F30000 ; Rapid lateral shake move across brush
G1 X45 ; Rapid lateral shake move across brush
G1 X-45 ; Final lateral shake move
G90 ; Restore absolute positioning mode
G1 Z10 F1200 ; Lift nozzle 10mm for clearance

M106 S255 ; Keep fan at 100% for the next phase
M211 R ; Pop/Restore software endstop status from the stack


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                 STEP 9: HEATSOAK TRAP (MQTT)                                   │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; ASA/ABS requires a chamber soak. We use the "Temperature Trap" (M109 S181) to keep the A1 
; responsive to the monitor_heatsoak.py script without deadlocking the firmware.
;<

G1 X108.000 Y-0.500 F30000 ; Move toolhead to the waiting/soak position
G1 Z0.300 F1200 ; Lower nozzle to heatsoak height
G2814 Z0.32 ; Internal positioning command for A1 stability

{if filament_type[initial_extruder]=="ASA" || filament_type[initial_extruder]=="ABS"} ; Check for high-temp materials
    M104 S180 ; Set nozzle to 180°C standby (Keeps firmware responsive to MQTT)
    M190 S100 ; (Wait) Confirm bed has reached 100°C to start enclosure heating
    M1002 gcode_claim_action : 16 ; Message: [HEATSOAKING] Waiting for Chamber... notification
    
    ; THE TRAP: Printer waits for nozzle to hit 181°C (which it won't reach without external help).
    ; The monitor_heatsoak.py script will send M104 S181 to "trip" the trap when SHT21 plateau is met.
    M109 S181 ; (Wait) Indefinite temperature trap until released by Python script
{endif} ; End material condition

M1002 gcode_claim_action : 0 ; Clear the heatsoak message from the printer screen
M104 S{nozzle_temperature_initial_layer[initial_extruder]} ; Set nozzle temperature back to initial print temp


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                        BED LEVELING                                            │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; Performing ABL AFTER the heatsoak ensures the mesh accounts for full thermal expansion.
;<

M1002 judge_flag g29_before_print_flag ; Check if automatic bed leveling is enabled
M622 J1 ; If bed leveling flag is set
    M1002 gcode_claim_action : 1 ; Message: Auto bed leveling notification
    G90 ; Ensure absolute positioning is set
    G1 X0 Y0 F30000 ; Move toolhead to the home position
    G29.2 S1 ; Enable automatic bed leveling logic
    M190 S[bed_temperature_initial_layer_single] ; (Wait) Confirm bed is at target print temperature
    M109 S140 ; (Wait) Drop nozzle to 140°C for safe probing (prevents plate damage)
    M106 S0 ; Turn off all fans (Noisy fans can interfere with sensitive leveling sensors)
    G29 A1 X{first_layer_print_min[0]} Y{first_layer_print_min[1]} I{first_layer_print_size[0]} J{first_layer_print_size[1]} ; Run ABL
    M400 ; Wait for the ABL process to completely finish
    M500 ; Save the newly created mesh data to the machine memory
M623 ; End of bed leveling block

M1002 judge_flag g29_before_print_flag ; Check if leveling was performed
M622 J0 ; If bed leveling was NOT performed
    M1002 gcode_claim_action : 13 ; Message: Homing toolhead notification
    G28 ; Perform a full home move to ensure accuracy if ABL was skipped
M623 ; End of skip-leveling block


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                  EXTRUDE CALIBRATION TEST                                      │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;<

M400 ; Wait for all movements to complete
M900 S ; Set internal extrusion calibration parameter S
M900 C ; Set internal extrusion calibration parameter C
G90 ; Ensure absolute positioning is set
M83 ; Set extruder to relative movement mode
M109 S{nozzle_temperature_initial_layer[initial_extruder]} ; (Wait) Confirm nozzle has reached print temp

; Draw the primary extrusion calibration verification line at various speeds
G0 X128 E8 F{outer_wall_volumetric_speed/(24/20)*60} ; Initial prime move
G0 X133 E.3742 F{outer_wall_volumetric_speed/(0.3*0.5)/4*60} ; Slow segment
G0 X138 E.3742 F{outer_wall_volumetric_speed/(0.3*0.5)*60} ; Fast segment
G0 X143 E.3742 F{outer_wall_volumetric_speed/(0.3*0.5)/4*60} ; Slow segment
G0 X148 E.3742 F{outer_wall_volumetric_speed/(0.3*0.5)*60} ; Fast segment
G0 X153 E.3742 F{outer_wall_volumetric_speed/(0.3*0.5)/4*60} ; Slow segment

G91 ; Switch to incremental positioning mode
G1 X1 Z-0.3 ; Final small squish move for the test line
G1 X4 ; Lateral move to detach the line
G1 Z1 F1200 ; Lift nozzle 1mm for travel
G90 ; Restore absolute positioning mode
M900 R ; Reset extrusion calibration parameters to print defaults

; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                     FINAL PREPARATION                                          │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;<

M1002 gcode_claim_action : 0 ; Clear any remaining messages from the printer screen
M400 ; Wait for all operations to settle

; Textured PEI Plate Z-offset adjustment logic
{if curr_bed_type=="Textured PEI Plate"} ; If the user selected a textured plate
    G29.1 Z{-0.02} ; Apply a slight extra squish (-0.02mm) to ensure adhesion in textures
{endif} ; End plate condition

M960 S1 P0 ; Turn off internal machine laser sensor 1
M960 S2 P0 ; Turn off internal machine laser sensor 2
M106 S0 ; Turn off the main part cooling fan
M106 P2 S0 ; Turn off the internal chassis cooling fan
M106 P3 S0 ; Turn off the internal chamber fan

M975 S1 ; Enable vibration suppression compensation for the print
G90 ; Ensure absolute positioning mode is set
M83 ; Set extruder to relative movement mode
T1000 ; Reset active tool head to default state

M211 X0 Y0 Z0 ; Disable software endstops one last time for the starting toolpath
M1007 S1 ; Enable material mass estimation tracking for the slicer
G29.4 ; Execute final start-of-print machine positioning command
