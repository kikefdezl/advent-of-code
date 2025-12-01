package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

const INPUT_FILE = "input.txt"

type Rotation struct {
	direction byte // 'L' or 'R'
	count     int
}

func parseRotations(input string) ([]Rotation, error) {
	lines := strings.Split(input, "\n")

	count := len(lines)
	rotations := make([]Rotation, 0, count)

	for _, line := range lines {
		if len(line) == 0 {
			break
		}

		direction := byte(line[0])
		if direction != 'L' && direction != 'R' {
			return nil, fmt.Errorf("Unknown direction")
		}

		count, err := strconv.ParseInt(string(line[1:]), 10, 64)
		if err != nil {
			return nil, fmt.Errorf("couldn't parse count")
		}

		rotations = append(rotations, Rotation{
			direction: direction,
			count:     int(count),
		})
	}
	return rotations, nil
}

func parts1And2(rotations []Rotation) {
	pos := 50
	zeroLands := 0
	zeroPasses := 0
	for _, rotation := range rotations {
		fullTurns := rotation.count / 100
		zeroPasses += fullTurns

		clicks := rotation.count % 100

		switch rotation.direction {
		case 'L':
			if pos == 0 {
				pos += 100
			}
			pos -= clicks
			if pos < 0 {
				pos += 100
				if pos != 0 {
					zeroPasses += 1
				}
			}
		case 'R':
			pos += clicks
			if pos > 99 {
				pos -= 100
				if pos != 0 {
					zeroPasses += 1
				}
			}
		}

		if pos == 0 {
			zeroLands += 1
		}
	}
	fmt.Printf("Landed on zero %d times\n", zeroLands)
	fmt.Printf("Passed zero %d times\n", zeroPasses)
	fmt.Printf("%d zeros total\n", zeroLands+zeroPasses)
}

func main() {
	input, err := os.ReadFile(INPUT_FILE)
	if err != nil {
		fmt.Println("couldn't read file")
		os.Exit(1)
	}
	rotations, err := parseRotations(string(input))
	if err != nil {
		fmt.Printf("failed to parse rotations %d", err)
		os.Exit(1)
	}
	parts1And2(rotations)
}
