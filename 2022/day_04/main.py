from dataclasses import dataclass
from pathlib import Path

INPUT_FILE: str = "input.txt"


@dataclass
class Assignment:
    start: int
    end: int


@dataclass
class Pair:
    a: Assignment
    b: Assignment

    def is_self_contained(self) -> bool:
        return (self.a.start >= self.b.start and self.a.end <= self.b.end) or (
            self.b.start >= self.a.start and self.b.end <= self.a.end
        )

    def overlaps(self) -> bool:
        return (
            (self.a.end >= self.b.start and self.a.end <= self.b.end)
            or (self.a.start <= self.b.end and self.a.start >= self.b.start)
            or self.is_self_contained()
        )


def parse_input(input: str) -> list[Pair]:
    pairs: list[Pair] = []
    for line in input.splitlines():
        parts = line.split(",")
        a_parts = parts[0].split("-")
        b_parts = parts[1].split("-")
        a_start = int(a_parts[0])
        a_end = int(a_parts[1])
        b_start = int(b_parts[0])
        b_end = int(b_parts[1])
        a = Assignment(start=a_start, end=a_end)
        b = Assignment(start=b_start, end=b_end)
        pairs.append(Pair(a=a, b=b))
    return pairs


def parts_1_and_2(pairs: list[Pair]):
    self_contained = 0
    overlapping = 0
    for pair in pairs:
        if pair.is_self_contained():
            self_contained += 1
        if pair.overlaps():
            overlapping += 1

    print(f"{self_contained} pairs are self contained")
    print(f"{overlapping} pairs overlap")


if __name__ == "__main__":
    input = Path(INPUT_FILE).read_text()
    pairs = parse_input(input)
    parts_1_and_2(pairs)
