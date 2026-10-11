import { readFileSync } from "fs";

const INPUT_FILE = "input.txt";
const CYCLE = 6;
const FIRST_CYCLE = 8;

function parseInput(input: string): number[] {
	return input
		.trim()
		.split(",")
		.map((v) => Number(v));
}

function hash(fish: number[], days: number): string {
	return fish.map((f) => String(f)).join(",") + "+" + String(days);
}

function reproduce(
	fish: number[],
	days: number,
	_cache: Map<string, number>,
): number {
	if (days == 0) {
		return fish.length;
	} else {
		const key = hash(fish, days);
		if (_cache.has(key)) {
			return _cache.get(key)!;
		}
	}

	let total = 0;
	for (const fih of fish) {
		const next = fih - 1;
		if (next == -1) {
			total += reproduce(
				[CYCLE, FIRST_CYCLE],
				days - 1,
				_cache,
			);
		} else {
			total += reproduce([next], days - 1, _cache);
		}
	}

	_cache.set(hash(fish, days), total);
	return total;
}

function solve(fish: number[], days: number) {
	const total = reproduce(fish, days, new Map());
	console.log(total, "lanternfish after", days, "days");
}

function part1(fish: number[]) {
	solve(fish, 80);
}

function part2(fish: number[]) {
	solve(fish, 256);
}

function main() {
	const input = readFileSync(INPUT_FILE, "utf8");
	const fish = parseInput(input);

	part1([...fish]);
	part2(fish);
}

main();
