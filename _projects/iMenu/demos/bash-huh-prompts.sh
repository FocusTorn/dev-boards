#!/usr/bin/env bash
# Demo script showing how to use prompt-huh from bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
IMENU_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
PROMPT_BIN="$IMENU_DIR/dist/bin/prompt-huh"

# Check if prompt-huh is built
if [ ! -f "$PROMPT_BIN" ]; then
    echo "❌ prompt-huh not found. Building it now..."
    cd "$IMENU_DIR"
    
    # Check if Go is installed, offer to install if not
    if ! command -v go &> /dev/null; then
        echo "❌ Go is not installed."
        echo ""
        read -p "Install Go automatically? (Y/n): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Nn]$ ]]; then
            if [ -f "$SCRIPT_DIR/bootstrap-go.sh" ]; then
                bash "$SCRIPT_DIR/bootstrap-go.sh"
            else
                echo "📥 Please install Go manually:"
                echo "   - Debian/Ubuntu: sudo apt-get install golang-go"
                echo "   - Or download: https://go.dev/dl/"
                exit 1
            fi
        else
            echo "❌ Cannot build without Go. Exiting."
            exit 1
        fi
    fi
    
    echo "🔨 Building prompt-huh..."
    mkdir -p "$IMENU_DIR/dist/bin"
    go mod tidy
    go build -o "$PROMPT_BIN" "$IMENU_DIR/cmd/prompt-huh"
    echo "✅ Built prompt-huh successfully!"
    echo ""
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Interactive Prompt Demo using huh?"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Example 1: Text input
echo "📝 Example 1: Text Input"
RESULT_FILE=$(mktemp)
"$PROMPT_BIN" input "What is your name?" --result-file "$RESULT_FILE" < /dev/tty
NAME=$(cat "$RESULT_FILE")
rm -f "$RESULT_FILE"
echo "✅ You entered: $NAME"
echo ""

# Example 2: Text input with default
echo "📝 Example 2: Text Input with Default"
RESULT_FILE=$(mktemp)
"$PROMPT_BIN" input "Repository name:" "my-repo" --result-file "$RESULT_FILE" < /dev/tty
REPO=$(cat "$RESULT_FILE")
rm -f "$RESULT_FILE"
echo "✅ Repository: $REPO"
echo ""

# Example 3: Select from options
echo "📝 Example 3: Select from Options"
RESULT_FILE=$(mktemp)
"$PROMPT_BIN" select "Choose a color:" "Red" "Blue" "Green" "Yellow" "Purple" --result-file "$RESULT_FILE" < /dev/tty
COLOR=$(cat "$RESULT_FILE")
rm -f "$RESULT_FILE"
echo "✅ You chose: $COLOR"
echo ""

# Example 4: Confirmation
echo "📝 Example 4: Confirmation"
RESULT_FILE=$(mktemp)
"$PROMPT_BIN" confirm "Do you want to continue with the demo?" --result-file "$RESULT_FILE" < /dev/tty
RESULT=$(cat "$RESULT_FILE")
rm -f "$RESULT_FILE"
if [ "$RESULT" = "yes" ]; then
    echo "✅ User confirmed - continuing..."
else
    echo "❌ User cancelled - stopping demo"
    exit 0
fi
echo ""

# Example 5: Multi-select
echo "📝 Example 5: Multi-Select"
echo "Select multiple colors (space to toggle, enter when done):"
RESULT_FILE=$(mktemp)
"$PROMPT_BIN" multiselect "Choose your favorite colors:" "Red" "Blue" "Green" "Yellow" "Purple" --result-file "$RESULT_FILE" < /dev/tty
SELECTED=$(cat "$RESULT_FILE")
rm -f "$RESULT_FILE"
echo "✅ You selected:"
if [ -n "$SELECTED" ]; then
    echo "$SELECTED" | while read -r item; do
        [ -n "$item" ] && echo "  • $item"
    done
else
    echo "  (none)"
fi
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Demo Complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
