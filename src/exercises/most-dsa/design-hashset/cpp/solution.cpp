class MyHashSet {
private:
    int data[31251] = {};

    int getMask(int key) {
        return 1 << (key % 32);
    }

public:
    MyHashSet() {
        // key is in the range [1, 1000000]
        // 31251 * 32 = 1000032
        // Array is zero-initialized above
    }

    void add(int key) {
        data[key / 32] |= getMask(key);
    }

    void remove(int key) {
        if (contains(key)) {
            data[key / 32] ^= getMask(key);
        }
    }

    bool contains(int key) {
        return (data[key / 32] & getMask(key)) != 0;
    }
};
