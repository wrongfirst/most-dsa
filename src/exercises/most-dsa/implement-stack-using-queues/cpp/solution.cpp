class MyStack {
private:
    struct Node {
        int val;
        shared_ptr<Node> next;
        Node(int v, shared_ptr<Node> n) : val(v), next(n) {}
    };
    shared_ptr<Node> q;

public:
    MyStack() : q(nullptr) {}

    void push(int x) {
        q = make_shared<Node>(x, q);
    }

    int pop() {
        if (!q) return -1;
        int top = q->val;
        q = q->next;
        return top;
    }

    int top() {
        if (!q) return -1;
        return q->val;
    }

    bool empty() {
        return q == nullptr;
    }
};
