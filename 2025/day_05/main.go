package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

const InputFile = "input.txt"

type IngredientId int

func (i *IngredientId) isInRange(r *Range) bool {
	if *i >= r.min && *i <= r.max {
		return true
	}
	return false
}

type Range struct {
	min IngredientId
	max IngredientId
}

func parseInput(input string) ([]Range, []IngredientId, error) {
	trimmed := strings.TrimSpace(input)
	lines := strings.Split(trimmed, "\n")

	nLines := len(lines)
	ranges := make([]Range, 0, nLines)
	ingredients := make([]IngredientId, 0, nLines)

	i := 0
	for lines[i] != "" {
		parts := strings.Split(lines[i], "-")
		min_, err := strconv.Atoi(parts[0])
		if err != nil {
			return nil, nil, err
		}
		max_, err := strconv.Atoi(parts[1])
		if err != nil {
			return nil, nil, err
		}
		range_ := Range{
			min: IngredientId(min_),
			max: IngredientId(max_),
		}
		ranges = append(ranges, range_)
		i++
	}
	i++

	for i < nLines {
		id, err := strconv.Atoi(lines[i])
		if err != nil {
			return nil, nil, err
		}
		ingredients = append(ingredients, IngredientId(id))
		i++
	}
	return ranges, ingredients, nil
}

func part1(ranges *[]Range, ingredients *[]IngredientId) {
	count := 0
outer:
	for _, ingredient := range *ingredients {
		for _, range_ := range *ranges {
			if ingredient.isInRange(&range_) {
				count += 1
				continue outer
			}
		}
	}
	fmt.Println(count, "ingredients in the list are fresh")
}

func part2(ranges []Range) {
	sort.Slice(ranges, func(i, j int) bool {
		return ranges[i].min < ranges[j].min
	})

	covered := make([]Range, 1, len(ranges))
	covered[0] = ranges[0]

	for _, new_ := range ranges[1:] {
		lastIdx := len(covered) - 1
		lastCov := covered[lastIdx]

		// fully contained
		if lastCov.max >= new_.max {
			continue
		}

		// clips
		if lastCov.max >= new_.min {
			covered[lastIdx].max = new_.max
			continue
		}
		covered = append(covered, new_)
	}

	count := 0
	for _, cov := range covered {
		count += int(cov.max) - int(cov.min) + 1
	}
	fmt.Println("Total fresh ingredients:", count)
}

func main() {
	input, err := os.ReadFile(InputFile)
	if err != nil {
		fmt.Println("error reading file", err)
		os.Exit(1)
	}
	ranges, ingredients, err := parseInput(string(input))
	if err != nil {
		fmt.Println("error parsing input", err)
		os.Exit(1)
	}

	part1(&ranges, &ingredients)
	part2(ranges)
}
