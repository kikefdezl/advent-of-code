from copy import deepcopy
from dataclasses import dataclass
from pathlib import Path

INPUT_FILE: str = "input.txt"


@dataclass
class Stack:
    data: list[str]


@dataclass
class Move:
    amount: int
    from_: int
    to: int


def parse_input(input_: str) -> tuple[list[Stack], list[Move]]:
    lines = input_.splitlines()
    start_idx = lines.index("") - 1
    stacks = [Stack([]) for _ in lines[start_idx].split()]
    for line in reversed(lines[:start_idx]):
        for i, char in enumerate(line):
            if char == "[":
                stack_id = i // 4
                stacks[stack_id].data.append(line[i + 1])

    moves: list[Move] = []
    for line in lines[start_idx + 2 :]:
        parts = line.split()
        move = Move(
            amount=int(parts[1]),
            from_=int(parts[3]) - 1,
            to=int(parts[5]) - 1,
        )
        moves.append(move)

    return stacks, moves


def part1(stacks: list[Stack], moves: list[Move]):
    for move in moves:
        for _ in range(move.amount):
            element = stacks[move.from_].data.pop()
            stacks[move.to].data.append(element)
    print("Crates on top:", end=" ")
    for stack in stacks:
        print(stack.data[-1], end="")
    print()


def part2(stacks: list[Stack], moves: list[Move]):
    for move in moves:
        popped = stacks[move.from_].data[-move.amount :]
        stacks[move.to].data.extend(popped)
        stacks[move.from_].data = stacks[move.from_].data[: -move.amount]
    print("Crates on top with CrateMover 9001:", end=" ")
    for stack in stacks:
        print(stack.data[-1], end="")
    print()


if __name__ == "__main__":
    input_ = Path(INPUT_FILE).read_text()
    stacks, moves = parse_input(input_)
    part1(deepcopy(stacks), moves)
    part2(stacks, moves)
