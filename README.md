# Aerospace Tree Visualizer

Visualize the [Aerospace](https://github.com/nikitabobko/AeroSpace) window manager's window tree structure using GraphViz.

This tool polls Aerospace's window layout and displays it as an auto-updating SVG diagram, grouped by workspace.

## Requirements

- **Aerospace** - The tiling window manager for macOS
- **Rust** - To build the tree parser
- **GraphViz** - For rendering the tree as SVG (`brew install graphviz`)
- A viewer (macOS uses `open` by default)

## Installation

```bash
# Install the tree parser
cargo install --path .

# Make the visualization script executable (already should be)
chmod +x bin/aerospace-tree-viz
```

## Usage

### Quick Start

Simply run the visualization script:

```bash
./bin/aerospace-tree-viz
```

This will:
1. Poll Aerospace's window list every second
2. Generate a GraphViz diagram showing windows grouped by workspace
3. Open and auto-update an SVG visualization

The visualization will continue updating until you close the viewer window.

### Manual Usage

You can also use the tool manually:

```bash
# Get current window tree as dot format
aerospace list-windows --all --json | aerospace-tree-dot | dot -Tsvg > windows.svg

# Open the result
open windows.svg
```

## How It Works

1. `aerospace list-windows --all --json` - Gets all windows in JSON format
2. `aerospace-tree-dot` - Parses the JSON and converts to GraphViz dot format
3. `dot -Tsvg` - Renders the dot graph as SVG
4. The script polls every second and updates the visualization when changes are detected

## Output Format

The visualization shows:
- **Workspace nodes** - Top-level nodes for each workspace
- **Window nodes** - Show the window title and application name
- Organized hierarchically by workspace

## Notes

- The script will consume some CPU/battery while running (it polls every second)
- Close the viewer window to stop the script
- Originally based on i3-tree-viz for the i3 window manager
