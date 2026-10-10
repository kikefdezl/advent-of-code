import * as fs from "fs";

const INPUT_FILE: string = "input.txt";

function parseInput(input: string): boolean[][] {
  const lines: string[] = input.split("\n");
  return lines.map((line: string) => [...line].map((c) => c === "1"));
}

function part1(readings: boolean[][]) {
  let gamma = 0;
  let epsilon = 0;
  for (let i = 0; i < readings[0].length; i++) {
    const base = readings[0].length - i - 1;
    let countTrue = 0;
    let countFalse = 0;
    for (let j = 0; j < readings.length; j++) {
      if (readings[j][i] === false) {
        countFalse += 1;
      } else {
        countTrue += 1;
      }
    }
    if (countTrue > countFalse) {
      gamma += 2 ** base;
    } else {
      epsilon += 2 ** base;
    }
  }
  console.log("Gamma:", gamma);
  console.log("Epsilon:", epsilon);
  console.log("Multiplied:", gamma * epsilon);
}

function boolArrToNum(arr: boolean[]): number {
  let val = 0;
  for (let i = 0; i < arr.length; i++) {
    if (arr[arr.length - i - 1] === true) {
      val += 2 ** i;
    }
  }
  return val;
}

function findRating(readings: boolean[][], keep: boolean) {
  let kept = readings;
  let bit = 0;
  while (kept.length > 1) {
    let countTrue = 0;
    let countFalse = 0;

    for (let j = 0; j < kept.length; j++) {
      if (kept[j][bit] === false) {
        countFalse += 1;
      } else {
        countTrue += 1;
      }
    }

    if (countTrue > countFalse) {
      kept = kept.filter((k) => k[bit] === keep);
    } else if (countFalse > countTrue) {
      kept = kept.filter((k) => k[bit] === !keep);
    } else {
      kept = kept.filter((k) => k[bit] === keep);
    }

    bit += 1;
  }
  return boolArrToNum(kept[0]);
}

function findOxygenGeneratorRating(readings: boolean[][]): number {
  return findRating(readings, true);
}

function findCO2ScrubberRating(readings: boolean[][]): number {
  return findRating(readings, false);
}

function part2(readings: boolean[][]) {
  const oxygenGeneratorRating = findOxygenGeneratorRating(readings);
  console.log("Oxygen generator rating:", oxygenGeneratorRating);

  const cO2ScrubberRating = findCO2ScrubberRating(readings);
  console.log("CO2 Scrubber Rating:", cO2ScrubberRating);
  console.log("Multiplied:", oxygenGeneratorRating * cO2ScrubberRating);
}

function main() {
  const input = fs.readFileSync(INPUT_FILE, "utf8");
  const readings = parseInput(input);
  part1(readings);
  part2(readings);
}

main();
