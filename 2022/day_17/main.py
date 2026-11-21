from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
from pathlib import Path

INPUT_FILE: str = "input.txt"
CHAMBER_WIDTH: int = 7
SPAWN_X: int = 2
SPAWN_Y_DELTA: int = 3
TOWER_HEIGHT_BUFFER: int = 7


@dataclass
class Coord:
    x: int
    y: int

    def offset(self, other: Coord) -> Coord:
        return Coord(
            x=self.x + other.x,
            y=self.y + other.y,
        )


HBAR: list[Coord] = [
    Coord(x=0, y=0),
    Coord(x=1, y=0),
    Coord(x=2, y=0),
    Coord(x=3, y=0),
]
PLUS: list[Coord] = [
    Coord(x=1, y=0),
    Coord(x=0, y=1),
    Coord(x=1, y=1),
    Coord(x=2, y=1),
    Coord(x=1, y=2),
]
LBAR: list[Coord] = [
    Coord(x=0, y=0),
    Coord(x=1, y=0),
    Coord(x=2, y=0),
    Coord(x=2, y=1),
    Coord(x=2, y=2),
]
VBAR: list[Coord] = [
    Coord(x=0, y=0),
    Coord(x=0, y=1),
    Coord(x=0, y=2),
    Coord(x=0, y=3),
]
BOX: list[Coord] = [
    Coord(x=0, y=0),
    Coord(x=1, y=0),
    Coord(x=0, y=1),
    Coord(x=1, y=1),
]


@dataclass
class Rock:
    pos: Coord
    blocks: list[Coord]

    def spawn(y: int, rock_idx: int) -> Rock:
        match rock_idx:
            case 0:
                blocks = HBAR
            case 1:
                blocks = PLUS
            case 2:
                blocks = LBAR
            case 3:
                blocks = VBAR
            case 4:
                blocks = BOX

        return Rock(
            pos=Coord(x=SPAWN_X, y=y),
            blocks=blocks,
        )

    def can_move_down(self, chamber: Chamber) -> bool:
        for block in self.blocks:
            coord = self.pos.offset(block)
            if coord.y == 0:
                return False
            if chamber.rows[coord.y - 1].blocks[coord.x] == Block.Rock:
                return False
        return True

    def move_down(self):
        self.pos.y -= 1

    def can_move_left(self, chamber: Chamber) -> bool:
        for block in self.blocks:
            coord = self.pos.offset(block)
            if coord.x == 0:
                return False
            if chamber.rows[coord.y].blocks[coord.x - 1] == Block.Rock:
                return False
        return True

    def move_left(self):
        self.pos.x -= 1

    def can_move_right(self, chamber: Chamber) -> bool:
        for block in self.blocks:
            coord = self.pos.offset(block)
            if coord.x >= CHAMBER_WIDTH - 1:
                return False
            if chamber.rows[coord.y].blocks[coord.x + 1] == Block.Rock:
                return False
        return True

    def move_right(self):
        self.pos.x += 1


class Block(Enum):
    Rock = 0
    Air = 1


class Stream(Enum):
    Left = 0
    Right = 1


def parse_streams(s: str) -> list[Stream]:
    streams = []
    for c in s:
        if c == "<":
            streams.append(Stream.Left)
        elif c == ">":
            streams.append(Stream.Right)
    return streams


def saturating_sub(a: int, b: int) -> int:
    return max(0, a - b)


@dataclass
class Chamber:
    rows: list[Row]

    def new() -> Chamber:
        return Chamber(rows=[])

    def add_row(self):
        self.rows.append(Row.new())

    def empty_top_rows(self) -> int:
        count = 0
        for i in reversed(
            range(saturating_sub(self.height, TOWER_HEIGHT_BUFFER), self.height)
        ):
            if not self.rows[i].has_rock():
                count += 1

        return count

    @property
    def height(self):
        return len(self.rows)

    @property
    def tower_height(self):
        return self.height - self.empty_top_rows()

    def buffer(self):
        if self.height < TOWER_HEIGHT_BUFFER:
            for _ in range(TOWER_HEIGHT_BUFFER - self.height):
                self.add_row()
        for _ in range(TOWER_HEIGHT_BUFFER - self.empty_top_rows()):
            self.add_row()

    def settle(self, rock: Rock):
        for block in rock.blocks:
            coord = rock.pos.offset(block)
            self.rows[coord.y].blocks[coord.x] = Block.Rock

    def simulate(
        self,
        streams: list[Stream],
        iters: int,
        start_stream_idx: int,
        start_rock_idx: int,
    ):
        stream_idx = start_stream_idx

        for i in range(iters):
            self.buffer()
            rock_idx = (start_rock_idx + i) % 5
            rock = Rock.spawn(self.tower_height + SPAWN_Y_DELTA, rock_idx)
            while True:
                # stream of air
                stream = streams[stream_idx]
                match stream:
                    case Stream.Left:
                        if rock.can_move_left(self):
                            rock.move_left()

                    case Stream.Right:
                        if rock.can_move_right(self):
                            rock.move_right()

                stream_idx += 1
                if stream_idx >= len(streams):
                    stream_idx = 0

                #  fall
                if not rock.can_move_down(self):
                    break
                rock.move_down()

            self.settle(rock)

    def surface_profile(self) -> tuple[int, ...]:
        top = self.tower_height
        profile = [0] * CHAMBER_WIDTH
        for col, _height in enumerate(profile):
            for row in reversed(range(top)):
                if self.rows[row].blocks[col] == Block.Rock:
                    profile[col] = top - row
                    break
        return tuple(profile)


@dataclass
class Row:
    blocks: list[Block]

    def new() -> Row:
        return Row(blocks=[Block.Air] * CHAMBER_WIDTH)

    def has_rock(self) -> bool:
        for b in self.blocks:
            if b == Block.Rock:
                return True
        return False


@dataclass
class LoopParams:
    # How many iterations a loop has
    iters: int
    # The height of every loop
    height: int
    # The rock idx at which the loop starts
    rock_idx: int
    # The stream idx at which the loop starts
    stream_idx: int


def find_loop_params(streams: list[Stream]) -> LoopParams:
    chamber = Chamber.new()
    stream_idx = 0
    visited = {}

    i = 0
    while True:
        chamber.buffer()
        rock_idx = i % 5
        rock = Rock.spawn(chamber.tower_height + SPAWN_Y_DELTA, rock_idx)
        while True:
            # stream of air
            stream = streams[stream_idx]
            match stream:
                case Stream.Left:
                    if rock.can_move_left(chamber):
                        rock.move_left()
                case Stream.Right:
                    if rock.can_move_right(chamber):
                        rock.move_right()

            stream_idx += 1
            if stream_idx >= len(streams):
                stream_idx = 0

            # fall
            if not rock.can_move_down(chamber):
                break
            rock.move_down()

        chamber.settle(rock)
        i += 1
        profile = chamber.surface_profile()
        key = (stream_idx, rock_idx % 5, profile)
        if key in visited:
            entry = visited[key]
            return LoopParams(
                iters=i - entry[0],
                height=chamber.tower_height - entry[2],
                rock_idx=entry[0],
                stream_idx=entry[1],
            )

        visited[key] = (i, stream_idx, chamber.tower_height)


def part1(streams: list[Stream]):
    rocks = 2022
    chamber = Chamber.new()
    chamber.simulate(streams, rocks, 0, 0)
    print(f"Height after {rocks} rocks: {chamber.tower_height}")


def part2(streams: list[Stream]):
    rocks = 1_000_000_000_000
    chamber = Chamber.new()
    loop_ = find_loop_params(streams)
    chamber.simulate(streams, loop_.rock_idx, 0, 0)
    rocks_after_prefix = rocks - loop_.rock_idx
    total_loops, remaining = divmod(rocks_after_prefix, loop_.iters)
    chamber.simulate(streams, remaining, loop_.stream_idx, loop_.rock_idx)
    total_height = chamber.tower_height + (total_loops * loop_.height)
    print(f"Height after {rocks} rocks: {total_height}")


def main():
    streams_input = Path(INPUT_FILE).read_text()
    streams = parse_streams(streams_input)
    part1(streams)
    part2(streams)


if __name__ == "__main__":
    main()
