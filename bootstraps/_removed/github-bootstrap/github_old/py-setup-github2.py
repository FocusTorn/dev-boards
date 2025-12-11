#!/usr/bin/env python3
"""
Simplified GitHub Setup Script
Uses prompt-wizard.exe to collect information and displays results
"""

import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path


def find_wizard_exe():
    """Find prompt-wizard.exe in common locations"""
    script_dir = Path(__file__).parent
    project_root = script_dir.parent.parent
    
    # Possible locations
    locations = [
        project_root / "projects" / "iMenu" / "cmd" / "prompt-wizard" / "prompt-wizard.exe",
        project_root / "projects" / "iMenu" / "dist" / "bin" / "prompt-wizard.exe",
        script_dir.parent / "prompt-wizard.exe",
        script_dir / "prompt-wizard.exe",
    ]
    
    for loc in locations:
        if loc.exists():
            return str(loc.resolve())
    
    return None


def build_wizard_steps():
    """Build simplified wizard steps"""
    steps = []
    
    # GitHub username
    steps.append({
        "type": "input",
        "title": "GitHub username",
        "key": "github_user",
        "placeholder": "username",
        "description": "Enter your GitHub username"
    })
    
    # Git user email
    steps.append({
        "type": "input",
        "title": "Git user email",
        "key": "git_email",
        "placeholder": "your.email@example.com",
        "description": "Email for Git commits"
    })
    
    steps.append({
        "type": "input",
        "title": "Git user name",
        "key": "git_nasame",
        "placeholder": "Your Name",
        "description": "Name for Git commits"
    })
    
    # Repository name
    repo_name = Path.cwd().name
    steps.append({
        "type": "input",
        "title": "Repository name",
        "key": "repo_name",
        "placeholder": repo_name,
        "default": repo_name,
        "description": "Name for the GitHub repository"
    })
    
    # Make private?
    steps.append({
        "type": "confirm",
        "title": "Make repository private?",
        "key": "is_private",
        "default": "no",
        "description": "Private repositories are only visible to you and collaborators"
    })
    
    # Git user name
    steps.append({
        "type": "input",
        "title": "Git user name",
        "key": "git_name",
        "placeholder": "Your Name",
        "description": "Name for Git commits"
    })
    
    return steps


def run_wizard(steps):
    """Run the prompt-wizard.exe with the given steps"""
    wizard_exe = find_wizard_exe()
    if not wizard_exe:
        print("❌ Error: prompt-wizard.exe not found", file=sys.stderr)
        print("   Checked locations:", file=sys.stderr)
        script_dir = Path(__file__).parent
        project_root = script_dir.parent.parent
        locations = [
            project_root / "projects" / "iMenu" / "cmd" / "prompt-wizard" / "prompt-wizard.exe",
            project_root / "projects" / "iMenu" / "dist" / "bin" / "prompt-wizard.exe",
            script_dir.parent / "prompt-wizard.exe",
        ]
        for loc in locations:
            print(f"     - {loc}", file=sys.stderr)
        sys.exit(1)
    
    # Create temp file for steps JSON
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False, encoding='utf-8') as steps_file:
        json.dump(steps, steps_file, indent=2)
        steps_path = steps_file.name
    
    # Create temp file for results
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False, encoding='utf-8') as result_file:
        result_path = result_file.name
    
    try:
        # Run wizard
        # Don't capture any output - let TUI display directly to terminal
        # This avoids Unicode encoding issues with Windows default encoding
        cmd = [wizard_exe, steps_path, "--result-file", result_path]
        
        # Run without capturing output to avoid encoding issues
        # The wizard is a TUI that writes directly to the terminal
        result = subprocess.run(cmd)
        
        if result.returncode != 0:
            print(f"❌ Error: Wizard exited with code {result.returncode}", file=sys.stderr)
            return None
        
        # Read results
        if not os.path.exists(result_path):
            print("❌ Error: Wizard did not create result file", file=sys.stderr)
            return None
        
        with open(result_path, 'r', encoding='utf-8') as f:
            results = json.load(f)
        
        return results
    
    finally:
        # Cleanup temp files
        try:
            os.unlink(steps_path)
        except:
            pass
        try:
            os.unlink(result_path)
        except:
            pass


def display_results(results):
    """Display the wizard results in a formatted way"""
    print("\n" + "=" * 70)
    print("WIZARD RESULTS")
    print("=" * 70)
    print()
    
    for key, value in results.items():
        # Format the key name
        display_key = key.replace('_', ' ').title()
        
        # Format the value
        if isinstance(value, bool):
            display_value = "Yes" if value else "No"
        elif isinstance(value, list):
            display_value = ", ".join(str(v) for v in value)
        else:
            display_value = str(value)
        
        print(f"  {display_key:.<30} {display_value}")
    
    print()
    print("=" * 70)
    print()


def main():
    """Main entry point"""
    print("GitHub Setup Wizard")
    print("=" * 70)
    print()
    
    # Build wizard steps
    steps = build_wizard_steps()
    
    # Run wizard
    results = run_wizard(steps)
    
    if not results:
        print("Wizard was cancelled or failed.", file=sys.stderr)
        sys.exit(1)
    
    # Display results
    display_results(results)


if __name__ == "__main__":
    main()

