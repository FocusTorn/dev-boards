#!/bin/bash
# BME680 Service Uninstallation Script

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PACKAGE_DIR="$SCRIPT_DIR"
IMENU_DIR="$PACKAGE_DIR/../_utilities/iMenu"

# Parse command line arguments
VERBOSE=false
for arg in "$@"; do
    case "$arg" in
        --verbose|-v)
            VERBOSE=true
            ;;
        --help|-h)
            echo "Usage: $0 [--verbose|-v] [--help|-h]"
            echo ""
            echo "Options:"
            echo "  --verbose, -v    Show detailed information messages"
            echo "  --help, -h       Show this help message"
            exit 0
            ;;
    esac
done

# Capture original user's home (before sudo)
ORIGINAL_USER="${SUDO_USER:-$USER}"
ORIGINAL_HOME=$(getent passwd "$ORIGINAL_USER" 2>/dev/null | cut -d: -f6)
if [ -z "$ORIGINAL_HOME" ]; then
    ORIGINAL_HOME="/home/$ORIGINAL_USER"
fi

INSTALL_ROOT="$ORIGINAL_HOME/.local/share/bme680-service"
INSTALL_BIN="$ORIGINAL_HOME/.local/bin"
CONFIG_DIR="$ORIGINAL_HOME/.config/bme680-monitor"
HA_PKG_DIR="$ORIGINAL_HOME/homeassistant/packages"
HA_CUSTOM_COMPONENTS="$ORIGINAL_HOME/homeassistant/custom_components/bme680_monitor"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

print_success() { #>
    echo -e "${GREEN}✅ $1${NC}"
} #<
print_error() { #>
    echo -e "${RED}❌ $1${NC}"
} #<
print_warning() { #>
    echo -e "${YELLOW}⚠️  $1${NC}"
} #<
print_info() { #>
    if [ "$VERBOSE" = true ]; then
        echo -e "${CYAN}ℹ️  $1${NC}"
    fi
} #<

remove_bme680_entities() { #>
    print_info "Removing BME680 entities from Home Assistant..."
    
    # Stop HA first to prevent file locks and entity recreation
    local HA_CONTAINER="homeassistant"
    local HA_CONFIG_PATH="$ORIGINAL_HOME/homeassistant"
    
    print_info "Stopping Home Assistant to prevent file locks..."
    if docker stop "${HA_CONTAINER}" >/dev/null 2>&1; then
        print_success "Home Assistant stopped"
    else
        print_warning "Could not stop Home Assistant (may already be stopped)"
    fi
    
    local removed_count=0
    local state_removed_count=0
    local device_removed_count=0
    local mqtt_removed_count=0
    local registry_file="${HA_CONFIG_PATH}/.storage/core.entity_registry"
    local state_file="${HA_CONFIG_PATH}/.storage/core.restore_state"
    local device_file="${HA_CONFIG_PATH}/.storage/core.device_registry"
    local discovery_file="${HA_CONFIG_PATH}/.storage/mqtt.discovery"
    
    # Remove entities using sudo for direct file access (like remove-ha-entities.sh)
    if [ -f "$registry_file" ]; then
        # Use Python to remove ALL entities that have 'bme680' in unique_id or entity_id
        removed_count=$(sudo python3 <<PYEOF 2>/dev/null
import json
import sys

registry_file = '$registry_file'

try:
    # Read registry
    with open(registry_file, 'r') as f:
        registry = json.load(f)
    
    # Find ALL entities to remove - match detection logic
    original_count = len(registry.get('data', {}).get('entities', []))
    entities_to_remove = []
    
    for entity in registry.get('data', {}).get('entities', []):
        entity_id = entity.get('entity_id', '')
        unique_id = entity.get('unique_id', '')
        
        # Remove any entity with 'bme680' in unique_id or entity_id (matches detection logic)
        if 'bme680' in unique_id.lower() or 'bme680' in entity_id.lower():
            entities_to_remove.append(entity_id)
    
    # Remove entities
    registry['data']['entities'] = [
        entity for entity in registry['data']['entities']
        if entity.get('entity_id') not in entities_to_remove
    ]
    removed_count = original_count - len(registry['data']['entities'])
    
    # Write back
    with open(registry_file, 'w') as f:
        json.dump(registry, f, indent=2)
    
    print(removed_count)
except Exception as e:
    print(0, file=sys.stderr)
    sys.exit(1)
PYEOF
)
    fi
    
    # Remove from state file (using sudo for direct file access)
    if [ -f "$state_file" ]; then
        state_removed_count=$(sudo python3 <<PYEOF 2>/dev/null
import json
import sys
import os

state_file = '$state_file'

try:
    if not os.path.exists(state_file):
        print(0)
        sys.exit(0)

    with open(state_file, 'r') as f:
        state_data = json.load(f)
    
    states = state_data.get('data', []) if isinstance(state_data.get('data'), list) else []
    original_count = len(states)
    states_to_keep = []
    
    for state_obj in states:
        if not isinstance(state_obj, dict):
            states_to_keep.append(state_obj)
            continue
        
        entity_id = None
        if 'state' in state_obj and isinstance(state_obj['state'], dict):
            entity_id = state_obj['state'].get('entity_id', '')
        elif 'entity_id' in state_obj:
            entity_id = state_obj.get('entity_id', '')
        
        if not entity_id or 'bme680' not in entity_id.lower():
            states_to_keep.append(state_obj)
    
    state_data['data'] = states_to_keep
    removed = original_count - len(states_to_keep)
    
    with open(state_file, 'w') as f:
        json.dump(state_data, f, indent=2)
    
    print(removed)
except Exception as e:
    print(f"Error: {e}", file=sys.stderr)
    print(0)
    sys.exit(1)
PYEOF
)
    fi
    
    # Remove from device registry (using sudo for direct file access)
    if [ -f "$device_file" ] && [ -f "$registry_file" ]; then
        device_removed_count=$(sudo python3 <<PYEOF 2>/dev/null
import json
import sys
import os

device_file = '$device_file'
entity_file = '$registry_file'

if not os.path.exists(device_file) or not os.path.exists(entity_file):
    print(0)
    sys.exit(0)

try:
    with open(device_file, 'r') as f:
        devices = json.load(f)
    
    # Get all device IDs that have entities matching the pattern
    with open(entity_file, 'r') as f:
        registry = json.load(f)
    
    matching_device_ids = set()
    for entity in registry.get('data', {}).get('entities', []):
        entity_id = entity.get('entity_id', '')
        unique_id = entity.get('unique_id', '')
        device_id = entity.get('device_id')
        
        if ('bme680' in unique_id.lower() or 'bme680' in entity_id.lower()) and device_id:
            matching_device_ids.add(device_id)
    
    # Now check if any devices have ALL their entities removed (orphaned)
    original_count = len(devices.get('data', {}).get('devices', []))
    devices_to_keep = []
    
    for device in devices.get('data', {}).get('devices', []):
        device_id = device.get('id')
        if device_id not in matching_device_ids:
            devices_to_keep.append(device)
    
    devices['data']['devices'] = devices_to_keep
    removed = original_count - len(devices_to_keep)
    
    with open(device_file, 'w') as f:
        json.dump(devices, f, indent=2)
    
    print(removed)
except Exception as e:
    print(f"Error: {e}", file=sys.stderr)
    print(0)
    sys.exit(1)
PYEOF
)
    fi
    
    # Remove from MQTT discovery storage (using sudo for direct file access)
    if [ -f "$discovery_file" ]; then
        mqtt_removed_count=$(sudo python3 <<PYEOF 2>/dev/null
import json
import sys
import os

discovery_file = '$discovery_file'

try:
    if not os.path.exists(discovery_file):
        print(0)
        sys.exit(0)
    
    with open(discovery_file, 'r') as f:
        discovery = json.load(f)
    
    items = discovery.get('data', {})
    original_count = len(items)
    
    # Remove items matching the pattern
    items_to_keep = {
        k: v for k, v in items.items()
        if 'bme680' not in k.lower()
    }
    
    discovery['data'] = items_to_keep
    removed = original_count - len(items_to_keep)
    
    with open(discovery_file, 'w') as f:
        json.dump(discovery, f, indent=2)
    
    print(removed)
except Exception as e:
    print(f"Error: {e}", file=sys.stderr)
    print(0)
    sys.exit(1)
PYEOF
)
    fi
    
    # Ensure all count variables are numeric
    removed_count=$(echo "${removed_count}" | tr -d '[:space:]' || echo "0")
    state_removed_count=$(echo "${state_removed_count}" | tr -d '[:space:]' || echo "0")
    device_removed_count=$(echo "${device_removed_count}" | tr -d '[:space:]' || echo "0")
    mqtt_removed_count=$(echo "${mqtt_removed_count}" | tr -d '[:space:]' || echo "0")
    
    if [ -n "$removed_count" ] && [ "$removed_count" -gt 0 ]; then
        print_success "Removed $removed_count BME680 entities from registry"
    else
        print_success "No BME680 entities found in registry (or already removed)"
    fi
    
    if [ -n "$state_removed_count" ] && [ "$state_removed_count" -gt 0 ]; then
        print_success "Removed $state_removed_count BME680 entity states from state file"
    fi
    
    if [ -n "$device_removed_count" ] && [ "$device_removed_count" -gt 0 ]; then
        print_success "Removed $device_removed_count BME680 device(s) from device registry"
    fi
    
    if [ -n "$mqtt_removed_count" ] && [ "$mqtt_removed_count" -gt 0 ]; then
        print_success "Removed $mqtt_removed_count MQTT discovery items"
    fi
    
    # Remove retained MQTT discovery messages from broker
    remove_mqtt_retained_messages
    
    # Note: HA restart is handled by the main uninstall flow, not here
    # This function just removes entities, doesn't restart
} #<

remove_bme680_entities_no_restart() { #>
    # Same as remove_bme680_entities but doesn't restart HA
    # This allows the main uninstall flow to control when HA restarts
    print_info "Removing BME680 entities from Home Assistant..."
    
    local HA_CONTAINER="homeassistant"
    local HA_CONFIG_PATH="$ORIGINAL_HOME/homeassistant"
    
    # HA should already be stopped by the main uninstall flow
    # But check just in case
    if docker ps --format '{{.Names}}' | grep -q "^${HA_CONTAINER}$"; then
        print_info "Stopping Home Assistant to prevent file locks..."
        if docker stop "${HA_CONTAINER}" >/dev/null 2>&1; then
            print_success "Home Assistant stopped"
        else
            print_warning "Could not stop Home Assistant (may already be stopped)"
        fi
    fi
    
    local removed_count=0
    local state_removed_count=0
    local device_removed_count=0
    local mqtt_removed_count=0
    local registry_file="${HA_CONFIG_PATH}/.storage/core.entity_registry"
    local state_file="${HA_CONFIG_PATH}/.storage/core.restore_state"
    local device_file="${HA_CONFIG_PATH}/.storage/core.device_registry"
    local discovery_file="${HA_CONFIG_PATH}/.storage/mqtt.discovery"
    
    # Remove entities using sudo for direct file access (like remove-ha-entities.sh)
    if [ -f "$registry_file" ]; then
        # Use Python to remove ALL entities that have 'bme680' in unique_id or entity_id
        removed_count=$(sudo python3 <<PYEOF 2>/dev/null
import json
import sys

registry_file = '$registry_file'

try:
    with open(registry_file, 'r') as f:
        registry = json.load(f)
    
    # Find ALL entities to remove - match detection logic
    original_count = len(registry.get('data', {}).get('entities', []))
    entities_to_remove = []
    
    for entity in registry.get('data', {}).get('entities', []):
        entity_id = entity.get('entity_id', '')
        unique_id = entity.get('unique_id', '')
        
        # Remove any entity with 'bme680' in unique_id or entity_id (matches detection logic)
        if 'bme680' in unique_id.lower() or 'bme680' in entity_id.lower():
            entities_to_remove.append(entity_id)
    
    # Remove entities
    registry['data']['entities'] = [
        entity for entity in registry['data']['entities']
        if entity.get('entity_id') not in entities_to_remove
    ]
    removed_count = original_count - len(registry['data']['entities'])
    
    # Write back
    with open(registry_file, 'w') as f:
        json.dump(registry, f, indent=2)
    
    print(removed_count)
except Exception as e:
    print(0, file=sys.stderr)
    sys.exit(1)
PYEOF
)
    fi
    
    # Remove from state file (using sudo for direct file access)
    if [ -f "$state_file" ]; then
        state_removed_count=$(sudo python3 <<PYEOF 2>/dev/null
import json
import sys
import os

state_file = '$state_file'

try:
    if not os.path.exists(state_file):
        print(0)
        sys.exit(0)

    with open(state_file, 'r') as f:
        state_data = json.load(f)
    
    states = state_data.get('data', []) if isinstance(state_data.get('data'), list) else []
    original_count = len(states)
    states_to_keep = []
    
    for state_obj in states:
        if not isinstance(state_obj, dict):
            states_to_keep.append(state_obj)
            continue
        
        entity_id = None
        if 'state' in state_obj and isinstance(state_obj['state'], dict):
            entity_id = state_obj['state'].get('entity_id', '')
        elif 'entity_id' in state_obj:
            entity_id = state_obj.get('entity_id', '')
        
        if not entity_id or 'bme680' not in entity_id.lower():
            states_to_keep.append(state_obj)
    
    state_data['data'] = states_to_keep
    removed = original_count - len(states_to_keep)
    
    with open(state_file, 'w') as f:
        json.dump(state_data, f, indent=2)
    
    print(removed)
except Exception as e:
    print(f"Error: {e}", file=sys.stderr)
    print(0)
    sys.exit(1)
PYEOF
)
    fi
    
    # Remove from device registry (using sudo for direct file access)
    if [ -f "$device_file" ] && [ -f "$registry_file" ]; then
        device_removed_count=$(sudo python3 <<PYEOF 2>/dev/null
import json
import sys
import os

device_file = '$device_file'
entity_file = '$registry_file'

if not os.path.exists(device_file) or not os.path.exists(entity_file):
    print(0)
    sys.exit(0)

try:
    with open(device_file, 'r') as f:
        devices = json.load(f)
    
    # Get all device IDs that have entities matching the pattern
    with open(entity_file, 'r') as f:
        registry = json.load(f)
    
    matching_device_ids = set()
    for entity in registry.get('data', {}).get('entities', []):
        entity_id = entity.get('entity_id', '')
        unique_id = entity.get('unique_id', '')
        device_id = entity.get('device_id')
        
        if ('bme680' in unique_id.lower() or 'bme680' in entity_id.lower()) and device_id:
            matching_device_ids.add(device_id)
    
    # Now check if any devices have ALL their entities removed (orphaned)
    original_count = len(devices.get('data', {}).get('devices', []))
    devices_to_keep = []
    
    for device in devices.get('data', {}).get('devices', []):
        device_id = device.get('id')
        if device_id not in matching_device_ids:
            devices_to_keep.append(device)
    
    devices['data']['devices'] = devices_to_keep
    removed = original_count - len(devices_to_keep)
    
    with open(device_file, 'w') as f:
        json.dump(devices, f, indent=2)
    
    print(removed)
except Exception as e:
    print(f"Error: {e}", file=sys.stderr)
    print(0)
    sys.exit(1)
PYEOF
)
    fi
    
    # Remove from MQTT discovery storage (using sudo for direct file access)
    if [ -f "$discovery_file" ]; then
        mqtt_removed_count=$(sudo python3 <<PYEOF 2>/dev/null
import json
import sys
import os

discovery_file = '$discovery_file'

try:
    if not os.path.exists(discovery_file):
        print(0)
        sys.exit(0)
    
    with open(discovery_file, 'r') as f:
        discovery = json.load(f)
    
    items = discovery.get('data', {})
    original_count = len(items)
    
    # Remove items matching the pattern
    items_to_keep = {
        k: v for k, v in items.items()
        if 'bme680' not in k.lower()
    }
    
    discovery['data'] = items_to_keep
    removed = original_count - len(items_to_keep)
    
    with open(discovery_file, 'w') as f:
        json.dump(discovery, f, indent=2)
    
    print(removed)
except Exception as e:
    print(f"Error: {e}", file=sys.stderr)
    print(0)
    sys.exit(1)
PYEOF
)
    fi
    
    # Ensure all count variables are numeric
    removed_count=$(echo "${removed_count}" | tr -d '[:space:]' || echo "0")
    state_removed_count=$(echo "${state_removed_count}" | tr -d '[:space:]' || echo "0")
    device_removed_count=$(echo "${device_removed_count}" | tr -d '[:space:]' || echo "0")
    mqtt_removed_count=$(echo "${mqtt_removed_count}" | tr -d '[:space:]' || echo "0")
    
    if [ -n "$removed_count" ] && [ "$removed_count" -gt 0 ]; then
        print_success "Removed $removed_count BME680 entities from registry"
    else
        print_success "No BME680 entities found in registry (or already removed)"
    fi
    
    if [ -n "$state_removed_count" ] && [ "$state_removed_count" -gt 0 ]; then
        print_success "Removed $state_removed_count BME680 entity states from state file"
    fi
    
    if [ -n "$device_removed_count" ] && [ "$device_removed_count" -gt 0 ]; then
        print_success "Removed $device_removed_count BME680 device(s) from device registry"
    fi
    
    if [ -n "$mqtt_removed_count" ] && [ "$mqtt_removed_count" -gt 0 ]; then
        print_success "Removed $mqtt_removed_count MQTT discovery items"
    fi
    
    # Remove retained MQTT discovery messages from broker
    remove_mqtt_retained_messages
    
    # Note: HA restart is handled by the main uninstall flow, not here
} #<

remove_bme680_device() { #>
    # Device removal is now handled in remove_bme680_entities()
    # This function is kept for backward compatibility but does nothing
    print_info "Device removal handled as part of entity removal"
} #<

reload_ha_core() { #>
    print_info "Reloading Home Assistant core configuration..."
    
    # Try using ha helper script first (if available)
    if command -v ha >/dev/null 2>&1; then
        # Try reload_core_config first (faster than full restart)
        if ha service call config.reload_core_config >/dev/null 2>&1; then
            print_success "Home Assistant core configuration reloaded (via ha helper)"
            return 0
        fi
        # Fallback to restart if reload doesn't work
        if ha restart >/dev/null 2>&1; then
            print_success "Home Assistant restarted (via ha helper)"
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
    print_info "  Please reload manually:"
    print_info "    - Run: ha restart"
    print_info "    - Or use: Developer Tools > YAML > Reload Core Configuration"
    return 1
} #<

remove_mqtt_retained_messages() { #>
    print_info "Checking for retained MQTT discovery messages..."
    
    # Check if mosquitto tools are available
    if ! command -v mosquitto_pub >/dev/null 2>&1; then
        print_info "  MQTT tools not available, skipping retained message cleanup"
        return 0
    fi
    
    # Use paho-mqtt to discover retained messages (like remove-ha-entities.sh)
    print_info "Scanning broker for retained messages matching pattern 'bme680'..."
    
    RETAINED_TOPICS=$(python3 <<PYEOF 2>/dev/null
import paho.mqtt.client as mqtt
import json
import sys
import time

pattern = 'bme680'
topics_to_remove = set()
received_messages = {}

def on_connect(client, userdata, flags, rc):
    if rc == 0:
        client.subscribe('homeassistant/#')
    else:
        sys.exit(1)

def on_message(client, userdata, msg):
    if not msg.retain:
        return
    
    topic = msg.topic
    try:
        payload = msg.payload.decode('utf-8')
        if pattern in topic.lower() or pattern in payload.lower():
            topics_to_remove.add(topic)
        try:
            data = json.loads(payload)
            payload_str = json.dumps(data).lower()
            if pattern in payload_str:
                topics_to_remove.add(topic)
        except:
            pass
    except:
        pass

try:
    client = mqtt.Client()
    client.on_connect = on_connect
    client.on_message = on_message
    
    client.connect('localhost', 1883, 60)
    client.loop_start()
    
    time.sleep(3)
    
    client.loop_stop()
    client.disconnect()
    
    for topic in sorted(topics_to_remove):
        print(topic)
except Exception as e:
    print('', file=sys.stderr)
    sys.exit(0)
PYEOF
)
    
    if [ -n "$RETAINED_TOPICS" ]; then
        TOTAL_TOPICS=$(echo "$RETAINED_TOPICS" | grep -v '^$' | wc -l)
        print_info "Attempting to remove ${TOTAL_TOPICS} retained MQTT discovery topics..."
        
        REMOVED_TOPICS=0
        while IFS= read -r topic; do
            if [ -n "$topic" ]; then
                if timeout 2 mosquitto_pub -h localhost -t "$topic" -r -n 2>/dev/null; then
                    REMOVED_TOPICS=$((REMOVED_TOPICS + 1))
                fi
            fi
        done <<< "$RETAINED_TOPICS"
        
        if [ "$REMOVED_TOPICS" -gt 0 ]; then
            print_success "Removed ${REMOVED_TOPICS} retained MQTT discovery messages from broker"
        elif [ "$TOTAL_TOPICS" -gt 0 ]; then
            print_warning "Attempted to remove ${TOTAL_TOPICS} retained MQTT topics, but removal may have failed"
        else
            print_info "No retained MQTT discovery topics found to remove"
        fi
    else
        print_info "No retained MQTT discovery topics found to remove"
    fi
} #<

if [ "$EUID" -ne 0 ]; then #> Auto-elevate to root if needed
    if [ "$VERBOSE" = true ]; then
        print_info "This script requires sudo privileges"
        print_info "Attempting to elevate privileges..."
    fi
    exec sudo "$0" "$@"
fi #<

uninstall_service() { #>
    local service_name=$1
    
    if ! systemctl list-units --all --type=service | grep -q "$service_name.service"; then
        print_info "Service $service_name not installed, skipping"
        return 0
    fi
    
    print_info "Stopping $service_name service..."
    systemctl stop "$service_name.service" 2>/dev/null || true
    
    print_info "Disabling $service_name service..."
    systemctl disable "$service_name.service" 2>/dev/null || true
    
    print_info "Removing service file..."
    rm -f "/etc/systemd/system/$service_name.service"
    
    print_success "Uninstalled $service_name"
} #<

cleanup_all_service_files() { #>
    print_info "Cleaning up any remaining bme680 service files..."
    
    # Find all bme680 service files
    local service_files
    service_files=$(find /etc/systemd/system -name "*bme680*" -type f 2>/dev/null)
    
    if [ -n "$service_files" ]; then
        while IFS= read -r service_file; do
            if [ -f "$service_file" ]; then
                local service_name
                service_name=$(basename "$service_file")
                # Remove .backup extension if present to get actual service name
                local actual_service="${service_name%.backup}"
                actual_service="${actual_service%.service}"
                
                print_info "  Removing $service_name..."
                systemctl stop "$actual_service.service" 2>/dev/null || true
                systemctl disable "$actual_service.service" 2>/dev/null || true
                rm -f "$service_file"
                print_success "  Removed $service_name"
            fi
        done <<< "$service_files"
    else
        print_info "  No additional service files found"
    fi
} #<

print_info "BME680 Service Uninstallation"
echo

# Source iMenu (includes iWizard)
if [ -f "$IMENU_DIR/iMenu.sh" ]; then
    source "$IMENU_DIR/iMenu.sh"
else
    print_warning "iMenu not found at $IMENU_DIR/iMenu.sh"
    print_warning "Falling back to simple prompts..."
fi

# Build dynamic menu options based on what exists
menu_options=()
option_types=()  # Track what type each option is: "service", "ha_entities", "ha_package", "config"

# Check for services
if systemctl list-units --all --type=service | grep -q "bme680-base-mqtt.service"; then
    menu_options+=("Base readings service (MQTT) - Includes sensor readings and heatsoak calculations")
    option_types+=("service:bme680-base-mqtt")
fi

if systemctl list-units --all --type=service | grep -q "bme680-iaq-mqtt.service"; then
    menu_options+=("IAQ monitor service (MQTT)")
    option_types+=("service:bme680-iaq-mqtt")
fi

# Legacy service names (for backward compatibility)
if systemctl list-units --all --type=service | grep -q "bme680-heatsoak-mqtt.service"; then
    menu_options+=("Heat soak detection service (MQTT) - DEPRECATED (now part of base service)")
    option_types+=("service:bme680-heatsoak-mqtt")
fi

if systemctl list-units --all --type=service | grep -q "bme680-base.service"; then
    menu_options+=("Base readings service (legacy) - bme680-base")
    option_types+=("service:bme680-base")
fi

if systemctl list-units --all --type=service | grep -q "bme680-readings.service"; then
    menu_options+=("Sensor readings service (legacy) - bme680-readings")
    option_types+=("service:bme680-readings")
fi

if systemctl list-units --all --type=service | grep -q "bme680-heat-soak.service"; then
    menu_options+=("Heat soak detection service (legacy) - bme680-heat-soak")
    option_types+=("service:bme680-heat-soak")
fi

# Check for HA entities (always show option if HA is available, even if count is 0)
if docker ps --format '{{.Names}}' | grep -q "^homeassistant$"; then
    # Check for all BME680 entities
    entity_count=$(docker exec homeassistant python3 -c "
import json
try:
    with open('/config/.storage/core.entity_registry', 'r') as f:
        registry = json.load(f)
        bme_entities = [e for e in registry.get('data', {}).get('entities', []) if 'bme680' in e.get('unique_id', '').lower() or 'bme680' in e.get('entity_id', '').lower()]
        print(len(bme_entities))
except:
    print(0)
" 2>/dev/null || echo "0")
    
    # Check specifically for MQTT platform entities
    mqtt_entity_count=$(docker exec homeassistant python3 -c "
import json
try:
    with open('/config/.storage/core.entity_registry', 'r') as f:
        registry = json.load(f)
        bme_entities = [e for e in registry.get('data', {}).get('entities', []) if ('bme680' in e.get('unique_id', '').lower() or 'bme680' in e.get('entity_id', '').lower()) and e.get('platform') == 'mqtt']
        print(len(bme_entities))
except:
    print(0)
" 2>/dev/null || echo "0")
    
    # Always show the option if HA is available (for cleanup even if count is 0)
    if [ -n "$entity_count" ]; then
        if [ "$entity_count" -gt 0 ]; then
            if [ "$mqtt_entity_count" -gt 0 ] && [ "$mqtt_entity_count" -eq "$entity_count" ]; then
                # All entities are MQTT, show MQTT-specific label
                menu_options+=("Home Assistant MQTT entities ($mqtt_entity_count BME680 MQTT entities in entity registry)")
            elif [ "$mqtt_entity_count" -gt 0 ]; then
                # Some are MQTT, show both counts
                menu_options+=("Home Assistant entities ($entity_count total, $mqtt_entity_count MQTT BME680 entities in entity registry)")
            else
                # None are MQTT
                menu_options+=("Home Assistant entities ($entity_count BME680 entities in entity registry)")
            fi
        else
            menu_options+=("Home Assistant entities (0 BME680 entities in entity registry)")
        fi
        option_types+=("ha_entities")
    fi
fi

# Check for HA package file
if [ -f "$HA_PKG_DIR/bme680_mqtt.yaml" ] || [ -f "$HA_PKG_DIR/bme680_heatsoak_mqtt.yaml" ] || [ -f "$HA_PKG_DIR/bme680_heatsoak_mqtt.yaml.disabled" ] || [ -d "$HA_CUSTOM_COMPONENTS" ]; then
    menu_options+=("Home Assistant integration files (MQTT packages and custom component)")
    option_types+=("ha_package")
fi

# Check for config file
if [ -d "$CONFIG_DIR" ] && [ -f "$CONFIG_DIR/config.yaml" ]; then
    menu_options+=("Configuration files ($CONFIG_DIR)")
    option_types+=("config")
fi

# Collect all prompts first, then process
services_to_uninstall=()
remove_config=false
remove_ha_integration=false
remove_ha_entities=false

# If nothing found, inform user and exit
if [ ${#menu_options[@]} -eq 0 ]; then
    print_info "No BME680 components found to uninstall"
    exit 0
fi

# Build wizard configuration with dynamic multiselect
# Start with base config
UNINSTALL_WIZARD_CONFIG='{"title": "BME680 Service Uninstallation", "steps": []}'

# Single multiselect step with all available options (all selected by default)
if [ ${#menu_options[@]} -gt 0 ]; then
    # Convert menu_options array to JSON array format (simple strings)
    options_json="["
    first=true
    for option in "${menu_options[@]}"; do
        if [ "$first" = true ]; then
            first=false
        else
            options_json="$options_json,"
        fi
        # Escape quotes in option text
        escaped_option=$(echo "$option" | sed 's/"/\\"/g')
        options_json="$options_json\"$escaped_option\""
    done
    options_json="$options_json]"
    
    # Build preselect array (all indices selected by default)
    preselect_json="["
    first=true
    for i in "${!menu_options[@]}"; do
        if [ "$first" = true ]; then
            first=false
        else
            preselect_json="$preselect_json,"
        fi
        preselect_json="$preselect_json$i"
    done
    preselect_json="$preselect_json]"
    
    # Add single multiselect step with all options preselected
    step1=$(cat <<EOF
{
    "type": "multiselect",
    "message": "ℹ️  What would you like to uninstall? (All selected by default)",
    "options": $options_json,
    "preselect": $preselect_json
}
EOF
)
    UNINSTALL_WIZARD_CONFIG=$(echo "$UNINSTALL_WIZARD_CONFIG" | jq --argjson step "$step1" '.steps += [$step]' 2>/dev/null || echo "$UNINSTALL_WIZARD_CONFIG")
fi

# Run single wizard with all steps
if type iwizard_run_inline >/dev/null 2>&1 && [ -t 0 ] && [ -t 1 ]; then
    wizard_results=$(iwizard_run_inline "$UNINSTALL_WIZARD_CONFIG")
    wizard_exit=$?
    
    if [ $wizard_exit -ne 0 ]; then
        print_info "Uninstallation cancelled"
        exit 0
    fi
    
    # Parse results - single multiselect step
    # Result is an array of selected values (indices as strings)
    result_array=$(echo "$wizard_results" | jq -r ".step0.result[]?" 2>/dev/null || echo "")
    
    # Check if result is null or empty array
    result_check=$(echo "$wizard_results" | jq -r ".step0.result" 2>/dev/null || echo "null")
    if [ "$result_check" = "null" ] || [ "$result_check" = "[]" ] || [ -z "$result_array" ]; then
        # If nothing explicitly selected, use defaults (all selected)
        # This handles the case where user just presses enter with all defaults selected
        for i in "${!menu_options[@]}"; do
            option_type="${option_types[$i]}"
            case "$option_type" in
                service:*)
                    service_name="${option_type#service:}"
                    services_to_uninstall+=("$service_name")
                    ;;
                ha_entities)
                    remove_ha_entities=true
                    ;;
                ha_package)
                    remove_ha_integration=true
                    ;;
                config)
                    remove_config=true
                    ;;
            esac
        done
    else
        # Process explicitly selected options
        for idx in $result_array; do
            # Convert string index to integer
            idx_int=$((idx + 0))
            option_type="${option_types[$idx_int]}"
            case "$option_type" in
                service:*)
                    # Extract service name after the colon
                    service_name="${option_type#service:}"
                    services_to_uninstall+=("$service_name")
                    ;;
                ha_entities)
                    remove_ha_entities=true
                    ;;
                ha_package)
                    remove_ha_integration=true
                    ;;
                config)
                    remove_config=true
                    ;;
            esac
        done
    fi
elif type interactive_menu >/dev/null 2>&1 && [ -t 0 ] && [ -t 1 ]; then
    # Fallback to old interactive_menu if available
    selected=$(interactive_menu "${menu_options[@]}")
    menu_exit=$?
    
    if [ $menu_exit -ne 0 ] || [ -z "$selected" ]; then
        print_info "Uninstallation cancelled"
        exit 0
    fi
    
    # Process selected options based on their types
    for idx in $selected; do
        option_type="${option_types[$idx]}"
        case "$option_type" in
            service:*)
                # Extract service name after the colon
                service_name="${option_type#service:}"
                services_to_uninstall+=("$service_name")
                ;;
            ha_entities)
                remove_ha_entities=true
                ;;
            ha_package)
                remove_ha_integration=true
                ;;
            config)
                remove_config=true
                ;;
        esac
    done
else
    # Fallback to simple prompts
    print_warning "Interactive menu not available, using simple prompts..."
    
    echo
    print_info "Available components to uninstall:"
    for i in "${!menu_options[@]}"; do
        echo "  $((i+1))) ${menu_options[$i]}"
    done
    echo
    
    read -p "Enter numbers (space-separated) of items to uninstall, or 'all' for everything [default: all]: " selection
    
    # Default to "all" if empty
    if [ -z "$selection" ]; then
        selection="all"
    fi
    
    if [ "$selection" = "all" ]; then
        # Select all options
        for i in "${!menu_options[@]}"; do
            option_type="${option_types[$i]}"
            case "$option_type" in
                service:*)
                    service_name="${option_type#service:}"
                    services_to_uninstall+=("$service_name")
                    ;;
                ha_entities)
                    remove_ha_entities=true
                    ;;
                ha_package)
                    remove_ha_integration=true
                    ;;
                config)
                    remove_config=true
                    ;;
            esac
        done
    else
        # Process selected indices
        for idx in $selection; do
            # Convert to 0-based index
            actual_idx=$((idx - 1))
            if [ $actual_idx -ge 0 ] && [ $actual_idx -lt ${#menu_options[@]} ]; then
                option_type="${option_types[$actual_idx]}"
                case "$option_type" in
                    service:*)
                        service_name="${option_type#service:}"
                        services_to_uninstall+=("$service_name")
                        ;;
                    ha_entities)
                        remove_ha_entities=true
                        ;;
                    ha_package)
                        remove_ha_integration=true
                        ;;
                    config)
                        remove_config=true
                        ;;
                esac
            fi
        done
    fi
fi

# If nothing selected, exit
if [ ${#services_to_uninstall[@]} -eq 0 ] && [ "$remove_ha_integration" = false ] && [ "$remove_ha_entities" = false ] && [ "$remove_config" = false ]; then
    print_info "Nothing selected. Uninstallation cancelled."
    exit 0
fi

# Show summary of what will be uninstalled
echo
print_info "Uninstalling the following:"
echo

if [ ${#services_to_uninstall[@]} -gt 0 ]; then
    for service in "${services_to_uninstall[@]}"; do
        echo "  • $service service"
    done
    # Package files and CLI are always removed if services are being uninstalled
    if [ -d "$INSTALL_ROOT" ]; then
        echo "  • Package files ($INSTALL_ROOT)"
    fi
    if [ -f "$INSTALL_BIN/bme680-cli" ]; then
        echo "  • CLI tool ($INSTALL_BIN/bme680-cli)"
    fi
fi

if [ "$remove_ha_entities" = true ]; then
    echo "  • Home Assistant entities (from entity registry)"
fi

if [ "$remove_ha_integration" = true ]; then
    echo "  • Home Assistant integration files:"
    if [ -f "$HA_PKG_DIR/bme680_mqtt.yaml" ]; then
        echo "    - MQTT package: $HA_PKG_DIR/bme680_mqtt.yaml"
    fi
    if [ -f "$HA_PKG_DIR/bme680_heatsoak_mqtt.yaml" ]; then
        echo "    - MQTT package: $HA_PKG_DIR/bme680_heatsoak_mqtt.yaml"
    fi
    if [ -f "$HA_PKG_DIR/bme680_heatsoak_mqtt.yaml.disabled" ]; then
        echo "    - MQTT package: $HA_PKG_DIR/bme680_heatsoak_mqtt.yaml.disabled"
    fi
    if [ -d "$HA_CUSTOM_COMPONENTS" ]; then
        echo "    - Custom component: $HA_CUSTOM_COMPONENTS"
    fi
fi

if [ "$remove_config" = true ]; then
    echo "  • Configuration files ($CONFIG_DIR)"
fi

echo

# Now process all the options in the correct order:
# 1. Stop HA (if HA files/entities are being removed)
# 2. Remove HA integration files
# 3. Remove HA entities (but don't restart yet)
# 4. Remove services
# 5. Remove config files
# 6. Remove package files and CLI
# 7. Restart HA at the end (if it was stopped)

HA_CONTAINER="homeassistant"
HA_WAS_RUNNING=false

# Step 1: Stop HA first if we're removing HA files or entities
if [ "$remove_ha_integration" = true ] || [ "$remove_ha_entities" = true ]; then
    if docker ps --format '{{.Names}}' | grep -q "^${HA_CONTAINER}$"; then
        print_info "Stopping Home Assistant..."
        if docker stop "${HA_CONTAINER}" >/dev/null 2>&1; then
            print_success "Home Assistant stopped"
            HA_WAS_RUNNING=true
        else
            print_warning "Could not stop Home Assistant (may already be stopped)"
        fi
    fi
fi

# Step 2: Remove HA integration files
if [ "$remove_ha_integration" = true ]; then
    print_info "Removing Home Assistant integration files..."
    
    # Remove MQTT package files
    if [ -f "$HA_PKG_DIR/bme680_mqtt.yaml" ]; then
        rm -f "$HA_PKG_DIR/bme680_mqtt.yaml"
        print_success "Removed MQTT package: bme680_mqtt.yaml"
    fi
    
    if [ -f "$HA_PKG_DIR/bme680_heatsoak_mqtt.yaml" ]; then
        rm -f "$HA_PKG_DIR/bme680_heatsoak_mqtt.yaml"
        print_success "Removed MQTT package: bme680_heatsoak_mqtt.yaml"
    fi
    
    if [ -f "$HA_PKG_DIR/bme680_heatsoak_mqtt.yaml.disabled" ]; then
        rm -f "$HA_PKG_DIR/bme680_heatsoak_mqtt.yaml.disabled"
        print_success "Removed MQTT package: bme680_heatsoak_mqtt.yaml.disabled"
    fi
    
    # Remove custom component (check both possible locations)
    if [ -d "$HA_CUSTOM_COMPONENTS" ]; then
        rm -rf "$HA_CUSTOM_COMPONENTS"
        print_success "Removed custom component: bme680_monitor"
    fi
    # Also check for old location (bme680-monitor instead of bme680_monitor)
    if [ -d "$ORIGINAL_HOME/homeassistant/bme680-monitor" ]; then
        rm -rf "$ORIGINAL_HOME/homeassistant/bme680-monitor"
        print_success "Removed custom component (old location): bme680-monitor"
    fi
fi

# Step 3: Remove HA entities (but don't restart yet - we'll do that at the end)
if [ "$remove_ha_entities" = true ]; then
    # Temporarily modify remove_bme680_entities to not restart HA
    # We'll restart at the end after all files are removed
    remove_bme680_entities_no_restart
fi

# Step 4: Remove services
if [ ${#services_to_uninstall[@]} -gt 0 ]; then
    print_info "Uninstalling services..."
    for service in "${services_to_uninstall[@]}"; do
        uninstall_service "$service"
    done
    
    print_info "Reloading systemd daemon..."
    systemctl daemon-reload
    
    # Clean up any remaining service files (backups, deprecated services, etc.)
    cleanup_all_service_files
    
    # Kill any remaining bme680 processes
    print_info "Stopping any remaining bme680 processes..."
    pkill -f "bme680.*wrapper" 2>/dev/null || true
    pkill -f "base-readings.py" 2>/dev/null || true
    pkill -f "monitor-heatsoak.py" 2>/dev/null || true
    pkill -f "monitor-iaq.py" 2>/dev/null || true
    sleep 1  # Give processes time to stop
fi

# Step 5: Remove config files
if [ "$remove_config" = true ]; then
    print_info "Removing configuration files..."
    
    # Check if config.yaml is a symlink
    if [ -L "$CONFIG_DIR/config.yaml" ]; then
        symlink_target=$(readlink -f "$CONFIG_DIR/config.yaml" 2>/dev/null)
        print_info "  Config file is a symlink pointing to: $symlink_target"
        print_info "  Removing symlink only (target file will be preserved)"
        rm -f "$CONFIG_DIR/config.yaml"
        # Remove directory only if it's empty
        if [ -d "$CONFIG_DIR" ] && [ -z "$(ls -A "$CONFIG_DIR" 2>/dev/null)" ]; then
            rmdir "$CONFIG_DIR" 2>/dev/null || true
        fi
    else
        # Regular file or directory - remove normally
        rm -rf "$CONFIG_DIR"
    fi
    print_success "Configuration files removed"
fi

# Step 6: Remove package files and CLI (if services were uninstalled)
if [ ${#services_to_uninstall[@]} -gt 0 ]; then
    if [ -d "$INSTALL_ROOT" ]; then
        print_info "Removing package files..."
        rm -rf "$INSTALL_ROOT"
        print_success "Package files removed"
    fi
    
    if [ -f "$INSTALL_BIN/bme680-cli" ]; then
        print_info "Removing CLI tool..."
        rm -f "$INSTALL_BIN/bme680-cli"
        print_success "CLI tool removed"
    fi
fi

# Step 7: Restart HA at the end (if it was stopped)
if [ "$HA_WAS_RUNNING" = true ]; then
    echo
    print_info "Restarting Home Assistant..."
    print_info "Waiting for HA to fully initialize and process storage files..."
    if docker start "${HA_CONTAINER}" >/dev/null 2>&1; then
        print_success "Home Assistant started"
        sleep 10
        sync  # Force filesystem sync
        sleep 5  # Additional wait for HA to process cleaned storage
        print_success "Home Assistant restarted"
    else
        print_error "Could not start Home Assistant"
        print_warning "⚠️  REQUIRED: Start HA manually with: docker start ${HA_CONTAINER}"
    fi
fi

echo
print_success "Uninstallation complete!"
