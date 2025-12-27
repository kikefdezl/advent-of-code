from pathlib import Path

INPUT_FILE: str = "input.txt"
SAMPLE_AT = [20, 60, 100, 140, 180, 220]


def part1(input_: str) -> list[int]:
    lines = input_.splitlines()
    cycles = [0] * len(lines) * 2
    idx = 0
    for line in lines:
        if line.startswith("addx"):
            cycles[idx + 1] += int(line[5:])
            idx += 2
        elif line.startswith("noop"):
            idx += 1
        else:
            raise ValueError(line)

    while cycles[-1] == 0:
        cycles.pop(-1)

    X, sum_ = 1, 0
    for i, cycle in enumerate(cycles, start=1):
        if i in SAMPLE_AT:
            sum_ += X * i
        X += cycle

    print(f"Sum of signal strengths: {sum_}")
    return cycles


def covers(sprite: int, pixel: int) -> bool:
    if sprite >= pixel - 1 and sprite <= pixel + 1:
        return True
    return False


def part2(cycles: list[int]):
    sprite = 1
    for i, cycle in enumerate(cycles):
        if covers(sprite, i % 40):
            print("#", end="")
        else:
            print(".", end="")
        if (i + 1) % 40 == 0:
            print()
        sprite += cycle
    print()


if __name__ == "__main__":
    input_ = Path(INPUT_FILE).read_text()
    cycles = part1(input_)
    part2(cycles)
