---
colors:
  primary: "#3B82F6"
  background: "#1E1E2E"
  surface: "#282A36"
  surface_border: "#44475A"
  text_primary: "#F8F8F2"
  text_muted: "#6272A4"
  status_active: "#50FA7B"
  status_inactive: "#FF5555"
typography:
  family: "system-ui, -apple-system, sans-serif"
  size_heading: 18
  size_body: 13
  size_caption: 11
spacing:
  margin: 12
  gap: 8
  padding: 6
---

# Visual Design System (egui Theme)

## Overview
Arthur Clicker utilizes a clean, compact dark theme inspired by the Dracula / Catppuccin palette, providing clear visual status indicators and high contrast for low cognitive load during active automation.

## Colors
- `primary`: Accent and active buttons (#3B82F6).
- `background`: Main window background (#1E1E2E).
- `surface`: Group boxes and section frames (#282A36).
- `status_active`: Green indicator when auto-clicking is active (#50FA7B).
- `status_inactive`: Red/gray indicator when stopped (#FF5555).

## Layout & Controls
- Window size: Fixed compact size ~380x560px for minimal desktop footprint.
- Grouping: Logical subsections surrounded by framed panels with rounded corners.
- Sliders / Inputs: Compact numeric inputs with clear unit suffixes (h, m, s, ms).
