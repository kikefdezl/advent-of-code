package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

const INPUT_FILE = "input.txt"

type Range struct {
	start int
	end   int
}

func parseRanges(input string) ([]Range, error) {
	trimmed := strings.TrimSpace(input)
	ranges_s := strings.Split(trimmed, ",")

	ranges := make([]Range, 0, len(ranges_s))
	for _, range_s := range ranges_s {
		parts := strings.Split(range_s, "-")
		start := parts[0]
		end := parts[1]

		start_int, err := strconv.ParseInt(start, 10, 64)
		if err != nil {
			return nil, fmt.Errorf("error parsing the start")
		}

		end_int, err := strconv.ParseInt(end, 10, 64)
		if err != nil {
			fmt.Println(err)
			return nil, fmt.Errorf("error parsing the end")
		}

		ranges = append(ranges, Range{start: int(start_int), end: int(end_int)})
	}
	return ranges, nil
}

func getIdLen(id int) int {
	len_ := 1
	var ref int = 10
	for id > ref {
		len_ += 1
		ref *= 10
	}
	return len_
}

func splitIdN(id int, n int) ([]int, error) {
	idLen := getIdLen(id)
	if idLen%n != 0 {
		return nil, fmt.Errorf("Can't divide into N equal parts")
	}
	partLen := idLen / n
	parts := make([]int, 0, n)
	factor := int(math.Pow10(partLen))
	num := id
	for range n {
		parts = append(parts, num%factor)
		num /= factor
	}
	return parts, nil
}

func allEqual[T comparable](s []T) bool {
	for i := 1; i < len(s); i++ {
		if s[0] != s[i] {
			return false
		}
	}
	return true
}

func isInvalidId2(id int) bool {
	id_len := getIdLen(id)
	if id_len%2 == 1 {
		return false
	}
	parts, err := splitIdN(id, 2)
	if err != nil {
		return false
	}
	if parts[0] == parts[1] {
		return true
	}
	return false
}

func isInvalidIdN(id int) bool {
	id_len := getIdLen(id)
	for i := 2; i <= id_len; i++ {
		parts, err := splitIdN(id, i)
		if err != nil {
			continue
		}
		if allEqual(parts) {
			fmt.Println("invalid:", id)
			fmt.Println(parts)
			fmt.Println()
			return true
		}
	}
	return false
}

func parts1And2(ranges *[]Range) {
	sum_repeat_2 := 0
	sum_repeat_n := 0
	for _, range_ := range *ranges {
		for i := range_.start; i <= range_.end; i++ {
			if isInvalidId2(i) {
				sum_repeat_2 += i
			}
			if isInvalidIdN(i) {
				sum_repeat_n += i
			}
		}
	}
	fmt.Println("Total sum of invalid IDs (repeat twice):", sum_repeat_2)
	fmt.Println("Total sum of invalid IDs (repeat N times):", sum_repeat_n)
}

func main() {
	input, err := os.ReadFile(INPUT_FILE)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	ranges, err := parseRanges(string(input))
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	fmt.Println(ranges)
	parts1And2(&ranges)
}
