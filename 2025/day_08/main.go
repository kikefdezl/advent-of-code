package main

import (
	"fmt"
	"os"
	"slices"
	"sort"
	"strconv"
	"strings"
)

const (
	InputFile = "input.txt"
	ConnectN  = 1000
)

type Box struct {
	x int
	y int
	z int
}

func (b Box) distanceTo(other Box) int {
	diffX := b.x - other.x
	diffY := b.y - other.y
	diffZ := b.z - other.z
	return diffX*diffX + diffY*diffY + diffZ*diffZ // no need for sqrt
}

func parseBoxes(input string) ([]Box, error) {
	trimmed := strings.TrimSpace(input)
	lines := strings.Split(trimmed, "\n")

	boxes := make([]Box, 0, len(lines))
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
		z, err := strconv.Atoi(parts[2])
		if err != nil {
			return nil, err
		}
		boxes = append(boxes, Box{x: x, y: y, z: z})
	}
	return boxes, nil
}

type Pair struct {
	boxA     Box
	boxB     Box
	distance int
}

func insertSorted(pairs []Pair, new Pair) []Pair {
	i := sort.Search(len(pairs), func(i int) bool {
		return pairs[i].distance > new.distance
	})

	pairs = append(pairs, Pair{})
	copy(pairs[i+1:], pairs[i:])
	pairs[i] = new
	return pairs
}

func getNClosestPairs(boxes []Box, n int) []Pair {
	pairs := make([]Pair, 0, len(boxes)/2)
	for a := 0; a < len(boxes)-1; a++ {
		for b := a + 1; b < len(boxes); b++ {
			pair := Pair{
				boxA:     boxes[a],
				boxB:     boxes[b],
				distance: boxes[a].distanceTo(boxes[b]),
			}
			pairs = insertSorted(pairs, pair)
			if len(pairs) > n {
				pairs = pairs[:n]
			}
		}
	}
	return pairs
}

type Circuit []Box

func (c Circuit) contains(box Box) bool {
	return slices.Contains(c, box)
}

func connectPair(circuits []Circuit, pair Pair) []Circuit {
	circuitA := -1
	for idx, circuit := range circuits {
		if circuit.contains(pair.boxA) {
			circuitA = idx
			break
		}
	}

	circuitB := -1
	for idx, circuit := range circuits {
		if circuit.contains(pair.boxB) {
			circuitB = idx
			break
		}
	}

	// neither are contained: Create a new circuit
	if circuitA == -1 && circuitB == -1 {
		circuits = append(circuits, []Box{pair.boxA, pair.boxB})
	} else

	// both are contained: Do nothing
	if circuitA == circuitB {
	} else

	// A contained, B not: Add B to the circuit
	if circuitA != -1 && circuitB == -1 {
		circuits[circuitA] = append(circuits[circuitA], pair.boxB)
	} else

	// B contained, A not: Add A to the circuit
	if circuitA == -1 && circuitB != -1 {
		circuits[circuitB] = append(circuits[circuitB], pair.boxA)
	} else

	// Both contained in different circuits: merge circuits
	if circuitA != -1 && circuitB != -1 && circuitA != circuitB {
		circuits[circuitA] = append(circuits[circuitA], circuits[circuitB]...)
		circuits = append(circuits[:circuitB], circuits[circuitB+1:]...) // removes circuitB
	}
	return circuits
}

func part1(boxes []Box) {
	pairs := getNClosestPairs(boxes, ConnectN)

	circuits := make([]Circuit, 0)
	for _, pair := range pairs {
		circuits = connectPair(circuits, pair)
	}

	lengths := make([]int, 0, len(circuits))
	for _, circuit := range circuits {
		lengths = append(lengths, len(circuit))
	}
	sort.Slice(lengths, func(i, j int) bool {
		return lengths[i] > lengths[j]
	})
	mul := lengths[0] * lengths[1] * lengths[2]
	fmt.Println("Largest 3 circuit lengths multiplied:", mul)
}

// part2 implementation is inefficient, but it does the job in milliseconds so i'll leave it.
//
// I calculate the first N pairs and connect the circuits like in part1.
// Then I calculate 2N closest pairs and loop until only one circuit is remaining.
// If still more than 1 circuit, I calculate 4N closest pairs and repeat the process.
// If still not closed, calculate 8N and so on...
func part2(boxes []Box) {
	pairs := getNClosestPairs(boxes, ConnectN)
	circuits := make([]Circuit, 0)

	idx := 0
	for range ConnectN {
		circuits = connectPair(circuits, pairs[idx])
		idx++
	}

	connectN := ConnectN
	for len(circuits) > 1 {
		// add a buffer and calculate the next batch
		connectN *= 2
		pairs = getNClosestPairs(boxes, connectN)

		for idx < len(pairs) && len(circuits) > 1 {
			circuits = connectPair(circuits, pairs[idx])
			idx++
		}
	}
	mul := pairs[idx-1].boxA.x * pairs[idx-1].boxB.x
	fmt.Println("Last juncture Xs multiplied:", mul)
}

func main() {
	input, err := os.ReadFile(InputFile)
	if err != nil {
		fmt.Println("error reading file", err)
	}
	boxes, err := parseBoxes(string(input))
	part1(boxes)
	part2(boxes)
}
