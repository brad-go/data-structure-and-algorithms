const fs = require("fs");
const path = process.platform == "linux" ? "/dev/stdin" : "../input.txt";
const input = fs.readFileSync(path).toString().trim().split("\n");

const n = Number.parseInt(input[0]);
const array = input[1].split(" ").map(Number);

const solution = (n, array) => {
  const min = Math.min(...array);
  const max = Math.max(...array);

  console.log(min, max);
};

solution(n, array);
