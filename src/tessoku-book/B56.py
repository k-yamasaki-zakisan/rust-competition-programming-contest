import sys

# ローリングハッシュ
input = sys.stdin.readline
N, Q = map(int, input().strip().split(" "))
S = input().strip()
pow100 = []
hashtable_a = [0]
hashtable_b = [0]  # reversed
h = 0
B = 1
p = 2147483647
for i in range(N + 1):
    pow100.append(B % p)
    B = B * 100 % p
for chr in S:
    h = (h + ord(chr) - 96) % p
    hashtable_a.append(h)
    h *= 100
h = 0
for chr in S[::-1]:
    h = (h + ord(chr) - 96) % p
    hashtable_b.append(h)
    h *= 100
for _ in range(Q):
    a, b = map(int, input().strip().split(" "))
    hash_a = (hashtable_a[b] - hashtable_a[a - 1] * pow100[b - a + 1]) % p
    hash_b = (hashtable_b[N - a + 1] - hashtable_b[N - b] * pow100[b - a + 1]) % p
    print("Yes" if hash_a == hash_b else "No")
