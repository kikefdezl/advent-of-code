package main

import (
	"fmt"
	"math"
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

const (
	N   = 13
	Eps = 1e-8
)

type Equation struct {
	a [N]float64
	b float64
}

type Variable struct {
	substitution Equation
	independent  bool
	value        int
	upperBound   int
}

func eval(v Variable, vals [N]int) float64 {
	if v.independent {
		return float64(v.value)
	}

	x := v.substitution.b
	for i := range N {
		x += v.substitution.a[i] * float64(vals[i])
	}
	return x
}

func (m *Machine) PressesToMeetJoltage() int {
	vars := make([]Variable, len(m.buttons))
	for i := range vars {
		vars[i].upperBound = math.MaxInt
	}

	equations := make([]Equation, len(m.targetJoltages))
	for i, joltage := range m.targetJoltages {
		equation := Equation{b: float64(-joltage)}
		for j, btn := range m.buttons {
			if slices.Contains(btn, i) {
				equation.a[j] = 1
				vars[j].upperBound = joltage
			}
		}
		equations[i] = equation
	}

	for i := range vars {
		vars[i].independent = true
		for _, eq := range equations {
			if expr, ok := isolate(eq, i); ok {
				vars[i].independent = false
				vars[i].substitution = expr
				for j := range equations {
					equations[j] = substitute(equations[j], i, expr)
				}
				break
			}
		}
	}

	free := make([]int, 0, len(vars))
	for i, v := range vars {
		if v.independent {
			free = append(free, i)
		}
	}

	best, _ := evalRecursive(vars, free, 0)
	return best
}

func isolate(eq Equation, idx int) (Equation, bool) {
	a := -eq.a[idx]
	if math.Abs(a) < Eps {
		return Equation{}, false
	}

	r := Equation{b: eq.b / a}
	for i := range len(eq.a) {
		if i != idx {
			r.a[i] = eq.a[i] / a
		}
	}
	return r, true
}

func substitute(eq Equation, idx int, expr Equation) Equation {
	r := Equation{}

	a := eq.a[idx]
	eq.a[idx] = 0

	for i := range len(eq.a) {
		r.a[i] = eq.a[i] + a*expr.a[i]
	}
	r.b = eq.b + a*expr.b
	return r
}

func evalRecursive(vars []Variable, free []int, index int) (int, bool) {
	if index == len(free) {
		vals := [N]int{}
		total := 0

		for i := len(vars) - 1; i >= 0; i-- {
			x := eval(vars[i], vals)
			if x < -Eps || math.Abs(x-math.Round(x)) > Eps {
				return 0, false
			}
			vals[i] = int(math.Round(x))
			total += vals[i]
		}

		return total, true
	}

	best, found := math.MaxInt, false
	for x := 0; x <= vars[free[index]].upperBound; x++ {
		vars[free[index]].value = x
		total, ok := evalRecursive(vars, free, index+1)

		if ok {
			found = true
			best = min(best, total)
		}
	}

	if found {
		return best, true
	} else {
		return 0, false
	}
}

// I admit part2 stumped me and I had to search online for help from other people.
// This method uses Gaussian Elimination to constrain dependent variables and then find a solution
// by checking combinations.
func part2(machines []Machine) {
	sum := 0
	for _, machine := range machines {
		sum += machine.PressesToMeetJoltage()
	}
	fmt.Println("Sum of lowest needed keypresses for setting joltages:", sum)
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
	part2(machines)
}
