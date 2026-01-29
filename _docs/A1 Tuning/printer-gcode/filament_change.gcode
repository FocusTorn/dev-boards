;===== A1 20240913 =======================
; ┌────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
; │                                    FILAMENT CHANGE SEQUENCE                                                       │
; │                                    Bambu Lab A1 Tool Change Script                                                │
; └────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘

;== INITIALIZATION ===========================================================================
M1007 S0 ; Turn off mass estimation (prevents interference during tool change)
G392 S0 ; Disable clog detection during tool change
M620 S[next_extruder]A ; Prepare tool change for next extruder
M204 S9000 ; Set high acceleration for rapid positioning moves

;== LIFT FROM PRINT ===========================================================================
{if toolchange_count > 1}
G17 ; Select XY plane for circular interpolation
G2 Z{max_layer_z + 0.4} I0.86 J0.86 P1 F10000 ; Spiral lift 0.4mm from second tool change onward
{endif}
G1 Z{max_layer_z + 3.0} F1200 ; Lift Z to 3mm above current layer height

M400 ; [Wait] Wait for all moves to complete before proceeding

;== PREPARE FOR TOOL CHANGE ===========================================================================
M106 P1 S0 ; Turn off auxiliary fan (P1)
M106 P2 S0 ; Turn off chamber fan (P2)

{if old_filament_temp > 142 && next_extruder < 255}
M104 S[old_filament_temp] ; [No Wait] Set old filament temperature (if above 142°C)
{endif}

G1 X267 F18000 ; Move to tool change position (X=267mm)

;== RETRACT OLD FILAMENT ===========================================================================
{if long_retractions_when_cut[previous_extruder]}
M620.11 S1 I[previous_extruder] E-{retraction_distances_when_cut[previous_extruder]} F1200 ; Long retraction for cut filament
{else}
M620.11 S0 ; No long retraction needed
{endif}
M400 ; [Wait] Wait for retraction to complete

;== CONFIGURE TOOL CHANGE ===========================================================================
M620.1 E F[old_filament_e_feedrate] T{nozzle_temperature_range_high[previous_extruder]} ; Configure old filament parameters
M620.10 A0 F[old_filament_e_feedrate] ; Set old filament feedrate (A0 = old filament)
T[next_extruder] ; Select next extruder tool
M620.1 E F[new_filament_e_feedrate] T{nozzle_temperature_range_high[next_extruder]} ; Configure new filament parameters
M620.10 A1 F[new_filament_e_feedrate] L[flush_length] H[nozzle_diameter] T[nozzle_temperature_range_high] ; Set new filament feedrate and flush parameters (A1 = new filament)

G1 Y128 F9000 ; Move to Y position for tool change

;== FILAMENT LOADING SEQUENCE ===========================================================================
{if next_extruder < 255}

{if long_retractions_when_cut[previous_extruder]}
M620.11 S1 I[previous_extruder] E{retraction_distances_when_cut[previous_extruder]} F{old_filament_e_feedrate} ; Restore long retraction distance
M628 S1 ; Enable filament loading mode
G92 E0 ; Reset extruder position to zero
G1 E{retraction_distances_when_cut[previous_extruder]} F[old_filament_e_feedrate] ; Extrude to restore position
M400 ; [Wait] Wait for extrusion to complete
M629 S1 ; Disable filament loading mode
{else}
M620.11 S0 ; No long retraction restoration needed
{endif}

M400 ; [Wait] Wait for all moves to complete
G92 E0 ; Reset extruder position to zero
M628 S0 ; Disable filament loading mode

;== FLUSH SEQUENCE 1 ===========================================================================
{if flush_length_1 > 1}
; FLUSH_START
; Always use highest temperature to flush old filament
M400 ; [Wait] Wait for all moves to complete
M1002 set_filament_type:UNKNOWN ; Set filament type to UNKNOWN for flushing
M109 S[nozzle_temperature_range_high] ; [Wait] Wait for nozzle to reach max temperature for flushing
M106 P1 S60 ; Turn on auxiliary fan at 60% for cooling during flush

{if flush_length_1 > 23.7}
; Pulsatile flushing pattern for longer flush lengths
G1 E23.7 F{old_filament_e_feedrate} ; Initial flush with old filament feedrate (no pulsatile needed for start)
G1 E{(flush_length_1 - 23.7) * 0.02} F50 ; Slow pulse 1 (2% of remaining flush)
G1 E{(flush_length_1 - 23.7) * 0.23} F{old_filament_e_feedrate} ; Fast flush with old filament (23% of remaining)
G1 E{(flush_length_1 - 23.7) * 0.02} F50 ; Slow pulse 2 (2% of remaining flush)
G1 E{(flush_length_1 - 23.7) * 0.23} F{new_filament_e_feedrate} ; Fast flush with new filament (23% of remaining)
G1 E{(flush_length_1 - 23.7) * 0.02} F50 ; Slow pulse 3 (2% of remaining flush)
G1 E{(flush_length_1 - 23.7) * 0.23} F{new_filament_e_feedrate} ; Fast flush with new filament (23% of remaining)
G1 E{(flush_length_1 - 23.7) * 0.02} F50 ; Slow pulse 4 (2% of remaining flush)
G1 E{(flush_length_1 - 23.7) * 0.23} F{new_filament_e_feedrate} ; Fast flush with new filament (23% of remaining)
{else}
G1 E{flush_length_1} F{old_filament_e_feedrate} ; Simple flush for shorter lengths
{endif}
; FLUSH_END
G1 E-[old_retract_length_toolchange] F1800 ; Retract old filament
G1 E[old_retract_length_toolchange] F300 ; Slow re-extrude to prevent oozing
M400 ; [Wait] Wait for all moves to complete
M1002 set_filament_type:{filament_type[next_extruder]} ; Set new filament type
{endif}

;== WIPE SEQUENCE 1 ===========================================================================
{if flush_length_1 > 45 && flush_length_2 > 1}
; WIPE
M400 ; [Wait] Wait for all moves to complete
M106 P1 S178 ; Turn on auxiliary fan at 178 (high speed for wiping)
M400 S3 ; [Wait] Wait 3 seconds for fan to reach speed
G1 X-38.2 F18000 ; Wipe move 1 (fast)
G1 X-48.2 F3000 ; Wipe move 2 (slow)
G1 X-38.2 F18000 ; Wipe move 3 (fast)
G1 X-48.2 F3000 ; Wipe move 4 (slow)
G1 X-38.2 F18000 ; Wipe move 5 (fast)
G1 X-48.2 F3000 ; Wipe move 6 (slow)
M400 ; [Wait] Wait for all moves to complete
M106 P1 S0 ; Turn off auxiliary fan
{endif}

;== FLUSH SEQUENCE 2 ===========================================================================
{if flush_length_2 > 1}
M106 P1 S60 ; Turn on auxiliary fan at 60% for cooling
; FLUSH_START
; Pulsatile flushing pattern for flush 2
G1 E{flush_length_2 * 0.18} F{new_filament_e_feedrate} ; Fast flush 1 (18% of flush length)
G1 E{flush_length_2 * 0.02} F50 ; Slow pulse 1 (2% of flush length)
G1 E{flush_length_2 * 0.18} F{new_filament_e_feedrate} ; Fast flush 2 (18% of flush length)
G1 E{flush_length_2 * 0.02} F50 ; Slow pulse 2 (2% of flush length)
G1 E{flush_length_2 * 0.18} F{new_filament_e_feedrate} ; Fast flush 3 (18% of flush length)
G1 E{flush_length_2 * 0.02} F50 ; Slow pulse 3 (2% of flush length)
G1 E{flush_length_2 * 0.18} F{new_filament_e_feedrate} ; Fast flush 4 (18% of flush length)
G1 E{flush_length_2 * 0.02} F50 ; Slow pulse 4 (2% of flush length)
G1 E{flush_length_2 * 0.18} F{new_filament_e_feedrate} ; Fast flush 5 (18% of flush length)
G1 E{flush_length_2 * 0.02} F50 ; Slow pulse 5 (2% of flush length)
G1 E{flush_length_2 * 0.18} F{new_filament_e_feedrate} ; Fast flush 6 (18% of flush length)
G1 E{flush_length_2 * 0.02} F50 ; Slow pulse 6 (2% of flush length)
; FLUSH_END
G1 E-[new_retract_length_toolchange] F1800 ; Retract new filament
G1 E[new_retract_length_toolchange] F300 ; Slow re-extrude to prevent oozing
{endif}

;== WIPE SEQUENCE 2 ===========================================================================
{if flush_length_2 > 45 && flush_length_3 > 1}
; WIPE
M400 ; [Wait] Wait for all moves to complete
M106 P1 S178 ; Turn on auxiliary fan at 178 (high speed for wiping)
M400 S3 ; [Wait] Wait 3 seconds for fan to reach speed
G1 X-38.2 F18000 ; Wipe move 1 (fast)
G1 X-48.2 F3000 ; Wipe move 2 (slow)
G1 X-38.2 F18000 ; Wipe move 3 (fast)
G1 X-48.2 F3000 ; Wipe move 4 (slow)
G1 X-38.2 F18000 ; Wipe move 5 (fast)
G1 X-48.2 F3000 ; Wipe move 6 (slow)
M400 ; [Wait] Wait for all moves to complete
M106 P1 S0 ; Turn off auxiliary fan
{endif}

;== FLUSH SEQUENCE 3 ===========================================================================
{if flush_length_3 > 1}
M106 P1 S60 ; Turn on auxiliary fan at 60% for cooling
; FLUSH_START
; Pulsatile flushing pattern for flush 3
G1 E{flush_length_3 * 0.18} F{new_filament_e_feedrate} ; Fast flush 1 (18% of flush length)
G1 E{flush_length_3 * 0.02} F50 ; Slow pulse 1 (2% of flush length)
G1 E{flush_length_3 * 0.18} F{new_filament_e_feedrate} ; Fast flush 2 (18% of flush length)
G1 E{flush_length_3 * 0.02} F50 ; Slow pulse 2 (2% of flush length)
G1 E{flush_length_3 * 0.18} F{new_filament_e_feedrate} ; Fast flush 3 (18% of flush length)
G1 E{flush_length_3 * 0.02} F50 ; Slow pulse 3 (2% of flush length)
G1 E{flush_length_3 * 0.18} F{new_filament_e_feedrate} ; Fast flush 4 (18% of flush length)
G1 E{flush_length_3 * 0.02} F50 ; Slow pulse 4 (2% of flush length)
G1 E{flush_length_3 * 0.18} F{new_filament_e_feedrate} ; Fast flush 5 (18% of flush length)
G1 E{flush_length_3 * 0.02} F50 ; Slow pulse 5 (2% of flush length)
G1 E{flush_length_3 * 0.18} F{new_filament_e_feedrate} ; Fast flush 6 (18% of flush length)
G1 E{flush_length_3 * 0.02} F50 ; Slow pulse 6 (2% of flush length)
; FLUSH_END
G1 E-[new_retract_length_toolchange] F1800 ; Retract new filament
G1 E[new_retract_length_toolchange] F300 ; Slow re-extrude to prevent oozing
{endif}

;== WIPE SEQUENCE 3 ===========================================================================
{if flush_length_3 > 45 && flush_length_4 > 1}
; WIPE
M400 ; [Wait] Wait for all moves to complete
M106 P1 S178 ; Turn on auxiliary fan at 178 (high speed for wiping)
M400 S3 ; [Wait] Wait 3 seconds for fan to reach speed
G1 X-38.2 F18000 ; Wipe move 1 (fast)
G1 X-48.2 F3000 ; Wipe move 2 (slow)
G1 X-38.2 F18000 ; Wipe move 3 (fast)
G1 X-48.2 F3000 ; Wipe move 4 (slow)
G1 X-38.2 F18000 ; Wipe move 5 (fast)
G1 X-48.2 F3000 ; Wipe move 6 (slow)
M400 ; [Wait] Wait for all moves to complete
M106 P1 S0 ; Turn off auxiliary fan
{endif}

;== FLUSH SEQUENCE 4 ===========================================================================
{if flush_length_4 > 1}
M106 P1 S60 ; Turn on auxiliary fan at 60% for cooling
; FLUSH_START
; Pulsatile flushing pattern for flush 4
G1 E{flush_length_4 * 0.18} F{new_filament_e_feedrate} ; Fast flush 1 (18% of flush length)
G1 E{flush_length_4 * 0.02} F50 ; Slow pulse 1 (2% of flush length)
G1 E{flush_length_4 * 0.18} F{new_filament_e_feedrate} ; Fast flush 2 (18% of flush length)
G1 E{flush_length_4 * 0.02} F50 ; Slow pulse 2 (2% of flush length)
G1 E{flush_length_4 * 0.18} F{new_filament_e_feedrate} ; Fast flush 3 (18% of flush length)
G1 E{flush_length_4 * 0.02} F50 ; Slow pulse 3 (2% of flush length)
G1 E{flush_length_4 * 0.18} F{new_filament_e_feedrate} ; Fast flush 4 (18% of flush length)
G1 E{flush_length_4 * 0.02} F50 ; Slow pulse 4 (2% of flush length)
G1 E{flush_length_4 * 0.18} F{new_filament_e_feedrate} ; Fast flush 5 (18% of flush length)
G1 E{flush_length_4 * 0.02} F50 ; Slow pulse 5 (2% of flush length)
G1 E{flush_length_4 * 0.18} F{new_filament_e_feedrate} ; Fast flush 6 (18% of flush length)
G1 E{flush_length_4 * 0.02} F50 ; Slow pulse 6 (2% of flush length)
; FLUSH_END
{endif}

;== FINALIZE FILAMENT LOADING ===========================================================================
M629 ; Disable filament loading mode

;== FINAL TEMPERATURE AND POSITIONING ===========================================================================
M400 ; [Wait] Wait for all moves to complete
M106 P1 S60 ; Turn on auxiliary fan at 60% for cooling
M109 S[new_filament_temp] ; [Wait] Wait for nozzle to reach new filament print temperature
G1 E6 F{new_filament_e_feedrate} ; Compensate for filament spillage during temperature wait
M400 ; [Wait] Wait for all moves to complete
G92 E0 ; Reset extruder position to zero
G1 E-[new_retract_length_toolchange] F1800 ; Final retract
M400 ; [Wait] Wait for all moves to complete
M106 P1 S178 ; Turn on auxiliary fan at 178 (high speed for final wipe)
M400 S3 ; [Wait] Wait 3 seconds for fan to reach speed
G1 X-38.2 F18000 ; Final wipe move 1 (fast)
G1 X-48.2 F3000 ; Final wipe move 2 (slow)
G1 X-38.2 F18000 ; Final wipe move 3 (fast)
G1 X-48.2 F3000 ; Final wipe move 4 (slow)
G1 X-38.2 F18000 ; Final wipe move 5 (fast)
G1 X-48.2 F3000 ; Final wipe move 6 (slow)
G1 X-38.2 F18000 ; Final wipe move 7 (fast)
G1 X-48.2 F3000 ; Final wipe move 8 (slow)
M400 ; [Wait] Wait for all moves to complete
G1 Z{max_layer_z + 3.0} F3000 ; Return to safe Z height
M106 P1 S0 ; Turn off auxiliary fan

;== RESTORE ACCELERATION SETTINGS ===========================================================================
{if layer_z <= (initial_layer_print_height + 0.001)}
M204 S[initial_layer_acceleration] ; Use initial layer acceleration for first layer
{else}
M204 S[default_acceleration] ; Use default acceleration for subsequent layers
{endif}
{else}
G1 X[x_after_toolchange] Y[y_after_toolchange] Z[z_after_toolchange] F12000 ; Move to position after tool change (if next_extruder >= 255)
{endif}

;== DYNAMIC EXTRUSION CALIBRATION ===========================================================================
M622.1 S0 ; Reset calibration flag
M9833 F{outer_wall_volumetric_speed/2.4} A0.3 ; Calibrate dynamic extrusion compensation
M1002 judge_flag filament_need_cali_flag ; Check if filament calibration is needed
M622 J1 ; Start calibration sequence if needed
  G92 E0 ; Reset extruder position to zero
  G1 E-[new_retract_length_toolchange] F1800 ; Retract before calibration wipe
  M400 ; [Wait] Wait for all moves to complete
  
  M106 P1 S178 ; Turn on auxiliary fan at 178 (high speed for calibration wipe)
  M400 S4 ; [Wait] Wait 4 seconds for fan to reach speed
  G1 X-38.2 F18000 ; Calibration wipe move 1 (fast)
  G1 X-48.2 F3000 ; Calibration wipe move 2 (slow)
  G1 X-38.2 F18000 ; Calibration wipe move 3 (fast) - wipe and shake
  G1 X-48.2 F3000 ; Calibration wipe move 4 (slow)
  G1 X-38.2 F12000 ; Calibration wipe move 5 (medium) - wipe and shake
  G1 X-48.2 F3000 ; Calibration wipe move 6 (slow)
  M400 ; [Wait] Wait for all moves to complete
  M106 P1 S0 ; Turn off auxiliary fan
M623 ; End calibration sequence

;== CLEANUP AND RE-ENABLE SYSTEMS ===========================================================================
M621 S[next_extruder]A ; Complete tool change for next extruder
G392 S0 ; Disable clog detection (redundant, but ensures state)

M1007 S1 ; Re-enable mass estimation
