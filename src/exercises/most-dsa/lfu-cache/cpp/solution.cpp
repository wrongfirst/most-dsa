class LFUCache {
    struct ListNode {
        int val;
        ListNode* prev;
        ListNode* next;

        ListNode(int val) : val(val), prev(nullptr), next(nullptr) {}
        ListNode(int val, ListNode* prev, ListNode* next) : val(val), prev(prev), next(next) {}
    };

    struct LinkedList {
        ListNode* left;
        ListNode* right;
        unordered_map<int, ListNode*> map;
        
        LinkedList() {
            left = new ListNode(0);
            right = new ListNode(0);
            left->next = right;
            right->prev = left;
        }
        
        ~LinkedList() {
            while (left->next != right) {
                ListNode* temp = left->next;
                left->next = temp->next;
                delete temp;
            }
            delete left;
            delete right;
        }

        int length() {
            return map.size();
        }

        void pushRight(int val) {
            ListNode* node = new ListNode(val, right->prev, right);
            map[val] = node;
            right->prev->next = node;
            right->prev = node;
        }

        void pop(int val) {
            if (map.find(val) != map.end()) {
                ListNode* node = map[val];
                ListNode* prev = node->prev;
                ListNode* next = node->next;
                prev->next = next;
                next->prev = prev;
                map.erase(val);
                delete node;
            }
        }

        int popLeft() {
            int res = left->next->val;
            pop(res);
            return res;
        }

        void update(int val) {
            pop(val);
            pushRight(val);
        }
    };

    int capacity;
    int lfuCount;
    unordered_map<int, int> valMap; // Map key -> value
    unordered_map<int, int> countMap; // Map key -> count
    unordered_map<int, LinkedList*> listMap; // Map count -> linked list

    void counter(int key) {
        int count = countMap[key];
        countMap[key] = count + 1;

        listMap[count]->pop(key);
        
        if (!listMap.count(count + 1)) {
            listMap[count + 1] = new LinkedList();
        }
        listMap[count + 1]->pushRight(key);

        if (count == lfuCount && listMap[count]->length() == 0) {
            lfuCount++;
        }
    }

public:
    LFUCache(int capacity) : capacity(capacity), lfuCount(0) {
        listMap[0] = new LinkedList();
    }
    
    ~LFUCache() {
        for (auto& pair : listMap) {
            delete pair.second;
        }
    }

    int get(int key) {
        if (valMap.find(key) == valMap.end()) {
            return -1;
        }
        counter(key);
        return valMap[key];
    }

    void put(int key, int value) {
        if (capacity == 0) {
            return;
        }

        if (valMap.find(key) == valMap.end() && valMap.size() == capacity) {
            int toRemove = listMap[lfuCount]->popLeft();
            valMap.erase(toRemove);
            countMap.erase(toRemove);
        }

        valMap[key] = value;
        if (countMap.find(key) == countMap.end()) {
            countMap[key] = 0;
            listMap[0]->pushRight(key);
            lfuCount = 0;
        }
        counter(key);
    }
};
