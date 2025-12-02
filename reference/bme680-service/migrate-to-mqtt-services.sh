#!/bin/bash
# Migration script: Old services → New MQTT-named services
# Stops old services, installs new MQTT services, removes old service files

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_SCRIPT="$SCRIPT_DIR/install.sh"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

print_header() {
    echo -e "${CYAN}════════════════════════════════════════${NC}"
    echo -e "${CYAN}  Migrate to MQTT-Named Services${NC}"
    echo -e "${CYAN}════════════════════════════════════════${NC}"
    echo
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_info() {
    echo -e "${CYAN}ℹ️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Auto-elevate to root if needed
if [ "$EUID" -ne 0 ]; then
    print_info "This script requires sudo privileges"
    print_info "Attempting to elevate privileges..."
    exec sudo "$0" "$@"
fi

print_header

# Step 1: Stop old services
print_info "Step 1: Stopping old services..."
OLD_SERVICES=(
    "bme680-base"
    "bme680-readings"
    "bme680-iaq"
    "bme680-heat-soak"
    "bme680-heatsoak"
)

for service in "${OLD_SERVICES[@]}"; do
    if systemctl is-active --quiet "$service.service" 2>/dev/null; then
        print_info "  Stopping $service.service..."
        systemctl stop "$service.service" 2>/dev/null || true
        print_success "  Stopped $service.service"
    fi
done

# Step 2: Disable old services
print_info "Step 2: Disabling old services..."
for service in "${OLD_SERVICES[@]}"; do
    if systemctl is-enabled --quiet "$service.service" 2>/dev/null; then
        print_info "  Disabling $service.service..."
        systemctl disable "$service.service" 2>/dev/null || true
        print_success "  Disabled $service.service"
    fi
done

# Step 3: Remove old service files
print_info "Step 3: Removing old service files..."
for service in "${OLD_SERVICES[@]}"; do
    if [ -f "/etc/systemd/system/$service.service" ]; then
        print_info "  Removing /etc/systemd/system/$service.service..."
        rm -f "/etc/systemd/system/$service.service"
        print_success "  Removed $service.service"
    fi
done

# Step 4: Reload systemd
print_info "Step 4: Reloading systemd daemon..."
systemctl daemon-reload
print_success "Systemd daemon reloaded"

# Step 5: Run install script to install new services
print_info "Step 5: Installing new MQTT-named services..."
echo
print_warning "The install script will prompt you to select services."
print_warning "Select the same services you had before (base readings, IAQ, heat soak)."
echo
read -p "Press Enter to continue with installation..." -r
echo

# Run install script (non-interactive mode - we'll pass through)
if [ -f "$INSTALL_SCRIPT" ]; then
    bash "$INSTALL_SCRIPT"
else
    print_error "Install script not found: $INSTALL_SCRIPT"
    exit 1
fi

# Step 6: Verify new services
echo
print_info "Step 6: Verifying new services..."
NEW_SERVICES=(
    "bme680-base-mqtt"
    "bme680-iaq-mqtt"
    "bme680-heatsoak-mqtt"
)

for service in "${NEW_SERVICES[@]}"; do
    if systemctl is-active --quiet "$service.service" 2>/dev/null; then
        print_success "$service.service is running"
    elif systemctl list-units --all --type=service | grep -q "$service.service"; then
        print_warning "$service.service exists but is not running"
    else
        print_warning "$service.service not found (may not have been selected during install)"
    fi
done

echo
print_success "Migration complete!"
echo
print_info "New service commands:"
echo "  systemctl status bme680-base-mqtt"
echo "  systemctl status bme680-iaq-mqtt"
echo "  systemctl status bme680-heatsoak-mqtt"
echo
print_info "Old service files have been removed."
print_info "Old scripts remain in ~/.local/share/bme680-service/ but are not used."
print_warning "You can manually clean up old scripts if desired (they're in the root directory)."

