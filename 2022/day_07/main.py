from __future__ import annotations

from pathlib import Path

INPUT_FILE: str = "input.txt"
MAX: int = 100_000
TOTAL: int = 70_000_000
REQUIRED: int = 30_000_000


class File:
    def __init__(self, name: str, size: int):
        self.name = name
        self.size = size

    def __str__(self) -> str:
        return f"- {self.name} (file, size={str(self.size)})"


class Directory:
    def __init__(self, name: str, files: dict[str, File] | None = None):
        self.name = name
        self.files = files or {}

    def add_file(self, file: File):
        self.files[file.name] = file

    def __str__(self) -> str:
        return f"- {self.name} (dir)"

    def size(self) -> int:
        return sum([f.size for f in self.files.values()])


class FileSystem:
    def __init__(self):
        self.root = Directory("/")
        self.dirs: dict[str, Directory] = {"/": self.root}
        self.cwd = "/"

    def populate(self, terminal_out: str):
        for line in terminal_out.splitlines():
            if line.startswith("$ cd"):
                self.cd(line[5:])
            elif line.startswith("dir"):
                self.mkdir(line[4:])
            elif line[0].isnumeric():
                parts = line.split()
                size = int(parts[0])
                name = parts[1]
                file = File(name=name, size=size)
                self.touch(file)
            elif line.startswith("$ ls"):
                pass
            else:
                raise ValueError(line)

    def cd(self, cmd: str):
        if cmd.startswith("/"):
            self.cwd = cmd
        elif cmd == "..":
            self.cwd = "/".join(self.cwd.split("/")[:-2]) + "/"
        else:
            self.cwd = f"{self.cwd}{cmd}/"

    def mkdir(self, name: str):
        abs_path = f"{self.cwd}{name}/"
        d = self.dirs.get(abs_path)
        if d is None:
            self.dirs[abs_path] = Directory(name)

    def touch(self, file: File):
        dir_ = self.dirs[self.cwd]
        dir_.add_file(file)
        self.dirs[self.cwd] = dir_

    def print(self, wd: str, indent: int = 0):
        curr = self.dirs[wd]
        prefix = " " * indent
        print(f"{prefix}{str(curr)}")

        subdirs = self.children(wd)
        for subdir in subdirs:
            self.print(subdir, indent + 4)

        for file in curr.files.values():
            print(f"{prefix}    {str(file)}")

    def children(self, wd: str) -> list[str]:
        dirs: set[str] = set()
        for path in self.dirs.keys():
            if not path.startswith(wd):
                continue
            relative = path.removeprefix(wd)
            if not relative:
                continue
            parts = relative.split("/")
            dirs.add(f"{wd}{parts[0]}/")
        return list(dirs)

    def sizeof(self, wd: str) -> int:
        curr = self.dirs[wd]

        total = curr.size()
        for child in self.children(wd):
            total += self.sizeof(child)
        return total


def part1(fs: FileSystem):
    sum_ = 0
    for dir_ in fs.dirs:
        size = fs.sizeof(dir_)
        if size <= MAX:
            sum_ += size
    print(f"Sum of sizes of dirs under {MAX}: {sum_}")


def part2(fs: FileSystem):
    used = fs.sizeof("/")
    to_free = used - (TOTAL - REQUIRED)

    best = 1e9
    for dir_ in fs.dirs:
        size = fs.sizeof(dir_)
        if size >= to_free and size < best:
            best = size
    print(f"Best directory to delete has size: {best}")


if __name__ == "__main__":
    input_ = Path(INPUT_FILE).read_text()
    fs = FileSystem()
    fs.populate(input_)
    fs.print("/", 0)
    part1(fs)
    part2(fs)
