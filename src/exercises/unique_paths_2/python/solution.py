def uniquePathsWithObstacles(grid: list[list[int]]) -> int:
    M, N = len(grid), len(grid[0])
    if grid[0][0] == 1 or grid[M - 1][N - 1] == 1:
        return 0
    
    grid[M - 1][N - 1] = 1
    
    for r in range(M - 1, -1, -1):
        for c in range(N - 1, -1, -1):
            if r == M - 1 and c == N - 1:
                continue

            if grid[r][c] == 1:
                grid[r][c] = 0
            else:
                down = grid[r + 1][c] if r + 1 < M else 0
                right = grid[r][c + 1] if c + 1 < N else 0
                grid[r][c] = down + right
    
    return grid[0][0]
