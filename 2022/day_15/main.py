from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path

INPUT_FILE: str = "input.txt"
ANALYZE_ROW: int = 2_000_000
SEARCH_SPACE: int = 4_000_000
FREQ_FACTOR_X: int = 4_000_000


@dataclass
class Coord:
    x: int
    y: int


@dataclass
class Beacon:
    coord: Coord


@dataclass
class Sensor:
    coord: Coord
    beacon: Beacon
    distance_to_beacon: int


def parse_sensors(input_: str) -> list[Sensor]:
    sensors = []
    for line in input_.splitlines():
        parts = line.split(":")
        sensor = parts[0].removeprefix("Sensor at ")
        beacon = parts[1].removeprefix(" closest beacon is at ")

        sensor_coords = sensor.split(", ")
        xs = int(sensor_coords[0][2:])
        ys = int(sensor_coords[1][2:])

        beacon_coords = beacon.split(", ")
        xb = int(beacon_coords[0][2:])
        yb = int(beacon_coords[1][2:])

        sensors.append(
            Sensor(
                coord=Coord(x=xs, y=ys),
                beacon=Beacon(coord=Coord(x=xb, y=yb)),
                distance_to_beacon=abs(xs - xb) + abs(ys - yb),
            )
        )
    return sensors


@dataclass
class Range:
    start: int
    end: int


@dataclass
class Ranges:
    ranges: list[Range]

    @classmethod
    def new(cls) -> Ranges:
        return cls([])

    def add(self, new: Range):
        i = 0

        while i < len(self.ranges) and self.ranges[i].end < new.start - 1:
            i += 1

        while i < len(self.ranges) and self.ranges[i].start <= new.end + 1:
            new.start = min(new.start, self.ranges[i].start)
            new.end = max(new.end, self.ranges[i].end)
            self.ranges.pop(i)

        self.ranges.insert(i, new)


def find_impossible_beacons(sensors: list[Sensor], y: int) -> Ranges:
    ranges = Ranges.new()
    for sensor in sensors:
        y_diff = abs(sensor.coord.y - y)
        if y_diff > sensor.distance_to_beacon:
            continue
        x_spread = sensor.distance_to_beacon - y_diff
        range_ = Range(
            start=sensor.coord.x - x_spread,
            end=sensor.coord.x + x_spread,
        )
        ranges.add(range_)
    return ranges


def part1(sensors: list[Sensor]):
    ranges = find_impossible_beacons(sensors, ANALYZE_ROW)

    count = 0
    for rng in ranges.ranges:
        count += rng.end - rng.start + 1

    beacons_at_y = set()
    for sensor in sensors:
        if sensor.beacon.coord.y == ANALYZE_ROW:
            beacons_at_y.add(sensor.beacon.coord.x)
    count -= len(beacons_at_y)
    print(f"At {ANALYZE_ROW}, {count} tiles can't have a beacon.")


def part2(sensors: list[Sensor]):
    freq = 0
    for y in range(SEARCH_SPACE):
        ranges = find_impossible_beacons(sensors, y)
        if len(ranges.ranges) > 1:
            freq = (FREQ_FACTOR_X * (ranges.ranges[0].end + 1)) + y
            break
    print(f"Tuning frequency: {freq}")


if __name__ == "__main__":
    input_ = Path(INPUT_FILE).read_text()
    sensors = parse_sensors(input_)
    part1(sensors)
    part2(sensors)
