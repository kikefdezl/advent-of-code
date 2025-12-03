package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

const INPUT_FILE = "input.txt"

type Bank []int

func parseBanks(input string) ([]Bank, error) {
	trimmed := strings.TrimSpace(input)
	lines := strings.Split(trimmed, "\n")

	banks := make([]Bank, 0, len(lines))
	for _, line := range lines {
		bank := make([]int, 0, len(line))
		for _, digit := range line {
			digitInt, err := strconv.Atoi(string(digit))
			if err != nil {
				return nil, err
			}
			bank = append(bank, digitInt)
		}
		banks = append(banks, bank)
	}
	return banks, nil
}

func maxJoltageForNBatteries(bank Bank, n int) int {
	bankLen := len(bank)
	selected := make([]int, n)

	lastSelectedIdx := -1
	for pos := range n {
		for i := lastSelectedIdx + 1; i <= bankLen-n+pos; i++ {
			if bank[i] > selected[pos] {
				lastSelectedIdx = i
				selected[pos] = bank[i]
			}
		}
	}

	maxJoltage := 0
	for i, digit := range selected {
		maxJoltage += digit * int(math.Pow10(n-i-1))
	}
	return maxJoltage
}

func part1(banks []Bank) {
	sum := 0
	for _, bank := range banks {
		sum += maxJoltageForNBatteries(bank, 2)
	}
	fmt.Println("Total joltage with 2 batteries:", sum)
}

func part2(banks []Bank) {
	sum := 0
	for _, bank := range banks {
		sum += maxJoltageForNBatteries(bank, 12)
	}
	fmt.Println("Total joltage with 12 batteries:", sum)
}

func main() {
	input, err := os.ReadFile(INPUT_FILE)
	if err != nil {
		fmt.Println("error reasing file", err)
		os.Exit(1)
	}
	banks, err := parseBanks(string(input))
	if err != nil {
		fmt.Println("error parsing banks", err)
		os.Exit(1)
	}

	part1(banks)
	part2(banks)
}
