class Solution:
    def canTraverseAllPairs(self, nums: List[int]) -> bool:
        N = len(nums)
        if N == 1:
            return True
        if any(num == 1 for num in nums):
            return False

        MAX = max(nums)
        sieve = [0] * (MAX + 1)
        p = 2
        while p * p <= MAX:
            if sieve[p] == 0:
                for composite in range(p * p, MAX + 1, p):
                    sieve[composite] = p
            p += 1

        adj = defaultdict(list)
        for i in range(N):
            num = nums[i]
            if sieve[num] == 0:  # num is prime
                adj[i].append(N + num)
                adj[N + num].append(i)
                continue

            while num > 1:
                prime = sieve[num] if sieve[num] != 0 else num
                adj[i].append(N + prime)
                adj[N + prime].append(i)
                while num % prime == 0:
                    num //= prime

        visited = set()
        queue = deque([0])
        visited.add(0)
        while queue:
            node = queue.popleft()
            for nei in adj[node]:
                if nei not in visited:
                    visited.add(nei)
                    queue.append(nei)

        for i in range(N):
            if i not in visited:
                return False
        return True
