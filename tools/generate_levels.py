#!/usr/bin/env python3
"""
Level generator for protocol: play - Bot Game.

Generates solvable levels with a built-in simulator that verifies each level
before outputting it. Levels use the marked-tile system: marked tiles become
the player's inventory (puzzle pieces to place).

Usage:
    python3 tools/generate_levels.py [--count N] [--difficulty easy|medium|hard]
"""

import json
import random
import argparse
import os
import sys
from dataclasses import dataclass, field
from typing import Optional

# ---------------------------------------------------------------------------
# Direction helpers
# ---------------------------------------------------------------------------
DIRECTIONS = ["North", "East", "South", "West"]

OPPOSITE = {"North": "South", "South": "North", "East": "West", "West": "East"}

DELTA = {
    "North": (0, -1),  # row decreases
    "East": (1, 0),    # col increases
    "South": (0, 1),   # row increases
    "West": (-1, 0),   # col decreases
}

# Turn arms: Turn(dir) has arms (arm1, arm2)
TURN_ARMS = {
    "North": ("East", "North"),
    "East": ("South", "East"),
    "South": ("West", "South"),
    "West": ("North", "West"),
}


def turn_exit(bot_dir, turn_dir):
    """Return new direction after hitting a Turn tile, or None if pass-through."""
    arm1, arm2 = TURN_ARMS[turn_dir]
    entry_side = OPPOSITE[bot_dir]
    if entry_side == arm1:
        return arm2
    if entry_side == arm2:
        return arm1
    return None


def needed_turn_dir(from_dir, to_dir):
    """Find which Turn direction redirects a bot moving from_dir to exit to_dir."""
    for td in DIRECTIONS:
        if turn_exit(from_dir, td) == to_dir:
            return td
    return None


# ---------------------------------------------------------------------------
# Tile representation
# ---------------------------------------------------------------------------
def tile_source(color, direction):
    return {"Source": [color, direction]}

def tile_goal(color):
    return {"Goal": color}

def tile_turn(color, direction):
    return {"Turn": [color, direction]}

def tile_turn_but(color, direction):
    return {"TurnBut": [color, direction]}

def tile_bounce(color):
    return {"Bounce": color}

def tile_bounce_but(color):
    return {"BounceBut": color}

def tile_arrow(color, direction):
    return {"Arrow": [color, direction]}

def tile_arrow_but(color, direction):
    return {"ArrowBut": [color, direction]}

def tile_painter(color):
    return {"Painter": color}

def tile_teleport(color, number):
    return {"Teleport": [color, number]}

def tile_door(is_open):
    return {"Door": is_open}

def tile_switch():
    return "Switch"

def tile_color_switch(color):
    return {"ColorSwitch": color}


# ---------------------------------------------------------------------------
# Simulator
# ---------------------------------------------------------------------------
@dataclass
class Bot:
    col: int
    row: int
    direction: str
    color: int
    alive: bool = True
    at_goal: bool = False


def get_tile_kind(grid, col, row, size):
    """Get the tile kind at (col, row), or None if out of bounds."""
    if col < 0 or row < 0 or col >= size or row >= size:
        return None
    return grid.get((col, row), "Empty")


def tile_type_name(tile):
    """Extract the type name from a tile kind value."""
    if isinstance(tile, str):
        return tile
    if isinstance(tile, dict):
        return list(tile.keys())[0]
    return "Empty"


def tile_params(tile):
    """Extract parameters from a tile kind value."""
    if isinstance(tile, dict):
        return list(tile.values())[0]
    return None


def simulate_level(board_size, tiles_list, max_steps=500):
    """
    Simulate a level and return (success, reason, steps).

    Args:
        board_size: grid dimension (NxN)
        tiles_list: list of (col, row, tile_kind) tuples (no is_marked)
        max_steps: maximum simulation steps before timeout

    Returns:
        (bool, str, int): (success, reason, steps_taken)
    """
    # Build grid
    grid = {}
    for col, row, kind in tiles_list:
        grid[(col, row)] = kind

    # Track door states
    doors = {}
    for (c, r), kind in grid.items():
        tn = tile_type_name(kind)
        if tn == "Door":
            doors[(c, r)] = tile_params(kind)  # is_open bool

    # Find sources and spawn bots
    bots = []
    for (c, r), kind in grid.items():
        tn = tile_type_name(kind)
        if tn == "Source":
            params = tile_params(kind)
            color, direction = params[0], params[1]
            bots.append(Bot(col=c, row=r, direction=direction, color=color))

    if not bots:
        return False, "No Source tiles found", 0

    # Find goals
    goals = {}
    for (c, r), kind in grid.items():
        tn = tile_type_name(kind)
        if tn == "Goal":
            goals[(c, r)] = tile_params(kind)  # color_index

    if not goals:
        return False, "No Goal tiles found", 0

    # Find teleport pairs
    teleports = {}
    for (c, r), kind in grid.items():
        tn = tile_type_name(kind)
        if tn in ("Teleport", "TeleportBut"):
            params = tile_params(kind)
            tp_color, tp_num = params[0], params[1]
            key = (tn, tp_color, tp_num)
            if key not in teleports:
                teleports[key] = []
            teleports[key].append((c, r))

    # Simulate step by step
    for step in range(max_steps):
        # Check win: all bots at matching goals
        if all(b.at_goal for b in bots) and len(bots) > 0:
            return True, "All bots reached their goals!", step

        # Check if all bots are stuck (at_goal or dead)
        active = [b for b in bots if b.alive and not b.at_goal]
        if not active:
            all_at_goal = all(b.at_goal for b in bots if b.alive)
            if all_at_goal:
                return True, "All bots reached their goals!", step
            return False, "All bots stopped but not all at goals", step

        for bot in active:
            dc, dr = DELTA[bot.direction]
            nc, nr = bot.col + dc, bot.row + dr

            # Check bounds
            if nc < 0 or nr < 0 or nc >= board_size or nr >= board_size:
                bot.alive = False
                continue

            tile = grid.get((nc, nr), "Empty")
            tn = tile_type_name(tile)
            params = tile_params(tile)

            # Empty = fall off
            if tn == "Empty":
                bot.alive = False
                continue

            # Move bot to new position
            bot.col, bot.row = nc, nr

            if tn in ("Floor", "Source"):
                pass  # keep going

            elif tn == "Goal":
                goal_color = params
                if goal_color == bot.color:
                    bot.at_goal = True

            elif tn == "Turn":
                t_color, t_dir = params[0], params[1]
                new_dir = turn_exit(bot.direction, t_dir)
                if new_dir:
                    bot.direction = new_dir

            elif tn == "TurnBut":
                t_color, t_dir = params[0], params[1]
                if t_color != bot.color:
                    new_dir = turn_exit(bot.direction, t_dir)
                    if new_dir:
                        bot.direction = new_dir

            elif tn == "Bounce":
                bot.direction = OPPOSITE[bot.direction]

            elif tn == "BounceBut":
                b_color = params
                if b_color != bot.color:
                    bot.direction = OPPOSITE[bot.direction]

            elif tn == "Arrow":
                a_color, a_dir = params[0], params[1]
                if bot.direction != a_dir:
                    bot.direction = a_dir

            elif tn == "ArrowBut":
                a_color, a_dir = params[0], params[1]
                if a_color != bot.color and bot.direction != a_dir:
                    bot.direction = a_dir

            elif tn == "Painter":
                bot.color = params

            elif tn == "Door":
                is_open = doors.get((nc, nr), True)
                if not is_open:
                    bot.direction = OPPOSITE[bot.direction]
                    bot.col, bot.row = bot.col - dc, bot.row - dr

            elif tn == "Switch":
                for dk in doors:
                    doors[dk] = not doors[dk]

            elif tn == "ColorSwitch":
                cs_color = params
                if cs_color == bot.color:
                    for dk in doors:
                        doors[dk] = not doors[dk]

            elif tn == "ColorSwitchBut":
                cs_color = params
                if cs_color != bot.color:
                    for dk in doors:
                        doors[dk] = not doors[dk]

            elif tn in ("Teleport", "TeleportBut"):
                tp_color, tp_num = params[0], params[1]
                should_tp = False
                if tn == "Teleport":
                    should_tp = (tp_color == 9 or tp_color == bot.color)
                else:
                    should_tp = (tp_color != bot.color)

                if should_tp:
                    key = (tn, tp_color, tp_num)
                    pair_locs = [p for p in teleports.get(key, []) if p != (nc, nr)]
                    if pair_locs:
                        dest = pair_locs[0]
                        bot.col, bot.row = dest
                    else:
                        bot.direction = OPPOSITE[bot.direction]

        # Check for dead bots
        for bot in bots:
            if not bot.alive:
                return False, f"Bot (color {bot.color}) fell off the board at ({bot.col}, {bot.row})", step

    return False, "Timeout: exceeded max steps", max_steps


# ---------------------------------------------------------------------------
# Level Generation
# ---------------------------------------------------------------------------
def trace_path(board_size, start_col, start_row, start_dir, num_turns, rng):
    """
    Trace a path through the grid with random turns.
    Returns list of (col, row, direction_at_this_cell) and turn positions.
    """
    path = [(start_col, start_row, start_dir)]
    col, row, direction = start_col, start_row, start_dir
    turns_placed = 0
    visited = {(col, row)}
    steps_since_turn = 0
    min_straight = 2  # minimum straight steps before a turn

    for _ in range(board_size * board_size):
        dc, dr = DELTA[direction]
        nc, nr = col + dc, row + dr

        # Check if next cell is valid
        if nc < 0 or nr < 0 or nc >= board_size or nr >= board_size:
            break
        if (nc, nr) in visited:
            break

        col, row = nc, nr
        visited.add((col, row))
        steps_since_turn += 1

        # Maybe place a turn
        if turns_placed < num_turns and steps_since_turn >= min_straight:
            # Check which turns are possible (won't go off-board next step)
            possible_dirs = []
            for new_dir in DIRECTIONS:
                if new_dir == direction or new_dir == OPPOSITE[direction]:
                    continue
                ndc, ndr = DELTA[new_dir]
                nnc, nnr = col + ndc, row + ndr
                if 0 <= nnc < board_size and 0 <= nnr < board_size and (nnc, nnr) not in visited:
                    possible_dirs.append(new_dir)

            if possible_dirs and rng.random() < 0.5:
                new_dir = rng.choice(possible_dirs)
                td = needed_turn_dir(direction, new_dir)
                if td:
                    path.append((col, row, f"TURN:{td}:{direction}"))
                    direction = new_dir
                    turns_placed += 1
                    steps_since_turn = 0
                    continue

        path.append((col, row, direction))

    return path


def generate_single_bot_level(board_size, difficulty, rng):
    """Generate a single-bot level. Returns (tiles_list, name) or None."""
    color = rng.randint(0, 5)

    # Pick start edge
    edge = rng.choice(["top", "bottom", "left", "right"])
    if edge == "top":
        start_col = rng.randint(1, board_size - 2)
        start_row = 0
        start_dir = "South"
    elif edge == "bottom":
        start_col = rng.randint(1, board_size - 2)
        start_row = board_size - 1
        start_dir = "North"
    elif edge == "left":
        start_col = 0
        start_row = rng.randint(1, board_size - 2)
        start_dir = "East"
    else:
        start_col = board_size - 1
        start_row = rng.randint(1, board_size - 2)
        start_dir = "West"

    num_turns = {"easy": rng.randint(1, 2), "medium": rng.randint(2, 4), "hard": rng.randint(3, 6)}[difficulty]

    path = trace_path(board_size, start_col, start_row, start_dir, num_turns, rng)

    if len(path) < 4:
        return None  # too short

    # Build tiles
    tiles = {}
    marked_positions = []

    # Source at start
    tiles[(path[0][0], path[0][1])] = (tile_source(color, start_dir), False)

    # Process path
    for i, (c, r, info) in enumerate(path[1:], 1):
        if isinstance(info, str) and info.startswith("TURN:"):
            parts = info.split(":")
            turn_dir = parts[1]
            tile = tile_turn(color, turn_dir)
            # Mark turns as puzzle pieces for player to place
            tiles[(c, r)] = (tile, True)
            marked_positions.append((c, r))
        elif i == len(path) - 1:
            # Goal at end
            tiles[(c, r)] = (tile_goal(color), False)
        else:
            tiles[(c, r)] = ("Floor", False)

    # Make sure we have a goal
    last = path[-1]
    lc, lr = last[0], last[1]
    if (lc, lr) not in tiles or tile_type_name(tiles[(lc, lr)][0]) != "Goal":
        tiles[(lc, lr)] = (tile_goal(color), False)

    # Add floor padding around the path for visual context
    path_cells = {(c, r) for c, r, _ in path}
    for c, r in list(path_cells):
        for dc, dr in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            nc, nr = c + dc, r + dr
            if 0 <= nc < board_size and 0 <= nr < board_size and (nc, nr) not in tiles:
                tiles[(nc, nr)] = ("Floor", False)

    # Build final tiles list
    tiles_list = []
    for r in range(board_size):
        for c in range(board_size):
            if (c, r) in tiles:
                kind, marked = tiles[(c, r)]
                tiles_list.append((c, r, kind, marked))
            else:
                tiles_list.append((c, r, "Empty", False))

    return tiles_list, marked_positions


def generate_two_bot_level(board_size, difficulty, rng):
    """Generate a two-bot level with separate paths. Returns (tiles_list, name) or None."""
    color1 = 0  # Red
    color2 = 4  # Dark Green

    # Place sources on opposite edges
    side = rng.choice(["horizontal", "vertical"])
    if side == "horizontal":
        s1_col, s1_row, s1_dir = 0, rng.randint(1, board_size // 2 - 1), "East"
        s2_col, s2_row, s2_dir = 0, rng.randint(board_size // 2 + 1, board_size - 2), "East"
    else:
        s1_col, s1_row, s1_dir = rng.randint(1, board_size // 2 - 1), 0, "South"
        s2_col, s2_row, s2_dir = rng.randint(board_size // 2 + 1, board_size - 2), 0, "South"

    num_turns = {"easy": 1, "medium": 2, "hard": 3}[difficulty]

    path1 = trace_path(board_size, s1_col, s1_row, s1_dir, num_turns, rng)
    path2 = trace_path(board_size, s2_col, s2_row, s2_dir, num_turns, rng)

    if len(path1) < 3 or len(path2) < 3:
        return None

    # Check for path overlap (excluding start)
    cells1 = {(c, r) for c, r, _ in path1[1:]}
    cells2 = {(c, r) for c, r, _ in path2[1:]}
    if cells1 & cells2:
        return None  # paths overlap, retry

    tiles = {}
    marked_positions = []

    # Bot 1 path
    tiles[(path1[0][0], path1[0][1])] = (tile_source(color1, s1_dir), False)
    for i, (c, r, info) in enumerate(path1[1:], 1):
        if isinstance(info, str) and info.startswith("TURN:"):
            td = info.split(":")[1]
            tiles[(c, r)] = (tile_turn(color1, td), True)
            marked_positions.append((c, r))
        elif i == len(path1) - 1:
            tiles[(c, r)] = (tile_goal(color1), False)
        else:
            tiles[(c, r)] = ("Floor", False)
    last1 = path1[-1]
    if tile_type_name(tiles.get((last1[0], last1[1]), ("Empty", False))[0]) != "Goal":
        tiles[(last1[0], last1[1])] = (tile_goal(color1), False)

    # Bot 2 path
    tiles[(path2[0][0], path2[0][1])] = (tile_source(color2, s2_dir), False)
    for i, (c, r, info) in enumerate(path2[1:], 1):
        if isinstance(info, str) and info.startswith("TURN:"):
            td = info.split(":")[1]
            tiles[(c, r)] = (tile_turn(color2, td), True)
            marked_positions.append((c, r))
        elif i == len(path2) - 1:
            tiles[(c, r)] = (tile_goal(color2), False)
        else:
            tiles[(c, r)] = ("Floor", False)
    last2 = path2[-1]
    if tile_type_name(tiles.get((last2[0], last2[1]), ("Empty", False))[0]) != "Goal":
        tiles[(last2[0], last2[1])] = (tile_goal(color2), False)

    # Floor padding
    all_path = {(c, r) for c, r, _ in path1} | {(c, r) for c, r, _ in path2}
    for c, r in list(all_path):
        for dc, dr in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            nc, nr = c + dc, r + dr
            if 0 <= nc < board_size and 0 <= nr < board_size and (nc, nr) not in tiles:
                tiles[(nc, nr)] = ("Floor", False)

    tiles_list = []
    for r in range(board_size):
        for c in range(board_size):
            if (c, r) in tiles:
                kind, marked = tiles[(c, r)]
                tiles_list.append((c, r, kind, marked))
            else:
                tiles_list.append((c, r, "Empty", False))

    return tiles_list, marked_positions


def generate_painter_level(board_size, difficulty, rng):
    """Generate a level where the bot must change color via Painter to reach goal."""
    start_color = 0  # Red
    goal_color = 5   # Light Blue

    edge = rng.choice(["top", "left"])
    if edge == "top":
        sc, sr, sd = rng.randint(1, board_size - 2), 0, "South"
    else:
        sc, sr, sd = 0, rng.randint(1, board_size - 2), "East"

    num_turns = {"easy": 1, "medium": 2, "hard": 3}[difficulty]
    path = trace_path(board_size, sc, sr, sd, num_turns + 1, rng)

    if len(path) < 5:
        return None

    tiles = {}
    marked_positions = []

    # Source
    tiles[(path[0][0], path[0][1])] = (tile_source(start_color, sd), False)

    # Place painter roughly in the middle of the path
    mid = len(path) // 2
    painter_placed = False

    for i, (c, r, info) in enumerate(path[1:], 1):
        if isinstance(info, str) and info.startswith("TURN:"):
            td = info.split(":")[1]
            tiles[(c, r)] = (tile_turn(start_color if not painter_placed else goal_color, td), True)
            marked_positions.append((c, r))
        elif i == mid and not painter_placed:
            tiles[(c, r)] = (tile_painter(goal_color), True)
            marked_positions.append((c, r))
            painter_placed = True
        elif i == len(path) - 1:
            tiles[(c, r)] = (tile_goal(goal_color), False)
        else:
            tiles[(c, r)] = ("Floor", False)

    last = path[-1]
    if tile_type_name(tiles.get((last[0], last[1]), ("Empty", False))[0]) != "Goal":
        tiles[(last[0], last[1])] = (tile_goal(goal_color), False)

    if not painter_placed:
        return None

    # Floor padding
    path_cells = {(c, r) for c, r, _ in path}
    for c, r in list(path_cells):
        for dc, dr in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            nc, nr = c + dc, r + dr
            if 0 <= nc < board_size and 0 <= nr < board_size and (nc, nr) not in tiles:
                tiles[(nc, nr)] = ("Floor", False)

    tiles_list = []
    for r in range(board_size):
        for c in range(board_size):
            if (c, r) in tiles:
                kind, marked = tiles[(c, r)]
                tiles_list.append((c, r, kind, marked))
            else:
                tiles_list.append((c, r, "Empty", False))

    return tiles_list, marked_positions


def generate_bounce_level(board_size, difficulty, rng):
    """Generate a level using Bounce tiles to redirect the bot."""
    color = rng.randint(0, 5)

    sc, sr, sd = rng.randint(1, board_size - 2), 0, "South"
    path = [(sc, sr, sd)]
    col, row, direction = sc, sr, sd
    visited = {(col, row)}
    bounces = 0
    max_bounces = {"easy": 1, "medium": 2, "hard": 3}[difficulty]
    tiles = {}
    marked = []

    tiles[(sc, sr)] = (tile_source(color, sd), False)

    for _ in range(board_size * board_size):
        dc, dr = DELTA[direction]
        nc, nr = col + dc, row + dr

        if nc < 0 or nr < 0 or nc >= board_size or nr >= board_size or (nc, nr) in visited:
            # Place bounce before going off-board
            if bounces < max_bounces and (col, row) != (sc, sr):
                # Place bounce at current position
                tiles[(col, row)] = (tile_bounce(color), True)
                marked.append((col, row))
                direction = OPPOSITE[direction]
                bounces += 1
                # Now move in new direction, picking a turn
                possible = [d for d in DIRECTIONS if d != direction and d != OPPOSITE[direction]]
                # Actually, bounce reverses. Let's add a turn after bounce.
                # Re-think: bounce reverses, then we need a turn to go sideways.
                # Simpler: just use turns instead of bounces for this.
                break
            break

        col, row = nc, nr
        visited.add((col, row))
        path.append((col, row, direction))
        tiles[(col, row)] = ("Floor", False)

    # Place goal at end
    if len(path) >= 3:
        gc, gr = path[-1][0], path[-1][1]
        tiles[(gc, gr)] = (tile_goal(color), False)
    else:
        return None

    # Floor padding
    for c, r, _ in path:
        for dc, dr in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            nc, nr = c + dc, r + dr
            if 0 <= nc < board_size and 0 <= nr < board_size and (nc, nr) not in tiles:
                tiles[(nc, nr)] = ("Floor", False)

    tiles_list = []
    for r in range(board_size):
        for c in range(board_size):
            if (c, r) in tiles:
                kind, mk = tiles[(c, r)]
                tiles_list.append((c, r, kind, mk))
            else:
                tiles_list.append((c, r, "Empty", False))

    return tiles_list, marked


# ---------------------------------------------------------------------------
# Verification
# ---------------------------------------------------------------------------
def verify_level(board_size, tiles_list_with_marks):
    """Verify a level by simulating it WITH all tiles (including marked ones)."""
    # Include all tiles for simulation
    sim_tiles = [(c, r, k) for c, r, k, _ in tiles_list_with_marks]
    success, reason, steps = simulate_level(board_size, sim_tiles)
    return success, reason, steps


# ---------------------------------------------------------------------------
# Level output
# ---------------------------------------------------------------------------
LEVEL_NAMES = {
    "easy": [
        "First Steps", "Gentle Curve", "Simple Path", "Easy Bend",
        "Quick Turn", "Straight Away", "Corner Case", "The Basics",
        "Baby Steps", "One Turn", "Slight Detour", "The Shortcut",
    ],
    "medium": [
        "Color Shift", "Dual Paths", "Zigzag", "The Maze",
        "Cross Roads", "Double Turn", "Paint Job", "Fork Road",
        "Two Bots", "Switcheroo", "The Spiral", "Serpentine",
    ],
    "hard": [
        "Labyrinth", "Triple Turn", "Color Chaos", "The Gauntlet",
        "Winding Road", "Complex Path", "Multi Turn", "Hard Left",
        "Twisted Path", "Knot Theory", "Puzzle Box", "Mind Bender",
    ],
}


def generate_level(difficulty, rng, board_size=None):
    """Try to generate a verified level. Returns (level_data, name) or None."""
    if board_size is None:
        board_size = {"easy": rng.randint(5, 6), "medium": rng.randint(6, 8), "hard": rng.randint(7, 10)}[difficulty]

    generators = [generate_single_bot_level]
    if difficulty in ("medium", "hard"):
        generators.extend([generate_painter_level, generate_two_bot_level])

    gen_func = rng.choice(generators)
    result = gen_func(board_size, difficulty, rng)
    if result is None:
        return None

    tiles_list, marked_positions = result

    # Verify
    success, reason, steps = verify_level(board_size, tiles_list)
    if not success:
        return None

    # Build solution from marked tiles
    solution = []
    for c, r, kind, is_marked in tiles_list:
        if is_marked:
            solution.append([c, r, kind])

    # Build level data
    level_tiles = []
    for c, r, kind, is_marked in tiles_list:
        level_tiles.append([c, r, kind, is_marked])

    name = rng.choice(LEVEL_NAMES[difficulty])

    level_data = {
        "name": name,
        "board_size": board_size,
        "tiles": level_tiles,
        "solution": solution,
    }

    return level_data


def main():
    parser = argparse.ArgumentParser(description="Generate bot game levels")
    parser.add_argument("--count", type=int, default=6, help="Number of levels to generate")
    parser.add_argument("--difficulty", choices=["easy", "medium", "hard", "mixed"], default="mixed")
    parser.add_argument("--output-dir", default="levels", help="Output directory")
    parser.add_argument("--seed", type=int, default=None, help="Random seed")
    parser.add_argument("--prefix", default="gen", help="Filename prefix")
    args = parser.parse_args()

    rng = random.Random(args.seed)

    os.makedirs(args.output_dir, exist_ok=True)

    difficulties = []
    if args.difficulty == "mixed":
        per = args.count // 3
        difficulties = ["easy"] * per + ["medium"] * per + ["hard"] * (args.count - 2 * per)
    else:
        difficulties = [args.difficulty] * args.count

    generated = 0
    for idx, diff in enumerate(difficulties):
        attempts = 0
        while attempts < 100:
            attempts += 1
            result = generate_level(diff, rng)
            if result is not None:
                level_data = result
                num = str(idx + 1).zfill(2)
                filename = f"{args.prefix}-{num}.json"
                filepath = os.path.join(args.output_dir, filename)
                level_data["name"] = f"{level_data['name']} ({num})"

                with open(filepath, "w") as f:
                    json.dump(level_data, f, indent=2)

                n_marked = sum(1 for t in level_data["tiles"] if t[3])
                n_tiles = sum(1 for t in level_data["tiles"] if tile_type_name(t[2]) != "Empty")
                print(f"  [{num}] {level_data['name']:30s} | {diff:6s} | "
                      f"{level_data['board_size']}x{level_data['board_size']} | "
                      f"{n_tiles} tiles, {n_marked} puzzle pieces | "
                      f"-> {filepath}")
                generated += 1
                break
        else:
            print(f"  [!!] Failed to generate {diff} level after 100 attempts", file=sys.stderr)

    print(f"\nGenerated {generated}/{len(difficulties)} levels.")

    # Verify all generated levels one more time
    print("\nVerification pass:")
    all_ok = True
    for idx in range(len(difficulties)):
        num = str(idx + 1).zfill(2)
        filepath = os.path.join(args.output_dir, f"{args.prefix}-{num}.json")
        if not os.path.exists(filepath):
            continue
        with open(filepath) as f:
            data = json.load(f)
        sim_tiles = [(t[0], t[1], t[2]) for t in data["tiles"]]
        success, reason, steps = simulate_level(data["board_size"], sim_tiles)
        status = "OK" if success else "FAIL"
        if not success:
            all_ok = False
        print(f"  [{num}] {status:4s} | {reason} (steps: {steps})")

    if all_ok:
        print("\nAll levels verified successfully!")
    else:
        print("\nSome levels FAILED verification!", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
