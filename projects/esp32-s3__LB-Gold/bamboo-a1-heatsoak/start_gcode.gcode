; Bamboo Labs A1 Start G-code with Heat Soak Pause
; This G-code sets initial temperatures, homes axes, then pauses
; waiting for external resume command based on chamber temperature

; Set hotend temperature to 150°C (will hold until resume)
M104 S150

; Set bed temperature (uses slicer variable for initial layer temp)
M140 S[bed_temperature_initial_layer_single]

; Home all axes
G28

; Wait for bed to reach temperature
M190 S[bed_temperature_initial_layer_single]

; Wait for hotend to reach 150°C
M109 S150

; Finish all moves and clear buffer
M400

; Pause print - waits for resume command (M24 or via API)
; The printer will maintain temperatures while paused
M25

; Resume point - execution continues here after resume command
; Normal print start continues below
