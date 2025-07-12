### This is a program to find the solution to [Peg Solitaire](https://en.wikipedia.org/wiki/Peg_solitaire) (english version)

<br>

## Valid Solution:

(format: peg end pos, x dir, y dir)

```
Move 1: pos 4, 4 dir 0, 1
Move 2: pos 4, 3 dir 0, -1
Move 3: pos 4, 4 dir 1, 0
Move 4: pos 4, 2 dir 0, -1
Move 5: pos 4, 3 dir 0, 1
Move 6: pos 3, 4 dir 0, 1
Move 7: pos 3, 3 dir 0, -1
Move 8: pos 3, 5 dir 1, 0
Move 9: pos 1, 5 dir 0, 1
Move 10: pos 4, 5 dir -1, 0
Move 11: pos 2, 5 dir -1, 0
Move 12: pos 3, 5 dir 1, 0
Move 13: pos 3, 4 dir 0, -1
Move 14: pos 3, 2 dir 0, -1
Move 15: pos 3, 3 dir 0, 1
Move 16: pos 5, 5 dir 0, -1
Move 17: pos 6, 5 dir 0, 1
Move 18: pos 6, 3 dir 1, 0
Move 19: pos 4, 3 dir 1, 0
Move 20: pos 5, 3 dir 0, 1
Move 21: pos 5, 2 dir 0, -1
Move 22: pos 5, 3 dir -1, 0
Move 23: pos 6, 3 dir 1, 0
Move 24: pos 7, 3 dir 0, -1
Move 25: pos 5, 3 dir -1, 0
Move 26: pos 5, 4 dir 0, 1
Move 27: pos 5, 6 dir 0, 1
Move 28: pos 5, 7 dir 1, 0
Move 29: pos 5, 5 dir 0, -1
Move 30: pos 4, 5 dir -1, 0
Move 31: pos 4, 4 dir 0, -1
```

It takes nearly 5 billion moves to find this answer but (for me, on a low-end computer) it's just over 7 minutes of running
