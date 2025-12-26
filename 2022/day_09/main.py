from dataclasses import dataclass
from enum import IntEnum
from pathlib import Path

INPUT_FILE: str = "input.txt"


class Direction(IntEnum):
    Up = 0
    Right = 1
    Down = 2
    Left = 3


@dataclass
class Command:
    count: int
    direction: Direction


def parse_commands(s: str) -> list[Command]:
    commands = []
    for line in s.splitlines():
        dir_ = line[0]
        if dir_ == "U":
            direction = Direction.Up
        elif dir_ == "R":
            direction = Direction.Right
        elif dir_ == "D":
            direction = Direction.Down
        elif dir_ == "L":
            direction = Direction.Left
        else:
            raise ValueError(dir_)
        count = int(line[2:])
        commands.append(Command(count=count, direction=direction))
    return commands


@dataclass(frozen=True)
class Coord:
    x: int
    y: int


class Rope:
    def __init__(self, knots: int = 2):
        assert knots >= 2

        self.knots = [Coord(0, 0) for _ in range(knots)]
        self.visited = set()

    def move(self, direction: Direction):
        if direction == Direction.Up:
            self.knots[0] = Coord(x=self.knots[0].x, y=self.knots[0].y - 1)
        if direction == Direction.Right:
            self.knots[0] = Coord(x=self.knots[0].x + 1, y=self.knots[0].y)
        if direction == Direction.Down:
            self.knots[0] = Coord(x=self.knots[0].x, y=self.knots[0].y + 1)
        if direction == Direction.Left:
            self.knots[0] = Coord(x=self.knots[0].x - 1, y=self.knots[0].y)
        self.resolve()
        self.visited.add(self.knots[-1])

    def resolve(self):
        for i in range(1, len(self.knots)):
            dx = self.knots[i - 1].x - self.knots[i].x
            dx = 0 if dx == 0 else (1 if dx > 0 else -1)
            dy = self.knots[i - 1].y - self.knots[i].y
            dy = 0 if dy == 0 else (1 if dy > 0 else -1)
            if self.knots[i].y - self.knots[i - 1].y > 1:  # up
                self.knots[i] = Coord(x=self.knots[i].x + dx, y=self.knots[i - 1].y + 1)
            if self.knots[i].x - self.knots[i - 1].x > 1:  # left
                self.knots[i] = Coord(x=self.knots[i - 1].x + 1, y=self.knots[i].y + dy)
            if self.knots[i - 1].y - self.knots[i].y > 1:  # down
                self.knots[i] = Coord(x=self.knots[i].x + dx, y=self.knots[i - 1].y - 1)
            if self.knots[i - 1].x - self.knots[i].x > 1:  # right
                self.knots[i] = Coord(x=self.knots[i - 1].x - 1, y=self.knots[i].y + dy)


def parts_1_and_2(commands: list[Command]):
    for knots in [2, 10]:
        rope = Rope(knots=knots)
        for cmd in commands:
            for _ in range(cmd.count):
                rope.move(cmd.direction)
        print(f"{knots} knot tail visited {len(rope.visited)} unique locations")


if __name__ == "__main__":
    input_ = Path(INPUT_FILE).read_text()
    commands = parse_commands(input_)
    parts_1_and_2(commands)
