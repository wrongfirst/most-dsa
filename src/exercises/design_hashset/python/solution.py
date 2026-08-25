class MyHashSet:

    def __init__(self):
        # key is in the range [1, 1000000]
        # 31251 * 32 = 1000032
        self.set = [0] * 31251

    def add(self, key: int) -> None:
        self.set[key // 32] |= self.getMask(key)

    def remove(self, key: int) -> None:
        if self.contains(key):
            self.set[key // 32] ^= self.getMask(key)

    def contains(self, key: int) -> bool:
        return self.set[key // 32] & self.getMask(key) != 0

    def getMask(self, key: int) -> int:
        return 1 << (key % 32)
