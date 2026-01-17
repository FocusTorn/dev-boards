;===== machine: A1 =========================
;===== date: 20240620 =====================

G392 S0     ; Reset clog detection sensor sensitivity
M9833.2     ; Internal logging/state initialization
;M400       ; Wait for moves to finish
;M73 P1.717 ; Set print progress percentage

;- start to heat heatbead&hotend ------------------------------------------------ 
M1002 gcode_claim_action : 2 ; Update Printer Screen Status: "Heating"
M1002 set_filament_type:{filament_type[initial_no_support_extruder]} ; Tell printer firmware what filament is loaded
M104 S140 ; Set nozzle temp to 140C (standby temp to prevent oozing)
M140 S[bed_temperature_initial_layer_single] ; Set bed temp to target (from slicer)



;- avoid end stop --------------------------------------------------------------- 
G91 ; Relative positioning
G380 S2 Z40 F1200 ; Bambu specific safe Z-move (lift nozzle)
G380 S3 Z-15 F1200 ; Bambu specific safe Z-move
G90 ; Absolute positioning

;- reset machine status --------------------------------------------------------- 
;M290 X39 Y39 Z8 ; Baby-stepping reset
M204 S6000 ; Set default acceleration to 6000 mm/s^2

M630 S0 P0 ; Reset First Layer Inspection/Lidar status (though A1 has no Lidar, code remains generic)
G91
M17 Z0.3 ; Lower Z-motor current (stealth/safe mode)

G90
M17 X0.65 Y1.2 Z0.6 ; Reset motor currents to run-mode defaults
M960 S5 P1 ; Turn on Toolhead Logo Lamp
G90
M220 S100 ; Reset Feedrate (Speed) to 100%
M221 S100 ; Reset Flowrate to 100%
M73.2 R1.0 ; Reset time estimation magnitude
;M211 X0 Y0 Z0 ; Turn off soft endstops

;====== cog noise reduction=================
M982.2 S1 ; Turn on Active Motor Noise Cancellation

M1002 gcode_claim_action : 13 ; Update Screen Status: "Homing"

G28 X ; Home X axis
G91
G1 Z5 F1200 ; Lift Z by 5mm
G90
G0 X128 F30000 ; Move X to center
G0 Y254 F3000 ; Move Y to back (near wiper)
G91
G1 Z-5 F1200 ; Lower Z back down

M109 S25 H140 ; Wait for nozzle to reach at least 25C, but allow up to 140C

M17 E0.3 ; Set Extruder motor current low
M83 ; Relative extrusion mode
G1 E10 F1200 ; Prime extruder 10mm
G1 E-0.5 F30 ; Retract 0.5mm
M17 D ; Restore default extruder current

G28 Z P0 T140; Home Z axis with low precision, allow nozzle temp up to 140C
M104 S{nozzle_temperature_initial_layer[initial_extruder]} ; Set target print temp

M1002 judge_flag build_plate_detect_flag ; Check if build plate detection is enabled
M622 S1 ; If flag is true:
  G39.4 ; Execute Build Plate Detection (Eddy sensor check)
  G90
  G1 Z5 F1200 ; Lift Z
M623 ; End If

;===== prepare print temperature and material ==========
M1002 gcode_claim_action : 24 ; Update Screen Status: "Changing Filament" (if needed)

M400 ; Wait for moves
M211 X0 Y0 Z0 ; Turn off soft endstops (allow full travel)
M975 S1 ; Turn on Vibration Compensation (Input Shaping)

G90
G1 X-28.5 F30000 ; Move to purge area (far left)
G1 X-48.2 F3000 ; Move into the cutter lever area

M620 M ; Enable AMS mapping
M620 S[initial_no_support_extruder]A   ; Switch material if AMS exists
    M1002 gcode_claim_action : 4 ; Update Screen Status: "Heating Nozzle"
    M400
    M1002 set_filament_type:UNKNOWN
    M109 S[nozzle_temperature_initial_layer] ; Wait for print temp
    M104 S250 ; Set temp to 250 (flush temp)
    M400
    T[initial_no_support_extruder] ; Select the correct tool/filament
    G1 X-48.2 F3000
    M400

    ; Calculate flush volume based on filament data
    M620.1 E F{filament_max_volumetric_speed[initial_no_support_extruder]/2.4053*60} T{nozzle_temperature_range_high[initial_no_support_extruder]}
    M109 S250 ; Ensure nozzle is at flush temp
    M106 P1 S0 ; Turn off part fan
    G92 E0 ; Reset extruder position
    G1 E50 F200 ; Purge 50mm of filament
    M400
    M1002 set_filament_type:{filament_type[initial_no_support_extruder]}
M621 S[initial_no_support_extruder]A ; Finish AMS mapping

M109 S{nozzle_temperature_range_high[initial_no_support_extruder]} H300 ; Wait for high temp
G92 E0
G1 E50 F200 ; Purge more filament
M400
M106 P1 S178 ; Turn on part fan partially
G92 E0
G1 E5 F200 ; Small purge
M104 S{nozzle_temperature_initial_layer[initial_no_support_extruder]} ; Set final print temp
G92 E0
G1 E-0.5 F300 ; Retract

; Wipe nozzle on the purge wiper (the "Shake" move)
G1 X-28.5 F30000
G1 X-48.2 F3000
G1 X-28.5 F30000 ; Wipe and shake
G1 X-48.2 F3000
G1 X-28.5 F30000 ; Wipe and shake
G1 X-48.2 F3000

M400
M106 P1 S0 ; Turn off fan
;===== prepare print temperature and material end =====

;===== auto extrude cali start =========================
; This is the Flow Dynamics Calibration (Pressure Advance)
M975 S1 ; Enable vibration compensation

G90
M83
T1000 ; Select tool
G1 X-48.2 Y0 Z10 F10000 ; Move to calibration area
M400
M1002 set_filament_type:UNKNOWN

M412 S1 ; Turn on Filament Runout Detection
M400 P10
M620.3 W1; Turn on Filament Tangle Detection (A1 specific)
M400 S2

M1002 set_filament_type:{filament_type[initial_no_support_extruder]}

M1002 judge_flag extrude_cali_flag ; Check if Flow Calibration is enabled in slicer

M622 J1 ; If Flow Calibration is ON:
    M1002 gcode_claim_action : 8 ; Update Screen Status: "Calibrating Extrusion"

    M109 S{nozzle_temperature[initial_extruder]} ; Wait for temp
    G1 E10 F{outer_wall_volumetric_speed/2.4*60} ; Prime
    M983 F{outer_wall_volumetric_speed/2.4} A0.3 H[nozzle_diameter]; Perform Dynamic Extrusion Calibration (K-factor)

    M106 P1 S255 ; Fan full speed
    M400 S5
    ; Wipe nozzle sequence
    G1 X-28.5 F18000
    G1 X-48.2 F3000
    G1 X-28.5 F18000
    G1 X-48.2 F3000
    G1 X-28.5 F12000
    G1 X-48.2 F3000
    M400
    M106 P1 S0

    M1002 judge_last_extrude_cali_success ; Check if calibration worked
    M622 J0 ; If it failed or needs retry:
        M983 F{outer_wall_volumetric_speed/2.4} A0.3 H[nozzle_diameter]; Retry Calibration
        M106 P1 S255
        M400 S5
        ; Wipe again
        G1 X-28.5 F18000
        G1 X-48.2 F3000
        G1 X-28.5 F18000
        G1 X-48.2 F3000
        G1 X-28.5 F12000
        M400
        M106 P1 S0
    M623
    
    G1 X-48.2 F3000
    M400
    ; Draw the calibration lines (The zig-zags on the edge of the plate)
    M984 A0.1 E1 S1 F{outer_wall_volumetric_speed/2.4} H[nozzle_diameter]
    M106 P1 S178
    M400 S7
    ; Final wipe
    G1 X-28.5 F18000
    G1 X-48.2 F3000
    G1 X-28.5 F18000
    G1 X-48.2 F3000
    G1 X-28.5 F12000
    G1 X-48.2 F3000
    M400
    M106 P1 S0
M623 ; End of Flow Calibration

;===== auto extrude cali end ========================

M104 S170 ; Cool nozzle to 170C for wiping (prevents oozing while probing)
M106 S255 ; Turn on fan to cool nozzle faster

;===== mech mode fast check start =====================
; This is the Vibration Compensation / Resonance check (Shaking)
M1002 gcode_claim_action : 3 ; Update Screen Status: "Vibration Calibration"

G1 X128 Y128 F20000 ; Move to center
G1 Z5 F1200
M400 P200
M970.3 Q1 A5 K0 O3 ; Start X-axis frequency sweep (Shake X)
M974 Q1 S2 P0 ; Analyze X-axis data

M970.2 Q1 K1 W58 Z0.1 ; Fine tune X
M974 S2

G1 X128 Y128 F20000
G1 Z5 F1200
M400 P200
M970.3 Q0 A10 K0 O1 ; Start Y-axis frequency sweep (Shake Y)
M974 Q0 S2 P0 ; Analyze Y-axis data

M970.2 Q0 K1 W78 Z0.1 ; Fine tune Y
M974 S2

M975 S1 ; Enable the calculated compensation
G1 F30000
G1 X0 Y5
G28 X ; Re-home X axis to ensure position accuracy after shaking

G1 Z4 F1200

;===== mech mode fast check end =======================

;===== wipe nozzle ===============================
M1002 gcode_claim_action : 14 ; Update Screen Status: "Cleaning Nozzle"

M975 S1
M106 S255 ; Fan on
M211 S; Push soft endstop status
M211 X0 Y0 Z0 ; Turn off Z axis endstop

;===== remove waste by touching start =====
; This section moves the nozzle to the waste chute area to knock off poop

M104 S170 ; Ensure temp is 170C

M83
G1 E-1 F500 ; Retract
G90
M83

M109 S170 ; Wait for 170C
G0 X108 Y-0.5 F30000 ; Move to purge area
; The following block is a "pecking" motion to knock waste down the chute
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X110 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X112 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X114 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X116 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X118 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X120 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X122 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X124 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X126 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X128 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X130 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X132 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X134 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X136 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X138 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X140 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X142 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X144 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X146 F10000
G380 S3 Z-5 F1200
G1 Z2 F1200
G1 X148 F10000
G380 S3 Z-5 F1200

G1 Z5 F30000
;===== remove waste by touching end =====

G1 Z10 F1200
G0 X118 Y261 F30000 ; Move to the silicone wiper at the back of the bed
G1 Z5 F1200
M109 S{nozzle_temperature_initial_layer[initial_extruder]-50} ; Cool down further for wiping

G28 Z P0 T300; Home Z with low precision
G29.2 S0 ; Turn off ABL (Auto Bed Leveling)
M104 S140 ; Set temp to 140C
G0 Z5 F20000

G0 X128 Y261 F20000  ; Move to exposed steel surface (wiper area)
G0 Z-1.01 F1200      ; Lower nozzle to touch the wiper

; Perform circular wiping motions (G2 = Arc Move)
G91
G2 I1 J0 X2 Y0 F2000.1
G2 I-0.75 J0 X-1.5
G2 I1 J0 X2
G2 I-0.75 J0 X-1.5
G2 I1 J0 X2
G2 I-0.75 J0 X-1.5
G2 I1 J0 X2
G2 I-0.75 J0 X-1.5
G2 I1 J0 X2
G2 I-0.75 J0 X-1.5
G2 I1 J0 X2
G2 I-0.75 J0 X-1.5
G2 I1 J0 X2
G2 I-0.75 J0 X-1.5
G2 I1 J0 X2
G2 I-0.75 J0 X-1.5
G2 I1 J0 X2
G2 I-0.75 J0 X-1.5
G2 I1 J0 X2
G2 I-0.75 J0 X-1.5

G90
G1 Z10 F1200

;===== brush material wipe nozzle =====
; Move to the brush area (if equipped/applicable on specific A1 revision)
G90
G1 Y250 F30000
G1 X55
G1 Z1.300 F1200
G1 Y262.5 F6000
G91
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

G90
G1 X30 Y250.000 F30000
G1 Z1.300 F1200
G1 Y262.5 F6000
G91
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

;===== brush material wipe nozzle end =====

G90
G1 Y250 F30000
G1 X138
G1 Y261
G0 Z-1.01 F1200      ; Stop the nozzle

; More circular wiping motions
G91
G2 I1 J0 X2 Y0 F2000.1
G2 I-0.75 J0 X-1.5
G2 I1 J0 X2
G2 I-0.75 J0 X-1.5
G2 I1 J0 X2
G2 I-0.75 J0 X-1.5
G2 I1 J0 X2
G2 I-0.75 J0 X-1.5
G2 I1 J0 X2
G2 I-0.75 J0 X-1.5
G2 I1 J0 X2
G2 I-0.75 J0 X-1.5
G2 I1 J0 X2
G2 I-0.75 J0 X-1.5
G2 I1 J0 X2
G2 I-0.75 J0 X-1.5
G2 I1 J0 X2
G2 I-0.75 J0 X-1.5
G2 I1 J0 X2
G2 I-0.75 J0 X-1.5

M109 S140
M106 S255 ; Turn on fan

M211 R; Pop softend status (Restore endstops)

;===== wipe nozzle end ================================

; BED LEVELING ---------------------------------------------------------------->> 
M1002 judge_flag g29_before_print_flag ; Check if Bed Leveling is enabled in slicer

G90
G1 Z5 F1200
G1 X0 Y0 F30000
G29.2 S1 ; Turn on ABL

M190 S[bed_temperature_initial_layer_single]; Wait for Bed Temp
M109 S140 ; Nozzle at 140C (prevents oozing during probing)
M106 S0 ; Turn off fan (reduces vibration/noise for probe)

M622 J1 ; If Bed Leveling is ON:
    M1002 gcode_claim_action : 1 ; Update Screen Status: "Bed Leveling"
    ; Run Auto Bed Leveling (Mesh Bed Leveling)
    G29 A1 X{first_layer_print_min[0]} Y{first_layer_print_min[1]} I{first_layer_print_size[0]} J{first_layer_print_size[1]}
    M400
    M500 ; Save calibration data
M623

;-------------------------------------------------------------------------------------------------<<
; HOME AFTER WIPE MOUTH ------------------------------------------------------->> 
M1002 judge_flag g29_before_print_flag
M622 J0 ; If Bed Leveling was OFF:
    M1002 gcode_claim_action : 13 ; Update Screen Status: "Homing"
    G28 ; Re-home axes
M623

;-------------------------------------------------------------------------------------------------<<


G1 X108.000 Y-0.500 F30000
G1 Z0.300 F1200
M400
G2814 Z0.32 ; Set Z-offset (Proprietary command)









M104 S{nozzle_temperature_initial_layer[initial_extruder]} ; Set final print temp

;===== nozzle load line ===============================
; (This section is commented out in original, standard prime line logic)
;G90
;M83
;G1 Z5 F1200
;G1 X88 Y-0.5 F20000
;G1 Z0.3 F1200
;M109 S{nozzle_temperature_initial_layer[initial_extruder]}
;G1 E2 F300
;G1 X168 E4.989 F6000
;G1 Z1 F1200
;===== nozzle load line end ===========================

;===== extrude cali test ===============================
; This section draws the "Prime Line" at the back of the plate
M400
    M900 S ; Linear Advance Start?
    M900 C
    G90
    M83

    M109 S{nozzle_temperature_initial_layer[initial_extruder]} ; Wait for print temp
    ; Draw the prime line (variable speed based on volumetric flow)
    G0 X128 E8  F{outer_wall_volumetric_speed/(24/20)    * 60}
    G0 X133 E.3742  F{outer_wall_volumetric_speed/(0.3*0.5)/4     * 60}
    G0 X138 E.3742  F{outer_wall_volumetric_speed/(0.3*0.5)     * 60}
    G0 X143 E.3742  F{outer_wall_volumetric_speed/(0.3*0.5)/4     * 60}
    G0 X148 E.3742  F{outer_wall_volumetric_speed/(0.3*0.5)     * 60}
    G0 X153 E.3742  F{outer_wall_volumetric_speed/(0.3*0.5)/4     * 60}
    G91
    G1 X1 Z-0.300
    G1 X4
    G1 Z1 F1200
    G90
    M400

M900 R

M1002 judge_flag extrude_cali_flag
M622 J1 ; If Flow Calibration was ON, draw an extra prime line
    G90
    G1 X108.000 Y1.000 F30000
    G91
    G1 Z-0.700 F1200
    G90
    M83
    G0 X128 E10  F{outer_wall_volumetric_speed/(24/20)    * 60}
    G0 X133 E.3742  F{outer_wall_volumetric_speed/(0.3*0.5)/4     * 60}
    G0 X138 E.3742  F{outer_wall_volumetric_speed/(0.3*0.5)     * 60}
    G0 X143 E.3742  F{outer_wall_volumetric_speed/(0.3*0.5)/4     * 60}
    G0 X148 E.3742  F{outer_wall_volumetric_speed/(0.3*0.5)     * 60}
    G0 X153 E.3742  F{outer_wall_volumetric_speed/(0.3*0.5)/4     * 60}
    G91
    G1 X1 Z-0.300
    G1 X4
    G1 Z1 F1200
    G90
    M400
M623

G1 Z0.2 ; Move Z to 0.2mm

M1002 gcode_claim_action : 0 ; Update Screen Status: "Printing"
M400

;===== for Textured PEI Plate , lower the nozzle as the nozzle was touching topmost of the texture when homing ==
;curr_bed_type={curr_bed_type}
{if curr_bed_type=="Textured PEI Plate"}
G29.1 Z{-0.02} ; Apply Z-offset squish (-0.02mm) for Textured PEI Plate
{endif}

M960 S1 P0  ; Turn off laser (if equipped)
M960 S2 P0  ; Turn off laser
M106 S0     ; Turn off fan
M106 P2 S0  ; Turn off big fan (Aux fan)
M106 P3 S0  ; Turn off chamber fan

M975 S1 ; Ensure vibration compensation is ON
G90
M83
T1000

M211 X0 Y0 Z0 ; Turn off soft endstops
;G392 S1 ; Turn on clog detection
M1007 S1 ; Turn on mass estimation (for dynamics)
G29.4 ; Finalize leveling/compensation