import { readFileSync } from "fs";

const INPUT_FILE = "input.txt";

type Point = {
	x: number;
	y: number;
};

function asString(p: Point): string {
	return `${p.x},${p.y}`;
}

type Line = {
	start: Point;
	end: Point;
};

function isVertical(line: Line): boolean {
	return line.start.x == line.end.x;
}

function isHorizontal(line: Line): boolean {
	return line.start.y == line.end.y;
}

function parseLines(input: string): Line[] {
	const rawLines = input.trim().split("\n");
	let parsedLines = [];
	for (const line of rawLines) {
		const parts = line.trim().split("->");
		const first = parts[0]!;
		const second = parts[1]!;
		const x1y1 = first.trim().split(",");
		const x2y2 = second.trim().split(",");

		const p1: Point = { x: Number(x1y1[0]!), y: Number(x1y1[1]!) };
		const p2: Point = { x: Number(x2y2[0]!), y: Number(x2y2[1]!) };
		parsedLines.push({ start: p1, end: p2 });
	}

	return parsedLines;
}

function getPointsFrom(line: Line): Point[] {
	const dx = Math.sign(line.end.x - line.start.x);
	const dy = Math.sign(line.end.y - line.start.y);
	const n =
		Math.max(
			Math.abs(line.end.x - line.start.x),
			Math.abs(line.end.y - line.start.y),
		) + 1;

	return Array.from({ length: n }, (_, i) => ({
		x: line.start.x + i * dx,
		y: line.start.y + i * dy,
	}));
}

function solve(lines: Line[], withDiagonal: boolean) {
	let visitedOnce = new Set<string>();
	let visitedMany = new Set<string>();

	for (const line of lines) {
		if (!withDiagonal && !isVertical(line) && !isHorizontal(line)) {
			continue;
		}

		const points = getPointsFrom(line);
		for (const point of points) {
			const key = asString({ x: point.x, y: point.y });
			if (visitedOnce.has(key)) {
				visitedMany.add(key);
			} else {
				visitedOnce.add(key);
			}
		}
	}

	console.log(visitedMany.size, "cells are visited more than once");
}

function part1(lines: Line[]) {
	process.stdout.write("Without diagonals: ");
	solve(lines, false);
}

function part2(lines: Line[]) {
	process.stdout.write("With diagonals: ");
	solve(lines, true);
}

function main() {
	const input = readFileSync(INPUT_FILE, "utf8");
	const lines = parseLines(input);
	part1(lines);
	part2(lines);
}

main();
