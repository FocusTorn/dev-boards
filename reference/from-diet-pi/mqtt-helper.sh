#!/usr/bin/env bash
# MQTT Helper - Orchestrator script for MQTT broker management
# Provides a unified interface for all MQTT operations

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PASSWD_FILE="/etc/mosquitto/passwd"
CONF_FILE="/etc/mosquitto/mosquitto.conf"
SECRETS_FILE="${HOME}/.secrets"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
error() {
    echo -e "${RED}❌ $1${NC}" >&2
}

success() {
    echo -e "${GREEN}✅ $1${NC}"
}

info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# Check if running as root
check_root() {
    if [ "$EUID" -ne 0 ]; then
        error "This command requires root privileges. Use sudo."
        exit 1
    fi
}

# Check if mosquitto is installed
check_mosquitto_installed() {
    command -v mosquitto &> /dev/null
}

# Check if mosquitto-clients are installed
check_mosquitto_clients_installed() {
    command -v mosquitto_pub &> /dev/null && command -v mosquitto_sub &> /dev/null
}

# Setup mosquitto broker (install on Debian, configure)
setup_mosquitto() {
    info "🚀 Setting up Mosquitto MQTT Broker..."
    echo ""
    
    # Check if mosquitto is installed
    if ! check_mosquitto_installed; then
        warning "Mosquitto broker is not installed"
        info "Installing mosquitto broker..."
        
        # Detect package manager and install
        if command -v apt-get &> /dev/null; then
            apt-get update
            apt-get install -y mosquitto mosquitto-clients
        elif command -v yum &> /dev/null; then
            yum install -y mosquitto mosquitto-clients
        elif command -v dnf &> /dev/null; then
            dnf install -y mosquitto mosquitto-clients
        elif command -v pacman &> /dev/null; then
            pacman -S --noconfirm mosquitto mosquitto-clients
        else
            error "Could not detect package manager to install mosquitto"
            exit 1
        fi
        
        success "mosquitto installed successfully"
    else
        success "Mosquitto broker is installed ($(mosquitto -v 2>&1 | head -1))"
    fi
    
    # Install mosquitto-clients if needed
    if ! check_mosquitto_clients_installed; then
        info "Installing mosquitto-clients (CLI tools)..."
        if command -v apt-get &> /dev/null; then
            apt-get install -y mosquitto-clients
        elif command -v yum &> /dev/null; then
            yum install -y mosquitto-clients
        elif command -v dnf &> /dev/null; then
            dnf install -y mosquitto-clients
        elif command -v pacman &> /dev/null; then
            pacman -S --noconfirm mosquitto-clients
        else
            error "Could not detect package manager"
            exit 1
        fi
        success "mosquitto-clients installed successfully"
    else
        success "mosquitto-clients already installed"
    fi
    
    # Check if password file exists, create if it doesn't
    if [ ! -f "$PASSWD_FILE" ]; then
        info "Creating password file..."
        touch "$PASSWD_FILE"
        chown root:mosquitto "$PASSWD_FILE"
        chmod 640 "$PASSWD_FILE"
        success "Password file created at $PASSWD_FILE"
    else
        success "Password file already exists at $PASSWD_FILE"
    fi
    
    # Set proper ownership and permissions for password file
    info "Setting up password file permissions..."
    chown root:mosquitto "$PASSWD_FILE"
    chmod 640 "$PASSWD_FILE"
    success "Password file permissions set (root:mosquitto, 640)"
    
    # Verify mosquitto.conf exists and has password_file configured
    if [ -f "$CONF_FILE" ]; then
        if ! grep -q "password_file" "$CONF_FILE"; then
            warning "password_file not found in $CONF_FILE"
            info "You may need to add: password_file $PASSWD_FILE"
        else
            success "password_file configured in mosquitto.conf"
        fi
    else
        warning "mosquitto.conf not found at $CONF_FILE"
    fi
    
    # Restart mosquitto service to apply changes
    info "Restarting mosquitto service..."
    if systemctl is-active --quiet mosquitto; then
        systemctl restart mosquitto
        success "Mosquitto service restarted"
    else
        warning "Mosquitto service is not running"
        info "Start it with: systemctl start mosquitto"
    fi
    
    # Verify service status
    echo ""
    info "Mosquitto service status:"
    systemctl status mosquitto --no-pager -l | head -10
    
    # Setup unified secrets file
    echo ""
    info "Setting up unified secrets file..."
    
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
        success "Created ~/.gitignore"
    else
        # Add .secrets to .gitignore if not already present
        if ! grep -q "^\.secrets$" "$HOME/.gitignore" 2>/dev/null; then
            echo ".secrets" >> "$HOME/.gitignore"
            success "Added .secrets to ~/.gitignore"
        else
            info ".secrets already in ~/.gitignore"
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
        success "Created ~/.secrets file"
        warning "IMPORTANT: Edit ~/.secrets and add your MQTT_PASSWORD"
    else
        success "Secrets file exists at $SECRETS_FILE"
        # Verify permissions
        PERMS=$(stat -c "%a" "$SECRETS_FILE" 2>/dev/null || stat -f "%OLp" "$SECRETS_FILE" 2>/dev/null || echo "unknown")
        if [ "$PERMS" != "600" ]; then
            info "Fixing secrets file permissions..."
            chmod 600 "$SECRETS_FILE"
            success "Secrets file permissions set to 600"
        else
            info "Secrets file permissions are correct (600)"
        fi
    fi
    
    echo ""
    success "Mosquitto setup complete!"
    echo ""
    info "Next steps:"
    echo "  • Set up authentication:"
    echo "    $0 setup-universal"
    echo ""
    echo "  • Test connection with CLI tools:"
    echo "    mosquitto_pub -h localhost -t test -m 'Hello MQTT'"
    echo "    mosquitto_sub -h localhost -t test"
}

# Get password from secrets file or environment
get_password_from_secrets() {
    if [ -f "$SECRETS_FILE" ]; then
        if grep -q "^MQTT_PASSWORD=" "$SECRETS_FILE" 2>/dev/null; then
            grep "^MQTT_PASSWORD=" "$SECRETS_FILE" | cut -d'=' -f2- | tr -d '"' | tr -d "'"
            return 0
        elif [ -s "$SECRETS_FILE" ]; then
            head -n1 "$SECRETS_FILE" | tr -d '\n'
            return 0
        fi
    fi
    
    if [ -n "$MQTT_PASSWORD" ]; then
        echo "$MQTT_PASSWORD"
        return 0
    fi
    
    return 1
}

# Setup universal authentication (all functionality contained in orchestrator)
setup_universal_auth() {
    check_root
    
    info "Setting up universal MQTT authentication..."
    
    # Always prompt for password (allow blank - user can just hit Enter)
    read -sp "Enter MQTT password (press Enter for blank): " PASSWORD
    echo
    read -sp "Confirm password: " PASSWORD_CONFIRM
    echo
    
    if [ "$PASSWORD" != "$PASSWORD_CONFIRM" ]; then
        error "Passwords don't match"
        exit 1
    fi
    
    # PASSWORD can be empty string "" - that's valid (blank password allowed)
    
    USERNAME="${MQTT_USERNAME:-mqtt}"
    
    # Add user using internal function
    if [ -z "$PASSWORD" ]; then
        warning "Using blank password - authentication will be disabled for this user"
    fi
    
    if ! add_mqtt_user_internal "$USERNAME" "$PASSWORD" "provided"; then
        error "Failed to set up authentication"
        exit 1
    fi
    
    # Disable anonymous access
    if [ -f "$CONF_FILE" ]; then
        if grep -q "^allow_anonymous" "$CONF_FILE"; then
            sed -i 's/^allow_anonymous.*/allow_anonymous false/' "$CONF_FILE"
        else
            if grep -q "^listener" "$CONF_FILE"; then
                sed -i '/^listener/i allow_anonymous false' "$CONF_FILE"
            else
                echo "allow_anonymous false" >> "$CONF_FILE"
            fi
        fi
        success "Configuration updated to disable anonymous access"
    fi
    
    # Restart service
    if systemctl is-active --quiet mosquitto 2>/dev/null; then
        systemctl restart mosquitto > /dev/null 2>&1
        success "Mosquitto service restarted"
    fi
    
    echo ""
    success "Universal authentication setup complete!"
    info "Username: $USERNAME"
    if [ -n "$PASSWORD" ]; then
        info "Password: [stored securely]"
    else
        info "Password: [blank - authentication disabled for this user]"
    fi
}

# Add MQTT user (internal function - all functionality contained in orchestrator)
add_mqtt_user_internal() {
    local username="$1"
    local password="$2"  # Can be empty string for blank password
    local mosquitto_passwd_cmd="/usr/bin/mosquitto_passwd"
    local password_provided=false
    
    # Check if password was explicitly provided (even if blank)
    # We use a third parameter to indicate if password was provided
    if [ $# -ge 3 ] && [ "$3" = "provided" ]; then
        password_provided=true
    elif [ $# -ge 2 ]; then
        # Password parameter exists (even if empty string)
        password_provided=true
    fi
    
    # Check if mosquitto_passwd is available
    if [ ! -f "$mosquitto_passwd_cmd" ] && ! command -v mosquitto_passwd &> /dev/null; then
        error "mosquitto_passwd not found"
        error "Please install mosquitto-clients package first"
        return 1
    fi
    
    # Use system command if available
    if command -v mosquitto_passwd &> /dev/null; then
        mosquitto_passwd_cmd="mosquitto_passwd"
    fi
    
    # Check if password file exists, create if it doesn't
    if [ ! -f "$PASSWD_FILE" ]; then
        info "Creating password file at $PASSWD_FILE..."
        touch "$PASSWD_FILE"
        chown root:mosquitto "$PASSWD_FILE"
        chmod 640 "$PASSWD_FILE"
    fi
    
    # Check if user already exists
    local user_exists=false
    if grep -q "^${username}:" "$PASSWD_FILE" 2>/dev/null; then
        user_exists=true
    fi
    
    # Add or update user
    local result=0
    local error_output=""
    
    if [ "$password_provided" = true ]; then
        # Password provided (can be blank)
        if [ -z "$password" ]; then
            # Blank password - use interactive mode (without -b flag)
            # Pass two newlines (Enter twice) for blank password confirmation
            error_output=$(printf "\n\n" | "$mosquitto_passwd_cmd" "$PASSWD_FILE" "$username" 2>&1)
            result=$?
        else
            # Non-blank password - use -b flag (batch mode)
            error_output=$(echo "$password" | "$mosquitto_passwd_cmd" -b "$PASSWD_FILE" "$username" 2>&1)
            result=$?
        fi
    else
        # No password provided - use interactive mode
        error_output=$("$mosquitto_passwd_cmd" "$PASSWD_FILE" "$username" 2>&1)
        result=$?
    fi
    
    if [ $result -eq 0 ]; then
        if [ "$user_exists" = true ]; then
            success "Password updated for user '$username'"
        else
            success "User '$username' added to password file"
        fi
    else
        error "Failed to add/update user"
        if [ -n "$error_output" ]; then
            echo "$error_output" | grep -v "^$" | while IFS= read -r line; do
                error "  $line"
            done
        fi
        return 1
    fi
    
    # Ensure proper permissions
    chown root:mosquitto "$PASSWD_FILE"
    chmod 640 "$PASSWD_FILE"
    
    # Restart mosquitto service
    if systemctl is-active --quiet mosquitto 2>/dev/null; then
        systemctl restart mosquitto > /dev/null 2>&1
        success "Mosquitto service restarted"
    fi
    
    return 0
}

# Add a user
add_user() {
    check_root
    
    if [ -z "$1" ]; then
        error "Usage: $0 user add <username> [password]"
        exit 1
    fi
    
    USERNAME="$1"
    PASSWORD="$2"
    
    # If password not provided, prompt for it
    if [ -z "$PASSWORD" ]; then
        read -sp "Enter password (press Enter for blank): " PASSWORD
        echo
        read -sp "Confirm password: " PASSWORD_CONFIRM
        echo
        
        if [ "$PASSWORD" != "$PASSWORD_CONFIRM" ]; then
            error "Passwords don't match"
            exit 1
        fi
    fi
    
    # Check if user already exists
    if grep -q "^${USERNAME}:" "$PASSWD_FILE" 2>/dev/null; then
        warning "User '$USERNAME' already exists in password file"
        read -p "Update password? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            info "Cancelled"
            exit 0
        fi
    fi
    
    # Add user using internal function (pass "provided" flag to indicate password was explicitly provided, even if blank)
    add_mqtt_user_internal "$USERNAME" "$PASSWORD" "provided"
    
    # List current users
    echo ""
    info "Current users in password file:"
    if [ -f "$PASSWD_FILE" ] && [ -s "$PASSWD_FILE" ]; then
        cut -d: -f1 "$PASSWD_FILE" | while read -r user; do
            echo "  • $user"
        done
    fi
    
    echo ""
    success "Done! User '$USERNAME' is now configured for MQTT authentication"
}

# List all users
list_users() {
    if [ ! -f "$PASSWD_FILE" ]; then
        warning "Password file not found. No users configured."
        return
    fi
    
    if [ ! -s "$PASSWD_FILE" ]; then
        warning "Password file is empty. No users configured."
        return
    fi
    
    info "MQTT Users:"
    cut -d: -f1 "$PASSWD_FILE" | while read -r user; do
        echo "  • $user"
    done
}

# Remove a user
remove_user() {
    check_root
    
    if [ -z "$1" ]; then
        error "Usage: $0 user remove <username>"
        exit 1
    fi
    
    USERNAME="$1"
    
    if [ ! -f "$PASSWD_FILE" ]; then
        error "Password file not found"
        exit 1
    fi
    
    if ! grep -q "^${USERNAME}:" "$PASSWD_FILE" 2>/dev/null; then
        error "User '$USERNAME' not found"
        exit 1
    fi
    
    # Remove user from password file
    sed -i "/^${USERNAME}:/d" "$PASSWD_FILE"
    
    # Restart service
    systemctl restart mosquitto > /dev/null 2>&1
    
    success "User '$USERNAME' removed"
}

# Show status
show_status() {
    info "MQTT Broker Status:"
    echo ""
    
    # Service status
    if systemctl is-active --quiet mosquitto; then
        success "Service: Running"
    else
        error "Service: Not running"
    fi
    
    # Port check
    if netstat -tlnp 2>/dev/null | grep -q ":1883 " || ss -tlnp 2>/dev/null | grep -q ":1883 "; then
        success "Port 1883: Listening"
    else
        warning "Port 1883: Not listening"
    fi
    
    # Anonymous access
    if [ -f "$CONF_FILE" ]; then
        if grep -q "^allow_anonymous.*true" "$CONF_FILE"; then
            warning "Anonymous access: ENABLED (insecure!)"
        else
            success "Anonymous access: DISABLED"
        fi
    fi
    
    # Users
    echo ""
    list_users
    
    # Broker info
    echo ""
    if command -v mosquitto &> /dev/null; then
        info "Broker version:"
        mosquitto -v 2>&1 | head -1 | sed 's/^/  /'
    fi
}

# Test connection
test_connection() {
    local host="${1:-localhost}"
    local port="${2:-1883}"
    local username="${3:-}"
    local password="${4:-}"
    
    info "Testing MQTT connection to $host:$port..."
    
    # Try to get credentials
    if [ -z "$username" ]; then
        if [ -f "$PASSWD_FILE" ] && [ -s "$PASSWD_FILE" ]; then
            username=$(cut -d: -f1 "$PASSWD_FILE" | head -1)
            password=$(get_password_from_secrets || echo "")
        fi
    fi
    
    if [ -z "$username" ]; then
        error "No username provided and no users found"
        return 1
    fi
    
    # Test publish
    if [ -n "$password" ]; then
        if mosquitto_pub -h "$host" -p "$port" -u "$username" -P "$password" -t "test/connection" -m "test" -W 2 > /dev/null 2>&1; then
            success "Connection successful!"
            info "  Host: $host:$port"
            info "  Username: $username"
            return 0
        else
            error "Connection failed"
            return 1
        fi
    else
        if mosquitto_pub -h "$host" -p "$port" -t "test/connection" -m "test" -W 2 > /dev/null 2>&1; then
            success "Connection successful (anonymous)"
            return 0
        else
            error "Connection failed"
            return 1
        fi
    fi
}

# Monitor topics
monitor() {
    local topic="${1:-#}"
    local host="${2:-localhost}"
    local username="${3:-}"
    local password="${4:-}"
    
    # Check if anonymous access is enabled
    local anonymous_enabled=false
    if [ -f "$CONF_FILE" ]; then
        if grep -q "^allow_anonymous.*true" "$CONF_FILE"; then
            anonymous_enabled=true
        fi
    fi
    
    # Get credentials if not provided and anonymous access is disabled
    if [ -z "$username" ] && [ "$anonymous_enabled" = "false" ]; then
        if [ -f "$PASSWD_FILE" ] && [ -s "$PASSWD_FILE" ]; then
            username=$(cut -d: -f1 "$PASSWD_FILE" | head -1)
            password=$(get_password_from_secrets || echo "")
            
            if [ -z "$password" ]; then
                error "Authentication required but password not found"
                info "Please provide credentials:"
                echo "  Option 1: Create secrets file:"
                echo "    echo 'MQTT_PASSWORD=your_password' >> ~/.secrets"
                echo "    chmod 600 ~/.secrets"
                echo ""
                echo "  Option 2: Use monitor with credentials:"
                echo "    mqtt monitor \"$topic\" $host <username> <password>"
                exit 1
            fi
        fi
    fi
    
    info "Monitoring topic: $topic"
    info "Press Ctrl+C to stop"
    echo ""
    
    if [ -n "$username" ] && [ -n "$password" ]; then
        mosquitto_sub -h "$host" -u "$username" -P "$password" -t "$topic" -v
    elif [ -n "$username" ]; then
        mosquitto_sub -h "$host" -u "$username" -t "$topic" -v
    else
        mosquitto_sub -h "$host" -t "$topic" -v
    fi
}

# List all broker items (clients, topics, retained messages)
list_broker() {
    local host="${1:-localhost}"
    local username="${2:-}"
    local password="${3:-}"
    
    # Get credentials if not provided
    if [ -z "$username" ] && [ -f "$PASSWD_FILE" ] && [ -s "$PASSWD_FILE" ]; then
        username=$(cut -d: -f1 "$PASSWD_FILE" | head -1)
        password=$(get_password_from_secrets || echo "")
    fi
    
    info "MQTT Broker Contents"
    echo ""
    
    # Connected Clients
    echo -e "${BLUE}📡 Connected Clients (recent):${NC}"
    if systemctl is-active --quiet mosquitto; then
        # Extract client info from recent logs - look for "as clientname"
        local clients=$(journalctl -u mosquitto --since "10 minutes ago" --no-pager 2>&1 | \
            grep -E "New client connected" | \
            sed -E 's/.*as ([^ ]+).*/\1/' | \
            sort -u)
        
        if [ -n "$clients" ]; then
            echo "$clients" | while read -r client; do
                # Check if client is still active (has recent activity)
                local last_activity=$(journalctl -u mosquitto --since "2 minutes ago" --no-pager 2>&1 | \
                    grep -c "$client" || echo "0")
                if [ "$last_activity" -gt 0 ]; then
                    echo "  • $client (active)"
                else
                    echo "  • $client (recent)"
                fi
            done
        else
            echo "  (No clients found in recent logs)"
        fi
    else
        warning "Mosquitto service is not running"
    fi
    
    echo ""
    
    # Recent Topics
    echo -e "${BLUE}📋 Recent Topics (last 10 minutes):${NC}"
    if systemctl is-active --quiet mosquitto; then
        local topics=$(journalctl -u mosquitto --since "10 minutes ago" --no-pager 2>&1 | \
            grep -E "Received PUBLISH" | \
            sed -E "s/.*'([^']+)'.*/\1/" | \
            sort -u)
        
        if [ -n "$topics" ]; then
            echo "$topics" | while read -r topic; do
                # Count messages for this topic
                local count=$(journalctl -u mosquitto --since "10 minutes ago" --no-pager 2>&1 | \
                    grep -c "Received PUBLISH.*'${topic}'" || echo "0")
                echo "  • $topic (${count} messages)"
            done
        else
            echo "  (No topics found in recent logs)"
        fi
    else
        warning "Mosquitto service is not running"
    fi
    
    echo ""
    
    # Retained Messages
    echo -e "${BLUE}💾 Retained Messages:${NC}"
    local retained_output=""
    local retained_error=""
    
    if [ -n "$username" ] && [ -n "$password" ]; then
        # Try to get retained messages by subscribing briefly
        retained_output=$(timeout 3 mosquitto_sub -h "$host" -u "$username" -P "$password" -t "#" --retained-only -C 100 -W 2 2>&1)
        retained_error=$(echo "$retained_output" | grep -i "error\|refused\|unauthorized" || echo "")
    elif [ -n "$username" ]; then
        retained_output=$(timeout 3 mosquitto_sub -h "$host" -u "$username" -t "#" --retained-only -C 100 -W 2 2>&1)
        retained_error=$(echo "$retained_output" | grep -i "error\|refused\|unauthorized" || echo "")
    else
        retained_output=$(timeout 3 mosquitto_sub -h "$host" -t "#" --retained-only -C 100 -W 2 2>&1)
        retained_error=$(echo "$retained_output" | grep -i "error\|refused\|unauthorized" || echo "")
    fi
    
    if [ -n "$retained_error" ]; then
        echo "  (Authentication required or no retained messages)"
    elif [ -n "$retained_output" ]; then
        # Filter out connection messages and show only topic:payload
        echo "$retained_output" | grep -v -E "Client|CONNECT|SUBSCRIBE|SUBACK|DISCONNECT" | \
            grep -v "^$" | head -20 | while IFS= read -r line; do
            if [ -n "$line" ]; then
                echo "  • $line"
            fi
        done
        local retained_count=$(echo "$retained_output" | grep -v -E "Client|CONNECT|SUBSCRIBE|SUBACK|DISCONNECT" | grep -v "^$" | wc -l)
        if [ "$retained_count" -gt 20 ]; then
            echo "  ... and $((retained_count - 20)) more"
        fi
    else
        echo "  (No retained messages found)"
    fi
    
    echo ""
    
    # Recent Activity Summary
    echo -e "${BLUE}📊 Recent Activity Summary:${NC}"
    if systemctl is-active --quiet mosquitto; then
        local recent_pub=$(journalctl -u mosquitto --since "10 minutes ago" --no-pager 2>&1 | grep -c "Received PUBLISH" || echo "0")
        local recent_conn=$(journalctl -u mosquitto --since "10 minutes ago" --no-pager 2>&1 | grep -c "New client connected" || echo "0")
        local recent_sub=$(journalctl -u mosquitto --since "10 minutes ago" --no-pager 2>&1 | grep -c "Received SUBSCRIBE" || echo "0")
        
        echo "  • Published messages: $recent_pub"
        echo "  • New connections: $recent_conn"
        echo "  • Subscriptions: $recent_sub"
    else
        warning "Mosquitto service is not running"
    fi
}

# Show usage
show_usage() {
    cat << EOF
MQTT Helper - Unified MQTT broker management tool

Usage: $0 <command> [options]

Commands:
  setup                    Set up mosquitto broker (install and configure)
  setup-universal          Set up universal authentication (one password for all)
  user add <name> [pass]   Add a new user (password optional, will prompt)
  user list                List all users
  user remove <name>       Remove a user
  status                   Show broker status
  list [host]              List all broker items (clients, topics, retained messages)
  test [host] [port]       Test connection to broker
  monitor [topic] [host]   Monitor MQTT topics (default: #, localhost)
  help                     Show this help message

Examples:
  $0 setup                    # Set up mosquitto broker
  $0 setup-universal          # Set up authentication
  $0 user add esp32 mypassword
  $0 user list
  $0 status
  $0 list                    # Show all broker contents
  $0 list 192.168.1.50       # Show contents from remote broker
  $0 test localhost 1883
  $0 monitor "sensors/#" 192.168.1.50

Environment Variables:
  MQTT_USERNAME            Default username (default: mqtt)
  MQTT_PASSWORD            Password (or use ~/.secrets file)

Secrets File:
  Store password in ~/.secrets:
    echo 'MQTT_PASSWORD=your_password' >> ~/.secrets
    chmod 600 ~/.secrets
EOF
}

# Main command dispatcher
main() {
    case "${1:-help}" in
        setup)
            setup_mosquitto
            ;;
        setup-universal)
            setup_universal_auth
            ;;
        user)
            case "${2:-}" in
                add)
                    add_user "$3" "$4"
                    ;;
                list)
                    list_users
                    ;;
                remove)
                    remove_user "$3"
                    ;;
                *)
                    error "Unknown user command: ${2:-}"
                    echo "  Use: add, list, or remove"
                    exit 1
                    ;;
            esac
            ;;
        status)
            show_status
            ;;
        list)
            list_broker "${2:-localhost}" "$3" "$4"
            ;;
        test)
            test_connection "$2" "$3" "$4" "$5"
            ;;
        monitor)
            monitor "${2:-#}" "${3:-localhost}" "$4" "$5"
            ;;
        help|--help|-h)
            show_usage
            ;;
        *)
            error "Unknown command: $1"
            echo ""
            show_usage
            exit 1
            ;;
    esac
}

main "$@"


