import os
import shutil

def sync_tracks():
    tracks_dir = os.path.join('conductor', 'tracks')
    commands_dir = os.path.join('.gemini', 'commands', 'conductor', 'resume')

    if not os.path.exists(tracks_dir):
        print(f"Tracks directory not found: {tracks_dir}")
        return

    os.makedirs(commands_dir, exist_ok=True)

    # Get current track folders
    tracks = [d for d in os.listdir(tracks_dir) if os.path.isdir(os.path.join(tracks_dir, d))]
    
    # Create TOML for each track
    for track in tracks:
        file_path = os.path.join(commands_dir, f"{track}.toml")
        content = (
            f"description = 'Resume work on the {track} track'\n"
            f"prompt = 'Resume work on the Conductor track: {track}. "
            f"Please read conductor/tracks/{track}/plan.md and check the current status to determine the next step.'"
        )
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)

    # Ensure manual_input.toml exists
    manual_path = os.path.join(commands_dir, 'manual_input.toml')
    if not os.path.exists(manual_path):
        manual_content = (
            "description = 'Resume a track by typing its name manually'\n"
            "prompt = 'I want to resume work on a Conductor track. "
            "Track specified: {{args}}. Please check if this track exists in conductor/tracks/ and proceed.'"
        )
        with open(manual_path, 'w', encoding='utf-8') as f:
            f.write(manual_content)

    # Cleanup removed tracks
    existing_tomls = [f for f in os.listdir(commands_dir) if f.endswith('.toml')]
    track_toml_names = [f"{t}.toml" for t in tracks]
    
    for toml in existing_tomls:
        if toml not in track_toml_names and toml != 'manual_input.toml':
            os.remove(os.path.join(commands_dir, toml))
            print(f"Removed stale command: {toml}")

    print(f"TUI Menu Synced: {len(tracks)} tracks found.")

if __name__ == "__main__":
    sync_tracks()
