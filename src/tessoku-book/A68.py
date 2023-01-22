# 最大流（max flow）
n, m = map(int, input().split())
E = [list(map(int, input().split())) for _ in range(m)]

edges = [[] for _ in range(n + 1)]
for a, b, c in E:
    edges[a].append([b, c, len(edges[b])])
    edges[b].append([a, 0, len(edges[a]) - 1])


def get_flow_dfs(pos, goal, F) -> int:
    if pos == goal:
        return F
    visited[pos] = True

    for i in range(len(edges[pos])):
        to, cap, rev = edges[pos][i]
        if visited[to]:
            continue
        if cap == 0:
            continue

        flow = get_flow_dfs(to, goal, min(F, cap))
        if flow > 0:
            edges[to][rev][1] += flow
            edges[pos][i][1] -= flow
            return flow
    return 0


ans = 0
while True:
    visited = [False] * (n + 1)
    flow = get_flow_dfs(1, n, INF := 5000)
    if flow == 0:
        break
    ans += flow
print(ans)
