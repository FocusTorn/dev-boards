# _spec Extension for Gemini CLI

**Measure twice, code once.**

_spec is a Gemini CLI extension that enables **Context-Driven Development**. It turns the Gemini CLI into a proactive project manager that follows a strict protocol to specify, plan, and implement software features and bug fixes.

Instead of just writing code, _spec ensures a consistent, high-quality lifecycle for every task: **Context -> Spec & Plan -> Implement**.

The philosophy behind _spec is simple: control your code. By treating context as a managed artifact alongside your code, you transform your repository into a single source of truth that drives every agent interaction with deep, persistent project awareness.

## Features

- **Plan before you build**: Create specs and plans that guide the agent for new and existing codebases.
- **Maintain context**: Ensure AI follows style guides, tech stack choices, and product goals.
- **Iterate safely**: Review plans before code is written, keeping you firmly in the loop.
- **Work as a team**: Set project-level context for your product, tech stack, and workflow preferences that become a shared foundation for your team.
- **Build on existing projects**: Intelligent initialization for both new (Greenfield) and existing (Brownfield) projects.
- **Smart revert**: A git-aware revert command that understands logical units of work (tracks, phases, tasks) rather than just commit hashes.

## Installation

The `_spec` commands are configured locally for this project. They are linked from `_spec/commands/spec` to `.gemini/commands/spec`.

To set up or refresh the local commands:

```powershell
# In PowerShell (Administrator if needed for symlinks)
if (!(Test-Path .gemini/commands)) { New-Item -ItemType Directory .gemini/commands }
cmd /c "mklink /D .gemini\commands\spec D:\_dev\_Projects\dev-boards\_spec\commands\spec"
```

## Usage

_spec is designed to manage the entire lifecycle of your development tasks using the following commands:

- `/spec:setup`: Scaffolds the project and sets up the _spec environment.
- `/spec:newTrack`: Starts a new feature or bug track.
- `/spec:implement`: Executes the tasks defined in the current track's plan.
- `/spec:status`: Displays the current progress of tracks and the active track.
- `/spec:resume`: Resumes work on an existing track.
- `/spec:close`: Finalizes a track, updates project docs, and archives the track.
- `/spec:revert`: Reverts a track, phase, or task.

## Commands Reference

| Command | Description | Artifacts |
| :--- | :--- | :--- |
| `/spec:setup` | Scaffolds the project and sets up the _spec environment. Run this once per project. | `_spec/product.md`<br>`_spec/product-guidelines.md`<br>`_spec/tech-stack.md`<br>`_spec/workflow.md`<br>`_spec/tracks.md` |
| `/spec:newTrack` | Starts a new feature or bug track. Generates `spec.md` and `plan.md`. | `_spec/tracks/<id>/spec.md`<br>`_spec/tracks/<id>/plan.md`<br>`_spec/tracks.md` |
| `/spec:implement` | Executes the tasks defined in the current track's plan. | `_spec/tracks.md`<br>`_spec/tracks/<id>/plan.md` |
| `/spec:resume` | Resumes work on an existing track, handling branch switching and context restoration. | Reads `_spec/tracks/<id>/plan.md` |
| `/spec:close` | Finalizes a track, updates project docs, and archives or deletes the track folder. | `_spec/archive/`<br>`_spec/tracks.md`<br>Project Context files |
| `/spec:status` | Displays the current progress of the tracks file and active tracks. | Reads `_spec/tracks.md` |
| `/spec:revert` | Reverts a track, phase, or task by analyzing git history. | Reverts git history |




