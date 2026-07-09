const filePath = process.platform === "linux" ? "/dev/stdin" : "../input.txt";
const input = require("fs")
  .readFileSync(filePath)
  .toString()
  .trim()
  .replace(/\r/g, "")
  .split("\n");

function solution(babbling) {
  const regex = /^(aya|ye|woo|ma)+$/;

  return babbling.filter((word) => regex.test(word)).length;
}

console.log(solution(input));
