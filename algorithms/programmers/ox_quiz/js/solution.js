const filePath = process.platform === "linux" ? "/dev/stdin" : "../input.txt";
const input = require("fs")
  .readFileSync(filePath)
  .toString()
  .trim()
  .replace(/\r/g, "")
  .split("\n");

function solution(quiz) {
  return quiz.map((expression) => {
    const [x, operator, y, _, z] = expression.split(" ");

    const result =
      operator === "+" ? Number(x) + Number(y) : Number(x) - Number(y);

    return result == Number(z) ? "O" : "X";
  });
}

console.log(solution(input));
