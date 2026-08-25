def transpose(matrix: list[list[int]]) -> list[list[int]]:
    ROWS, COLS = len(matrix), len(matrix[0])

    if ROWS == COLS:
        for r in range(ROWS):
            for c in range(r):
                matrix[r][c], matrix[c][r] = matrix[c][r], matrix[r][c]
        
        return matrix

    res = [[0] * ROWS for _ in range(COLS)]

    for r in range(ROWS):
        for c in range(COLS):
            res[c][r] = matrix[r][c]

    return res
