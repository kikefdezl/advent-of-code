import * as fs from "fs";

const INPUT_FILE: string = "input.txt";

type Direction = "forward" | "up" | "down";

type Action = {
  direction: Direction;
  count: number;
};

function parseInput(input: string): Action[] {
  const lines = input.split("\n");
  let actions: Action[] = [];
  for (let line of lines) {
    if (line.startsWith("forward")) {
      const count = Number(line.slice(8, line.length));
      actions.push({ direction: "forward", count: count });
    } else if (line.startsWith("up")) {
      const count = Number(line.slice(3, line.length));
      actions.push({ direction: "up", count: count });
    } else if (line.startsWith("down")) {
      const count = Number(line.slice(5, line.length));
      actions.push({ direction: "down", count: count });
    }
  }
  return actions;
}

function part1(actions: Action[]) {
  var x = 0;
  var y = 0;
  for (let action of actions) {
    if (action.direction == "forward") {
      x += action.count;
    } else if (action.direction == "up") {
      y -= action.count;
    } else {
      y += action.count;
    }
  }
  console.log("x:", x, "y:", y, "- Multiplied:", x * y);
}

function part2(actions: Action[]) {
  var x = 0;
  var y = 0;
  var aim = 0;
  for (let action of actions) {
    if (action.direction == "forward") {
      x += action.count;
      y += aim * action.count;
    } else if (action.direction == "up") {
      aim -= action.count;
    } else {
      aim += action.count;
    }
  }
  console.log("With aim: x:", x, "y:", y, "- Multiplied:", x * y);
}

function main() {
  const input = fs.readFileSync(INPUT_FILE, "utf8");
  const actions = parseInput(input);
  part1(actions);
  part2(actions);
}

main();
