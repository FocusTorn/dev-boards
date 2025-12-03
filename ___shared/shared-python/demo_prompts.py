#!/usr/bin/env python3
"""
Simple demonstration of the three prompts from git-py:
1. GitHub username (or organization name)
2. Repository name
3. Repository visibility
"""

import sys

try:
    from pyprompt import text, select, HAS_PROMPT_TOOLKIT
except ImportError:
    print("Error: pyprompt package not found.", file=sys.stderr)
    print("Please install pyprompt first:", file=sys.stderr)
    print("  pip install -e ../_projects/pyprompt", file=sys.stderr)
    print("  # or", file=sys.stderr)
    print("  cd ../_projects/pyprompt && pip install -e .", file=sys.stderr)
    sys.exit(1)

if not HAS_PROMPT_TOOLKIT:
    print("Error: prompt_toolkit is not installed", file=sys.stderr)
    print("Install it with: pip install prompt-toolkit", file=sys.stderr)
    sys.exit(1)


def main():
    """Demonstrate the three prompts from git-py."""
    print("=== Git Repository Setup Prompts ===\n")
    
    # Prompt 1: GitHub username (or organization name)
    github_user = text(
        "GitHub username (or organization name):",
        default=""
    )
    
    if github_user is None:
        print("\nCancelled.", file=sys.stderr)
        return 1
    
    if not github_user.strip():
        print("Error: GitHub username is required.", file=sys.stderr)
        return 1
    
    print(f"GitHub username: {github_user}\n")
    
    # Prompt 2: Repository name
    repo_name = text(
        "Repository name:",
        default="Trial"
    )
    
    if repo_name is None:
        print("\nCancelled.", file=sys.stderr)
        return 1
    
    if not repo_name.strip():
        print("Error: Repository name is required.", file=sys.stderr)
        return 1
    
    print(f"Repository name: {repo_name}\n")
    
    visibility = select(
        "Repository visibility:",
        choices=["Public", "Private"],
        pointer=" »"
    )
    
    if visibility is None:
        print("\nCancelled.", file=sys.stderr)
        return 1
    
    is_private = (visibility == "Private")
    
    print(f"Repository visibility: {visibility}\n")
    
    # Display summary
    print("=== Summary ===")
    print(f"GitHub User: {github_user}")
    print(f"Repository: {repo_name}")
    print(f"Visibility: {visibility} ({'Private' if is_private else 'Public'})")
    print(f"Full URL: https://github.com/{github_user}/{repo_name}")
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

