# Makefile for ESP32-S3 BME680 Sensor
# Usage: make [target]

# Configuration
FQBN = esp32:esp32:esp32s3
SKETCH = bme680
PORT = COM9
BAUDRATE = 115200

# Sketch selection: "bme680" (full with calculations) or "bme680-simplified" (raw data only)
# Usage: make SKETCH=bme680-simplified build
#        make SKETCH=bme680 build
# Default: bme680 (full version)
SKETCH_FILE = $(SKETCH).ino
SKETCH_ACTIVE = bme680.ino

PROJECT_ROOT = D:/_dev/_projects/dev-boards/
LIBRARY_PATH = $(PROJECT_ROOT)/_libs/esp32-s3
ARDUINO_CLI = $(PROJECT_ROOT)/Arduino/arduino-cli.exe
SKETCH_DIR = $(PROJECT_ROOT)/projects/esp32-s3__LB-Gold/bme680
BUILD_PATH = $(SKETCH_DIR)/build
PYTHON_SCRIPT = $(SKETCH_DIR)/select_sketch.py

# Colors for output (Git Bash compatible)
GREEN = \033[0;32m
YELLOW = \033[1;33m
RED = \033[0;31m
NC = \033[0m

.PHONY: help build compile progress upload monitor clean all select-sketch select-sketch-interactive select-sketch-apply sketch-status

# Default target
help:
	@echo -e "$(GREEN)ESP32-S3 BME680 Sensor - Build Targets$(NC)"
	@echo ""
	@echo "  make build       - Compile the sketch (alias for compile)"
	@echo "  make compile     - Compile the sketch (verbose output)"
	@echo "  make progress    - Compile with progress bar"
	@echo "  make upload      - Upload (no compile)"
	@echo "  make monitor     - Open serial monitor"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make all         - Compile and upload"
	@echo ""
	@echo "Sketch Selection:"
	@echo "  make SKETCH=bme680 build          - Full version (with calculations)"
	@echo "  make SKETCH=bme680-simplified build - Simplified (raw data only)"
	@echo "  make sketch-status                 - Show current sketch status"
	@echo ""
	@echo "Configuration:"
	@echo "  FQBN: $(FQBN)"
	@echo "  PORT: $(PORT)"
	@echo "  Active Sketch: $(SKETCH_ACTIVE)"
	@echo "  Selected Sketch: $(SKETCH_FILE)"

# Interactive sketch selection (called by select-sketch if SKETCH not set)
select-sketch-interactive:
	@if [ -f "$(PYTHON_SCRIPT)" ]; then \
		echo -e "$(YELLOW)Launching interactive sketch selector...$(NC)"; \
		SELECTED_SKETCH=$$(python3 "$(PYTHON_SCRIPT)" 2>/dev/null || python "$(PYTHON_SCRIPT)" 2>/dev/null); \
		if [ -n "$$SELECTED_SKETCH" ]; then \
			$(MAKE) SKETCH=$$SELECTED_SKETCH select-sketch-apply; \
		else \
			echo -e "$(RED)Error: Failed to select sketch$(NC)"; \
			exit 1; \
		fi; \
	else \
		echo -e "$(YELLOW)Python selector not found, using default: $(SKETCH)$(NC)"; \
		$(MAKE) select-sketch-apply; \
	fi

# Apply selected sketch (internal target)
select-sketch-apply:
	@if [ ! -f "$(SKETCH_DIR)/$(SKETCH_FILE)" ]; then \
		echo -e "$(RED)Error: Sketch file '$(SKETCH_FILE)' not found!$(NC)"; \
		echo "Available sketches:"; \
		ls -1 "$(SKETCH_DIR)"/*.ino 2>/dev/null | xargs -n1 basename || true; \
		exit 1; \
	fi
	@if [ -f "$(SKETCH_DIR)/$(SKETCH_ACTIVE)" ] && [ "$(SKETCH_FILE)" != "$(SKETCH_ACTIVE)" ]; then \
		echo -e "$(YELLOW)Switching sketch: $(SKETCH_ACTIVE) → $(SKETCH_FILE)$(NC)"; \
		cp "$(SKETCH_DIR)/$(SKETCH_FILE)" "$(SKETCH_DIR)/$(SKETCH_ACTIVE)"; \
		echo -e "$(GREEN)✓ Sketch switched to $(SKETCH_FILE)$(NC)"; \
	elif [ "$(SKETCH_FILE)" = "$(SKETCH_ACTIVE)" ]; then \
		echo -e "$(GREEN)✓ Using sketch: $(SKETCH_FILE)$(NC)"; \
	fi

# Select and prepare sketch before building
# If SKETCH was explicitly set (e.g., make SKETCH=bme680 build), use it directly
# Otherwise, launch interactive selector
select-sketch:
	@if [ "$(SKETCH)" != "bme680" ] || [ -n "$(filter SKETCH=%,$(MAKEFLAGS))" ] || [ -n "$(filter SKETCH=%,$(MAKECMDGOALS))" ]; then \
		$(MAKE) select-sketch-apply; \
	else \
		$(MAKE) select-sketch-interactive; \
	fi

# Build/Compile the sketch (build is alias for compile)
build: select-sketch compile

# Compile the sketch
compile: select-sketch
	@echo -e "$(YELLOW)========================================$(NC)"
	@echo -e "$(YELLOW)Compiling $(SKETCH_ACTIVE) ($(SKETCH))...$(NC)"
	@echo -e "$(YELLOW)========================================$(NC)"
	@echo ""
	$(ARDUINO_CLI) compile --fqbn $(FQBN) --libraries "$(LIBRARY_PATH)" --build-path "$(BUILD_PATH)" --verbose "$(SKETCH_DIR)" || (echo -e "$(RED)[FAIL] Compilation failed!$(NC)" && exit 1)
	@echo ""
	@echo -e "$(GREEN)[OK] Compilation successful!$(NC)"

# Compile with progress bar
progress: select-sketch
	@echo -e "$(YELLOW)========================================$(NC)"
	@echo -e "$(YELLOW)Compiling $(SKETCH_ACTIVE) ($(SKETCH)) with progress...$(NC)"
	@echo -e "$(YELLOW)========================================$(NC)"
	@echo ""
	@bash -c '\
		BAR_WIDTH=40; \
		FILES_COMPILED=0; \
		TOTAL_FILES=0; \
		STAGE="initializing"; \
		LAST_PERCENT=0; \
		START_TIME=$$(date +%s); \
		TEMP_OUTPUT=$$(mktemp); \
		ERROR_DETECTED=0; \
		\
		show_progress() { \
			local percent=$$1; \
			local filled=$$((BAR_WIDTH * percent / 100)); \
			local empty=$$((BAR_WIDTH - filled)); \
			printf "\r$(YELLOW)[$(NC)"; \
			for i in $$(seq 1 $$filled); do printf "="; done; \
			for i in $$(seq 1 $$empty); do printf " "; done; \
			printf "$(YELLOW)] $(NC)%3d%%" $$percent; \
		}; \
		\
		check_errors() { \
			if grep -qiE "(fatal error|error:|error during|compilation terminated|Error during build|has no member|undefined reference|multiple definition)" $$TEMP_OUTPUT 2>/dev/null; then \
				ERROR_DETECTED=1; \
				return 0; \
			fi; \
			return 1; \
		}; \
		\
		show_progress 0; \
		\
		{ $(ARDUINO_CLI) compile --fqbn $(FQBN) --libraries "$(LIBRARY_PATH)" --build-path "$(BUILD_PATH)" --verbose "$(SKETCH_DIR)" >$$TEMP_OUTPUT 2>&1; \
		  EXIT_CODE=$$?; } & \
		COMPILE_PID=$$!; \
		\
		while kill -0 $$COMPILE_PID 2>/dev/null; do \
			if [ -f $$TEMP_OUTPUT ]; then \
				check_errors; \
				if [ $$ERROR_DETECTED -eq 1 ]; then \
					kill $$COMPILE_PID 2>/dev/null; \
					break; \
				fi; \
				\
				CURRENT_TIME=$$(date +%s); \
				ELAPSED=$$((CURRENT_TIME - START_TIME)); \
				\
				COMPILE_LINES=$$(grep -E "Compiling.*\.(cpp|c|ino|S)" $$TEMP_OUTPUT 2>/dev/null | wc -l || echo 0); \
				\
				if [ $$COMPILE_LINES -gt 0 ]; then \
					if [ $$TOTAL_FILES -eq 0 ]; then \
						TOTAL_FILES=$$COMPILE_LINES; \
					fi; \
					FILES_COMPILED=$$(grep -E "\.(cpp|c|ino|S)\.o|Compiled.*\.(cpp|c|ino|S)" $$TEMP_OUTPUT 2>/dev/null | wc -l || echo 0); \
					if [ $$FILES_COMPILED -gt $$TOTAL_FILES ]; then \
						FILES_COMPILED=$$TOTAL_FILES; \
					fi; \
				fi; \
				\
				if [ "$$STAGE" = "initializing" ]; then \
					if grep -qE "Compiling|Using.*library" $$TEMP_OUTPUT 2>/dev/null; then \
						STAGE="compiling"; \
					fi; \
				fi; \
				if [ "$$STAGE" = "compiling" ]; then \
					if grep -qE "Linking everything together|Linking|Linking.*\.elf" $$TEMP_OUTPUT 2>/dev/null; then \
						STAGE="linking"; \
					fi; \
				fi; \
				if [ "$$STAGE" = "linking" ]; then \
					if grep -qE "Generating.*\.bin|Creating.*\.bin|Building.*\.bin|Converting to.*\.bin" $$TEMP_OUTPUT 2>/dev/null; then \
						STAGE="generating"; \
					fi; \
				fi; \
				\
				PERCENT=$$LAST_PERCENT; \
				if [ "$$STAGE" = "initializing" ]; then \
					PERCENT=$$((1 + (ELAPSED / 2))); \
					if [ $$PERCENT -gt 5 ]; then PERCENT=5; fi; \
				elif [ "$$STAGE" = "compiling" ]; then \
					if [ $$TOTAL_FILES -gt 0 ] && [ $$FILES_COMPILED -ge 0 ]; then \
						PERCENT=$$((5 + (FILES_COMPILED * 60 / TOTAL_FILES))); \
						if [ $$PERCENT -gt 65 ]; then PERCENT=65; fi; \
					else \
						PERCENT=$$((5 + (ELAPSED / 3))); \
						if [ $$PERCENT -gt 65 ]; then PERCENT=65; fi; \
					fi; \
				elif [ "$$STAGE" = "linking" ]; then \
					PERCENT=$$((65 + (ELAPSED / 5))); \
					if [ $$PERCENT -gt 90 ]; then PERCENT=90; fi; \
				elif [ "$$STAGE" = "generating" ]; then \
					PERCENT=$$((90 + (ELAPSED / 10))); \
					if [ $$PERCENT -gt 99 ]; then PERCENT=99; fi; \
				fi; \
				\
				if [ $$PERCENT -gt $$LAST_PERCENT ]; then \
					show_progress $$PERCENT; \
					LAST_PERCENT=$$PERCENT; \
				fi; \
			fi; \
			sleep 0.2; \
		done; \
		\
		if [ $$ERROR_DETECTED -eq 0 ]; then \
			wait $$COMPILE_PID 2>/dev/null; \
			EXIT_CODE=$$?; \
		else \
			EXIT_CODE=1; \
		fi; \
		\
		if [ $$ERROR_DETECTED -eq 0 ] && [ $$EXIT_CODE -eq 0 ]; then \
			show_progress 100; \
			echo ""; \
			SUMMARY_START=$$(grep -n "Sketch uses" $$TEMP_OUTPUT 2>/dev/null | cut -d: -f1 | head -1); \
			if [ -n "$$SUMMARY_START" ]; then \
				tail -n +$$SUMMARY_START $$TEMP_OUTPUT 2>/dev/null | head -15 || true; \
			fi; \
			echo ""; \
		else \
			echo ""; \
			echo -e "$(RED)=== Compilation Errors ===$(NC)"; \
			grep -iE "(error|fatal|undefined|multiple definition)" $$TEMP_OUTPUT 2>/dev/null | head -30 || cat $$TEMP_OUTPUT; \
			echo ""; \
			rm -f $$TEMP_OUTPUT; \
			exit 1; \
		fi; \
		rm -f $$TEMP_OUTPUT' || (echo "" && echo -e "$(RED)[FAIL] Compilation failed!$(NC)" && exit 1)
	@echo ""
	@echo -e "$(GREEN)[OK] Compilation successful!$(NC)"

# Upload to ESP32-S3 (no compile)
upload:
	@echo -e "$(YELLOW)========================================$(NC)"
	@echo -e "$(YELLOW)Uploading to ESP32-S3 on $(PORT)...$(NC)"
	@echo -e "$(YELLOW)========================================$(NC)"
	@echo ""
	$(ARDUINO_CLI) upload -p $(PORT) --fqbn $(FQBN) --build-path "$(BUILD_PATH)" "$(SKETCH_DIR)" || (echo -e "$(RED)[FAIL] Upload failed!$(NC)" && exit 1)
	@echo ""
	@echo -e "$(GREEN)[OK] Upload successful!$(NC)"

# Open serial monitor
monitor:
	@echo -e "$(YELLOW)Opening serial monitor on $(PORT) at $(BAUDRATE) baud...$(NC)"
	@$(ARDUINO_CLI) monitor -p $(PORT) --config baudrate=$(BAUDRATE)

# Compile and upload in one step
all: select-sketch compile upload
	@echo -e "$(GREEN)[OK] Build and upload complete!$(NC)"

# Clean build artifacts
clean:
	@echo -e "$(YELLOW)Cleaning build artifacts...$(NC)"
	@rm -rf "$(BUILD_PATH)" 2>/dev/null || true
	$(ARDUINO_CLI) compile --fqbn $(FQBN) --clean "$(SKETCH_DIR)"
	@echo -e "$(GREEN)[OK] Clean complete!$(NC)"

# Verify sketch (check for errors without full compile)
verify:
	@echo -e "$(YELLOW)Verifying sketch...$(NC)"
	$(ARDUINO_CLI) compile --fqbn $(FQBN) --libraries "$(LIBRARY_PATH)" --only-compilation-database "$(SKETCH_DIR)"

# Show board info
board-info:
	@echo -e "$(YELLOW)Board information:$(NC)"
	@$(ARDUINO_CLI) board details --fqbn $(FQBN)

# List connected boards
list-boards:
	@echo -e "$(YELLOW)Connected boards:$(NC)"
	@$(ARDUINO_CLI) board list

# Show current sketch status
sketch-status:
	@echo -e "$(YELLOW)Sketch Status:$(NC)"
	@echo "  Selected: $(SKETCH_FILE)"
	@echo "  Active: $(SKETCH_ACTIVE)"
	@if [ -f "$(SKETCH_DIR)/$(SKETCH_ACTIVE)" ]; then \
		echo "  Active file exists: ✓"; \
		SKETCH_SOURCE=$$(head -n 1 "$(SKETCH_DIR)/$(SKETCH_ACTIVE)" | grep -o "bme680[^ ]*" | head -1 || echo "unknown"); \
		if [ -n "$$SKETCH_SOURCE" ]; then \
			echo "  Source: $$SKETCH_SOURCE"; \
		fi; \
	else \
		echo "  Active file exists: ✗"; \
	fi
	@echo ""
	@echo "Available sketches:"
	@ls -1 "$(SKETCH_DIR)"/*.ino 2>/dev/null | xargs -n1 basename || echo "  (none found)"

