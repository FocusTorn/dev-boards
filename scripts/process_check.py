#!/usr/bin/env python3
"""
Development Process Checker
Lists running processes related to Python, Rust, Go, and dev tools.
"""

import sys
import os

try:
    import psutil
except ImportError:
    print("ERROR: 'psutil' is not installed.")
    print("Please run: uv pip install psutil")
    sys.exit(1)

def list_dev_processes():
    # Targets to look for in the process name or command line
    targets = [
        'python', 'rust', 'go', 'node', 'cargo', 
        'arduino', 'orca-slicer', 'msedgewebview2', 'mosquitto', 'idf.py'
    ]
    
    # Summary data
    stats = {t: {'count': 0, 'memory': 0} for t in targets}
    stats['other'] = {'count': 0, 'memory': 0}
    
    found_any = False
    
    # Sort processes by name for easier reading
    processes = []
    for proc in psutil.process_iter(['pid', 'name', 'cmdline', 'memory_info']):
        try:
            name = proc.info['name'] or ""
            cmdline_list = proc.info['cmdline'] or []
            cmdline_str = " ".join(cmdline_list).lower()
            
            # Check which targets match
            matched_targets = [t for t in targets if t in name.lower() or t in cmdline_str]
            
            if matched_targets:
                # Add memory info to process info
                mem_bytes = proc.info['memory_info'].rss if proc.info['memory_info'] else 0
                proc.info['memory_bytes'] = mem_bytes
                processes.append(proc.info)
                
                # Attribute to each matched target for the summary
                for t in matched_targets:
                    stats[t]['count'] += 1
                    stats[t]['memory'] += mem_bytes
                
                found_any = True
        except (psutil.NoSuchProcess, psutil.AccessDenied):
            continue
            
    if not processes:
        print("No matching development processes found.")
        return

    # Sort by name, then PID
    processes.sort(key=lambda x: (x['name'].lower(), x['pid']))

    # Calculate dynamic column widths
    pid_width = max([len(str(p['pid'])) for p in processes] + [len('PID')])
    name_width = max([len(p['name'] or "") for p in processes] + [len('Name')])
    
    # Header
    header = f"{'PID':<{pid_width}} | {'Name':<{name_width}} | {'Command Line'}"
    print(header)
    print("-" * (pid_width + name_width + 6) + "-" * 40)
    
    for info in processes:
        pid = info['pid']
        name = info['name']
        cmdline_list = info['cmdline'] or []
        
        if not cmdline_list:
            print(f"{str(pid):<{pid_width}} | {name:<{name_width}} | ")
            continue
            
        # Print first argument
        print(f"{str(pid):<{pid_width}} | {name:<{name_width}} | {cmdline_list[0]}")
        
        # Print remaining arguments indented
        indent_prefix = f"{'':<{pid_width}} | {'':<{name_width}} |     "
        for arg in cmdline_list[1:]:
            print(f"{indent_prefix}{arg}")
            
    # Print Summary Table
    print("\n" + "=" * 40)
    print(f"{'Category':<15} | {'Count':<5} | {'Memory Use':<10}")
    print("-" * 40)
    
    total_mem = 0
    total_count = len(processes) # Total unique processes
    
    # Only show categories with activity
    active_targets = sorted([t for t in targets if stats[t]['count'] > 0], 
                           key=lambda x: stats[x]['memory'], reverse=True)
    
    for t in active_targets:
        mem_mb = stats[t]['memory'] / (1024 * 1024)
        print(f"{t:<15} | {stats[t]['count']:<5} | {mem_mb:>8.2f} MB")
        
    # Calculate true total memory (avoiding double counting if a process matched multiple targets)
    unique_total_mem = sum([p['memory_bytes'] for p in processes])
    
    print("-" * 40)
    print(f"{'TOTAL (unique)':<15} | {total_count:<5} | {unique_total_mem / (1024 * 1024):>8.2f} MB")
    print("=" * 40)

if __name__ == "__main__":
    try:
        list_dev_processes()
    except KeyboardInterrupt:
        print("\nExiting...")
