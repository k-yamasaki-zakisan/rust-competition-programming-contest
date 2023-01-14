from collections import deque
from bisect import bisect

N = int(input())
A = list(map(int, input().split()))
ans = [-1]
bisect_array = deque()
memo = {a: i + 1 for i, a in enumerate(A)}
for a in A:
    if len(bisect_array) == 0:
        bisect_array.append(a)
        continue

    if bisect_array[-1] < a:
        ans.append(-1)
    else:
        i = bisect(bisect_array, a)
        ans.append(memo[bisect_array[i]])

    while len(bisect_array) and bisect_array[0] < a:
        bisect_array.popleft()
    bisect_array.appendleft(a)
print(*ans)
