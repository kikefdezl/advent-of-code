from pathlib import Path

INPUT_FILE: str = "input.txt"


def insert_sorted(elements: list[int], new: int):
    idx = 0
    while elements[idx] > new:
        idx += 1

    for i in reversed(range(idx + 1, len(elements))):
        elements[i] = elements[i - 1]
    elements[idx] = new


def parts_1_and_2(text: str):
    lines = text.splitlines()

    max_ = [0, 0, 0]
    curr = 0
    for line in lines:
        if len(line) == 0:
            if curr > max_[-1]:
                insert_sorted(max_, curr)
            curr = 0
        else:
            curr += int(line)

    print(f"Top 3 max calories: {max_} = {sum(max_)}")


if __name__ == "__main__":
    text = Path(INPUT_FILE).read_text()
    parts_1_and_2(text)
