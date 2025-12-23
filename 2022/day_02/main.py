from __future__ import annotations

from enum import IntEnum
from pathlib import Path

INPUT_FILE: str = "input.txt"


class Shape(IntEnum):
    ROCK = 1
    PAPER = 2
    SCISSORS = 3

    @classmethod
    def from_str(cls, s: str) -> Shape:
        map_ = {
            "A": cls.ROCK,
            "X": cls.ROCK,
            "B": cls.PAPER,
            "Y": cls.PAPER,
            "C": cls.SCISSORS,
            "Z": cls.SCISSORS,
        }
        return map_[s]

    def points(self) -> int:
        return self

    def play(self, other: Shape) -> Result:
        if self == other:
            return Result.DRAW
        wins = {
            Shape.ROCK: Shape.SCISSORS,
            Shape.PAPER: Shape.ROCK,
            Shape.SCISSORS: Shape.PAPER,
        }
        return Result.WIN if wins[self] == other else Result.LOSS

    def for_result(self, result: Result) -> Shape:
        if result == Result.DRAW:
            return self
        if result == Result.WIN:
            return {
                Shape.ROCK: Shape.PAPER,
                Shape.PAPER: Shape.SCISSORS,
                Shape.SCISSORS: Shape.ROCK,
            }[self]
        return {
            Shape.ROCK: Shape.SCISSORS,
            Shape.PAPER: Shape.ROCK,
            Shape.SCISSORS: Shape.PAPER,
        }[self]


class Result(IntEnum):
    WIN = 6
    DRAW = 3
    LOSS = 0

    @classmethod
    def from_str(cls, s: str) -> Result:
        map_ = {"X": cls.LOSS, "Y": cls.DRAW, "Z": cls.WIN}
        return map_[s]

    def points(self) -> int:
        return self


def part1(input: str):
    points = 0
    for line in input.splitlines():
        him = Shape.from_str(line[0])
        you = Shape.from_str(line[2])
        res = you.play(him)
        points += you.points() + res.points()
    print(f"Total points: {points}")


def part2(input: str):
    points = 0
    for line in input.splitlines():
        him = Shape.from_str(line[0])
        res = Result.from_str(line[2])
        you = him.for_result(res)
        points += you.points() + res.points()
    print(f"Total points after following correct instructions: {points}")


if __name__ == "__main__":
    input = Path(INPUT_FILE).read_text()
    part1(input)
    part2(input)
