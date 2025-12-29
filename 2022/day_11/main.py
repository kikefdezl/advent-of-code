from __future__ import annotations

from copy import deepcopy
from math import lcm
from pathlib import Path
from typing import Callable

INPUT_FILE: str = "input.txt"


def parse_op(s: str) -> Callable[[int], int]:
    """
    Options:
        new = old + N
        new = old * old
        new = old * N
    """
    s = s.strip()
    if s == "new = old * old":
        return lambda x: x * x
    if "*" in s:
        val = int(s[s.find("*") + 2 :])
        return lambda x: x * val
    if "+" in s:
        val = int(s[s.find("+") + 2 :])
        return lambda x: x + val
    raise ValueError(s)


class Monkey:
    def __init__(
        self,
        id: int,
        items: list[int],
        op: Callable[[int], int],
        test_divisible_by: int,
        next_if_true: int,
        next_if_false: int,
    ):
        self.id = id
        self.items = items
        self.operation = op
        self.test_divisible_by = test_divisible_by
        self.next_if_true = next_if_true
        self.next_if_false = next_if_false

        self.inspected = 0

    @classmethod
    def from_str(cls, s: str) -> Monkey:
        lines = s.splitlines()
        id_ = int(lines[0].removesuffix(":")[7:])
        items = [int(p) for p in lines[1][lines[1].find(":") + 1 :].split(",")]
        op = parse_op(lines[2][lines[2].find(":") + 1 :])
        test_divisible_by = int(lines[3][lines[3].find("by") + 3 :])
        next_if_true = int(lines[4][lines[4].find("monkey") + 7 :])
        next_if_false = int(lines[5][lines[5].find("monkey") + 7 :])
        return cls(
            id=id_,
            items=items,
            op=op,
            test_divisible_by=test_divisible_by,
            next_if_true=next_if_true,
            next_if_false=next_if_false,
        )

    def has_items(self) -> bool:
        return len(self.items) > 0

    def inspect_item(self) -> int:
        self.inspected += 1
        return self.items.pop(0)

    def throw_item(self, item: int):
        self.items.append(item)

    def test(self, item: int) -> bool:
        if item % self.test_divisible_by == 0:
            return True
        return False


def parse_monkeys(input_: str) -> list[Monkey]:
    monkeys = []
    lines = input_.splitlines()
    for i, line in enumerate(lines):
        if line.startswith("Monkey"):
            monkey = Monkey.from_str("\n".join(lines[i : i + 6]))
            monkeys.append(monkey)
    return monkeys


def simulate(
    monkeys: list[Monkey], rounds: int, drop_factor: int | None
) -> list[Monkey]:
    lcm_ = lcm(*[m.test_divisible_by for m in monkeys])

    monkeymap = {m.id: m for m in monkeys}
    for _ in range(rounds):
        for monkey_id in range(len(monkeymap)):
            monkey = monkeymap[monkey_id]
            while monkey.has_items():
                item = monkey.inspect_item()
                item = monkey.operation(item)
                if drop_factor:
                    item //= drop_factor
                else:
                    item %= lcm_
                test = monkey.test(item)
                if test:
                    monkeymap[monkey.next_if_true].throw_item(item)
                else:
                    monkeymap[monkey.next_if_false].throw_item(item)

    monkeys = sorted(monkeys, key=lambda m: m.inspected, reverse=True)
    return monkeys


def part1(monkeys: list[Monkey]):
    monkeys = simulate(monkeys, 20, 3)
    monkey_biz = monkeys[0].inspected * monkeys[1].inspected
    print(f"20 rounds of monkey shenanigans, drop factor 3: {monkey_biz}")


def part2(monkeys: list[Monkey]):
    monkeys = simulate(monkeys, 10_000, None)
    monkey_biz = monkeys[0].inspected * monkeys[1].inspected
    print(f"10k rounds of monkey shenanigans, no drop factor: {monkey_biz}")


if __name__ == "__main__":
    input_ = Path(INPUT_FILE).read_text()
    monkeys = parse_monkeys(input_)
    part1(deepcopy(monkeys))
    part2(monkeys)
