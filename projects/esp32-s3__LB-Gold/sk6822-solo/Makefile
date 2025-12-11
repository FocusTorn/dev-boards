# Makefile for ESP32-S3 SK6812 LED Controller
# Usage: make [target]

# Configuration
FQBN = esp32:esp32:esp32s3
SKETCH = sk6822-solo.ino
PORT = COM9
BAUDRATE = 115200

PROJECT_ROOT = D:/_dev/_projects/dev-boards/
LIBRARY_PATH = $(PROJECT_ROOT)/_libs/esp32-s3
ARDUINO_CLI = $(PROJECT_ROOT)/Arduino/arduino-cli.exe
SKETCH_DIR = $(PROJECT_ROOT)/projects/esp32-s3__LB-Gold/sk6822-solo
BUILD_PATH = $(SKETCH_DIR)/build

# Colors for output (Git Bash compatible)
GREEN = \033[0;32m
YELLOW = \033[1;33m
RED = \033[0;31m
NC = \033[0m

.PHONY: help build compile progress upload monitor clean all

# Default target
help:
	@echo -e "$(GREEN)ESP32-S3 SK6812 LED Controller - Build Targets$(NC)"
	@echo ""
	@echo "  make build       - Compile the sketch (alias for compile)"
	@echo "  make compile     - Compile the sketch (verbose output)"
	@echo "  make progress    - Compile with progress bar"
	@echo "  make upload      - Upload (no compile)"
	@echo "  make monitor      - Open serial monitor"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make all         - Compile and upload"
	@echo ""
	@echo "Configuration:"
	@echo "  FQBN: $(FQBN)"
	@echo "  PORT: $(PORT)"
	@echo "  Sketch: $(SKETCH)"

# Build/Compile the sketch (build is alias for compile)
build: compile

# Compile the sketch
compile:
	@echo -e "$(YELLOW)========================================$(NC)"
	@echo -e "$(YELLOW)Compiling $(SKETCH)...$(NC)"
	@echo -e "$(YELLOW)========================================$(NC)"
	@echo ""
	$(ARDUINO_CLI) compile --fqbn $(FQBN) --libraries "$(LIBRARY_PATH)" --build-path "$(BUILD_PATH)" --verbose "$(SKETCH_DIR)" || (echo -e "$(RED)[FAIL] Compilation failed!$(NC)" && exit 1)
	@echo ""
	@echo -e "$(GREEN)[OK] Compilation successful!$(NC)"

# Compile with progress bar
progress:
	@echo -e "$(YELLOW)========================================$(NC)"
	@echo -e "$(YELLOW)Compiling $(SKETCH) with progress...$(NC)"
	@echo -e "$(YELLOW)========================================$(NC)"
	@echo ""
	@bash -c '\
		BAR_WIDTH=40; \
		FILES_COMPILED=0; \
		TOTAL_FILES=0; \
		STAGE="compiling"; \
		LAST_PERCENT=0; \
		TEMP_OUTPUT=$$(mktemp); \
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
		show_progress 0; \
		\
		{ $(ARDUINO_CLI) compile --fqbn $(FQBN) --libraries "$(LIBRARY_PATH)" --build-path "$(BUILD_PATH)" --verbose "$(SKETCH_DIR)" >$$TEMP_OUTPUT 2>&1; \
		  EXIT_CODE=$$?; } & \
		COMPILE_PID=$$!; \
		\
		while kill -0 $$COMPILE_PID 2>/dev/null; do \
			if [ -f $$TEMP_OUTPUT ]; then \
				NEW_FILES=$$(grep -cE "Compiling (sketch|libraries|library|core)" $$TEMP_OUTPUT 2>/dev/null || echo 0); \
				NEW_FILES=$${NEW_FILES//[^0-9]/}; \
				if [ -z "$$NEW_FILES" ]; then NEW_FILES=0; fi; \
				\
				if [ $$NEW_FILES -gt $$TOTAL_FILES ]; then \
					TOTAL_FILES=$$NEW_FILES; \
					FILES_COMPILED=$$NEW_FILES; \
				fi; \
				\
				if [ "$$STAGE" = "compiling" ]; then \
					if grep -qE "Linking everything together" $$TEMP_OUTPUT 2>/dev/null; then \
						STAGE="linking"; \
					fi; \
				fi; \
				if [ "$$STAGE" != "generating" ]; then \
					if grep -qE "Generating.*\.bin|Creating.*\.bin" $$TEMP_OUTPUT 2>/dev/null; then \
						STAGE="generating"; \
					fi; \
				fi; \
				\
				PERCENT=$$LAST_PERCENT; \
				if [ "$$STAGE" = "compiling" ]; then \
					if [ $$TOTAL_FILES -gt 0 ]; then \
						PERCENT=$$((5 + (FILES_COMPILED * 65 / TOTAL_FILES))); \
						if [ $$PERCENT -gt 70 ]; then PERCENT=70; fi; \
					else \
						PERCENT=5; \
					fi; \
				elif [ "$$STAGE" = "linking" ]; then \
					PERCENT=85; \
				elif [ "$$STAGE" = "generating" ]; then \
					PERCENT=95; \
				fi; \
				\
				if [ $$PERCENT -gt $$LAST_PERCENT ]; then \
					show_progress $$PERCENT; \
					LAST_PERCENT=$$PERCENT; \
				fi; \
			fi; \
			sleep 0.3; \
		done; \
		\
		wait $$COMPILE_PID; \
		EXIT_CODE=$$?; \
		show_progress 100; \
		echo ""; \
		\
		HAS_ERRORS=0; \
		if [ $$EXIT_CODE -ne 0 ]; then \
			HAS_ERRORS=1; \
		elif grep -qiE "(fatal error|error during|compilation terminated|Error during build)" $$TEMP_OUTPUT; then \
			HAS_ERRORS=1; \
		fi; \
		\
		if [ $$HAS_ERRORS -ne 0 ]; then \
			echo ""; \
			echo -e "$(RED)=== Compilation Errors ===$(NC)"; \
			cat $$TEMP_OUTPUT; \
			echo ""; \
			rm -f $$TEMP_OUTPUT; \
			exit 1; \
		else \
			echo ""; \
			SUMMARY_START=$$(grep -n "Sketch uses" $$TEMP_OUTPUT 2>/dev/null | cut -d: -f1 | head -1); \
			if [ -n "$$SUMMARY_START" ]; then \
				tail -n +$$SUMMARY_START $$TEMP_OUTPUT 2>/dev/null | head -15 || true; \
			fi; \
			echo ""; \
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
all: compile upload
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
