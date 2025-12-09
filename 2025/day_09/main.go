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

type Segment struct {
	A Tile
	B Tile
}

func computeGreenSegments(redTiles []Tile) []Segment {
	segments := make([]Segment, 0, len(redTiles)+1)
	for i := range len(redTiles) - 1 {
		segment := Segment{A: redTiles[i], B: redTiles[i+1]}
		segments = append(segments, segment)
	}
	// connect the last two
	segments = append(segments, Segment{A: redTiles[len(redTiles)-1], B: redTiles[0]})
	return segments
}

func (s *Segment) intersectsRect(rectA Tile, rectB Tile) bool {
	recMinX := min(rectA.x, rectB.x) + 1
	recMaxX := max(rectA.x, rectB.x) - 1
	recMinY := min(rectA.y, rectB.y) + 1
	recMaxY := max(rectA.y, rectB.y) - 1

	segMinX := min(s.A.x, s.B.x)
	segMaxX := max(s.A.x, s.B.x)
	segMinY := min(s.A.y, s.B.y)
	segMaxY := max(s.A.y, s.B.y)

	if segMaxX < recMinX || segMinX > recMaxX {
		return false
	}
	if segMaxY < recMinY || segMinY > recMaxY {
		return false
	}
	return true
}

// Compute the green tile segments. For every possible rectangle, if any green segment
// intersects it's invalid, so we discard.
func part2(redTiles []Tile) {
	greenSegments := computeGreenSegments(redTiles)

	maxRect := 0
	for i := 0; i < len(redTiles)-1; i++ {
	main:
		for j := i + 1; j < len(redTiles); j++ {
			area := redTiles[i].areaWith(&redTiles[j])
			if area < maxRect {
				continue
			}
			for _, greenSegment := range greenSegments {
				if greenSegment.intersectsRect(redTiles[i], redTiles[j]) {
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
