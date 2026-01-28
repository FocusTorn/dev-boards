import os
import shutil

def sync_tracks():
    workspace_root = os.environ.get('WORKSPACE_ROOT')
    if not workspace_root:
        raise EnvironmentError("WORKSPACE_ROOT environment variable is not set. This is required for path resolution.")

    # Source: The core command definitions
    src_commands_dir = os.path.join(workspace_root, '_spec', 'commands', '_spec')
    # Destination: The CLI command directory
    dest_commands_dir = os.path.join(workspace_root, '.gemini', 'commands', '_spec')

    if not os.path.exists(src_commands_dir):
        print(f"Core commands directory not found: {src_commands_dir}")
        # Fallback to flatter structure if the nested one doesn't exist
        src_commands_dir = os.path.join(workspace_root, '_spec', 'commands')
        if not os.path.exists(src_commands_dir):
            print("No source commands found.")
            return

    os.makedirs(dest_commands_dir, exist_ok=True)

    # 1. Synchronize core commands (non-recursive, just the .toml files in the root)
    core_commands = [f for f in os.listdir(src_commands_dir) if f.endswith('.toml')]
    
    for cmd in core_commands:
        src_path = os.path.join(src_commands_dir, cmd)
        dest_path = os.path.join(dest_commands_dir, cmd)
        shutil.copy2(src_path, dest_path)
        print(f"Synced core command: {cmd}")

    # 2. Cleanup: Remove old subdirectories (resume/ and close/) if they exist in destination
    for sub_dir in ['resume', 'close']:
        target_path = os.path.join(dest_commands_dir, sub_dir)
        if os.path.exists(target_path) and os.path.isdir(target_path):
            shutil.rmtree(target_path)
            print(f"Cleaned up legacy command directory: {sub_dir}")

    # 3. Cleanup stale core commands in destination that no longer exist in source
    dest_core_commands = [f for f in os.listdir(dest_commands_dir) if f.endswith('.toml')]
    for cmd in dest_core_commands:
        if cmd not in core_commands and cmd != 'manual_input.toml':
            os.remove(os.path.join(dest_commands_dir, cmd))
            print(f"Removed stale core command: {cmd}")

    print("_spec commands have been synchronized and cleaned.")

if __name__ == "__main__":
    sync_tracks()
