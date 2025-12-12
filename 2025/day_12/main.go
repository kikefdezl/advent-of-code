package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

const InputFile = "input.txt"

const (
	Empty bool = false
	Full  bool = true
)

type Present [][]bool

func (p Present) area() int {
	area := 0
	for _, row := range p {
		for _, x := range row {
			if x == Full {
				area++
			}
		}
	}
	return area
}

type RegionData struct {
	width    int
	height   int
	presents []int
}

func (rd *RegionData) area() int {
	return rd.width * rd.height
}

func parseInput(input string) ([]Present, []RegionData, error) {
	trimmed := strings.TrimSpace(input)
	lines := strings.Split(trimmed, "\n")
	i := 0
	presents := make([]Present, 0)
	var present Present
	for !strings.Contains(lines[i], "x") {
		if strings.Contains(lines[i], ":") {
			present = make(Present, 0)
		} else if len(lines[i]) == 0 {
			presents = append(presents, present)
		} else {
			parts := make([]bool, 0, len(lines[i]))
			for _, c := range lines[i] {
				switch c {
				case '#':
					parts = append(parts, Full)
				case '.':
					parts = append(parts, Empty)
				}
			}
			present = append(present, parts)
		}
		i++
	}

	regions := make([]RegionData, 0, len(lines)-i)
	for j := i; j < len(lines); j++ {
		x := strings.Index(lines[j], "x")
		width, err := strconv.Atoi(lines[j][:x])
		if err != nil {
			return nil, nil, err
		}
		col := strings.Index(lines[j], ":")
		height, err := strconv.Atoi(lines[j][x+1 : col])
		if err != nil {
			return nil, nil, err
		}

		parts := strings.Split(lines[j][col+2:], " ")
		presents := make([]int, 0, len(parts))
		for _, part := range parts {
			count, err := strconv.Atoi(part)
			if err != nil {
				return nil, nil, err
			}
			presents = append(presents, count)
		}
		regions = append(regions, RegionData{width: width, height: height, presents: presents})
	}
	return presents, regions, nil
}

// This is an approximation method but it worked on the real input.
// It works by checking the total area of the presents to check if it fits in the area of the region.
// My assumption was that since the presents are so small and so many compared to the region,
// there would almost always be a way to pack them compactly minimizing empty space to almost zero.
// But it isn't a 100% solution (i.e it doesn't work on the examples)
func part1(presents []Present, regions []RegionData) {
	sum := 0
	for _, region := range regions {
		regionArea := region.area()
		fmt.Println("area of the region", regionArea)
		presentsArea := 0
		for presentId, presentCount := range region.presents {
			presentsArea += presentCount * presents[presentId].area()
		}

		fmt.Println("presents area", presentsArea)
		if presentsArea < regionArea {
			sum++
		}
	}
	fmt.Printf("A total of %d regions fit all presents\n", sum)
}

func main() {
	input, err := os.ReadFile(InputFile)
	if err != nil {
		fmt.Println("error loading file", err)
		os.Exit(1)
	}
	presents, regions, err := parseInput(string(input))
	if err != nil {
		fmt.Println("error parsing input", err)
		os.Exit(1)
	}
	fmt.Println(presents)
	fmt.Println(regions)
	part1(presents, regions)
}
