"""
Example usage of indeneder package.
"""

from indeneder import (
    write_header,
    write_header_fat,
    write_boxed_header,
    start_region,
    get_region_indent,
    COLOR_GREEN,
    COLOR_YELLOW,
    COLOR_RED,
    COLOR_RESET,
    BOLD_CHECK,
)


def main():
    print("=== indeneder Examples ===\n")
    
    # Simple header with automatic indentation
    with write_header("Step 1: Setup"):
        print("This will be indented by 2 spaces")
        print("So will this")
        print("All output is automatically indented")
    
    # Fat header (no indentation)
    write_header_fat("Major Section")
    print("This is not indented")
    
    # Boxed header
    write_boxed_header("Important Notice")
    print("This is not indented")
    
    # Nested headers
    with write_header("Main Section"):
        print("Level 1 indentation")
        
        with write_header("Sub-section"):
            print("Level 2 indentation")
            
            with write_header("Sub-sub-section"):
                print("Level 3 indentation")
    
    # Using colors with headers
    with write_header("Status Report"):
        print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} Operation completed")
        print(f"{COLOR_YELLOW}⚠{COLOR_RESET} Warning: Check configuration")
        print(f"{COLOR_RED}✗{COLOR_RESET} Error: Failed to connect")
    
    # Manual region without header
    with start_region("Custom Region"):
        print("This is indented without a header")
        print("Useful for custom formatting")
    
    # Get current indentation
    with write_header("Indentation Example"):
        indent = get_region_indent()
        print(f"Current indentation: '{indent}' (length: {len(indent)})")
        
        with start_region("Nested"):
            indent = get_region_indent()
            print(f"Nested indentation: '{indent}' (length: {len(indent)})")
    
    print("\nAll examples completed!")


if __name__ == "__main__":
    main()

