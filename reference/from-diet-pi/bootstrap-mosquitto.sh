#!/usr/bin/env bash
# Bootstrap Mosquitto MQTT Broker setup for DietPi
# This script sets up Mosquitto with password authentication and CLI tools

set -e

echo "🚀 Bootstrapping Mosquitto MQTT Broker setup..."

# Check if mosquitto is installed
if ! command -v mosquitto &> /dev/null; then
    echo "❌ Error: Mosquitto broker is not installed"
    echo "   Please install Mosquitto via DietPi programs first"
    exit 1
fi

echo "✅ Mosquitto broker is installed ($(mosquitto -v 2>&1 | head -1))"

# Install mosquitto-clients (CLI tools)
echo "📦 Installing mosquitto-clients (CLI tools)..."
if ! command -v mosquitto_pub &> /dev/null || ! command -v mosquitto_sub &> /dev/null; then
    if command -v apt-get &> /dev/null; then
        sudo apt-get update && sudo apt-get install -y mosquitto-clients
    elif command -v yum &> /dev/null; then
        sudo yum install -y mosquitto-clients
    elif command -v dnf &> /dev/null; then
        sudo dnf install -y mosquitto-clients
    elif command -v pacman &> /dev/null; then
        sudo pacman -S --noconfirm mosquitto-clients
    else
        echo "❌ Error: Could not detect package manager to install mosquitto-clients"
        exit 1
    fi
    echo "✅ mosquitto-clients installed successfully"
else
    echo "✅ mosquitto-clients already installed"
fi

# Check if password file exists, create if it doesn't
PASSWD_FILE="/etc/mosquitto/passwd"
if [ ! -f "$PASSWD_FILE" ]; then
    echo "📝 Creating password file..."
    sudo touch "$PASSWD_FILE"
    echo "✅ Password file created at $PASSWD_FILE"
else
    echo "✅ Password file already exists at $PASSWD_FILE"
fi

# Set proper ownership and permissions for password file
echo "🔐 Setting up password file permissions..."
sudo chown root:mosquitto "$PASSWD_FILE"
sudo chmod 640 "$PASSWD_FILE"
echo "✅ Password file permissions set (root:mosquitto, 640)"

# Verify mosquitto.conf exists and has password_file configured
CONF_FILE="/etc/mosquitto/mosquitto.conf"
if [ -f "$CONF_FILE" ]; then
    if ! grep -q "password_file" "$CONF_FILE"; then
        echo "⚠️  Warning: password_file not found in $CONF_FILE"
        echo "   You may need to add: password_file /etc/mosquitto/passwd"
    else
        echo "✅ password_file configured in mosquitto.conf"
    fi
else
    echo "⚠️  Warning: mosquitto.conf not found at $CONF_FILE"
fi

# Restart mosquitto service to apply changes
echo "🔄 Restarting mosquitto service..."
if sudo systemctl is-active --quiet mosquitto; then
    sudo systemctl restart mosquitto
    echo "✅ Mosquitto service restarted"
else
    echo "⚠️  Mosquitto service is not running"
    echo "   Start it with: sudo systemctl start mosquitto"
fi

# Verify service status
echo ""
echo "📊 Mosquitto service status:"
sudo systemctl status mosquitto --no-pager -l | head -10

# Setup unified secrets file
echo ""
echo "🔐 Setting up unified secrets file..."
SECRETS_FILE="$HOME/.secrets"

# Create .gitignore if it doesn't exist
if [ ! -f "$HOME/.gitignore" ]; then
    cat > "$HOME/.gitignore" << 'EOF'
# Unified Secrets File
.secrets

# Other sensitive files
*.key
*.pem
*.p12
.env
.env.local
EOF
    echo "✅ Created ~/.gitignore"
else
    # Add .secrets to .gitignore if not already present
    if ! grep -q "^\.secrets$" "$HOME/.gitignore" 2>/dev/null; then
        echo ".secrets" >> "$HOME/.gitignore"
        echo "✅ Added .secrets to ~/.gitignore"
    else
        echo "✅ .secrets already in ~/.gitignore"
    fi
fi

# Create secrets file if it doesn't exist
if [ ! -f "$SECRETS_FILE" ]; then
    cat > "$SECRETS_FILE" << 'EOF'
# Unified Secrets File
# Add your passwords/keys here
# 
# Format: KEY=VALUE (one per line)
# Comments start with #

# ============================================
# MQTT Broker
# ============================================
MQTT_PASSWORD=
MQTT_USERNAME=mqtt

# ============================================
# GitHub (if needed)
# ============================================
# GITHUB_TOKEN=

# ============================================
# Other APIs / Services
# ============================================
# API_KEY_SERVICE1=
# API_KEY_SERVICE2=
EOF
    chmod 600 "$SECRETS_FILE"
    echo "✅ Created ~/.secrets file"
    echo "⚠️  IMPORTANT: Edit ~/.secrets and add your MQTT_PASSWORD"
    echo "   Use: sudo mqtt setup-universal (will prompt for password)"
else
    echo "✅ Secrets file exists at $SECRETS_FILE"
    # Verify permissions
    PERMS=$(stat -c "%a" "$SECRETS_FILE" 2>/dev/null || stat -f "%OLp" "$SECRETS_FILE" 2>/dev/null || echo "unknown")
    if [ "$PERMS" != "600" ]; then
        echo "🔒 Fixing secrets file permissions..."
        chmod 600 "$SECRETS_FILE"
        echo "✅ Secrets file permissions set to 600"
    else
        echo "✅ Secrets file permissions are correct (600)"
    fi
fi

echo ""
echo "✅ Mosquitto bootstrap complete!"
echo ""
echo "📝 Next steps:"
echo "   • Set up secrets file (if not done):"
echo "     cp ~/.secrets.example ~/.secrets"
echo "     chmod 600 ~/.secrets"
echo "     # Edit ~/.secrets and add MQTT_PASSWORD=your_password"
echo ""
echo "   • Use the mqtt-helper to add users:"
echo "     sudo mqtt user add <username> [password]"
echo ""
echo "   • Or use setup-universal for one password:"
echo "     sudo mqtt setup-universal"
echo ""
echo "   • Test connection with CLI tools:"
echo "     mosquitto_pub -h localhost -t test -m 'Hello MQTT'"
echo "     mosquitto_sub -h localhost -t test"


