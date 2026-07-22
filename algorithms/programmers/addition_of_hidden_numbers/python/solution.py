import sys
from pathlib import Path
import re

LOCAL = sys.platform == "win32"

if LOCAL:
    input_path = Path(__file__).parent.parent / "input.txt"
    sys.stdin = input_path.open("r", encoding="utf-8")

input = sys.stdin.read().strip()

def solution(my_string):
  return sum(map(int, re.findall(r"\d+", my_string)))

print(solution(input));