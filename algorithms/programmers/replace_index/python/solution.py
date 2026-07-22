import sys
from pathlib import Path

LOCAL = sys.platform == "win32"

if LOCAL:
  input_path = Path(__file__).parent.parent / "input.txt"
  sys.stdin = input_path.open("r", encoding="utf-8")

input = sys.stdin.read().strip().split(" ")

def solution(my_string, num1, num2):
  chars = list(my_string)
  chars[num1], chars[num2] = chars[num2], chars[num1]

  return "".join(chars)

print(solution(input[0], int(input[1]), int(input[2])))