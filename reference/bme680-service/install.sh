#!/bin/bash
# BME680 Service Installation Script
# Installs self-contained package to ~/.local/share/bme680-service/

set -e

# Determine script directory - works in both bash and zsh
if [ -n "${BASH_SOURCE[0]:-}" ]; then
    # Bash: use BASH_SOURCE
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
elif [ -n "${0:-}" ] && [ "${0}" != "-" ] && [ "${0}" != "main" ] && [ "${0}" != "bash" ] && [ "${0}" != "zsh" ] && [ -f "${0}" ]; then
    # Zsh or other: use $0 if it's a valid file path
    SCRIPT_DIR="$(cd "$(dirname "${0}")" && pwd)"
else
    # Fallback: try to find script in current directory
    SCRIPT_DIR="$(pwd)"
fi

PACKAGE_DIR="$SCRIPT_DIR"
DETECTOR="$PACKAGE_DIR/detectors/detect-bme680.sh"
SERVICE_DIR="$PACKAGE_DIR/services"
DATA_DIR="$PACKAGE_DIR/data"
IMENU_DIR="$PACKAGE_DIR/../_utilities/iMenu"

# Capture original user's home directory (before sudo changes $HOME)
ORIGINAL_USER="${SUDO_USER:-$USER}"
ORIGINAL_HOME=$(getent passwd "$ORIGINAL_USER" | cut -d: -f6)

# Fallback if getent fails
if [ -z "$ORIGINAL_HOME" ]; then
    ORIGINAL_HOME="/home/$ORIGINAL_USER"
fi

# Installation paths - use original user's home, not root's
INSTALL_ROOT="$ORIGINAL_HOME/.local/share/bme680-service"
INSTALL_BIN="$ORIGINAL_HOME/.local/bin"
VENV_DIR="$INSTALL_ROOT/.venv"
CONFIG_DIR="$ORIGINAL_HOME/.config/bme680-monitor"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

print_header() {
    echo -e "${CYAN}════════════════════════════════════════${NC}"
    echo -e "${CYAN}  BME680 Service Installation${NC}"
    echo -e "${CYAN}════════════════════════════════════════${NC}"
    echo
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Check if I2C is enabled in config
check_i2c_enabled() {
    local config_file="$1"
    if [ ! -f "$config_file" ]; then
        return 1  # Config file doesn't exist, assume I2C check needed
    fi
    
    # Check if I2C is enabled in YAML config
    # Look for "enabled: true" or "enabled: false" under i2c section
    local i2c_enabled
    i2c_enabled=$(grep -A 5 "^i2c:" "$config_file" 2>/dev/null | grep -E "^\s*enabled:\s*(true|false)" | awk '{print $2}' | tr -d ' ')
    
    if [ "$i2c_enabled" = "false" ]; then
        return 1  # I2C is disabled
    fi
    
    return 0  # I2C is enabled or not specified (default to enabled)
}

# Detect BME680 sensor
detect_sensor() {
    print_info "Detecting BME680 sensor on I2C bus 1..."
    print_info "Checking both addresses (0x76 and 0x77) and verifying chip ID (0x61)..."
    
    if [ ! -f "$DETECTOR" ]; then
        print_error "Sensor detector script not found: $DETECTOR"
        exit 1
    fi
    
    chmod +x "$DETECTOR"
    
    # Run detector - checks both 0x76 and 0x77, verifies chip ID 0x61
    if SENSOR_ADDR=$("$DETECTOR" 1 2>/dev/null); then
        # Clean up output (first line is the detected address)
        SENSOR_ADDR=$(echo "$SENSOR_ADDR" | head -1 | tr -d '\n\r')
        
        print_success "BME680 sensor detected at I2C address: $SENSOR_ADDR"
        print_info "- Verified chip ID: 0x61 (confirmed BME680, not another sensor)"
        print_info "- Checked both addresses: 0x76 and 0x77"
        
        return 0
    else
        print_warning "BME680 sensor not detected on I2C bus 1"
        print_info "Verification performed:"
        
        # Check what's actually at each address
        CHECK_76=$(i2cget -y 1 0x76 0xD0 b 2>/dev/null || echo "none")
        CHECK_77=$(i2cget -y 1 0x77 0xD0 b 2>/dev/null || echo "none")
        
        if [ "$CHECK_76" != "none" ]; then
            if [ "$CHECK_76" = "0x61" ]; then
                print_info "- Address 0x76: BME680 detected (chip ID 0x61)"
            else
                print_info "- Address 0x76: Device present but chip ID is $CHECK_76 (not BME680)"
            fi
        else
            print_info "- Address 0x76: No device"
        fi
        
        if [ "$CHECK_77" != "none" ]; then
            if [ "$CHECK_77" = "0x61" ]; then
                print_info "- Address 0x77: BME680 detected (chip ID 0x61)"
            else
                print_info "- Address 0x77: Device present but chip ID is $CHECK_77 (not BME680)"
            fi
        else
            print_info "- Address 0x77: No device"
        fi
        print_info ""
        print_info "Checking if I2C is enabled..."
        
        if [ ! -c /dev/i2c-1 ]; then
            print_error "I2C device /dev/i2c-1 not found"
            print_info "Enable I2C in raspi-config: sudo raspi-config → Interface Options → I2C"
            exit 1
        fi
        
        print_warning "I2C is enabled but no BME680 detected"
        print_info "Possible reasons:"
        print_info "- Sensor not connected or powered"
        print_info "- Wrong I2C address (check SDO pin wiring)"
        print_info "- Different sensor model (not BME680)"
        print_info ""
        read -p "Continue anyway? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_error "Installation cancelled"
            exit 1
        fi
        return 1
    fi
}

# Install package files
install_package_files() {
    print_info "Installing package files to $INSTALL_ROOT..."
    
    # Create installation directory
    mkdir -p "$INSTALL_ROOT"
    
    # Copy monitor library
    if [ -d "$DATA_DIR/monitor" ]; then
        print_info "- Installing BME680 library..."
        cp -r "$DATA_DIR/monitor" "$INSTALL_ROOT/"
        print_success "BME680 library installed"
    else
        print_error "Monitor library not found in package: $DATA_DIR/monitor"
        exit 1
    fi
    
    # Copy MQTT base-readings script (consolidated - includes heatsoak calculations)
    if [ -d "$PACKAGE_DIR/mqtt/data" ]; then
        mkdir -p "$INSTALL_ROOT/mqtt"
        if [ -f "$PACKAGE_DIR/mqtt/data/base-readings.py" ]; then
            print_info "- Installing base-readings.py..."
            cp "$PACKAGE_DIR/mqtt/data/base-readings.py" "$INSTALL_ROOT/mqtt/"
            chmod +x "$INSTALL_ROOT/mqtt/base-readings.py"
            print_success "base-readings.py (MQTT) installed"
        fi
        
        # Wrapper script removed - Python script now reads YAML config directly
        
        # IAQ script - only install if IAQ service is selected
        if [ "${install_iaq:-false}" = true ] && [ -f "$PACKAGE_DIR/mqtt/data/monitor-iaq.py" ]; then
            print_info "- Installing BME680 monitor-iaq.py..."
            cp "$PACKAGE_DIR/mqtt/data/monitor-iaq.py" "$INSTALL_ROOT/mqtt/"
            chmod +x "$INSTALL_ROOT/mqtt/monitor-iaq.py"
            print_success "monitor-iaq.py (MQTT) installed"
        fi
    fi
    
    # Install CLI tool to ~/.local/bin
    if [ -f "$DATA_DIR/bme680-cli" ]; then
        print_info "- Installing bme680-cli to $INSTALL_BIN..."
        mkdir -p "$INSTALL_BIN"
        cp "$DATA_DIR/bme680-cli" "$INSTALL_BIN/"
        chmod +x "$INSTALL_BIN/bme680-cli"
        print_success "bme680-cli installed"
    fi
}

# Create virtual environment and install dependencies
setup_python_environment() {
    print_info "Setting up Python virtual environment..."
    
    # Check for Python 3
    if ! command -v python3 &> /dev/null; then
        print_error "Python 3 not found. Please install Python 3."
        exit 1
    fi
    
    PYTHON_VERSION=$(python3 --version)
    print_info "- Using $PYTHON_VERSION"
    
    # Check if uv is available (preferred) or fall back to python3 -m venv + pip
    # Note: uv might be in user's ~/.local/bin which isn't in root's PATH
    USE_UV=false
    UV_CMD=""
    
    # Try to find uv in standard locations or user's local bin
    if command -v uv &> /dev/null; then
        UV_CMD="uv"
        USE_UV=true
    elif [ -f "$ORIGINAL_HOME/.local/bin/uv" ]; then
        UV_CMD="$ORIGINAL_HOME/.local/bin/uv"
        USE_UV=true
    elif [ -f "/usr/local/bin/uv" ]; then
        UV_CMD="/usr/local/bin/uv"
        USE_UV=true
    fi
    
    if [ "$USE_UV" = true ]; then
        print_info "- Using uv for virtual environment and package management"
        print_info "- Found at: $UV_CMD"
    else
        print_info "- uv not available, falling back to venv + pip"
    fi
    
    if [ "$USE_UV" = true ]; then
        # Use uv for venv creation and package management
        if [ ! -d "$VENV_DIR" ]; then
            print_info "- Creating virtual environment with uv..."
            "$UV_CMD" venv "$VENV_DIR" 2>&1 | while IFS= read -r line; do
                print_info "- $line"
            done
            print_success "Virtual environment created with uv"
        else
            # Check if venv is valid (has Python executable)
            if [ ! -f "$VENV_DIR/bin/python" ]; then
                print_warning "Virtual environment appears corrupted, recreating..."
                rm -rf "$VENV_DIR"
                "$UV_CMD" venv "$VENV_DIR" 2>&1 | while IFS= read -r line; do
                    print_info "- $line"
                done
                print_success "Virtual environment recreated with uv"
            else
                print_info "Virtual environment already exists, updating..."
            fi
        fi
        
        # Install dependencies with uv
        if [ -f "$DATA_DIR/requirements.txt" ]; then
            print_info "- Installing dependencies from requirements.txt with uv..."
            "$UV_CMD" pip install -r "$DATA_DIR/requirements.txt" --python "$VENV_DIR/bin/python" --quiet
            print_success "Dependencies installed"
        else
            print_warning "requirements.txt not found, skipping dependency installation"
        fi
    else
        # Fallback to traditional venv/pip
        if [ ! -d "$VENV_DIR" ]; then
            print_info "- Creating virtual environment..."
            python3 -m venv "$VENV_DIR" 2>&1 | while IFS= read -r line; do
                print_info "- $line"
            done
            PYTHON_VERSION=$(python3 --version 2>&1 | awk '{print $2}')
            print_success "Virtual environment created with Python $PYTHON_VERSION"
        else
            # Check if venv is valid (has Python and pip executables)
            if [ ! -f "$VENV_DIR/bin/python" ] || [ ! -f "$VENV_DIR/bin/pip" ]; then
                print_warning "Virtual environment appears corrupted or incomplete, recreating..."
                rm -rf "$VENV_DIR"
                python3 -m venv "$VENV_DIR" 2>&1 | while IFS= read -r line; do
                    print_info "- $line"
                done
                PYTHON_VERSION=$(python3 --version 2>&1 | awk '{print $2}')
                print_success "Virtual environment recreated with Python $PYTHON_VERSION"
            else
                print_info "Virtual environment already exists, updating..."
            fi
        fi
        
        # Activate venv and upgrade pip (only if pip exists)
        if [ -f "$VENV_DIR/bin/pip" ]; then
            print_info "- Upgrading pip..."
            "$VENV_DIR/bin/pip" install --upgrade pip --quiet
        else
            print_error "pip not found in virtual environment"
            exit 1
        fi
        
        # Install dependencies
        if [ -f "$DATA_DIR/requirements.txt" ]; then
            print_info "- Installing dependencies from requirements.txt..."
            "$VENV_DIR/bin/pip" install -r "$DATA_DIR/requirements.txt" --quiet
            print_success "Dependencies installed"
        else
            print_warning "requirements.txt not found, skipping dependency installation"
        fi
    fi
    
    print_success "Python environment ready"
}

# Check if Mosquitto is installed
check_mosquitto_installed() {
    if command -v mosquitto >/dev/null 2>&1 || systemctl list-units --all --type=service | grep -q "mosquitto.service"; then
        return 0  # Installed
    else
        return 1  # Not installed
    fi
}

# Reload Home Assistant core configuration
reload_ha_core() {
    print_info "Reloading Home Assistant core configuration..."
    
    # Try using ha helper script first (if available)
    if command -v ha >/dev/null 2>&1; then
        if ha restart >/dev/null 2>&1; then
            print_success "Home Assistant core configuration reloaded (via ha helper)"
            return 0
        fi
    fi
    
    # Try docker exec with supervisor API (if supervised installation)
    if docker ps --format '{{.Names}}' | grep -q "^homeassistant$"; then
        # Try supervisor API first (for supervised installations)
        if docker exec homeassistant curl -s -X POST http://supervisor/core/reload 2>/dev/null | grep -q "ok"; then
            print_success "Home Assistant core configuration reloaded (via supervisor API)"
            return 0
        fi
        
        # Fallback: Try HA API with localhost (may require token, but worth trying)
        # Note: This will fail if token is required, but we'll catch it
        if docker exec homeassistant curl -s -X POST -H "Content-Type: application/json" http://localhost:8123/api/services/config/reload_core_config 2>/dev/null | grep -q -E "(reload|ok|success)"; then
            print_success "Home Assistant core configuration reloaded (via HA API)"
            return 0
        fi
    fi
    
    # If all methods fail, inform user
    print_warning "Could not automatically reload Home Assistant core configuration"
    print_info "Please reload manually:"
    print_info "- Run: ha restart"
    print_info "- Or use: Developer Tools > YAML > Reload Core Configuration"
    return 1
}

# Setup config file (must be first - services depend on it)
setup_config_file() {
    print_info "Setting up configuration file..."
    
    mkdir -p "$CONFIG_DIR"
    chown "$ORIGINAL_USER:$ORIGINAL_USER" "$CONFIG_DIR"
    chmod 700 "$CONFIG_DIR"
    local config_file="$CONFIG_DIR/config.yaml"
    
    # Use tracked config file if it exists
    local tracked_config="$PACKAGE_DIR/config.yaml"
    if [ -f "$tracked_config" ]; then
        print_info "- Copying tracked config file from $tracked_config"
        # Remove existing file/symlink if it exists
        if [ -e "$config_file" ]; then
            rm -f "$config_file"
        fi
        # Copy tracked config file
        cp "$tracked_config" "$config_file"
        chmod 600 "$config_file"
        chown "$ORIGINAL_USER:$ORIGINAL_USER" "$config_file"
        print_success "Configuration file installed in ~/.config"
    else
        print_warning "Tracked config file not found at $tracked_config"
        print_info "Services will use default values if config file doesn't exist"
    fi
}

# Install systemd service
install_service() {
    local service_name=$1
    local service_type=${2:-mqtt}  # Default to mqtt
    local service_file="$PACKAGE_DIR/$service_type/services/$service_name-$service_type.service"
    local installed_service_name="$service_name-$service_type"
    
    if [ ! -f "$service_file" ]; then
        print_error "Service file not found: $service_file"
        return 1
    fi
    
    print_info "Installing $installed_service_name service..."
    
    # Copy service file and substitute user if needed
    if grep -q "User=pi" "$service_file"; then
        # Service file already has User=pi, use as-is
        cp "$service_file" "/etc/systemd/system/$installed_service_name.service"
    else
        # Create service file with proper user
        sed "s|User=.*|User=$ORIGINAL_USER|g" "$service_file" > "/etc/systemd/system/$installed_service_name.service"
    fi
    print_success "Service file copied to /etc/systemd/system/"
    
    # Reload systemd
    systemctl daemon-reload
    print_success "Systemd daemon reloaded"
    
    # Enable service
    enable_output=$(systemctl enable "$installed_service_name.service" 2>&1)
    if [ -n "$enable_output" ]; then
        echo "$enable_output" | while IFS= read -r line; do
            print_info "- $line"
        done
    fi
    print_success "Service enabled (will start on boot)"
    
    # Check if service should be started
    if systemctl is-active --quiet "$installed_service_name.service" 2>/dev/null; then
        print_info "Service is already running"
    else
        print_info "Starting $installed_service_name service..."
        if systemctl start "$installed_service_name.service"; then
            print_success "Service started successfully"
        else
            print_warning "Service start failed, check status with: systemctl status $installed_service_name"
        fi
    fi
}

# Main installation
main() {
    # Auto-elevate if needed (must be first)
    if [ "$EUID" -ne 0 ]; then
        # Preserve script path when elevating - use absolute path
        # When zsh runs a script directly: zsh "/path/to/script.sh", $0 is the script path
        # When bash runs a script: BASH_SOURCE[0] is set
        # When sourced: both might be wrong, so we need fallbacks
        
        script_path=""
        
        # Method 1: Try BASH_SOURCE (works in bash when script is executed, not sourced)
        if [ -n "${BASH_SOURCE[0]:-}" ] && [ -f "${BASH_SOURCE[0]}" ]; then
            script_path="${BASH_SOURCE[0]}"
        # Method 2: Try $0 if it's an absolute path and exists
        elif [ -n "${0:-}" ] && [ "${0:0:1}" = "/" ] && [ -f "${0}" ]; then
            script_path="$0"
        # Method 3: Try $0 if it's a relative path and exists
        elif [ -n "${0:-}" ] && [ "${0:0:1}" != "/" ] && [ -f "${0}" ]; then
            script_path="$(cd "$(dirname "${0}")" && pwd)/$(basename "${0}")"
        fi
        
        # Method 4: Use SCRIPT_DIR if available (set at top of script)
        if [ -z "$script_path" ] || [ ! -f "$script_path" ]; then
            if [ -n "${SCRIPT_DIR:-}" ] && [ -f "${SCRIPT_DIR}/install.sh" ]; then
                script_path="${SCRIPT_DIR}/install.sh"
            # Check if we're in the bme680-service directory
            elif [ -f "$(pwd)/install.sh" ]; then
                script_path="$(cd "$(pwd)" && pwd)/install.sh"
            # Check parent directory
            elif [ -f "$(dirname "$(pwd)")/install.sh" ]; then
                script_path="$(cd "$(dirname "$(pwd)")" && pwd)/install.sh"
            fi
        fi
        
        # Convert to absolute path if relative and still valid
        if [ -n "$script_path" ] && [ -f "$script_path" ] && [ ! "${script_path:0:1}" = "/" ]; then
            script_path="$(cd "$(dirname "$script_path")" && pwd)/$(basename "$script_path")"
        fi
        
        # Final verification
        if [ -z "$script_path" ] || [ ! -f "$script_path" ]; then
            echo "Error: Could not determine script path" >&2
            echo "  BASH_SOURCE[0]=${BASH_SOURCE[0]:-}" >&2
            echo "  \$0=$0" >&2
            echo "  PWD=$(pwd)" >&2
            exit 1
        fi
        
        exec sudo bash "$script_path" "$@"
    fi
    
    print_header
    
    # Always re-resolve paths after sudo (sudo changes working directory context)
    # Re-resolve ORIGINAL_USER and ORIGINAL_HOME after sudo
    ORIGINAL_USER="${SUDO_USER:-$USER}"
    ORIGINAL_HOME=$(getent passwd "$ORIGINAL_USER" 2>/dev/null | cut -d: -f6)
    if [ -z "$ORIGINAL_HOME" ]; then
        ORIGINAL_HOME="/home/$ORIGINAL_USER"
    fi
    
    # Re-resolve installation paths
    INSTALL_ROOT="$ORIGINAL_HOME/.local/share/bme680-service"
    INSTALL_BIN="$ORIGINAL_HOME/.local/bin"
    VENV_DIR="$INSTALL_ROOT/.venv"
    CONFIG_DIR="$ORIGINAL_HOME/.config/bme680-monitor"
    
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    PACKAGE_DIR="$SCRIPT_DIR"
    DETECTOR="$PACKAGE_DIR/detectors/detect-bme680.sh"
    SERVICE_DIR="$PACKAGE_DIR/services"
    DATA_DIR="$PACKAGE_DIR/data"
    IMENU_DIR="$PACKAGE_DIR/../_utilities/iMenu"
    
    # Source iMenu (includes iWizard)
    if [ -f "$IMENU_DIR/iMenu.sh" ]; then
        source "$IMENU_DIR/iMenu.sh"
    else
        print_warning "iMenu not found at $IMENU_DIR/iMenu.sh"
        print_warning "Falling back to simple prompts..."
    fi
    
    # Check if Mosquitto is already installed
    mosquitto_installed=false
    if check_mosquitto_installed; then
        mosquitto_installed=true
        print_success "Mosquitto MQTT broker is already installed"
    fi
    
    # Build wizard config - only include broker step if not already installed
    if [ "$mosquitto_installed" = true ]; then
        INSTALL_WIZARD_CONFIG='
{
    "title": "BME680 Service Installation",
    "steps": [
        {
            "type": "multiselect",
            "message": "ℹ️  Which services would you like to install?",
            "options": [
                "Base readings (MQTT) - Includes sensor readings and heatsoak calculations",
                "IAQ monitor (MQTT) - Air quality calculation"
            ],
            "preselect": [0]
        },
        {
            "type": "multiselect",
            "message": "ℹ️  Which installation(s) would you like to perform?",
            "options": [
                "Standalone MQTT",
                "Home Assistant MQTT Integration",
                "Home Assistant Custom Integration"
            ],
            "preselect": [0, 1]
        }
    ]
}
'
    else
        INSTALL_WIZARD_CONFIG='
{
    "title": "BME680 Service Installation",
    "steps": [
        {
            "type": "multiselect",
            "message": "ℹ️  Which services would you like to install?",
            "options": [
                "Base readings (MQTT) - Includes sensor readings and heatsoak calculations",
                "IAQ monitor (MQTT) - Air quality calculation"
            ],
            "preselect": [0]
        },
        {
            "type": "multiselect",
            "message": "ℹ️  Which installation(s) would you like to perform?",
            "options": [
                "Standalone MQTT",
                "Home Assistant MQTT Integration",
                "Home Assistant Custom Integration"
            ],
            "preselect": [0, 1]
        },
        {
            "type": "confirm",
            "message": "ℹ️  Install Mosquitto MQTT broker?",
            "initial": true
        }
    ]
}
'
    fi
    
    # Use iWizard for installation steps
    if type iwizard_run_inline >/dev/null 2>&1 && [ -t 0 ] && [ -t 1 ]; then
        wizard_results=$(iwizard_run_inline "$INSTALL_WIZARD_CONFIG")
        wizard_exit=$?
        
        if [ $wizard_exit -ne 0 ]; then
            print_info "Installation cancelled"
            exit 0
        fi
        
        # Parse results
        # Multiselect results are space-separated indices like "0 1"
        # Confirm results are "true" or "false"
        step1_result=$(echo "$wizard_results" | jq -r '.step0.result' 2>/dev/null || echo "")
        step2_result=$(echo "$wizard_results" | jq -r '.step1.result' 2>/dev/null || echo "")
        
        # Parse step1: services
        install_base=false
        install_iaq=false
        for idx in $step1_result; do
            case $idx in
                0) install_base=true ;;
                1) install_iaq=true ;;
            esac
        done
        
        # Parse step2: installation types
        do_standalone=false
        do_ha_mqtt=false
        do_ha_integration=false
        for idx in $step2_result; do
            case $idx in
                0) do_standalone=true ;;
                1) do_ha_mqtt=true ;;
                2) do_ha_integration=true ;;
            esac
        done
        
        # Parse step3: broker installation (only if MQTT selected and not already installed)
        install_broker=false
        if [ "$mosquitto_installed" = false ]; then
            step3_result=$(echo "$wizard_results" | jq -r '.step2.result' 2>/dev/null || echo "")
            want_mqtt=false
            for idx in $step2_result; do
                case $idx in
                    0|1) want_mqtt=true ;;
                esac
            done
            if [ "$want_mqtt" = true ] && [ "$step3_result" = "true" ]; then
                install_broker=true
            fi
        fi
    else
        # Fallback to simple prompts
        print_warning "Using simple prompts (iWizard not available)"
        
        echo "Which services would you like to install?"
        echo "  1) Base readings (MQTT) - Includes sensor readings and heatsoak calculations"
        echo "  2) IAQ monitor (MQTT)"
        echo "  3) All"
        echo "  4) Cancel"
        read -p "Choice [1-4]: " choice
        case $choice in
            1) install_base=true ;;
            2) install_iaq=true ;;
            3) install_base=true; install_iaq=true ;;
            4) print_info "Installation cancelled"; exit 0 ;;
            *) print_error "Invalid choice"; exit 1 ;;
        esac
        
        echo
        echo "Which installation(s) would you like to perform?"
        echo "  1) Standalone MQTT"
        echo "  2) Home Assistant MQTT Integration"
        echo "  3) Home Assistant Custom Integration"
        echo "  4) All"
        echo "  5) Cancel"
        read -p "Choice [1-5]: " ch0
        case $ch0 in
            1) do_standalone=true ;;
            2) do_ha_mqtt=true ;;
            3) do_ha_integration=true ;;
            4) do_standalone=true; do_ha_mqtt=true; do_ha_integration=true ;;
            5) print_info "Installation cancelled"; exit 0 ;;
            *) print_error "Invalid choice"; exit 1 ;;
        esac
        
        if [ "$do_standalone" = true ] || [ "$do_ha_mqtt" = true ]; then
            if [ "$mosquitto_installed" = false ]; then
                read -p "Install Mosquitto MQTT broker? (Y/n): " ans
                ans=${ans:-Y}
                if [[ "${ans^^}" = "Y" ]]; then
                    install_broker=true
                fi
            else
                install_broker=false
            fi
        fi
    fi
    
    # Check if user cancelled (no services selected)
    if [ "$install_base" = false ] && [ "$install_iaq" = false ]; then
        print_info "No services selected. Installation cancelled."
        exit 0
    fi
    
    # Prompt for sensor label if HA MQTT integration is selected (before any actions)
    SENSOR_LABEL="MQTT"
    SENSOR_LABEL_LOWER="mqtt"
    
    if [ "$do_ha_mqtt" = true ]; then
        local ha_yaml_template="$PACKAGE_DIR/mqtt/ha/sensors-bme680-mqtt.yaml.template"
        if [ -f "$ha_yaml_template" ]; then
            echo
            print_info "Customize sensor label for Home Assistant integration:"
            read -p "Sensor label [default: MQTT]: " input_label
            if [ -n "$input_label" ]; then
                SENSOR_LABEL="$input_label"
                # Convert to lowercase for unique_ids and identifiers
                SENSOR_LABEL_LOWER=$(echo "$SENSOR_LABEL" | tr '[:upper:]' '[:lower:]' | tr ' ' '_')
            fi
            
            print_info "  Sensor label: $SENSOR_LABEL (ID: $SENSOR_LABEL_LOWER)"
            echo
        fi
    fi
    
    # Check config file to see if I2C is enabled before detecting sensor
    local tracked_config="$PACKAGE_DIR/config.yaml"
    if check_i2c_enabled "$tracked_config"; then
        # I2C is enabled, detect sensor
        echo
        detect_sensor
    else
        # I2C is disabled in config, skip sensor detection
        print_info "I2C is disabled in configuration, skipping sensor detection"
    fi
    
    # Proceed with installation
    echo
    print_info "Installing BME680 service package..."
    echo 
    
    # Setup config file FIRST (services depend on it)
    setup_config_file
    
    # Install package files
    install_package_files
    
    # Setup Python environment
    setup_python_environment
    
    
    # Execute based on selections
    if [ "$do_standalone" = true ] || [ "$do_ha_mqtt" = true ] || [ "$do_ha_integration" = true ]; then
        # Broker if MQTT path selected
        if [ "$do_standalone" = true ] || [ "$do_ha_mqtt" = true ]; then
            if [ "$mosquitto_installed" = true ]; then
                print_success "Mosquitto MQTT broker is already installed"
            elif [ "$install_broker" = true ]; then
                print_info "Installing Mosquitto MQTT broker..."
                apt-get update -y >/dev/null 2>&1 || true
                apt-get install -y mosquitto mosquitto-clients >/dev/null 2>&1 || true
                systemctl enable --now mosquitto >/dev/null 2>&1 || true
                print_success "Mosquitto installed (or already present)."
            fi
        fi

        # Install MQTT services
        if [ "$install_base" = true ]; then
            install_service "bme680-base" "mqtt"
        fi
        if [ "$install_iaq" = true ]; then
            install_service "bme680-iaq" "mqtt"
        fi

        # HA MQTT wiring (install into HA packages and ensure include + mqtt: broker)
        if [ "$do_ha_mqtt" = true ]; then
            print_info "Configuring Home Assistant packages include and MQTT wiring..."
            local HA_CONF="$ORIGINAL_HOME/homeassistant/configuration.yaml"
            local HA_PKG_DIR="$ORIGINAL_HOME/homeassistant/packages"
            local HA_CONTAINER="homeassistant"
            local HA_CONFIG_PATH="$ORIGINAL_HOME/homeassistant"
            
            # Sensor labels were already prompted before any actions
            local ha_yaml_template="$PACKAGE_DIR/mqtt/ha/sensors-bme680-mqtt.yaml.template"
            local ha_yaml_src="$PACKAGE_DIR/mqtt/ha/sensors-bme680-mqtt.yaml"
            
            # Stop HA first to prevent file locks during installation
            if docker ps --format '{{.Names}}' | grep -q "^${HA_CONTAINER}$"; then
                print_info "Stopping Home Assistant to prevent file locks during installation..."
                if docker stop "${HA_CONTAINER}" >/dev/null 2>&1; then
                    print_success "Home Assistant stopped"
                else
                    print_warning "Could not stop Home Assistant (may already be stopped)"
                fi
            fi
            
            # Copy consolidated MQTT sensors config (includes all data including heatsoak)
            local ha_yaml_dst="$HA_PKG_DIR/bme680_mqtt.yaml"
            if [ -f "$ha_yaml_template" ]; then
                # Use template file and replace placeholders
                mkdir -p "$HA_PKG_DIR"
                # Remove existing file/symlink if it exists
                if [ -e "$ha_yaml_dst" ]; then
                    rm -f "$ha_yaml_dst"
                fi
                # Copy template and replace placeholders with sed
                sed -e "s/{{SENSOR_LABEL}}/$SENSOR_LABEL/g" \
                    -e "s/{{SENSOR_LABEL_LOWER}}/$SENSOR_LABEL_LOWER/g" \
                    "$ha_yaml_template" > "$ha_yaml_dst"
                chown "$ORIGINAL_USER:$ORIGINAL_USER" "$ha_yaml_dst"
                print_success "Generated MQTT sensors config from template to $ha_yaml_dst"
                print_info "  Note: This includes all sensor data including heatsoak calculations"
            elif [ -f "$ha_yaml_src" ]; then
                # Fallback to non-template file if template doesn't exist
                mkdir -p "$HA_PKG_DIR"
                # Remove existing file/symlink if it exists
                if [ -e "$ha_yaml_dst" ]; then
                    rm -f "$ha_yaml_dst"
                fi
                # Copy tracked HA sensors file
                cp "$ha_yaml_src" "$ha_yaml_dst"
                chown "$ORIGINAL_USER:$ORIGINAL_USER" "$ha_yaml_dst"
                print_success "Copied MQTT sensors config to $ha_yaml_dst"
                print_info "  Note: This includes all sensor data including heatsoak calculations"
            else
                print_warning "MQTT sensors YAML template or source not found (expected at $ha_yaml_template or $ha_yaml_src)."
            fi
            
            # Disable deprecated heatsoak package if it exists
            if [ -f "$HA_PKG_DIR/bme680_heatsoak_mqtt.yaml" ]; then
                mv "$HA_PKG_DIR/bme680_heatsoak_mqtt.yaml" "$HA_PKG_DIR/bme680_heatsoak_mqtt.yaml.disabled" 2>/dev/null || true
                print_info "Disabled deprecated heatsoak package (consolidated into main package)"
            fi
            
            # Restart HA to pick up new package and clear old entities
            if docker ps -a --format '{{.Names}}' | grep -q "^${HA_CONTAINER}$"; then
                echo
                print_info "Restarting Home Assistant to pick up new MQTT sensors..."
                print_info "Waiting for HA to fully initialize and process storage files..."
                if docker start "${HA_CONTAINER}" >/dev/null 2>&1; then
                    print_success "Home Assistant started"
                    sleep 10
                    sync  # Force filesystem sync
                    sleep 5  # Additional wait for HA to process cleaned storage
                    print_success "Home Assistant restarted - new entities should now be available"
                else
                    print_warning "Could not start Home Assistant"
                    print_warning "⚠️  REQUIRED: Start HA manually with: docker start ${HA_CONTAINER}"
                    print_warning "   Or use: Developer Tools > YAML > Reload Core Configuration"
                fi
            else
                echo
                print_warning "Home Assistant container not found"
                print_info "  When HA is started, it will automatically pick up the new MQTT sensors"
            fi
            echo
            
            # ensure packages include present
            if grep -Eq "^\s*packages:\s*!include_dir_named\s+packages" "$HA_CONF"; then
                true
            elif grep -Eq "^homeassistant:\s*$" "$HA_CONF"; then
                sed -i '/^homeassistant:\s*$/a\  packages: !include_dir_named packages' "$HA_CONF"
                print_success "Added packages include under homeassistant:"
            else
                printf '\n# Added by bme680 installer - Home Assistant packages include\n' >> "$HA_CONF"
                printf 'homeassistant:\n  packages: !include_dir_named packages\n' >> "$HA_CONF"
                print_success "Appended homeassistant: packages include"
            fi
            # Note: do not auto-add mqtt: broker block here; user configures MQTT integration
        fi

        # HA Custom Integration scaffold (install into HA custom_components)
        if [ "$do_ha_integration" = true ]; then
            print_info "Installing Home Assistant custom integration..."
            local comp_src="$PACKAGE_DIR/ha/custom_components/bme680_monitor"
            local comp_dst="$ORIGINAL_HOME/homeassistant/custom_components/bme680_monitor"
            if [ -d "$comp_src" ]; then
                mkdir -p "$(dirname "$comp_dst")"
                rm -rf "$comp_dst"
                cp -r "$comp_src" "$comp_dst"
                print_success "Installed custom component to $comp_dst"
                print_info "  Note: Integration is YAML-only (no UI config flow)"
            else
                print_warning "Custom component scaffold not found at $comp_src"
            fi
        fi
    fi
    
    echo
    print_success "Installation complete!"
    echo
    print_info "Package installed to: $INSTALL_ROOT"
    print_info "Virtual environment: $VENV_DIR"
    echo
    print_info "Useful commands:"
    echo "  systemctl status bme680-base-mqtt         # Base readings + heatsoak (MQTT)"
    echo "  systemctl status bme680-iaq-mqtt          # IAQ monitor (MQTT)"
    echo "  sudo systemctl restart bme680-base-mqtt   # Restart after config changes"
    echo "  mqtt 'sensors/bme680/raw'                 # View MQTT messages"
}

main "$@"
