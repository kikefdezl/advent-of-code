from __future__ import annotations

from collections import deque
from dataclasses import dataclass
from enum import IntEnum
from pathlib import Path

INPUT_FILE: str = "input.txt"
START: str = "S"
END: str = "E"


@dataclass(frozen=True)
class Coord:
    x: int
    y: int


class Direction(IntEnum):
    North = 0
    East = 1
    South = 2
    West = 3


class Heightmap:
    def __init__(self, heights: list[list[int]], start: Coord, end: Coord):
        self.heights = heights
        self.start = start
        self.end = end

    @property
    def height(self):
        return len(self.heights)

    @property
    def width(self):
        return len(self.heights[0])

    @classmethod
    def from_str(cls, s: str) -> Heightmap:
        start, end = Coord(0, 0), Coord(0, 0)
        heights = []
        for y, line in enumerate(s.splitlines()):
            row = []
            for x, char in enumerate(line):
                if char == START:
                    start = Coord(x=x, y=y)
                    height = ord("a")
                elif char == END:
                    end = Coord(x=x, y=y)
                    height = ord("z")
                else:
                    height = ord(char)
                row.append(height - 97)
            heights.append(row)
        return Heightmap(heights, start=start, end=end)

    def get(self, x: int, y: int) -> int:
        return self.heights[y][x]

    def reachable(self, coord: Coord, dir: Direction) -> bool:
        if dir == Direction.North:
            neigh = self.get(coord.x, coord.y - 1)
        elif dir == Direction.East:
            neigh = self.get(coord.x + 1, coord.y)
        elif dir == Direction.South:
            neigh = self.get(coord.x, coord.y + 1)
        elif dir == Direction.West:
            neigh = self.get(coord.x - 1, coord.y)
        if neigh <= self.get(coord.x, coord.y) + 1:
            return True
        return False


def bfs(start: Coord) -> int:
    start_ = {"coord": start, "steps": 0}
    visited = set()
    paths = deque([start_])

    while paths:
        current = paths.popleft()
        coord = current["coord"]

        if coord == heightmap.end:
            return current["steps"]

        if coord in visited:
            continue

        visited.add(coord)

        if coord.y > 0 and heightmap.reachable(coord, Direction.North):
            next_coord = Coord(x=coord.x, y=coord.y - 1)
            paths.append({"coord": next_coord, "steps": current["steps"] + 1})
        if coord.x < heightmap.width - 1 and heightmap.reachable(coord, Direction.East):
            next_coord = Coord(x=coord.x + 1, y=coord.y)
            paths.append({"coord": next_coord, "steps": current["steps"] + 1})
        if coord.y < heightmap.height - 1 and heightmap.reachable(coord, Direction.South):  # fmt: skip
            next_coord = Coord(x=coord.x, y=coord.y + 1)
            paths.append({"coord": next_coord, "steps": current["steps"] + 1})
        if coord.x > 0 and heightmap.reachable(coord, Direction.West):
            next_coord = Coord(x=coord.x - 1, y=coord.y)
            paths.append({"coord": next_coord, "steps": current["steps"] + 1})
    raise RuntimeError("No valid path found")


def part1(heightmap: Heightmap):
    min_steps = bfs(heightmap.start)
    print(f"Min steps from S to E: {min_steps}")


def part2(heightmap: Heightmap):
    min_steps = 1e9
    for y, row in enumerate(heightmap.heights):
        for x, height in enumerate(row):
            if height != 0:
                continue
            try:
                steps = bfs(Coord(x=x, y=y))
            except RuntimeError:
                continue
            if steps < min_steps:
                min_steps = steps
    print(f"Min steps from any 'a': {min_steps}")


if __name__ == "__main__":
    input_ = Path(INPUT_FILE).read_text()
    heightmap = Heightmap.from_str(input_)
    part1(heightmap)
    part2(heightmap)
