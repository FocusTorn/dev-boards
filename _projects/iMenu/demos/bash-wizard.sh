#!/usr/bin/env bash
# Demo script showing the wizard mode with back navigation

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
IMENU_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
WIZARD_BIN="$IMENU_DIR/dist/bin/prompt-wizard"

# Check if wizard is built
if [ ! -f "$WIZARD_BIN" ]; then
    echo "❌ prompt-wizard not found. Building it now..."
    cd "$IMENU_DIR"
    if ! command -v go &> /dev/null; then
        echo "❌ Go is not installed."
        exit 1
    fi
    mkdir -p "$IMENU_DIR/dist/bin"
    go mod tidy
    go build -o "$WIZARD_BIN" "$IMENU_DIR/cmd/prompt-wizard"
    echo "✅ Built prompt-wizard successfully!"
    echo ""
fi

# Define wizard steps as JSON
STEPS='[
  {
    "type": "input",
    "title": "What is your name?",
    "description": "Enter your full name",
    "key": "name",
    "placeholder": "John Doe"
  },
  {
    "type": "select",
    "title": "Choose your favorite color",
    "description": "Select one option",
    "key": "color",
    "options": ["Red", "Blue", "Green", "Yellow", "Purple"]
  },
  {
    "type": "multiselect",
    "title": "Select your hobbies",
    "description": "You can select multiple options",
    "key": "hobbies",
    "options": ["Reading", "Gaming", "Sports", "Music", "Travel"]
  },
  {
    "type": "confirm",
    "title": "Do you want to continue?",
    "description": "Final confirmation",
    "key": "continue"
  }
]'

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Wizard Demo with Back Navigation"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Press 'B' at any step to go back to the previous step"
echo ""

RESULT_FILE=$(mktemp)
"$WIZARD_BIN" "$STEPS" --result-file "$RESULT_FILE" < /dev/tty

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Results:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
cat "$RESULT_FILE" | python3 -m json.tool 2>/dev/null || cat "$RESULT_FILE"
rm -f "$RESULT_FILE"
echo ""

