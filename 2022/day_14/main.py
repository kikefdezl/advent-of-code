from __future__ import annotations

from pathlib import Path

INPUT_FILE: str = "input.txt"

AIR: str = "."
ROCK: str = "#"
SAND: str = "o"

SOURCE_X: int = 500


def make_segment(pt_a: tuple[int, int], pt_b: tuple[int, int]) -> list[tuple[int, int]]:
    if pt_a[0] == pt_b[0]:
        min_y = min(pt_a[1], pt_b[1])
        max_y = max(pt_a[1], pt_b[1])
        return [(pt_a[0], y) for y in range(min_y, max_y + 1)]
    elif pt_a[1] == pt_b[1]:
        min_x = min(pt_a[0], pt_b[0])
        max_x = max(pt_a[0], pt_b[0])
        return [(x, pt_a[1]) for x in range(min_x, max_x + 1)]
    raise ValueError((pt_a, pt_b))


class Cave:
    def __init__(self, tiles: list[list[str]]):
        self.tiles = tiles
        self.source = (SOURCE_X, 0)

        self.sand_units = 0

    @classmethod
    def new_empty(cls):
        tiles = [[AIR] * SOURCE_X]
        return cls(tiles)

    @property
    def width(self) -> int:
        return len(self.tiles[0])

    @property
    def height(self) -> int:
        return len(self.tiles)

    def expand_right(self, amt: int):
        for row in self.tiles:
            row.extend([AIR] * amt)

    def expand_left(self, amt: int):
        for row in self.tiles:
            for _ in range(amt):
                row.insert(0, AIR)
        self.source = (self.source[0] + amt, 0)

    def expand_down(self, amt: int):
        for _ in range(amt):
            self.tiles.append([AIR] * self.width)

    @classmethod
    def from_input(cls, input_: str) -> Cave:
        cave = Cave.new_empty()
        for line in input_.splitlines():
            parts = line.split(" -> ")
            coords_str = [p.split(",") for p in parts]
            coords = [(int(c[0]), int(c[1])) for c in coords_str]

            # expand the cave if any coord can't fit
            for coord in coords:
                if coord[0] > cave.width - 1:
                    cave.expand_right(coord[0] - cave.width + 1)
                if coord[1] > cave.height - 1:
                    cave.expand_down(coord[1] - cave.height + 1)

            # populate
            for i in range(len(coords) - 1):
                pts = make_segment(coords[i], coords[i + 1])
                for pt in pts:
                    cave.tiles[pt[1]][pt[0]] = ROCK
        cave.trim()
        return cave

    def trim(self):  # useful for visualization to fit in the terminal
        min_x = 10000  # min y is always 0
        max_x, max_y = 0, 0

        for y, row in enumerate(self.tiles):
            for x, tile in enumerate(row):
                if tile == ROCK:
                    if x < min_x:
                        min_x = x
                    if x > max_x:
                        max_x = x
                    if y > max_y:
                        max_y = y

        new_tiles = []
        for y in range(0, max_y + 1):
            new_row = []
            for x in range(min_x, max_x + 1):
                new_row.append(self.tiles[y][x])
            new_tiles.append(new_row)

        self.tiles = new_tiles
        self.source = (self.source[0] - min_x, 0)

    def print(self):
        for row in self.tiles:
            print("".join(row))
        print(f"Source: {self.source}")

    def get(self, x: int, y: int) -> str:
        return self.tiles[y][x]

    def drop_one(self):
        pos = self.source

        while True:
            if pos[1] >= self.height - 1:  # fell below
                return
            elif self.get(pos[0], pos[1] + 1) == AIR:
                pos = (pos[0], pos[1] + 1)
            elif pos[0] == 0:  # fell left
                return
            elif self.get(pos[0] - 1, pos[1] + 1) == AIR:
                pos = (pos[0] - 1, pos[1] + 1)
            elif pos[0] >= self.width - 1:  # fell right
                return
            elif self.get(pos[0] + 1, pos[1] + 1) == AIR:
                pos = (pos[0] + 1, pos[1] + 1)
            elif self.get(pos[0], pos[1]) == SAND:  # cave is full
                return
            else:  # place the sand
                self.tiles[pos[1]][pos[0]] = SAND
                self.sand_units += 1
                return

    def add_floor(self):
        self.expand_down(2)
        pyramid_width = (2 * self.height) + 1
        self.expand_right(pyramid_width // 2 - (self.width - self.source[0]))
        self.expand_left(pyramid_width // 2 - self.source[0])

        for i in range(self.width):
            self.tiles[-1][i] = ROCK


def part1(cave: Cave):
    prev = -1
    while cave.sand_units > prev:
        prev = cave.sand_units
        cave.drop_one()
    cave.print()
    print(f"Cave holds {cave.sand_units} units of sand")


def part2(cave: Cave):
    cave.add_floor()

    prev = -1
    while cave.sand_units > prev:
        prev = cave.sand_units
        cave.drop_one()
    print(f"With floor, it holds {cave.sand_units} units")


if __name__ == "__main__":
    input_ = Path(INPUT_FILE).read_text()
    cave = Cave.from_input(input_)
    part1(cave)
    part2(cave)
