from bisect import bisect_left, bisect_right

N, W, L, R = map(int, input().split())
X = [0] + list(map(int, input().split())) + [W]
dp = [0] * (N + 3)
dp[0] = 1
dp[1] = -1
MOD = 10**9 + 7

for v in range(N + 1):
    l = bisect_left(X, X[v] + L)
    r = bisect_right(X, X[v] + R)
    dp[l] += dp[v]
    dp[l] %= MOD
    dp[r] -= dp[v]
    dp[r] %= MOD
    dp[v + 1] += dp[v]
    dp[v + 1] %= MOD

print(dp[N + 1])
