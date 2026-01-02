#!/usr/bin/env bash
# Add a username and password to Mosquitto MQTT broker
# Usage: add-mqtt-user.sh <username> [password]
#   If password is not provided, it will be prompted securely

set -e

PASSWD_FILE="/etc/mosquitto/passwd"
MOSQUITTO_PASSWD="/usr/bin/mosquitto_passwd"

# Check if running as root or with sudo
if [ "$EUID" -ne 0 ]; then
    echo "❌ Error: This script must be run as root or with sudo"
    exit 1
fi

# Check if mosquitto_passwd is available
if [ ! -f "$MOSQUITTO_PASSWD" ]; then
    echo "❌ Error: mosquitto_passwd not found at $MOSQUITTO_PASSWD"
    echo "   Please install mosquitto-clients package first"
    exit 1
fi

# Check if password file exists
if [ ! -f "$PASSWD_FILE" ]; then
    echo "📝 Creating password file at $PASSWD_FILE..."
    touch "$PASSWD_FILE"
    chown root:mosquitto "$PASSWD_FILE"
    chmod 640 "$PASSWD_FILE"
fi

# Get username
if [ -z "$1" ]; then
    echo "Usage: $0 <username> [password]"
    echo "   If password is not provided, it will be prompted securely"
    exit 1
fi

USERNAME="$1"

# Check if user already exists
if grep -q "^${USERNAME}:" "$PASSWD_FILE" 2>/dev/null; then
    echo "⚠️  User '$USERNAME' already exists in password file"
    read -p "Update password? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ Cancelled"
        exit 0
    fi
    # Update existing user
    if [ -n "$2" ]; then
        # Password provided as argument
        echo "$2" | "$MOSQUITTO_PASSWD" -b "$PASSWD_FILE" "$USERNAME"
    else
        # Prompt for password
        "$MOSQUITTO_PASSWD" "$PASSWD_FILE" "$USERNAME"
    fi
    echo "✅ Password updated for user '$USERNAME'"
else
    # Add new user
    if [ -n "$2" ]; then
        # Password provided as argument
        echo "$2" | "$MOSQUITTO_PASSWD" -b "$PASSWD_FILE" "$USERNAME"
    else
        # Prompt for password
        "$MOSQUITTO_PASSWD" "$PASSWD_FILE" "$USERNAME"
    fi
    echo "✅ User '$USERNAME' added to password file"
fi

# Ensure proper permissions
chown root:mosquitto "$PASSWD_FILE"
chmod 640 "$PASSWD_FILE"

# Restart mosquitto to apply changes
echo "🔄 Restarting mosquitto service..."
if systemctl is-active --quiet mosquitto; then
    systemctl restart mosquitto
    echo "✅ Mosquitto service restarted"
else
    echo "⚠️  Mosquitto service is not running"
    echo "   Start it with: systemctl start mosquitto"
fi

# List current users
echo ""
echo "📋 Current users in password file:"
cut -d: -f1 "$PASSWD_FILE" | while read -r user; do
    echo "   • $user"
done

echo ""
echo "✅ Done! User '$USERNAME' is now configured for MQTT authentication"

