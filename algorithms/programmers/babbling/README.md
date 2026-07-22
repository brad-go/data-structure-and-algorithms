# 옹알이 (1)

## 문제 설명

머쓱이는 태어난 지 6개월 된 조카를 돌보고 있습니다. 조카는 아직 "aya", "ye", "woo", "ma" 네 가지 발음을 최대 한 번씩 사용해 조합한(이어 붙인) 발음밖에 하지 못합니다. 문자열 배열 `babbling`이 매개변수로 주어질 때, 머쓱이의 조카가 발음할 수 있는 단어의 개수를 return하도록 solution 함수를 완성해주세요.

## 제한사항

- 1 ≤ babbling의 길이 ≤ 100
- 1 ≤ babbling[i]의 길이 ≤ 15
- babbling의 각 문자열에서 "aya", "ye", "woo", "ma"는 각각 최대 한 번씩만 등장합니다.
  - 즉, 각 문자열의 가능한 모든 부분 문자열 중에서 "aya", "ye", "woo", "ma"가 한 번씩만 등장합니다.
- 문자열은 알파벳 소문자로만 이루어져 있습니다.

## 입출력 예

| babbling                                    | result |
| ------------------------------------------- | ------ |
| ["aya", "yee", "u", "maa", "wyeoo"]         | 1      |
| ["ayaye", "uuuma", "ye", "yemawoo", "ayaa"] | 3      |

## 입출력 예 설명

### 입출력 예 #1

- ["aya", "yee", "u", "maa", "wyeoo"]에서 발음할 수 있는 것은 "aya"뿐입니다. 따라서 1을 return합니다.

### 입출력 예 #2

- ["ayaye", "uuuma", "ye", "yemawoo", "ayaa"]에서 발음할 수 있는 것은 "aya" + "ye" = "ayaye", "ye", "ye" + "ma" + "woo" = "yemawoo"로 3개입니다. 따라서 3을 return합니다.

## 유의사항

- 네 가지를 붙여 만들 수 있는 발음 이외에는 어떤 발음도 할 수 없는 것으로 규정합니다. 예를 들어 "woowo"는 "woo"는 발음할 수 있지만 "wo"를 발음할 수 없기 때문에 할 수 없는 발음입니다.

## 풀이

### 풀이 1

```js
const filePath = process.platform === "linux" ? "/dev/stdin" : "../input.txt";
const input = require("fs")
  .readFileSync(filePath)
  .toString()
  .trim()
  .replace(/\r/g, "")
  .split("\n");

const speakableWords = ["aya", "ye", "woo", "ma"];

function solution(babbling) {
  let answer = 0;

  babbling.forEach((word) => {
    let current = word;

    speakableWords.forEach((speakable) => {
      if (current.includes(speakable)) {
        let splits = current.split(speakable);

        current = splits.join(" ");

        console.log(current);
      }
    });

    if (current.trim().length == 0) {
      answer += 1;
    }
  });

  return answer;
}

console.log(solution(input));
```

### 개선 1

```js
function solution(babbling) {
  // ^               문자열 시작
  // (aya|ye|woo|ma) 이 네 발음 중 하나
  // +               하나 이상 반복
  // $               문자열 끝
  // 문자열 전체가 "aya", "ye", "woo", "ma"만 이어붙여져 있으면 통과
  const regex = /^(aya|ye|woo|ma)+$/;

  return babbling.filter((word) => regex.test(word)).length;
}
```

### 개선 2

```js
function solution(babbling) {
  const speakableWords = ["aya", "ye", "woo", "ma"];

  let answer = 0;

  for (const word of babbling) {
    let index = 0;

    while (index < word.length) {
      console.log(index);

      let matched = false;

      for (const speakable of speakableWords) {
        if (word.startsWith(speakable, index)) {
          index += speakable.length;
          matched = true;
          break;
        }
      }

      if (!matched) {
        break;
      }
    }

    if (index === word.length) {
      answer += 1;
    }
  }

  return answer;
}
```

## Classification

- [문자열](../README.md/#implementation)

## Level

- [Level 0](../../programmers/README.md/#level-0)
