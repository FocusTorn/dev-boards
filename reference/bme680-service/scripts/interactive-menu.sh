#!/bin/bash
# Interactive Menu Functions
# Multi-select menu similar to npm-check-updates (ncu)
# Space to toggle, 'a' to select all, Enter to confirm

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

# Hide cursor (output to stderr so visible even when stdout captured)
hide_cursor() {
    tput civis >&2 2>/dev/null || printf "\033[?25l" >&2 || true
}

# Show cursor (output to stderr so visible even when stdout captured)
show_cursor() {
    tput cnorm >&2 2>/dev/null || printf "\033[?25h" >&2 || true
}

# Clear menu area - move cursor up and clear lines
clear_menu() {
    local lines=$1
    # Move cursor up N lines (to stderr so visible even when stdout is captured)
    for ((i=0; i<lines; i++)); do
        printf "\033[A" >&2 2>/dev/null || tput cuu1 >&2 2>/dev/null || true
        # Clear the entire line
        printf "\r" >&2; tput el >&2 2>/dev/null || printf "\033[K" >&2 2>/dev/null || true
    done
    # Ensure cursor is at column 0
    printf "\r" >&2
}

# Interactive multi-select menu
# Usage: source this file, then call interactive_menu "Option 1" "Option 2" "Option 3"
# Usage with help: interactive_menu "opt1" "opt2" --help-text "$help_content"
# Returns: space-separated list of selected indices (0-based) via stdout
interactive_menu() {
    local options=("$@")
    local help_text=""
    local preselect_indices=""
    local i

    # Parse optional flags: --help-text TEXT, --preselect "idx idx ..."
    for ((i=0; i<${#options[@]}; i++)); do
        if [ "${options[i]}" = "--help-text" ]; then
            help_text="${options[i+1]}"
            unset 'options[i]' 'options[i+1]'
        fi
        if [ "${options[i]}" = "--preselect" ]; then
            preselect_indices="${options[i+1]}"
            unset 'options[i]' 'options[i+1]'
        fi
    done
    # Re-pack array
    local tmp=()
    for v in "${options[@]}"; do [ -n "$v" ] && tmp+=("$v"); done
    options=("${tmp[@]}")

    local selected=()
    local current=0
    local num_options=${#options[@]}
    local show_help=false
    
    # Initialize - nothing selected
    for ((i=0; i<num_options; i++)); do
        selected[i]=false
    done
    # Apply preselection
    for idx in $preselect_indices; do
        if [ "$idx" -ge 0 ] && [ "$idx" -lt "$num_options" ]; then
            selected[$idx]=true
        fi
    done
    
    hide_cursor
    
    # Count lines we'll display (for clearing later) - will be updated based on help state
    local menu_lines=$((num_options + 3))  # keybindings + blank + options + trailing blank
    local help_lines=0
    if [ -n "$help_text" ]; then
        # Count lines in help text (it already includes all content)
        help_lines=$(echo "$help_text" | wc -l)
    fi
    
    while true; do
        # Display menu or help (to stderr so it's visible even when stdout is captured)
        if [ "$show_help" = true ] && [ -n "$help_text" ]; then
            # Clear previous menu before showing help
            if [ $menu_lines -gt 0 ]; then
                clear_menu $menu_lines
            fi
            # Draw help panel two lines lower
            printf "\n\n" >&2
            # Show help panel (interpret color escapes)
            printf '%b\n' "${help_text}" >&2
            # Update menu_lines for clearing - help_lines already includes all content plus two leading blanks
            menu_lines=$((help_lines + 2))
        else
            # Show normal menu in requested order:
            # (blank line) + options with markers + (blank line)
            printf "\n" >&2
            for ((i=0; i<num_options; i++)); do
                local marker="○"
                if [ "${selected[i]}" = true ]; then
                    marker="●"
                fi
                
                local prefix="  "
                if [ $i -eq $current ]; then
                    prefix="${CYAN}❯${NC} "
                fi
                
                # Print option line, interpreting escapes in prefix (no carriage return between)
                printf '%b%s %s\n' "$prefix" "$marker" "${options[i]}" >&2
            done
            printf "\n" >&2
            # We printed: 1 (blank) + num_options + 1 (blank)
            menu_lines=$((num_options + 2))
        fi
        
        # Read input - single char, silent (prevents echo like ^[[A/^[[B)
        local key
        IFS= read -rsn1 key 2>/dev/null
        
        # If showing help, any key returns to menu (except ESC which cancels)
        if [ "$show_help" = true ]; then
            if [ "$key" = $'\x1b' ]; then
                # ESC cancels from help
                show_cursor
                return 1
            else
                # Any other key returns to menu - clear help panel and redraw menu
                show_help=false
                # Clear help panel before redrawing menu
                clear_menu $menu_lines
                continue
            fi
        fi
        
        # Check for Enter key - newline, carriage return, or empty (fallback)
        if [ "$key" = $'\n' ] || [ "$key" = $'\r' ] || [ -z "$key" ]; then
            break
        fi
        
        case "$key" in
            "?")  # Help key
                if [ -n "$help_text" ]; then
                    show_help=true
                    clear_menu $menu_lines
                    continue
                fi
                ;;
            " ")  # Space bar
                selected[$current]=$([ "${selected[$current]}" = true ] && echo false || echo true)
                ;;
            "a"|"A")  # Toggle all
                local all_selected=true
                for ((i=0; i<num_options; i++)); do
                    if [ "${selected[i]}" != true ]; then
                        all_selected=false
                        break
                    fi
                done
                if [ "$all_selected" = true ]; then
                    for ((i=0; i<num_options; i++)); do selected[i]=false; done
                else
                    for ((i=0; i<num_options; i++)); do selected[i]=true; done
                fi
                ;;
            "b"|"B")  # Back
                show_cursor
                return 2
                ;;
            $'\x1b')  # Escape sequence (ESC key or arrow keys)
                # Read next character to determine if it's ESC or arrow key
                local esc_char
                if IFS= read -rsn1 -t 0.05 esc_char 2>/dev/null; then
                    if [[ "$esc_char" == "[" ]]; then
                        # Arrow key sequence: read final byte
                        local arrow
                        IFS= read -rsn1 -t 0.05 arrow 2>/dev/null || arrow=""
                        case "$arrow" in
                            "A")  # Up arrow
                                current=$(( (current - 1 + num_options) % num_options ))
                                ;;
                            "B")  # Down arrow
                                current=$(( (current + 1) % num_options ))
                                ;;
                        esac
                    else
                        # Bare ESC pressed (cancel)
                        show_cursor
                        return 1
                    fi
                else
                    # Bare ESC (no follow-up)
                    show_cursor
                    return 1
                fi
                ;;
            "q"|"Q")  # Quit/Cancel
                show_cursor
                return 1
                ;;
        esac
        
        # Clear menu for redraw
        clear_menu $menu_lines
    done
    
    show_cursor
    
    # Output selected indices (space-separated) to stdout only
    # Display output goes to stderr so it's visible on terminal
    local result=""
    for ((i=0; i<num_options; i++)); do
        if [ "${selected[i]}" = true ]; then
            result="$result $i"
        fi
    done
    
    # Clear the menu area one last time before outputting result
    clear_menu $menu_lines >&2
    
    # Trim leading space and output result to stdout (for capture)
    result="${result# }"
    if [ -n "$result" ]; then
        echo "$result"
        return 0
    else
        return 1
    fi
}

# Interactive single-select menu with Back support
# Returns: selected index via stdout
# Exit codes: 0=confirm, 1=cancel, 2=back
single_select_menu() {
    local options=("$@")
    # Optional: --preselect N
    local preselect=-1
    local i
    for ((i=0; i<${#options[@]}; i++)); do
        if [ "${options[i]}" = "--preselect" ]; then
            preselect=${options[i+1]}
            unset 'options[i]' 'options[i+1]'
        fi
    done
    local tmp=()
    for v in "${options[@]}"; do [ -n "$v" ] && tmp+=("$v"); done
    options=("${tmp[@]}")

    local current=0
    local num_options=${#options[@]}
    local selected_idx=0
    if [ $preselect -ge 0 ] && [ $preselect -lt $num_options ]; then
        current=$preselect
        selected_idx=$preselect
    fi

    hide_cursor
    local menu_lines=$((num_options + 1))
    while true; do
        printf "\n" >&2
        for ((i=0; i<num_options; i++)); do
            local marker="○"
            if [ $i -eq $selected_idx ]; then marker="●"; fi
            local prefix="  "
            if [ $i -eq $current ]; then prefix="${CYAN}❯${NC} "; fi
            printf '%b%s %s\n' "$prefix" "$marker" "${options[i]}" >&2
        done

        local key
        IFS= read -rsn1 key 2>/dev/null
        if [ "$key" = $'\n' ] || [ "$key" = $'\r' ] || [ -z "$key" ]; then
            break
        fi
        case "$key" in
            $'\x1b')
                local esc_char
                if IFS= read -rsn1 -t 0.05 esc_char 2>/dev/null; then
                    if [[ "$esc_char" == "[" ]]; then
                        local arrow
                        IFS= read -rsn1 -t 0.05 arrow 2>/dev/null || arrow=""
                        case "$arrow" in
                            "A") current=$(((current - 1 + num_options) % num_options)) ;;
                            "B") current=$(((current + 1) % num_options)) ;;
                        esac
                    else
                        show_cursor; return 1
                    fi
                else
                    show_cursor; return 1
                fi
                ;;
            " ") selected_idx=$current ;;
            "b"|"B") show_cursor; return 2 ;;
            "q"|"Q") show_cursor; return 1 ;;
        esac
        clear_menu $menu_lines
    done
    show_cursor
    echo "$selected_idx"
    return 0
}

# Yes/No prompt with default (Y/N), Back supported via 'b'
# Exit codes: 0=yes, 1=no/cancel, 2=back
yes_no_prompt() {
    local prompt="$1"
    local def="$2"
    hide_cursor
    printf '\n' >&2
    local choice="$def"
    while true; do
        printf "ℹ️  %s\n" "$prompt" >&2
        if [ "$choice" = "Y" ]; then
            printf "❯ [Y]es\n  [N]o\n" >&2
        else
            printf "  [Y]es\n❯ [N]o\n" >&2
        fi
        IFS= read -rsn1 key 2>/dev/null
        if [ "$key" = $'\n' ] || [ "$key" = $'\r' ] || [ -z "$key" ]; then
            break
        fi
        case "$key" in
            " ") choice=$([ "$choice" = "Y" ] && echo N || echo Y) ;;
            "y"|"Y") choice=Y ;;
            "n"|"N") choice=N ;;
            "b"|"B") show_cursor; return 2 ;;
            $'\x1b') show_cursor; return 1 ;;
            "q"|"Q") show_cursor; return 1 ;;
        esac
        clear_menu 6
    done
    show_cursor
    if [ "$choice" = "Y" ]; then return 0; else return 1; fi
}
