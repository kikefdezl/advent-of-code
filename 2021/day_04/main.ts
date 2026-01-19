import * as fs from "fs";

const INPUT_FILE: string = "input.txt";
const BOARD_SIZE: number = 5;

type BoardNum = {
  value: number;
  marked: boolean;
};

type Board = BoardNum[][];

function markBoard(board: Board, number: number): boolean {
  for (let y = 0; y < board.length; y++) {
    for (let x = 0; x < board[y].length; x++) {
      if (board[y][x].value == number) {
        board[y][x].marked = true;
        return isBingo(board, x, y);
      }
    }
  }
  return false;
}

function boardScore(board: Board): number {
  let sum = 0;
  for (let row of board) {
    for (let num of row) {
      if (num.marked === false) {
        sum += num.value;
      }
    }
  }
  return sum;
}

function isRowBingo(board: Board, y: number): boolean {
  for (let x = 0; x < board[y].length; x++) {
    if (board[y][x].marked === false) {
      return false;
    }
  }
  return true;
}

function isColBingo(board: Board, x: number): boolean {
  for (let y = 0; y < board.length; y++) {
    if (board[y][x].marked === false) {
      return false;
    }
  }
  return true;
}

function isBingo(board: Board, x: number, y: number): boolean {
  if (isRowBingo(board, y)) {
    return true;
  }
  return isColBingo(board, x);
}

function parseInput(input: string) {
  const lines = input.split("\n");
  const numbers: number[] = lines[0].split(",").map((n) => Number(n));

  let boards: Board[] = [];
  for (let i = 2; i < lines.length; i = i + BOARD_SIZE + 1) {
    let board: BoardNum[][] = [];
    for (let j = 0; j < BOARD_SIZE; j++) {
      const row: BoardNum[] = lines[i + j]
        .trim()
        .split(/\s+/)
        .map((n) => {
          return { value: Number(n), marked: false };
        });
      board.push(row);
    }
    boards.push(board);
  }
  return { numbers, boards };
}

function playBingo(numbers: number[], boards: Board[]) {
  for (let num of numbers) {
    for (let board of boards) {
      const res = markBoard(board, num);
      if (res === true) {
        return { board, num };
      }
    }
  }
  throw new Error();
}

function playBingoContinue(numbers: number[], boards: Board[]) {
  for (let num of numbers) {
    const solved = boards.filter((b) => markBoard(b, num));
    boards = boards.filter((b) => !markBoard(b, num));
    if (boards.length === 0) {
      return { board: solved[0], num };
    }
  }
  throw new Error();
}

function part1(numbers: number[], boards: Board[]) {
  const { board, num } = playBingo(numbers, boards);
  console.log("Best score:", boardScore(board) * num);

}

function part2(numbers: number[], boards: Board[]) {
  const { board, num } = playBingoContinue(numbers, boards);
  console.log("Worst score:", boardScore(board) * num);
}

function main() {
  const input = fs.readFileSync(INPUT_FILE, "utf8");
  const { numbers, boards } = parseInput(input);
  part1(numbers, boards);
  part2(numbers, boards);
}

main();
