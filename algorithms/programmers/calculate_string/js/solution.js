const filePath = process.platform === "linux" ? "/dev/stdin" : "../input.txt";
const input = require("fs").readFileSync(filePath).toString().trim();

function solution(my_string) {
  const tokens = my_string.split(" ");
  let answer = Number(tokens[0]);

  for (let i = 1; i < tokens.length; i += 2) {
    const operator = tokens[i];
    const number = Number(tokens[i + 1]);

    if (operator === "+") {
      answer += number;
    } else {
      answer -= number;
    }
  }

  return answer;
}

console.log(solution(input));
