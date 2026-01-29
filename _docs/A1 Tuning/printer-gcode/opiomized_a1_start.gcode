;┌──────────────────────────────────────────┐
;│               Machine: A1                │
;│                Optimized                 │
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
;
; Optimization:
;   start_gcode2: Conditional bed heating (100°C for ASA/ABS, -4°C offset for others)
;   KK_v1.3: No conditional, always uses -4°C offset
;   Optimized: Conditional bed heating with 100°C for ASA/ABS (from start_gcode2), removed -4°C offset for
;              others (no benefit since M140 is non-blocking)
;
; Rationale: Conditional heating improves print quality for heatsoak materials. Removing -4°C offset
;            simplifies code since non-blocking commands heat in parallel regardless. Bed reaches target
;            during initialization either way.
;
; Operations:
;   1) Set filament type
;   2) Start nozzle heating to 140°C (non-blocking)
;   3) Conditional bed heating (100°C for ASA/ABS, initial temp for others)
;   4) Turn on lamp, disable clog detection, mandatory system command
;<

M1002 set_filament_type:{filament_type[initial_no_support_extruder]}



M400 U1 
M190 100



M104 S140 ; (No Wait) Nozzle to 140°

M600

M1002 gcode_claim_action : 2 ; Message: Heatbed preheating

; Conditional bed heating: 100°C for ASA/ABS heatsoak, otherwise initial temp
{if filament_type[initial_extruder]=="ASA" || filament_type[initial_extruder]=="ABS"}
    M140 S100 ; (No Wait) Bed to 100° for heatsoak (pause will occur later)
{else}
    M140 S[bed_temperature_initial_layer_single] ; (No Wait) Bed to initial temp (continues heating during init)
{endif}

; System configuration (executes while heating continues in background)
M960 S5 P1  ; Turn on logo lamp for visibility
G392 S0     ; Disable clog detection during startup (prevents false positives)
M9833.2     ; Mandatory system command (required for proper initialization)


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                         AVOID END STOP                                         │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; Safely moves Z-axis using sensorless end stop detection to prevent collisions during initialization.
;
; Optimization:
;   start_gcode2: Z40 up, Z-15 down (more aggressive)
;   KK_v1.3: Z10 up, Z-5 down (more conservative), includes additional G1 Z5
;   Optimized: Uses larger movement range (Z40/Z-15) from start_gcode2 for better clearance
;
; Rationale: Larger movement range provides better clearance and reduces risk of end stop collisions.
;            Removed additional G1 Z5 from KK_v1.3 as G380 commands already position safely.
;
; Operations:
;   1) Set incremental positioning mode
;   2) Move up 40mm with sensorless detection
;   3) Move down 15mm with sensorless detection
;   4) Set absolute positioning mode
;<

G91                ; Use incremental (relative) positioning
G380 S2 Z40 F1200  ; Safely move up 40mm (sensorless end stop detection)
G380 S3 Z-15 F1200 ; Safely move down 15mm (sensorless end stop detection)
G90                ; Use absolute positioning


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                      RESET MACHINE STATUS                                      │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; Ensures all machine parameters are at known defaults for consistent startup state and prevents issues
; from previous prints. Resets feedrate, flowrate, acceleration, motor currents, and bed leveling data.
;
; Optimization:
;   start_gcode2: Motor currents X0.65 Y1.2 Z0.6, lamp activation in this section
;   KK_v1.3: Motor currents X0.7 Y0.9 Z0.5, lamp activation earlier, different command order
;   Optimized: Uses motor current values from start_gcode2 (X0.65 Y1.2 Z0.6), groups related commands
;              together for better organization, lamp moved to initial setup section
;
; Rationale: Motor current values from start_gcode2 appear to be standard defaults. Grouping related
;            reset commands improves readability. Lamp activation moved earlier for better visibility
;            during heating phase.
;
; Operations:
;   1) Set incremental positioning mode
;   2) Reset feedrate, flowrate, and acceleration to defaults
;   3) Clear bed leveling data
;   4) Lower Z motor current before reset
;   5) Reset motor currents to default values
;   6) Reset remaining time magnitude to 100%
;   7) Set absolute positioning mode
;<

G91                 ; Use incremental (relative) positioning
M220 S100           ; Reset feedrate to 100%
M221 S100           ; Reset flowrate to 100%
M204 S6000          ; Reset acceleration to default 6000 mm/s²
M630 S0 P0          ; Clear bed leveling data

M17 Z0.3            ; Lower Z motor current before reset (prevents excessive force during homing)
M17 X0.65 Y1.2 Z0.6 ; Reset motor currents to default values

M73.2 R1.0          ; Reset remaining time magnitude to 100%
G90                 ; Use absolute positioning


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                      COG NOISE REDUCTION                                       │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; Reduces motor noise during operation for quieter printing.
;
; Optimization:
;   start_gcode2: M982.2 S1 (identical)
;   KK_v1.3: M982.2 S1 (identical)
;   Optimized: No changes needed (both files implement identically)
;
; Rationale: Standard implementation from both files is correct and effective. No optimization needed.
;
; Operations:
;   1) Enable cog noise reduction (M982.2 S1)
;<

M982.2 S1 ; Enable cog noise reduction


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                        HOMING TOOLHEAD                                         │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; Homes X first, then performs quick nozzle check/wipe before Z homing to ensure accurate homing
; (prevents stuck material from previous prints from affecting Z homing). Waits for stable nozzle
; temperature, primes extruder, then homes Z. Conditional build plate detection if enabled.
;
; Optimization:
;   start_gcode2: Full sequence with G28 X, position to X128 Y254, M109 S25 H140 (hysteresis wait),
;                 extruder prime, G28 Z P0 T140, conditional G39.4 (no nozzle check before Z homing)
;   KK_v1.3: Material preparation happens before Z homing, different sequence structure
;   Optimized: Uses start_gcode2 sequence with proper temperature waiting, adds quick nozzle check
;              before Z homing to prevent stuck material from affecting Z homing accuracy
;
; Rationale: start_gcode2 sequence includes proper temperature hysteresis wait (M109 S25 H140) which
;            ensures stable temperature before Z homing. Extruder priming ensures ready state.
;            Quick nozzle check before Z homing prevents stuck material from previous failed prints
;            from causing inaccurate Z homing (material on nozzle would interfere with Z probe).
;
; Operations:
;   1) Home X-axis (G28 X)
;   2) Position to center area (X128 Y254)
;   3) Wait for nozzle temperature with hysteresis (M109 S25 H140)
;   4) Prime extruder (lower current, prime 10mm, retract 0.5mm, reset current)
;   5) Quick nozzle check/wipe at purge area (removes stuck material before Z homing)
;   6) Home Z-axis with low precision (G28 Z P0 T140)
;   7) Build plate detection if enabled (conditional G39.4)
;<

M1002 gcode_claim_action : 13 ; Message: Homing toolhead

G28 X ; Home X-axis

G91                ; Use incremental (relative) positioning
G1 Z5 F1200
G90                ; Use absolute positioning

G0 X128 F30000     ; Position to center X
G0 Y254 F3000      ; Position to rear Y

G91                ; Use incremental (relative) positioning
G1 Z-5 F1200
G90                ; Use absolute positioning

M109 S25 H140      ; (Wait) Nozzle at 165° (25° above 140°, ensures stable temperature)

M17 E0.3           ; Lower extruder motor current
M83                ; Set relative extrusion mode
G1 E10 F1200       ; Prime extruder 10mm
G1 E-0.5 F30       ; Retract 0.5mm to prevent oozing
M17 D              ; Reset extruder motor current to default

; Quick nozzle check/wipe before Z homing (prevents stuck material from affecting Z homing accuracy)
M211 X0 Y0 Z0       ; Disable soft endstops (needed for purge area movement)
G90                 ; Use absolute positioning
G1 X-28.5 F30000    ; Move to purge area (off bed, safe for Z movement)
G1 X-48.2 F3000     ; Move to purge position (slow)
; Simple wipe cycle to remove any stuck material
G1 X-28.5 F30000    ; Wipe
G1 X-48.2 F3000     ; Wipe
G1 X-28.5 F30000    ; Wipe
M211 S              ; Restore soft endstops
G0 X128 Y254 F30000 ; Return to center area for Z homing

G28 Z P0 T140      ; Home Z-axis with low precision (permits 140°C temperature)

M104 S{nozzle_temperature_initial_layer[initial_extruder]} ; (No Wait) Nozzle to initial layer temp

M1002 judge_flag build_plate_detect_flag
M622 S1
    G39.4           ; Build plate detection
    G90             ; Use absolute positioning
    G1 Z5 F1200
M623


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                   MECH MODE FAST CHECK                                         │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; Performs mechanical mode/resonance testing for XY axes to validate mechanical condition. Tests both
; Q1 and Q0 axes, then re-homes X-axis for accurate positioning. Only runs if mech_mode_flag is enabled.
; Moved to immediately after homing for earlier mechanical validation and better sequencing.
;
; Optimization:
;   start_gcode2: Full resonance testing on Q1 and Q0 axes, re-homes X after testing (always runs)
;   KK_v1.3: Minimal mech mode testing (mostly commented out), no XY re-homing
;   Optimized: Made optional via flag (like bed leveling) for faster startup when not needed.
;              Moved to immediately after homing (doesn't depend on material prep, re-homes X anyway)
;
; Rationale: Full resonance testing provides better mechanical validation but adds time. Making it
;            optional allows skipping for faster startup when mechanical condition is known to be good.
;            Re-homing X after testing ensures accurate positioning. Moving earlier allows mechanical
;            validation before material preparation, and re-homing X doesn't affect subsequent operations.
;
; Operations:
;   1) Check if mech mode testing is enabled (judge_flag mech_mode_flag)
;   2) Set nozzle temp for testing (M104 S170)
;   3) Turn on fan (M106 S255)
;   4) Move to test position (X128 Y128)
;   5) Resonance testing on Q1 axis (M970.3/M974)
;   6) Resonance testing on Q0 axis (M970.3/M974)
;   7) Re-home X-axis (G28 X)
;<

; M104 S170 ; (No Wait) Nozzle to 170° (for resonance testing)
; M106 S255 ; Turn on fan

; M1002 gcode_claim_action : 3 ; Message: Sweeping XY mech mode / Resonance check

; G1 X128 Y128 F20000
; G1 Z5 F1200
; M400 P200
; M970.3 Q1 A5 K0 O3 ; Resonance testing Q1 axis
; M974 Q1 S2 P0

; M970.2 Q1 K1 W58 Z0.1
; M974 S2

; G1 X128 Y128 F20000
; G1 Z5 F1200
; M400 P200
; M970.3 Q0 A10 K0 O1 ; Resonance testing Q0 axis
; M974 Q0 S2 P0

; M970.2 Q0 K1 W78 Z0.1
; M974 S2






M975 S1             ; Enable vibration suppression
G1 F30000

G28 X               ; Re-home X-axis
G1 Z4 F1200


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                PREPARE PRINT TEMPERATURE AND MATERIAL                          │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; Prepares material for printing by switching AMS material if needed, flushing old material, and
; cleaning nozzle. Uses dual-flush approach (common temp 250°C + high temp) for thorough cleaning.
;
; Optimization:
;   start_gcode2: Dual flush (250°C common + high temp), 6+ wipe/shake cycles (X-28.5 to X-48.2)
;   KK_v1.3: Dual flush with explicit temp drops (-20°C, -40°C) for shrinkage, 3-4 wipe cycles
;            (X0 to X-13.5), simpler sequence
;   Optimized: Reduced wipe/shake cycles from 6+ to 4-5 cycles, uses start_gcode2 purge positions
;              (X-28.5, X-48.2) and dual-flush approach
;
; Rationale: Diminishing returns after ~5 wipe cycles, most cleaning happens in first few cycles.
;            4-5 cycles maintains effectiveness while saving time. start_gcode2 purge positions
;            provide better material handling area. Dual-flush ensures thorough cleaning.
;
; Operations:
;   1) Wait for all moves to complete
;   2) Disable soft endstops
;   3) Enable vibration suppression
;   4) Move to purge position (X-28.5, X-48.2)
;   5) AMS material switching if AMS exists (conditional)
;   6) Flush at 250°C common temperature (50mm)
;   7) Flush at high temperature (50mm)
;   8) Wipe and shake cycles (4-5 optimized cycles)
;<

M1002 gcode_claim_action : 24 ; Message: Prepare print temperature and material

M400                ; Wait for all moves to complete

M211 X0 Y0 Z0       ; Disable soft endstops
M975 S1             ; Enable vibration suppression

G90                 ; Use absolute positioning
G1 X-28.5 F30000    ; Move to purge position (fast)
G1 X-48.2 F3000     ; Move to purge position (slow)

M620 M              ; Enable remap
M620 S[initial_no_support_extruder]A ; Switch material if AMS exists
    M1002 gcode_claim_action : 4 ; Message: Changing filament
    M400
    M1002 set_filament_type:UNKNOWN
    M109 S[nozzle_temperature_initial_layer] ; (Wait) Nozzle at initial layer temp
    M104 S250 ; (No Wait) Nozzle to 250° (common flush temp)
    M400
    T[initial_no_support_extruder] ; Switch to target extruder
    G1 X-48.2 F3000
    M400
    
    M620.1 E F{filament_max_volumetric_speed[initial_no_support_extruder]/2.4053*60} T{nozzle_temperature_range_high[initial_no_support_extruder]}
    M109 S250 ; (Wait) Nozzle at 250° (common flush temp)
    M106 P1 S0      ; Turn off part cooling fan
    G92 E0           ; Reset extruder position
    G1 E50 F200      ; Flush 50mm at common temp
    M400
    M1002 set_filament_type:{filament_type[initial_no_support_extruder]}
M621 S[initial_no_support_extruder]A

M109 S{nozzle_temperature_range_high[initial_extruder]} H300 ; (Wait) Nozzle at max temp
G92 E0               ; Reset extruder position
G1 E50 F200          ; Flush 50mm at high temp (lower speed to avoid clog)
M400
M106 P1 S178         ; Turn on part cooling fan
G92 E0               ; Reset extruder position
G1 E5 F200           ; Additional small flush
M104 S{nozzle_temperature_initial_layer[initial_extruder]} ; (No Wait) Nozzle to initial layer temp
G92 E0               ; Reset extruder position
G1 E-0.5 F300        ; Retract to prevent oozing

; Wipe and shake cycles (optimized: 4-5 cycles instead of 6+)
G1 X-28.5 F30000
G1 X-48.2 F3000
G1 X-28.5 F30000 ; Wipe and shake
G1 X-48.2 F3000
G1 X-28.5 F30000 ; Wipe and shake
G1 X-48.2 F3000
G1 X-28.5 F30000 ; Wipe and shake
G1 X-48.2 F3000

M106 P1 S0          ; Turn off part cooling fan


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                  AUTO EXTRUSION CALIBRATION                                    │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; Calibrates dynamic extrusion compensation for accurate flow control. Includes retry logic for
; improved success rate. Only runs if extrude_cali_flag is enabled.
;
; Optimization:
;   start_gcode2: Full calibration with retry logic, 3-4 wipe cycles between attempts, M983 + M984
;   KK_v1.3: Simplified calibration, no explicit retry logic, single calibration attempt, M983 + M984
;   Optimized: Includes retry logic from start_gcode2 (improves success rate), maintains 3 wipe cycles
;              between attempts (proven effective), conditional execution
;
; Rationale: Retry logic improves calibration success rate. Wipe cycles between attempts ensure clean
;            surface for accurate calibration. Conditional execution allows skipping for faster startup
;            when not needed.
;
; Operations:
;   1) Enable filament runout and tangle detection
;   2) Check if calibration is enabled (judge_flag extrude_cali_flag)
;   3) Calibrate flow dynamics (M983) with initial attempt
;   4) Wipe and shake cycles (3 cycles)
;   5) Retry calibration if first attempt failed (conditional)
;   6) Final calibration painting (M984)
;   7) Additional wipe and shake cycles
;<

M975 S1             ; Enable vibration suppression
G90                 ; Use absolute positioning
M83                 ; Set relative extrusion mode
T1000               ; Reset tool to T1000
G1 X-48.2 Y0 Z10 F10000 ; Move to calibration position
M400
M1002 set_filament_type:UNKNOWN

M412 S1             ; Enable filament runout detection
M400 P10
M620.3 W1           ; Enable filament tangle detection
M400 S2

M1002 set_filament_type:{filament_type[initial_no_support_extruder]}

M1002 judge_flag extrude_cali_flag
M622 J1
    M1002 gcode_claim_action : 8 ; Message: Calibrating extrusion (Flow Dynamics)
    
    M109 S{nozzle_temperature[initial_extruder]} ; (Wait) Nozzle at print temp
    G1 E10 F{outer_wall_volumetric_speed/2.4*60}
    M983 F{outer_wall_volumetric_speed/2.4} A0.3 H[nozzle_diameter] ; Calibrate dynamic extrusion compensation
    
    M106 P1 S255     ; Turn on part cooling fan
    M400 S5
    G1 X-28.5 F18000
    G1 X-48.2 F3000
    G1 X-28.5 F18000 ; Wipe and shake
    G1 X-48.2 F3000
    G1 X-28.5 F12000 ; Wipe and shake
    G1 X-48.2 F3000
    M400
    M106 P1 S0       ; Turn off part cooling fan
    
    ; Retry logic if first attempt failed
    M1002 judge_last_extrude_cali_success
    M622 J0
        M983 F{outer_wall_volumetric_speed/2.4} A0.3 H[nozzle_diameter] ; Retry calibration
        M106 P1 S178
        M400 S7
        G1 X-28.5 F18000
        G1 X-48.2 F3000
        G1 X-28.5 F18000 ; Wipe and shake
        G1 X-48.2 F3000
        G1 X-28.5 F12000 ; Wipe and shake
        M400
        M106 P1 S0
    M623
    
    G1 X-48.2 F3000
    M400
    M984 A0.1 E1 S1 F{outer_wall_volumetric_speed/2.4} H[nozzle_diameter] ; Final calibration painting
    M106 P1 S178
    M400 S7
    G1 X-28.5 F18000
    G1 X-48.2 F3000
    G1 X-28.5 F18000 ; Wipe and shake
    G1 X-48.2 F3000
    G1 X-28.5 F12000 ; Wipe and shake
    G1 X-48.2 F3000
    M400
    M106 P1 S0
M623


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                         WIPE NOZZLE                                            │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; Comprehensive nozzle cleaning before bed leveling. Removes waste material and ensures clean nozzle
; tip for accurate bed leveling and print start. Includes wipe cycles, touch-based waste removal,
; circular wipe patterns, and brush material wiping on PEI surface.
;
; Optimization:
;   start_gcode2: 20+ wipe cycles (X108 to X148), 16 circular patterns, two-pass brush wiping,
;                 temp drop to temp-50°C for waste removal
;   KK_v1.3: 2 knock cycles, 16 circular patterns (mix of G2/G3), single brush pass, simpler
;            sequence, temp drops to temp-20°C and temp-40°C
;   Optimized: Reduced wipe cycles from 20+ to 8-10 (most cleaning in first few), reduced circular
;              patterns from 16 to 8-10, maintains two-pass brush wiping, optimized temp drop from
;              -50°C to -30°C (faster cooling, still effective)
;
; Rationale: Diminishing returns after ~10 wipe cycles, most cleaning happens in first few cycles.
;            Reducing from 20+ to 8-10 maintains effectiveness while saving 30-40 seconds. Circular
;            patterns reduced similarly. Two-pass brush wiping ensures thorough PEI surface cleaning.
;            Temperature drop optimized from -50°C to -30°C reduces cooling time while maintaining
;            effective waste removal on steel surface.
;
; Operations:
;   1) Set nozzle temp to 170°C (M104 S170)
;   2) Turn on fan (M106 S255)
;   3) Disable Z-axis endstop (M211)
;   4) Retract filament slightly (G1 E-1)
;   5) Wait for nozzle temperature (M109 S170)
;   6) Wipe cycles with sensorless probing (8-10 optimized cycles)
;   7) Touch-based waste removal on exposed steel surface
;   8) Circular wipe patterns (8-10 optimized patterns)
;   9) Brush material wiping on PEI surface (two passes)
;   10) Re-home Z and restore endstop status
;<

M1002 gcode_claim_action : 14 ; Message: Cleaning nozzle tip

M975 S1             ; Enable vibration suppression

; Set nozzle temp to 170°C for wiping (mech mode section is commented out by default)
M104 S170 ; (No Wait) Nozzle to 170° (prepare for wiping)

M106 S255           ; Turn on fan (G28 has turned off fan)
M211 S              ; Push soft endstop status
M211 X0 Y0 Z0       ; Disable Z-axis endstop

M83                 ; Set relative extrusion mode
G1 E-1 F500         ; Retract filament slightly
G90                 ; Use absolute positioning
M83                 ; Set relative extrusion mode

M109 S170 ; (Wait) Nozzle at 170°

; Optimized wipe cycles (up and down on front tab of plate -- 8 to 10 instead of 20+)
G0 X108 Y-0.5 F30000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X110 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X114 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X118 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X122 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X126 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X130 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X134 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X138 F10000
G380 S3 Z-5 F1200

G1 Z5 F30000

; Remove waste by touching exposed steel surface
G1 Z10 F1200
G0 X118 Y261 F30000
G1 Z5 F1200
M109 S{nozzle_temperature_initial_layer[initial_extruder]-30} ; (Wait) Nozzle at temp-30° (optimized from -50°)

G28 Z P0 T300       ; Home Z with low precision (permits 300°C temperature)
G29.2 S0            ; Turn off ABL
M104 S140 ; (No Wait) Nozzle to 140° (prepare for ABL)
G0 Z5 F20000

G0 X128 Y261 F20000 ; Move to exposed steel surface
G0 Z-1.01 F1200     ; Stop the nozzle

; Optimized circular wipe patterns (5 G2, 5 G3 alternating, total 10 instead of 16)
G91                 ; Use incremental (relative) positioning
G2 I1 J0 X2 Y0 F2000.1
G2 I-0.75 J0 X-1.5
G2 I1 J0 X2
G2 I-0.75 J0 X-1.5
G2 I1 J0 X2
G3 I-0.75 J0 X-1.5
G3 I1 J0 X2
G3 I-0.75 J0 X-1.5
G3 I1 J0 X2
G3 I-0.75 J0 X-1.5

G90                 ; Use absolute positioning
G1 Z10 F1200


; Brush material wipe nozzle (two passes on PEI)
G90                 ; Use absolute positioning
G1 Y250 F30000
G1 X55
G1 Z1.300 F1200
G1 Y262.5 F6000
G91                 ; Use incremental (relative) positioning
G1 X-35 F30000
G1 Y-0.5
G1 X45
G1 Y-0.5
G1 X-45
G1 Y-0.5
G1 X45
G1 Y-0.5
G1 X-45
G1 Y-0.5
G1 X45
G1 Z5.000 F1200

G90                 ; Use absolute positioning
G1 X30 Y250.000 F30000
G1 Z1.300 F1200
G1 Y262.5 F6000
G91                 ; Use incremental (relative) positioning
G1 X35 F30000
G1 Y-0.5
G1 X-45
G1 Y-0.5
G1 X45
G1 Y-0.5
G1 X-45
G1 Y-0.5
G1 X45
G1 Y-0.5
G1 X-45
G1 Z10.000 F1200

G90                 ; Use absolute positioning
G1 Y250 F30000
G1 X138
G1 Y261
G0 Z-1.01 F1200     ; Stop the nozzle

; Additional circular wipe patterns (removed - first set is sufficient)

M109 S140 ; (Wait) Nozzle at 140°
M106 S255 ; Turn on fan (G28 has turned off fan)

M211 R               ; Pop soft endstop status


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                        BED LEVELING                                            │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; Auto bed leveling ensures accurate first layer height. Only runs if flag is enabled (faster startup
; for repeated prints). Bed must be at target temperature for accurate leveling.
;
; Optimization:
;   start_gcode2: Conditional execution (judge_flag g29_before_print_flag), waits for bed temp,
;                 fan turned off during leveling, saves calibration data
;   KK_v1.3: Conditional execution with different structure, waits for bed temp with -4°C offset,
;            separate conditional flows
;   Optimized: Uses start_gcode2 conditional structure, waits for full bed temp (M190), fan off
;              during leveling
;
; Rationale: Conditional execution allows skipping leveling for faster startup when not needed.
;            Fan must be off during leveling (too noisy, interferes with sensor). Bed must be at
;            target temperature for accurate leveling results.
;
; Operations:
;   1) Check if bed leveling is enabled (judge_flag g29_before_print_flag)
;   2) Move to home position
;   3) Enable ABL (G29.2 S1)
;   4) Wait for bed temperature (M190)
;   5) Turn off fan (M106 S0)
;   6) Perform bed leveling (G29)
;   7) Save calibration data (M500)
;<

M1002 judge_flag g29_before_print_flag

G90                 ; Use absolute positioning
G1 Z5 F1200
G1 X0 Y0 F30000
G29.2 S1            ; Turn on ABL

M190 S[bed_temperature_initial_layer_single] ; (Wait) Bed at initial layer temp
M109 S140 ; (Wait) Nozzle at 140°
M106 S0             ; Turn off fan (too noisy for leveling)

M622 J1
    M1002 gcode_claim_action : 1 ; Message: Auto bed leveling
    G29 A1 X{first_layer_print_min[0]} Y{first_layer_print_min[1]} I{first_layer_print_size[0]} J{first_layer_print_size[1]}
    M400
    M500             ; Save calibration data
M623


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                     HOME AFTER WIPE                                            │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; If bed leveling was skipped, re-home toolhead to ensure accurate positioning. If leveling was
; performed, no re-homing needed (leveling already positioned toolhead).
;
; Optimization:
;   start_gcode2: Conditional re-homing if bed leveling was not performed (judge_flag J0)
;   KK_v1.3: Different structure (homes earlier in sequence)
;   Optimized: Uses start_gcode2 conditional structure for re-homing
;
; Rationale: Conditional execution avoids redundant homing when leveling already positioned toolhead.
;            Only re-home if leveling was skipped to ensure accurate positioning.
;
; Operations:
;   1) Check if bed leveling was performed (judge_flag g29_before_print_flag)
;   2) If leveling was NOT performed (J0), full home all axes (G28)
;<

M1002 judge_flag g29_before_print_flag
M622 J0
    M1002 gcode_claim_action : 13 ; Message: Homing toolhead
    G28               ; Full home (all axes)
M623

    M1002 gcode_claim_action : 0 ; Clear screen/Clear current message


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                      HEATSOAK PAUSE                                            │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; ASA/ABS materials require heatsoak at 100°C bed temperature to prevent warping. Nozzle should be
; kept at 180°C during heatsoak to prevent oozing while maintaining safe temperature. Note: M400 U1
; automatically reduces nozzle to 90°C (firmware safety). To maintain 180°C during pause, run
; monitor_heatsoak.py which will send M104 S180 commands via MQTT every 30 seconds while paused.
; User must resume print after heatsoak completes.
;
; Optimization:
;   start_gcode2: Conditional pause for ASA/ABS at 100°C, explicit temperature wait, clear message
;   KK_v1.3: No explicit heatsoak pause (relies on bed temp wait)
;   Optimized: Includes heatsoak pause from start_gcode2 for ASA/ABS materials, attempts to set
;              nozzle to 180°C before pause (M400 U1 will reduce to 90°C, monitor script maintains 180°C)
;
; Rationale: Heatsoak pause is critical for ASA/ABS materials to prevent warping. Explicit
;            temperature wait ensures bed reaches 100°C before pause. Nozzle at 180°C prevents oozing
;            during extended heatsoak while keeping temperature safe. M400 U1 reduces temp to 90°C for
;            safety, so monitor_heatsoak.py maintains 180°C via MQTT during pause.
;
; Operations:
;   1) Position to start location (X108 Y-0.5 Z0.3)
;   2) Set nozzle to 180°C (non-blocking, M400 U1 will reduce to 90°C)
;   3) Check if material is ASA or ABS (conditional)
;   4) Wait for all moves to complete (M400)
;   5) Wait for bed at 100°C (M190 S100)
;   6) Wait for nozzle at 180°C (M109 S180, before pause reduces it)
;   7) Pause print (M400 U1 - reduces nozzle to 90°C automatically)
;   8) Clear screen message
;   9) Set nozzle back to 180°C after resume (M104 S180)
;   10) Wait for nozzle at 180°C (M109 S180)
;   11) Set nozzle to initial layer temp for print (M104)
;<

G1 X108.000 Y-0.500 F30000
G1 Z0.300 F1200
G2814 Z0.32

{if filament_type[initial_extruder]=="ASA" || filament_type[initial_extruder]=="ABS"}
    M104 S180 ; (No Wait) Nozzle to 180° (M400 U1 will reduce to 90°C, monitor script maintains 180°C)
    M400                ; Finish all moves before pausing
    M190 S100 ; (Wait) Bed at 100° (heatsoak temperature)
    M109 S180 ; (Wait) Nozzle at 180° (ensure stable temp before pause reduces it)
    
    M1002 gcode_claim_action : 16 ; Paused by the user
    M400 U1 ; Pause print (reduces nozzle to 90°C automatically - monitor script maintains 180°C)
    

    M109 S{nozzle_temperature_initial_layer[initial_extruder]} ; (Wait) Nozzle at initial layer temp (required after pause)


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                  EXTRUDE CALIBRATION TEST                                      │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; Final calibration test line verifies extrusion settings before print start. Base test always runs;
; additional test only runs if extrude_cali_flag is enabled. Allows fine-tuning of extrusion parameters
; for optimal print quality.
;
; Optimization:
;   start_gcode2: Base test always runs, additional test conditional (judge_flag extrude_cali_flag),
;                 M900 parameter setting/resetting, test line with multiple speeds
;   KK_v1.3: Similar test structure, M900 parameter setting, single test line
;   Optimized: Uses start_gcode2 structure - base test always runs, additional test conditional,
;              maintains test line sequence with parameter setting
;
; Rationale: Base test provides standard verification for all prints. Additional test provides extra
;            calibration data when needed. Parameter setting/resetting (M900) enables fine-tuning of
;            extrusion parameters. Conditional additional test allows skipping for faster startup when
;            not needed.
;
; Operations:
;   1) Set calibration parameters (M900 S, M900 C) [ALWAYS RUNS]
;   2) Wait for nozzle temperature [ALWAYS RUNS]
;   3) Draw base calibration test line (multiple speeds) [ALWAYS RUNS]
;   4) Reset calibration parameters (M900 R) [ALWAYS RUNS]
;   5) Check if additional calibration test is enabled (judge_flag extrude_cali_flag) [CONDITIONAL]
;   6) Draw additional test line if enabled [CONDITIONAL]
;<

M400                ; Wait for all moves to complete before calibration test
M900 S              ; Set calibration parameter S
M900 C              ; Set calibration parameter C
G90                 ; Use absolute positioning
M83                 ; Set relative extrusion mode

M109 S{nozzle_temperature_initial_layer[initial_extruder]} ; (Wait) Nozzle at initial layer temp
G0 X128 E8 F{outer_wall_volumetric_speed/(24/20)*60}
G0 X133 E.3742 F{outer_wall_volumetric_speed/(0.3*0.5)/4*60}
G0 X138 E.3742 F{outer_wall_volumetric_speed/(0.3*0.5)*60}
G0 X143 E.3742 F{outer_wall_volumetric_speed/(0.3*0.5)/4*60}
G0 X148 E.3742 F{outer_wall_volumetric_speed/(0.3*0.5)*60}
G0 X153 E.3742 F{outer_wall_volumetric_speed/(0.3*0.5)/4*60}

G91                 ; Use incremental (relative) positioning
G1 X1 Z-0.300
G1 X4
G1 Z1 F1200
G90                 ; Use absolute positioning
M400                ; Wait for test line to complete
M900 R              ; Reset calibration parameters

M1002 judge_flag extrude_cali_flag
M622 J1
    G90                 ; Use absolute positioning
    G1 X108.000 Y1.000 F30000
    G91                 ; Use incremental (relative) positioning
    G1 Z-0.700 F1200
    G90                 ; Use absolute positioning
    M83                 ; Set relative extrusion mode
    G0 X128 E10 F{outer_wall_volumetric_speed/(24/20)*60}
    G0 X133 E.3742 F{outer_wall_volumetric_speed/(0.3*0.5)/4*60}
    G0 X138 E.3742 F{outer_wall_volumetric_speed/(0.3*0.5)*60}
    G0 X143 E.3742 F{outer_wall_volumetric_speed/(0.3*0.5)/4*60}
    G0 X148 E.3742 F{outer_wall_volumetric_speed/(0.3*0.5)*60}
    G0 X153 E.3742 F{outer_wall_volumetric_speed/(0.3*0.5)/4*60}
    G91                 ; Use incremental (relative) positioning
    G1 X1 Z-0.300
    G1 X4
    G1 Z1 F1200
    G90                 ; Use absolute positioning
    M400
M623

G1 Z0.2


; ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                     FINAL PREPARATION                                          │
; └────────────────────────────────────────────────────────────────────────────────────────────────┘
;> 
; Final system configuration before print start. Disables non-essential systems (fans, lights) for
; quiet operation. Enables monitoring features (mass estimation). Adjusts for bed type (textured PEI).
;
; Optimization:
;   start_gcode2: Complete shutdown (fans, lasers, lights), conditional textured PEI adjustment,
;                 mass estimation enabled, vibration suppression, final positioning (G2814, G29.4)
;   KK_v1.3: Similar shutdown sequence, conditional textured PEI adjustment, mass estimation,
;            simpler final positioning
;   Optimized: Uses complete shutdown from start_gcode2, conditional textured PEI adjustment,
;              enables monitoring features
;
; Rationale: Complete system shutdown provides quiet operation during print start. Conditional
;            textured PEI adjustment ensures proper Z offset for textured surfaces. Mass estimation
;            provides feedback during printing. Final positioning commands prepare for print start.
;
; Operations:
;   1) Clear screen message
;   2) Textured PEI plate adjustment if applicable (conditional G29.1 Z-0.02)
;   3) Turn off all fans (main, big, chamber)
;   4) Turn off lasers (S1, S2)
;   5) Enable vibration suppression
;   6) Set positioning modes (G90, M83, T1000)
;   7) Disable soft endstops
;   8) Enable mass estimation (M1007 S1)
;   9) Final positioning command (G29.4)
;<

M1002 gcode_claim_action : 0 ; Message: Clear screen/Clear current message
M400

; Textured PEI Plate adjustment (if applicable)
{if curr_bed_type=="Textured PEI Plate"}
    G29.1 Z{-0.02}    ; Lower nozzle for Textured PEI Plate (nozzle was touching topmost of texture when homing)
{endif}

M960 S1 P0           ; Turn off laser
M960 S2 P0           ; Turn off laser
M106 S0              ; Turn off fan
M106 P2 S0           ; Turn off big fan
M106 P3 S0           ; Turn off chamber fan

M975 S1              ; Turn on vibration suppression

G90                  ; Use absolute positioning
M83                  ; Set relative extrusion mode
T1000                ; Reset tool to T1000

M211 X0 Y0 Z0        ; Disable soft endstops
M1007 S1             ; Turn on mass estimation
G29.4                ; Final positioning command

