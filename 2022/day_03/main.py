from pathlib import Path

INPUT_FILE: str = "input.txt"


def priority_value(item: str) -> int:
    priority = ord(item)
    if 65 <= priority and priority <= 90:
        priority -= 38
    elif 97 <= priority and priority <= 122:
        priority -= 96
    else:
        raise ValueError(item)
    return priority


def part1(input: str):
    sum_ = 0
    for line in input.splitlines():
        split_idx = len(line) // 2
        first = line[:split_idx]
        second = line[split_idx:]

        for item in first:
            if item in second:
                sum_ += priority_value(item)
                break
    print(f"Priority sum of double items: {sum_}")


def part2(input: str):
    lines = input.splitlines()
    sum_ = 0
    for i in range(0, len(lines), 3):
        for item in lines[i]:
            if item in lines[i + 1] and item in lines[i + 2]:
                sum_ += priority_value(item)
                break
    print(f"Priority sum of badges: {sum_}")


if __name__ == "__main__":
    input = Path(INPUT_FILE).read_text()
    part1(input)
    part2(input)
