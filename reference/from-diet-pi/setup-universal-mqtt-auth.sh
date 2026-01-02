#!/usr/bin/env bash
# Setup universal MQTT authentication with a single password
# This creates one user that all devices can use
# Password can be stored in secrets file or environment variable

set -e

PASSWD_FILE="/etc/mosquitto/passwd"
MOSQUITTO_PASSWD="/usr/bin/mosquitto_passwd"
CONF_FILE="/etc/mosquitto/mosquitto.conf"
SECRETS_FILE="${HOME}/.secrets"  # Unified secrets file

# Check if running as root or with sudo
if [ "$EUID" -ne 0 ]; then
    echo "❌ Error: This script must be run as root or with sudo"
    exit 1
fi

# Function to get password from various sources
get_password() {
    # 1. Check environment variable
    if [ -n "$MQTT_PASSWORD" ]; then
        echo "$MQTT_PASSWORD"
        return 0
    fi
    
    # 2. Check secrets file
    if [ -f "$SECRETS_FILE" ]; then
        # Try to read password from file (first line, or MQTT_PASSWORD= line)
        if grep -q "^MQTT_PASSWORD=" "$SECRETS_FILE" 2>/dev/null; then
            grep "^MQTT_PASSWORD=" "$SECRETS_FILE" | cut -d'=' -f2- | tr -d '"' | tr -d "'"
            return 0
        elif [ -s "$SECRETS_FILE" ]; then
            # If file exists and has content, use first line as password
            head -n1 "$SECRETS_FILE" | tr -d '\n'
            return 0
        fi
    fi
    
    # 3. Prompt user
    read -sp "Enter MQTT password: " password
    echo
    read -sp "Confirm password: " password_confirm
    echo
    
    if [ "$password" != "$password_confirm" ]; then
        echo "❌ Passwords don't match"
        exit 1
    fi
    
    echo "$password"
}

# Get password
PASSWORD=$(get_password)

if [ -z "$PASSWORD" ]; then
    echo "❌ Error: Password cannot be empty"
    exit 1
fi

# Universal username (can be customized)
USERNAME="${MQTT_USERNAME:-mqtt}"

echo "🔐 Setting up universal MQTT authentication..."
echo "   Username: $USERNAME"
echo "   Password: [hidden]"

# Create password file if it doesn't exist
if [ ! -f "$PASSWD_FILE" ]; then
    echo "📝 Creating password file..."
    touch "$PASSWD_FILE"
fi

# Create or update the universal user
if grep -q "^${USERNAME}:" "$PASSWD_FILE" 2>/dev/null; then
    echo "🔄 Updating existing user '$USERNAME'..."
    echo "$PASSWORD" | "$MOSQUITTO_PASSWD" -b "$PASSWD_FILE" "$USERNAME"
else
    echo "➕ Creating new user '$USERNAME'..."
    echo "$PASSWORD" | "$MOSQUITTO_PASSWD" -b "$PASSWD_FILE" "$USERNAME"
fi

# Set proper permissions
chown root:mosquitto "$PASSWD_FILE"
chmod 640 "$PASSWD_FILE"

# Update mosquitto.conf to disable anonymous access
if [ -f "$CONF_FILE" ]; then
    if grep -q "^allow_anonymous" "$CONF_FILE"; then
        echo "🔒 Disabling anonymous access..."
        sed -i 's/^allow_anonymous.*/allow_anonymous false/' "$CONF_FILE"
    else
        echo "🔒 Adding 'allow_anonymous false' to config..."
        # Add before listener line if it exists, or at end
        if grep -q "^listener" "$CONF_FILE"; then
            sed -i '/^listener/i allow_anonymous false' "$CONF_FILE"
        else
            echo "allow_anonymous false" >> "$CONF_FILE"
        fi
    fi
    echo "✅ Configuration updated"
else
    echo "⚠️  Warning: $CONF_FILE not found"
fi

# Restart mosquitto
echo "🔄 Restarting mosquitto service..."
if systemctl is-active --quiet mosquitto; then
    systemctl restart mosquitto
    echo "✅ Mosquitto service restarted"
else
    echo "⚠️  Mosquitto service is not running"
    echo "   Start it with: systemctl start mosquitto"
fi

echo ""
echo "✅ Universal MQTT authentication setup complete!"
echo ""
echo "📋 Credentials:"
echo "   Username: $USERNAME"
echo "   Password: [stored securely]"
echo ""
echo "💡 Usage:"
echo "   All devices should use:"
echo "     Username: $USERNAME"
echo "     Password: [from secrets file or environment]"
echo ""
echo "📝 To store password in secrets file:"
echo "   echo 'MQTT_PASSWORD=your_password_here' >> ~/.secrets"
echo "   chmod 600 ~/.secrets"
echo ""
echo "📝 Or use environment variable:"
echo "   export MQTT_PASSWORD='your_password_here'"
echo "   sudo -E $0"


