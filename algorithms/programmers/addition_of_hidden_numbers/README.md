# O숨어있는 숫자의 덧셈 (2)

## 문제 설명

문자열 `my_string`이 매개변수로 주어집니다. `my_string`은 소문자, 대문자, 자연수로만 구성되어있습니다. `my_string`안의 자연수들의 합을 return하도록 solution 함수를 완성해주세요.

## 제한사항

- 1 ≤ my_string의 길이 ≤ 1,000
- 1 ≤ my_string 안의 자연수 ≤ 1000
- 연속된 수는 하나의 숫자로 간주합니다.
- 000123과 같이 0이 선행하는 경우는 없습니다.
- 문자열에 자연수가 없는 경우 0을 return 해주세요.

## 입출력 예

| my_string       | result |
| --------------- | ------ |
| "aAb1B2cC34oOp" | 37     |
| "1a2b3c4d123Z"  | 133    |

## 입출력 예 설명

### 입출력 예 #1

- "aAb1B2cC34oOp"안의 자연수는 1, 2, 34 입니다. 따라서 1 + 2 + 34 = 37 을 return합니다.

### 입출력 예 #2

- "1a2b3c4d123Z"안의 자연수는 1, 2, 3, 4, 123 입니다. 따라서 1 + 2 + 3 + 4 + 123 = 133 을 return합니다.

## 풀이

```py
def solution(my_string):
  answer = 0
  current = ""

  for i in range(len(my_string)):
    print(my_string[i])

    if not my_string[i].isdigit():
      continue

    if i < len(my_string) - 1 and my_string[i + 1].isdigit():
      current += my_string[i]
      continue

    if not current:
      current = my_string[i]
    else:
      current += my_string[i]

    answer += int(current)
    current = ""

  return answer
```

## 개선

```py
def solution(my_string):
  return sum(map(int, re.findall(r"\d+", my_string)))
```

## Classification

- [구현](../README.md/#implemenation)
- [문자열](../README.md/#string)

## Level

- [Level 0](../../programmers/README.md/#level-0)
