import * as fs from "fs";

const INPUT_FILE: string = "input.txt";

function parseInput(input: string): number[] {
  const lines = input.split("\n");
  return lines.map((line) => Number(line));
}

function solve(heights: number[], windowSize: number) {
  let count = 0;

  for (let i = windowSize; i < heights.length; i++) {
    let sumA = 0;
    let sumB = 0;

    for (let j = 0; j < windowSize; j++) {
      sumA += heights[i - j - 1];
      sumB += heights[i - j];
    }
    console.log(sumA);
    if (sumB > sumA) {
      count += 1;
    }
  }
  return count;
}

function part1(heights: number[]) {
  const count = solve(heights, 1);
  console.log("There are", count, "level increases with sliding window 1");
}

function part2(heights: number[]) {
  const count = solve(heights, 3);
  console.log("There are", count, "level increases with sliding window 3");
}

function main() {
  const input = fs.readFileSync(INPUT_FILE, "utf8");
  const heights = parseInput(input);
  part1(heights);
  part2(heights);
}

main();
