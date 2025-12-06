package main

import (
	"bytes"
	"fmt"
	"os"
	"strconv"
	"strings"
)

const InputFile = "input.txt"

type Op byte

type Problem struct {
	nums []string // parse as string because indent level is important
	op   Op
}

func (p *Problem) getOp() func(int, int) int {
	if p.op == '+' {
		return sum
	}
	return mul
}

func sum(a int, b int) int {
	return a + b
}

func mul(a int, b int) int {
	return a * b
}

func parseInput(input string) ([]Problem, error) {
	trimmed := strings.TrimSpace(input)
	lines := strings.Split(trimmed, "\n")

	opIdxs := make([]int, 0, len(lines[len(lines)-1]))
	for c, char := range lines[len(lines)-1] {
		if char == '+' || char == '*' {
			opIdxs = append(opIdxs, c)
		}
	}

	problems := make([]Problem, 0)
	for c, col := range opIdxs {
		var next int
		if c < len(opIdxs)-1 {
			next = opIdxs[c+1]
		} else {
			next = len(lines[0]) + 1
		}

		nums := make([]string, 0, len(lines)-1)
		for _, row := range lines[:len(lines)-1] {
			nums = append(nums, row[col:next-1])
		}

		problem := Problem{
			nums: nums,
			op:   Op(lines[len(lines)-1][col]),
		}
		problems = append(problems, problem)
	}

	return problems, nil
}

func part1(problems *[]Problem) error {
	total := 0
	for _, problem := range *problems {
		op := problem.getOp()

		res, err := strconv.Atoi(strings.TrimSpace(problem.nums[0]))
		if err != nil {
			return fmt.Errorf("couldn't parse %s to int", problem.nums[0])
		}

		for _, item := range problem.nums[1:] {
			num, err := strconv.Atoi(strings.TrimSpace(item))
			if err != nil {
				return fmt.Errorf("couldn't parse %s to int", item)
			}
			res = op(res, num)
		}
		total += res
	}
	fmt.Println("Total sum of results:", total)
	return nil
}

func part2(problems *[]Problem) error {
	total := 0

	for _, problem := range *problems {
		newNums := make([]string, 0)

		for i := range len(problem.nums[0]) {
			var buffer bytes.Buffer
			for j := range len(problem.nums) {
				buffer.WriteString(string(problem.nums[j][i]))
			}
			newNums = append(newNums, buffer.String())
		}

		op := problem.getOp()

		res, err := strconv.Atoi(strings.TrimSpace(newNums[0]))
		if err != nil {
			return fmt.Errorf("couldn't parse %s to int", newNums[0])
		}

		for _, item := range newNums[1:] {
			num, err := strconv.Atoi(strings.TrimSpace(item))
			if err != nil {
				return fmt.Errorf("couldn't parse %s to int", item)
			}
			res = op(res, num)
		}
		total += res
	}
	fmt.Println("Total sum with Cephalopod math:", total)
	return nil
}

func main() {
	input, err := os.ReadFile(InputFile)
	if err != nil {
		fmt.Println("error reading file", err)
		os.Exit(1)
	}
	problems, err := parseInput(string(input))
	if err != nil {
		fmt.Println("error parsing input", err)
		os.Exit(1)
	}
	fmt.Println(problems)
	err = part1(&problems)
	if err != nil {
		fmt.Println("error in part 1", err)
		os.Exit(1)
	}
	err = part2(&problems)
	if err != nil {
		fmt.Println("error in part 2", err)
		os.Exit(1)
	}
}
