from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path

INPUT_FILE: str = "input.txt"


@dataclass(frozen=True)
class Cube:
    x: int
    y: int
    z: int

    def neighbors(self) -> list[Cube]:
        neighbors = []
        neighbors.append(Cube(x=self.x - 1, y=self.y, z=self.z))
        neighbors.append(Cube(x=self.x + 1, y=self.y, z=self.z))
        neighbors.append(Cube(x=self.x, y=self.y - 1, z=self.z))
        neighbors.append(Cube(x=self.x, y=self.y + 1, z=self.z))
        neighbors.append(Cube(x=self.x, y=self.y, z=self.z - 1))
        neighbors.append(Cube(x=self.x, y=self.y, z=self.z + 1))
        return neighbors


def parse_cubes(s: str) -> list[Cube]:
    cubes = []
    for line in s.splitlines():
        parts = line.split(",")
        cubes.append(
            Cube(
                x=int(parts[0]),
                y=int(parts[1]),
                z=int(parts[2]),
            )
        )
    return cubes


def area(cubes: list[Cube]) -> int:
    occupied = set(cubes)
    count = 0
    for cube in cubes:
        neighbors = cube.neighbors()
        for neighbor in neighbors:
            if neighbor not in occupied:
                count += 1
    return count


@dataclass
class Space:
    x_min: int
    x_max: int
    y_min: int
    y_max: int
    z_min: int
    z_max: int

    @classmethod
    def from_cubes(cls, cubes: list[Cube]) -> Space:
        x_min, x_max = int(1e9), int(-1e9)
        y_min, y_max = int(1e9), int(-1e9)
        z_min, z_max = int(1e9), int(-1e9)

        for cube in cubes:
            x_min = min(x_min, cube.x)
            x_max = max(x_max, cube.x)
            y_min = min(y_min, cube.y)
            y_max = max(y_max, cube.y)
            z_min = min(z_min, cube.z)
            z_max = max(z_max, cube.z)

        return cls(x_min, x_max, y_min, y_max, z_min, z_max)

    def buffer(self) -> Space:
        return Space(
            x_min=self.x_min - 1,
            x_max=self.x_max + 1,
            y_min=self.y_min - 1,
            y_max=self.y_max + 1,
            z_min=self.z_min - 1,
            z_max=self.z_max + 1,
        )

    def flood(self, start: Cube, cubes: list[Cube]) -> list[Cube]:
        occupied = {c for c in cubes}

        flooded = set()
        to_propagate = {start}

        while to_propagate:
            cube = to_propagate.pop()

            neighbors = cube.neighbors()
            neighbors = [n for n in neighbors if not self.out_of_bounds(n)]
            for neighbor in neighbors:
                if (
                    neighbor in flooded
                    or neighbor in to_propagate
                    or neighbor in occupied
                ):
                    continue
                to_propagate.add(neighbor)

            flooded.add(cube)
        return list(flooded)

    def out_of_bounds(self, cube: Cube) -> bool:
        return (
            cube.x < self.x_min
            or cube.x > self.x_max
            or cube.y < self.y_min
            or cube.y > self.y_max
            or cube.z < self.z_min
            or cube.z > self.z_max
        )

    def boundary_area(self) -> int:
        x = self.x_max - self.x_min + 1
        y = self.y_max - self.y_min + 1
        z = self.z_max - self.z_min + 1
        return 2 * (x * y + x * z + y * z)


def part1(cubes: list[Cube]):
    total_area = area(cubes)
    print(f"{total_area} total area")


def part2(cubes: list[Cube]):
    space = Space.from_cubes(cubes).buffer()
    start = Cube(space.x_min, space.y_min, space.z_min)
    flooded = space.flood(start, cubes)

    total_area = area(flooded) - space.boundary_area()
    print(f"{total_area} external cube sides")


def main():
    cubes = parse_cubes(Path(INPUT_FILE).read_text())
    part1(cubes)
    part2(cubes)


if __name__ == "__main__":
    main()
