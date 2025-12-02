#!/usr/bin/env bash
# Wizard Helper Functions (Bash)
# Provides easy-to-use wrapper functions for the prompt-wizard

# Get the directory where this script is located
IWIZARD_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
IWIZARD_DIST="$IWIZARD_DIR/dist"
IWIZARD_BIN="$IWIZARD_DIST/bin/prompt-wizard"

# Ensure wizard is built
_ensure_wizard_built() {
    if [ ! -f "$IWIZARD_BIN" ]; then
        if ! command -v go &> /dev/null; then
            echo "❌ Go is not installed. Cannot build wizard." >&2
            return 1
        fi
        echo "🔨 Building prompt-wizard..." >&2
        cd "$IWIZARD_DIR"
        mkdir -p "$IWIZARD_DIST/bin"
        go mod tidy 2>/dev/null || true
        go build -o "$IWIZARD_BIN" ./cmd/prompt-wizard 2>/dev/null
        if [ ! -f "$IWIZARD_BIN" ]; then
            echo "❌ Failed to build prompt-wizard" >&2
            return 1
        fi
    fi
}

# Run wizard with JSON input (auto-detects file vs string)
# Usage: iwizard_run_json '<json-string>' or iwizard_run_json '/path/to/file.json'
iwizard_run_json() {
    local json_input="$1"
    local result_file="${2:-$(mktemp)}"
    
    if [ -z "$json_input" ]; then
        echo "❌ Error: No JSON input provided" >&2
        echo "Usage: iwizard_run_json '<json-string>' [result-file]" >&2
        echo "   Or: iwizard_run_json '/path/to/file.json' [result-file]" >&2
        return 1
    fi
    
    _ensure_wizard_built || return 1
    
    # Run wizard - it auto-detects file vs string
    # Need to ensure wizard has access to terminal for display
    if [ -t 1 ] && [ -c /dev/tty ] 2>/dev/null; then
        # We have a terminal - use /dev/tty for input and ensure output goes to terminal
        "$IWIZARD_BIN" "$json_input" --result-file "$result_file" < /dev/tty > /dev/tty 2>&1
    elif [ -t 0 ] && [ -c /dev/tty ] 2>/dev/null; then
        # Try with /dev/tty for input
        "$IWIZARD_BIN" "$json_input" --result-file "$result_file" < /dev/tty
    else
        # Fallback to stdin if /dev/tty not available
        "$IWIZARD_BIN" "$json_input" --result-file "$result_file"
    fi
    
    local exit_code=$?
    
    if [ $exit_code -eq 0 ]; then
        # Output results
        cat "$result_file"
        # Clean up temp file if we created it
        if [ -z "$2" ]; then
            rm -f "$result_file"
        fi
        return 0
    else
        # Clean up temp file if we created it
        if [ -z "$2" ]; then
            rm -f "$result_file"
        fi
        return $exit_code
    fi
}

# Run wizard with inline JSON string
# Usage: iwizard_run_inline '<json-string>' [result-file]
iwizard_run_inline() {
    local json_string="$1"
    local result_file="${2:-$(mktemp)}"
    
    if [ -z "$json_string" ]; then
        echo "❌ Error: No JSON string provided" >&2
        echo "Usage: iwizard_run_inline '<json-string>' [result-file]" >&2
        return 1
    fi
    
    _ensure_wizard_built || return 1
    
    # Run wizard with JSON string
    # Pass JSON as argument (not via stdin) so stdin is free for wizard input
    # The wizard uses bubbletea which handles terminal I/O itself
    # Only use stdin piping if we're in a non-interactive environment
    if [ -t 1 ] && [ -t 0 ]; then
        # Interactive terminal - pass JSON as argument, let wizard use stdin/stdout
        "$IWIZARD_BIN" "$json_string" --result-file "$result_file"
    else
        # Non-interactive - try piping (will likely fail but preserves behavior)
        echo "$json_string" | "$IWIZARD_BIN" --result-file "$result_file"
    fi
    
    local exit_code=$?
    
    if [ $exit_code -eq 0 ]; then
        # Output results
        cat "$result_file"
        # Clean up temp file if we created it
        if [ -z "$2" ]; then
            rm -f "$result_file"
        fi
        return 0
    else
        # Clean up temp file if we created it
        if [ -z "$2" ]; then
            rm -f "$result_file"
        fi
        return $exit_code
    fi
}

# Export functions for use in other scripts
export -f iwizard_run_json
export -f iwizard_run_inline
export -f _ensure_wizard_built

