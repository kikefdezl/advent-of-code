from pathlib import Path

INPUT_FILE: str = "input.txt"


class Forest:
    def __init__(self, trees: list[list[int]]) -> None:
        self.trees: list[list[int]] = trees

    @classmethod
    def from_str(cls, s: str):
        trees = []
        for line in s.splitlines():
            row = []
            for char in line:
                row.append(int(char))
            trees.append(row)
        return cls(trees)

    @property
    def height(self):
        return len(self.trees)

    @property
    def width(self):
        return len(self.trees[0])

    def count_visible(self):
        count = 0
        for y in range(self.height):
            for x in range(self.width):
                if self.is_visible(x, y):
                    count += 1
        return count

    def is_visible(self, x: int, y: int) -> bool:
        if x == 0 or y == 0 or x >= self.width - 1 or y >= self.height - 1:
            return True

        tree = self.trees[y][x]

        if all([self.trees[_y][x] < tree for _y in range(0, y)]):
            return True
        if all([self.trees[y][_x] < tree for _x in range(0, x)]):
            return True
        if all([self.trees[_y][x] < tree for _y in range(y + 1, self.height)]):
            return True
        if all([self.trees[y][_x] < tree for _x in range(x + 1, self.width)]):
            return True
        return False

    def best_scenic_score(self) -> int:
        best = 0
        for y in range(self.height):
            for x in range(self.width):
                score = self.scenic_score(x, y)
                if score > best:
                    best = score
        return best

    def scenic_score(self, x: int, y: int) -> int:
        if x == 0 or y == 0 or x >= self.width - 1 or y >= self.height - 1:
            return 0

        tree = self.trees[y][x]

        north, east, south, west = 1, 1, 1, 1
        while y - north > 0 and self.trees[y - north][x] < tree:
            north += 1
        while x - west > 0 and self.trees[y][x - west] < tree:
            west += 1
        while y + south < self.height - 1 and self.trees[y + south][x] < tree:
            south += 1
        while x + east < self.width - 1 and self.trees[y][x + east] < tree:
            east += 1
        return north * east * south * west


def part1(forest: Forest):
    visible = forest.count_visible()
    print(f"There are {visible} visible trees")


def part2(forest: Forest):
    score = forest.best_scenic_score()
    print(f"Best scenic score is {score}")


if __name__ == "__main__":
    input_ = Path(INPUT_FILE).read_text()
    forest = Forest.from_str(input_)
    part1(forest)
    part2(forest)
