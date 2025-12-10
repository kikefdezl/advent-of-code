package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

const InputFile = "input.txt"

type state bool

func (s *state) toggle() {
	if *s == On {
		*s = Off
	} else {
		*s = On
	}
}

const (
	Off state = false
	On  state = true
)

type Lights []state

func (l Lights) equal(other Lights) bool {
	return slices.Equal(l, other)
}

func (l Lights) press(button Button) Lights {
	result := make(Lights, len(l))
	copy(result, l)
	for _, b := range button {
		result[b].toggle()
	}
	return result
}

type Button []int

type Joltages []int

func (j Joltages) equal(other Joltages) bool {
	return slices.Equal(j, other)
}

func (j Joltages) press(button Button) Joltages {
	result := make(Joltages, len(j))
	copy(result, j)
	for _, b := range button {
		result[b] += 1
	}
	return result
}

type Machine struct {
	buttons        []Button
	targetLights   Lights
	targetJoltages Joltages
}

func parseMachines(input string) ([]Machine, error) {
	trimmed := strings.TrimSpace(input)
	lines := strings.Split(trimmed, "\n")

	machines := make([]Machine, 0, len(lines))
	for _, line := range lines {
		// lights
		start := strings.Index(line, "[")
		end := strings.Index(line, "]")
		lights := make(Lights, 0, end-start-1)
		for j := start + 1; j < end; j++ {
			switch line[j] {
			case '.':
				lights = append(lights, Off)
			case '#':
				lights = append(lights, On)
			}
		}

		// buttons
		parts := strings.Split(line, "(")[1:]
		buttons := make([]Button, 0, len(parts))
		for _, part := range parts {
			end := strings.Index(part, ")")
			cleaned := part[:end]
			nums := strings.Split(cleaned, ",")
			button := make(Button, 0, len(nums))
			for _, num := range nums {
				numInt, err := strconv.Atoi(num)
				if err != nil {
					return nil, err
				}
				button = append(button, numInt)
			}
			buttons = append(buttons, button)
		}

		// joltage requirements
		start = strings.Index(line, "{")
		end = strings.Index(line, "}")
		nums := strings.Split(line[start+1:end], ",")
		joltageRequirements := make(Joltages, 0, len(nums))
		for _, num := range nums {
			numInt, err := strconv.Atoi(num)
			if err != nil {
				return nil, err
			}
			joltageRequirements = append(joltageRequirements, numInt)
		}

		machines = append(machines, Machine{
			targetLights:   lights,
			buttons:        buttons,
			targetJoltages: joltageRequirements,
		})

	}
	return machines, nil
}

func hashLights(l Lights) string {
	return fmt.Sprint(l)
}

func (m *Machine) PressesToTurnOn() int {
	type State struct {
		lights  Lights
		presses int
	}

	startLights := make(Lights, len(m.targetLights))
	queue := []State{{lights: startLights, presses: 0}}

	checked := make(map[string]bool)
	var state State
	for {
		state, queue = queue[0], queue[1:]

		for _, button := range m.buttons {
			newLights := state.lights.press(button)
			hashed := hashLights(newLights)
			if chk, _ := checked[hashed]; chk {
				continue
			}
			if newLights.equal(m.targetLights) {
				return state.presses + 1
			}

			newState := State{lights: newLights, presses: state.presses + 1}
			queue = append(queue, newState)
			checked[hashed] = true
		}
	}
}

func part1(machines []Machine) {
	sum := 0
	for _, machine := range machines {
		sum += machine.PressesToTurnOn()
	}
	fmt.Println("Sum of lowest needed keypresses for turning lights on:", sum)
}

func main() {
	input, err := os.ReadFile(InputFile)
	if err != nil {
		fmt.Println("error reading file", err)
		os.Exit(1)
	}
	machines, err := parseMachines(string(input))
	if err != nil {
		fmt.Println("error parsing input", err)
		os.Exit(1)
	}
	part1(machines)
}
