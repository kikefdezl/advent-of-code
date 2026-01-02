from __future__ import annotations

from collections import deque
from dataclasses import dataclass
from pathlib import Path

INPUT_FILE: str = "input.txt"
MINUTES: int = 30
MINS_FOR_ELEPHANT: int = 4
START: str = "AA"


@dataclass(frozen=True)
class Valve:
    id: str
    flow_rate: int
    leads_to: tuple[str, ...]


def parse_valves(input_: str) -> list[Valve]:
    valves = []
    for line in input_.splitlines():
        words = line.split(" ")
        id_ = words[1]
        flow_rate = int(words[4].removeprefix("rate=").removesuffix(";"))
        leads_to = words[9:]
        leads_to = tuple([lt.removesuffix(",") for lt in leads_to])
        valves.append(Valve(id=id_, flow_rate=flow_rate, leads_to=leads_to))
    return valves


@dataclass(frozen=True)
class State:
    valve: str
    opened: int  # bitmask
    minutes_left: int
    pressure: int

    def to_key(self) -> tuple:
        return self.valve, self.opened, self.pressure


def precompute_distances(
    valvemap: dict[str, Valve], useful: list[str]
) -> dict[str, dict[str, int]]:
    distances = {}
    for v in useful + [START]:
        dist = {v: 0}
        q = deque([v])
        while q:
            curr = q.popleft()
            for adj in valvemap[curr].leads_to:
                if adj not in dist:
                    dist[adj] = dist[curr] + 1
                    q.append(adj)
        distances[v] = dist
    return distances


def find_paths(valves: list[Valve], minutes: int) -> dict[int, int]:
    """
    BFS exploration of all possible action sequences.
    Uses precomputed distances to jump directly to useful valves.
    Returns {opened_bitmask: pressure}
    """
    valvemap = {v.id: v for v in valves}
    useful = [v for v in valves if v.flow_rate > 0]
    index = {v.id: i for i, v in enumerate(useful)}
    dist_map = precompute_distances(valvemap, [v.id for v in useful])

    start = State(valve=START, opened=0, minutes_left=minutes, pressure=0)
    states = deque([start])
    paths: dict[int, int] = {}
    checked = set()

    while states:
        state = states.popleft()
        if state.minutes_left <= 0:
            continue

        key = state.to_key()
        if key in checked:
            continue
        checked.add(key)

        if state.opened not in paths or state.pressure > paths[state.opened]:
            paths[state.opened] = state.pressure

        # consider moving to any unopened useful valve
        for target in useful:
            bit = 1 << index[target.id]
            if state.opened & bit:
                continue

            travel = dist_map[state.valve][target.id] + 1  # +1 to open it
            if state.minutes_left <= travel:
                continue

            new_state = State(
                valve=target.id,
                opened=state.opened | bit,
                minutes_left=state.minutes_left - travel,
                pressure=state.pressure
                + target.flow_rate * (state.minutes_left - travel),
            )
            states.append(new_state)

    return paths


def best_disjoint_sum(paths: list[tuple[int, int]]) -> int:
    best_total = 0
    for i in range(len(paths) - 1):
        for j in range(i + 1, len(paths)):
            if paths[i][0] & paths[j][0] == 0:
                total = paths[i][1] + paths[j][1]
                if total > best_total:
                    best_total = total
    return best_total


def part1(valves: list[Valve]):
    paths = find_paths(valves, MINUTES)
    best = max([p for p in paths.values()])
    print(f"Max pressure alone and {MINUTES} mins: {best}")


def part2(valves: list[Valve]):
    paths = find_paths(valves, MINUTES - MINS_FOR_ELEPHANT)
    best = best_disjoint_sum(list(paths.items()))
    print(f"With elephant and {MINUTES - MINS_FOR_ELEPHANT} mins: {best}")


if __name__ == "__main__":
    input_ = Path(INPUT_FILE).read_text()
    valves = parse_valves(input_)
    part1(valves)
    part2(valves)
