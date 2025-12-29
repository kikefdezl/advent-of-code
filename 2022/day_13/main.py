from __future__ import annotations

import json
from dataclasses import dataclass
from enum import IntEnum
from functools import cmp_to_key
from pathlib import Path

INPUT_FILE: str = "input.txt"
DIVIDER_2 = [[2]]
DIVIDER_6 = [[6]]


class Result(IntEnum):
    Ordered = 0
    Unordered = 1
    Undefined = 2


def compare(a: int | list, b: int | list) -> Result:
    if isinstance(a, int) and isinstance(b, int):
        if a < b:
            return Result.Ordered
        if b < a:
            return Result.Unordered
        return Result.Undefined

    if isinstance(a, list) and isinstance(b, list):
        for ai, bi in zip(a, b):
            rez = compare(ai, bi)
            if rez in [Result.Ordered, Result.Unordered]:
                return rez
        if len(a) < len(b):
            return Result.Ordered
        if len(b) < len(a):
            return Result.Unordered
        return Result.Undefined

    if isinstance(a, int):
        return compare([a], b)
    if isinstance(b, int):
        return compare(a, [b])
    return Result.Undefined


def cmp_adapter(a: Packet, b: Packet) -> int:
    rez = compare(a.data, b.data)
    match rez:
        case Result.Ordered:
            return -1
        case Result.Unordered:
            return 1
        case Result.Undefined:
            return 0


@dataclass
class Packet:
    data: list[int | list]


def parse_packets(input_: str) -> list[Packet]:
    packets = []
    for line in input_.splitlines():
        if line:
            packets.append(Packet(json.loads(line)))
    return packets


def part1(packets: list[Packet]):
    sum_ = 0
    for i in range(0, len(packets), 2):
        rez = compare(packets[i].data, packets[i + 1].data)
        if rez == Result.Ordered:
            sum_ += (i // 2) + 1
    print(f"Sum of ordered idxs: {sum_}")


def part2(packets: list[Packet]):
    packets.append(Packet([[2]]))
    packets.append(Packet([[6]]))

    packets = sorted(packets, key=cmp_to_key(cmp_adapter))

    idx2, idx6 = 0, 0
    for i, packet in enumerate(packets, start=1):
        if packet.data == DIVIDER_2:
            idx2 = i
        if packet.data == DIVIDER_6:
            idx6 = i
    print(f"[[2]] & [[6]] indexes multiplied: {idx2 * idx6}")


if __name__ == "__main__":
    input_ = Path(INPUT_FILE).read_text()
    pairs = parse_packets(input_)
    part1(pairs)
    part2(pairs)
