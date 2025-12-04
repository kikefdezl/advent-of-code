package main

import (
	"fmt"
	"os"
	"strings"
)

const (
	InputFile   = "input.txt"
	MaxAdjacent = 3
)

type Cell byte

const (
	Empty Cell = '.'
	Roll  Cell = '@'
)

type Grid struct {
	cells [][]Cell
}

func parseGrid(input string) (Grid, error) {
	trimmed := strings.TrimSpace(input)
	lines := strings.Split(trimmed, "\n")

	rows := make([][]Cell, 0, len(lines))
	for _, line := range lines {
		row := make([]Cell, 0, len(line))
		for _, char := range line {
			switch char {
			case '.':
				row = append(row, Empty)
			case '@':
				row = append(row, Roll)
			default:
				return Grid{}, fmt.Errorf("unknown char %d when parsing", char)
			}
		}
		rows = append(rows, row)
	}
	return Grid{cells: rows}, nil
}

func (g *Grid) countAdjacentAt(x int, y int) int {
	adjacents := 0
	maxY := len(g.cells) - 1
	maxX := len(g.cells[0]) - 1

	// NW
	if x > 0 && y > 0 {
		if g.cells[y-1][x-1] == Roll {
			adjacents += 1
		}
	}

	// N
	if y > 0 {
		if g.cells[y-1][x] == Roll {
			adjacents += 1
		}
	}

	// NE
	if y > 0 && x < maxX {
		if g.cells[y-1][x+1] == Roll {
			adjacents += 1
		}
	}

	// E
	if x < maxX {
		if g.cells[y][x+1] == Roll {
			adjacents += 1
		}
	}

	// SE
	if y < maxY && x < maxX {
		if g.cells[y+1][x+1] == Roll {
			adjacents += 1
		}
	}

	// S
	if y < maxY {
		if g.cells[y+1][x] == Roll {
			adjacents += 1
		}
	}

	// SW
	if y < maxY && x > 0 {
		if g.cells[y+1][x-1] == Roll {
			adjacents += 1
		}
	}

	// W
	if x > 0 {
		if g.cells[y][x-1] == Roll {
			adjacents += 1
		}
	}
	return adjacents
}

func (g *Grid) removeAccessibleRolls() int {
	removed := 0
	for y, row := range g.cells {
		for x, cell := range row {
			if cell == Roll && g.countAdjacentAt(x, y) <= MaxAdjacent {
				g.cells[y][x] = Empty
				removed += 1
			}
		}
	}
	return removed
}

func part1(grid *Grid) {
	locations := 0
	for y, row := range grid.cells {
		for x, cell := range row {
			if cell == Roll && grid.countAdjacentAt(x, y) <= MaxAdjacent {
				locations += 1
			}
		}
	}
	fmt.Println(locations, "roll locations are accessible")
}

func part2(grid *Grid) {
	sum := 0
	removed := 1

	for removed > 0 {
		removed = grid.removeAccessibleRolls()
		sum += removed
	}

	fmt.Println(sum, "rolls can be removed iteratively")
}

func main() {
	input, err := os.ReadFile(InputFile)
	if err != nil {
		fmt.Println("Error reading file", err)
		os.Exit(1)
	}
	grid, err := parseGrid(string(input))
	if err != nil {
		fmt.Println("Error parsing", err)
		os.Exit(1)
	}
	part1(&grid)
	part2(&grid)
}
