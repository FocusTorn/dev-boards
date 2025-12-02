#!/usr/bin/env bash
# Demo script showing all 4 methods of providing wizard input

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
IMENU_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
source "$IMENU_DIR/wizard.sh"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  iMenu Wizard - 4 Methods Demo"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# ┌────────────────────────────────────────────────────────────────────────────┐
# │         Method 1: Inline JSON string directly in the function call         │
# └────────────────────────────────────────────────────────────────────────────┘
echo "Method 1: Inline JSON string" >&2
results1=$(iwizard_run_inline '[
    {
        "type": "confirm",
        "title": "Proceed with setup?",
        "key": "proceed",
        "description": "Continue with the demo"
    },
    {
        "type": "input",
        "title": "Enter your name:",
        "key": "name",
        "placeholder": "User"
    }
]')
exit_code1=$?

if [ $exit_code1 -eq 0 ]; then
    echo "Results (Method 1):" >&2
    echo "$results1" | python3 -m json.tool 2>/dev/null || echo "$results1" >&2
else
    echo "Wizard cancelled (Method 1)" >&2
fi
echo ""

# ┌────────────────────────────────────────────────────────────────────────────┐
# │         Method 2: JSON string in a variable, then pass to function         │
# └────────────────────────────────────────────────────────────────────────────┘

echo "Method 2: JSON in variable" >&2
wizard_config='
[
    {
        "type": "select",
        "title": "Select service type:",
        "key": "service",
        "options": [
            "Web Server",
            "Database",
            "Cache"
        ]
    },
    {
        "type": "multiselect",
        "title": "Select features:",
        "key": "features",
        "options": [
            "SSL/TLS",
            "Monitoring",
            "Backup"
        ]
    }
]
'

results2=$(iwizard_run_inline "$wizard_config")
exit_code2=$?

if [ $exit_code2 -eq 0 ]; then
    echo "Results (Method 2):" >&2
    echo "$results2" | python3 -m json.tool 2>/dev/null || echo "$results2" >&2
else
    echo "Wizard cancelled (Method 2)" >&2
fi
echo ""

# ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
# │            Method 3: Using iwizard_run_json directly (auto-detects file vs string)             │
# └────────────────────────────────────────────────────────────────────────────────────────────────┘

echo "Method 3: Direct call with JSON string (auto-detect)" >&2
results3=$(iwizard_run_json '[
    {
        "type": "confirm",
        "title": "Complete setup?",
        "key": "complete",
        "description": "Finish the demo"
    }
]')
exit_code3=$?

if [ $exit_code3 -eq 0 ]; then
    echo "Results (Method 3):" >&2
    echo "$results3" | python3 -m json.tool 2>/dev/null || echo "$results3" >&2
else
    echo "Wizard cancelled (Method 3)" >&2
fi
echo ""

# ┌────────────────────────────────────────────────────────────────────────────┐
# │         Method 4: JSON file path (with comments support)                    │
# └────────────────────────────────────────────────────────────────────────────┘

echo "Method 4: JSON file with comments" >&2

# Use the example JSON file (or create a temporary one for demo)
WIZARD_FILE="$SCRIPT_DIR/wizard-example.json"
if [ ! -f "$WIZARD_FILE" ]; then
    # Create temporary file if example doesn't exist
    WIZARD_FILE="$SCRIPT_DIR/wizard_input.json"
    cat > "$WIZARD_FILE" << 'EOF'
[
    {
        "type": "confirm",
        "title": "Proceed?",
        "key": "proceed",
        "description": "This is from a file"
    },
    // This is a single-line comment
    {
        "type": "multiselect",
        "title": "Which services?",
        "key": "services",
        "options": [
            "Sensor readings",
            "IAQ (Air quality calculation)",
            "Heat soak detection"
        ]
    },
    {
        "type": "confirm",
        "title": "Final confirmation?",
        "key": "final",
        "description": "Last step"
    }
    
    /* This is a
       multi-line comment */
]
EOF
fi

results4=$(iwizard_run_json "$WIZARD_FILE")
exit_code4=$?

if [ $exit_code4 -eq 0 ]; then
    echo "Results (Method 4):" >&2
    echo "$results4" | python3 -m json.tool 2>/dev/null || echo "$results4" >&2
else
    echo "Wizard cancelled (Method 4)" >&2
fi

# Clean up (only if it was a temporary file)
if [ "$WIZARD_FILE" = "$SCRIPT_DIR/wizard_input.json" ]; then
    rm -f "$WIZARD_FILE"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Demo Complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

