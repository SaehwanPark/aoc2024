## Problem Overview

**Day 15: Warehouse Woes** involves simulating a robot moving boxes in a warehouse according to a sequence of directional commands.

**Part 1**: Standard warehouse with single-cell boxes (`O`) and robot (`@`)
**Part 2**: Scaled warehouse where everything except the robot doubles in width -- boxes become `[]` (two-cell wide)

The goal is to calculate the sum of GPS coordinates (100 × row + column) for all boxes after movement completion.

## Algorithm Steps

### Core Data Structures
- `Cell` enum: represents different warehouse entities (Wall, Box, BoxLeft/Right, Robot, Empty)
- `Position` struct: handles coordinates and movement calculations
- `Warehouse` struct: manages the grid state and robot position

### Movement Logic

**Simple Boxes (Part 1)**:
1. Check if robot can move in direction
2. If blocked by box, scan linearly until finding empty space or wall
3. If empty space found, shift all boxes in sequence
4. Move robot to new position

**Wide Boxes (Part 2)**:
1. Use BFS to identify all connected box parts that would be affected
2. For vertical movement: both left `[` and right `]` parts move together
3. For horizontal movement: only the leading edge matters
4. Clear all affected positions, then place boxes in new locations

### Key Implementation Details

- **Collision Detection**: different logic for simple vs wide boxes
- **Box Movement**:
  - Simple: linear chain pushing
  - Wide: coordinated movement of connected box components
- **GPS Calculation**: sum coordinates of box origins (left part for wide boxes)
