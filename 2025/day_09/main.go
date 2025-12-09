package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

const InputFile = "input.txt"

type Tile struct {
	x int
	y int
}

func (rt *Tile) areaWith(other *Tile) int {
	return (abs(rt.x-other.x) + 1) * (abs(rt.y-other.y) + 1)
}

func abs(val int) int {
	if val < 0 {
		return -val
	}
	return val
}

func parseRedTiles(input string) ([]Tile, error) {
	trimmed := strings.TrimSpace(input)
	lines := strings.Split(trimmed, "\n")
	redTiles := make([]Tile, 0, len(lines))

	for _, line := range lines {
		parts := strings.Split(line, ",")
		x, err := strconv.Atoi(parts[0])
		if err != nil {
			return nil, err
		}
		y, err := strconv.Atoi(parts[1])
		if err != nil {
			return nil, err
		}

		rt := Tile{x: x, y: y}
		redTiles = append(redTiles, rt)
	}
	return redTiles, nil
}

func part1(tiles []Tile) {
	maxRect := 0
	for i := 0; i < len(tiles)-1; i++ {
		for j := i + 1; j < len(tiles); j++ {
			area := tiles[i].areaWith(&tiles[j])
			if area > maxRect {
				maxRect = area
			}
		}
	}
	fmt.Println("Max Red Tile rectangle has area", maxRect)
}

func connectTiles(tileA *Tile, tileB *Tile) []Tile {
	newTiles := make([]Tile, 0)
	if tileA.x == tileB.x {
		minY := min(tileA.y, tileB.y)
		maxY := max(tileA.y, tileB.y)
		for y := minY; y < maxY; y++ {
			newTiles = append(newTiles, Tile{x: tileA.x, y: y})
		}
	} else if tileA.y == tileB.y {
		minX := min(tileA.x, tileB.x)
		maxX := max(tileA.x, tileB.x)
		for x := minX; x < maxX; x++ {
			newTiles = append(newTiles, Tile{x: x, y: tileA.y})
		}
	} else {
		fmt.Println("tiles aren't aligned!")
	}
	return newTiles
}

func computeGreenTiles(redTiles []Tile) []Tile {
	greenTiles := make([]Tile, 0, len(redTiles)*10)
	for i := range len(redTiles) - 1 {
		newTiles := connectTiles(&redTiles[i], &redTiles[i+1])
		greenTiles = append(greenTiles, newTiles...)
	}
	// connect the last two
	newTiles := connectTiles(&redTiles[len(redTiles)-1], &redTiles[0])
	greenTiles = append(greenTiles, newTiles...)
	return greenTiles
}

func tileInRectangle(tile Tile, rectA Tile, rectB Tile) bool {
	minX := min(rectA.x, rectB.x)
	maxX := max(rectA.x, rectB.x)
	minY := min(rectA.y, rectB.y)
	maxY := max(rectA.y, rectB.y)

	if tile.x > minX && tile.x < maxX && tile.y > minY && tile.y < maxY {
		return true
	}
	return false
}

// I figured that a way to know whether a rectangle is illegal:
// If any tile of the ribbon that connects all the red tiles is inside the rectangle, 
// it's invalid.
//
// So this is a bruteforce approach that for every rectangle pair we check whether 
// any of the perimeter green tiles are in it.
func part2(redTiles []Tile) {
	greenTiles := computeGreenTiles(redTiles)

	maxRect := 0
	for i := 0; i < len(redTiles)-1; i++ {
	main:
		for j := i + 1; j < len(redTiles); j++ {
			area := redTiles[i].areaWith(&redTiles[j])
			if area < maxRect {
				continue
			}
			for _, greenTile := range greenTiles {
				if tileInRectangle(greenTile, redTiles[i], redTiles[j]) {
					continue main
				}
			}
			maxRect = area
		}
	}
	fmt.Println("Max rectangle of Green Tiles has area", maxRect)
}

func main() {
	input, err := os.ReadFile(InputFile)
	if err != nil {
		fmt.Println("error reading file", err)
		os.Exit(1)
	}
	redTiles, err := parseRedTiles(string(input))
	if err != nil {
		fmt.Println("error parsing input", err)
		os.Exit(1)
	}
	part1(redTiles)
	part2(redTiles)
}
