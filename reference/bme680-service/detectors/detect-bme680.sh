#!/bin/bash
# BME680 Sensor Detection Script
# Detects if BME680 sensor is present on I2C bus
# Checks BOTH addresses (0x76 and 0x77) and verifies chip ID (0x61) to ensure it's a BME680

set -e

I2C_BUS="${1:-1}"
BME680_ADDR_PRIMARY="0x76"
BME680_ADDR_SECONDARY="0x77"
CHIP_ID_REGISTER="0xD0"  # BME680 chip ID register
EXPECTED_CHIP_ID="0x61"   # BME680 unique chip identifier

# Colors (for verbose output only)
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

detect_bme680() {
    local i2c_addr=$1
    local addr_dec=$((16#${i2c_addr#0x}))
    
    # Check if device responds at this address
    if ! i2cget -y "$I2C_BUS" "$addr_dec" "$CHIP_ID_REGISTER" b &>/dev/null; then
        return 1  # No device at this address
    fi
    
    # Read chip ID register
    local chip_id_raw=$(i2cget -y "$I2C_BUS" "$addr_dec" "$CHIP_ID_REGISTER" b 2>/dev/null)
    
    # Normalize chip ID (remove newlines, ensure lowercase for comparison)
    local chip_id=$(echo "$chip_id_raw" | tr -d '\n\r' | tr '[:upper:]' '[:lower:]')
    local expected_id=$(echo "$EXPECTED_CHIP_ID" | tr '[:upper:]' '[:lower:]')
    
    # Verify it's actually a BME680 (chip ID must be 0x61)
    if [ "$chip_id" = "$expected_id" ]; then
        echo "$i2c_addr"
        return 0
    fi
    
    # Device exists but is NOT a BME680 (wrong chip ID)
    return 2
}

# Check both addresses - always check BOTH
FOUND_PRIMARY=false
FOUND_SECONDARY=false

# Check primary address (0x76)
if PRIMARY_ADDR=$(detect_bme680 "$BME680_ADDR_PRIMARY" 2>/dev/null); then
    FOUND_PRIMARY=true
fi

# Check secondary address (0x77)
if SECONDARY_ADDR=$(detect_bme680 "$BME680_ADDR_SECONDARY" 2>/dev/null); then
    FOUND_SECONDARY=true
fi

# Report results
if [ "$FOUND_PRIMARY" = true ] && [ "$FOUND_SECONDARY" = true ]; then
    # Both addresses have BME680 - unusual but possible if multiple sensors
    echo "$BME680_ADDR_PRIMARY"
    echo "$BME680_ADDR_SECONDARY" >&2  # Secondary goes to stderr for info
    exit 0
elif [ "$FOUND_PRIMARY" = true ]; then
    echo "$BME680_ADDR_PRIMARY"
    exit 0
elif [ "$FOUND_SECONDARY" = true ]; then
    echo "$BME680_ADDR_SECONDARY"
    exit 0
else
    # No BME680 found at either address
    exit 1
fi

