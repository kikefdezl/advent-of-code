package main

import (
	"fmt"
	"os"
	"slices"
	"strings"
)

const InputFile = "input.txt"

const (
	Start    byte = 'S'
	Empty    byte = '.'
	Splitter byte = '^'
)

type Beam struct {
	x int
	y int
}

type Row []byte

type TachyonManifold struct {
	start int
	rows  []Row

	_memo map[Beam]int
}

func (tm *TachyonManifold) print() {
	for _, row := range tm.rows {
		fmt.Println(string(row))
	}
}

func (tm *TachyonManifold) getTimelinesForBeam(beam Beam) int {
	val, ok := tm._memo[beam]
	if ok == true {
		return val
	}

	if beam.y == len(tm.rows)-1 {
		tm._memo[beam] = 1
		return 1
	}

	next := tm.rows[beam.y+1][beam.x]

	var timelines int
	switch next {
	case Splitter:
		leftBeam := Beam{x: beam.x - 1, y: beam.y + 1}
		rightBeam := Beam{x: beam.x + 1, y: beam.y + 1}
		timelines = tm.getTimelinesForBeam(leftBeam) + tm.getTimelinesForBeam(rightBeam)
	case Empty:
		newBeam := Beam{x: beam.x, y: beam.y + 1}
		timelines = tm.getTimelinesForBeam(newBeam)
	}

	tm._memo[beam] = timelines
	return timelines
}

func parseTachyonManifold(input string) TachyonManifold {
	trimmed := strings.TrimSpace(input)
	lines := strings.Split(trimmed, "\n")

	rows := make([]Row, 0, len(lines))
	for _, row := range lines {
		rows = append(rows, []byte(row))
	}

	return TachyonManifold{
		start: slices.Index(rows[0], Start),
		rows:  rows,
		_memo: map[Beam]int{},
	}
}

func part1(manifold *TachyonManifold) {
	beams := map[int]bool{manifold.start: true}

	splits := 0
	for row := range len(manifold.rows) - 1 {
		for beam, val := range beams {
			if val == false {
				continue
			}
			next := manifold.rows[row+1][beam]
			if next == Splitter {
				beams[beam] = false
				beams[beam-1] = true
				beams[beam+1] = true
				splits += 1
			}
		}
	}
	fmt.Printf("Total of %d splits in the Tachyon Manifold\n", splits)
}

func part2(manifold *TachyonManifold) {
	beam := Beam{x: manifold.start, y: 0}
	timelines := manifold.getTimelinesForBeam(beam)
	fmt.Printf("And a total of %d timelines using the Quantum Tachyon Manifold\n", timelines)
}

func main() {
	input, err := os.ReadFile(InputFile)
	if err != nil {
		fmt.Println("error reading file", err)
		os.Exit(1)
	}
	manifold := parseTachyonManifold(string(input))
	manifold.print()

	part1(&manifold)
	part2(&manifold)
}
