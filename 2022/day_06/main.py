from pathlib import Path

INPUT_FILE: str = "input.txt"


def find_start_marker(stream: str, marker_len: int) -> int:
    for i in range(marker_len - 1, len(stream)):
        if len(set(stream[i - marker_len : i])) == marker_len:
            return i
    return -1


def part1(input_: str):
    packet_idx = find_start_marker(input_, marker_len=4)
    print(f"Packet starts at {packet_idx}")


def part2(input_: str):
    msg_idx = find_start_marker(input_, marker_len=14)
    print(f"Message starts at {msg_idx}")


if __name__ == "__main__":
    input_ = Path(INPUT_FILE).read_text()
    part1(input_)
    part2(input_)
