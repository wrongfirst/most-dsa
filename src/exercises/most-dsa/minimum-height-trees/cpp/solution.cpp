class Solution {
public:
    vector<int> findMinHeightTrees(int n, vector<vector<int>>& edges) {
        if (n == 1) return {0};

        vector<vector<int>> adj(n);
        for (auto& edge : edges) {
            adj[edge[0]].push_back(edge[1]);
            adj[edge[1]].push_back(edge[0]);
        }

        vector<int> edge_cnt(n);
        queue<int> leaves;

        for (int i = 0; i < n; ++i) {
            edge_cnt[i] = adj[i].size();
            if (adj[i].size() == 1)
                leaves.push(i);
        }

        while (!leaves.empty()) {
            if (n <= 2) {
                vector<int> result;
                while (!leaves.empty()) {
                    result.push_back(leaves.front());
                    leaves.pop();
                }
                return result;
            }
            int size = leaves.size();
            for (int i = 0; i < size; ++i) {
                int node = leaves.front();
                leaves.pop();
                --n;
                for (int& nei : adj[node]) {
                    --edge_cnt[nei];
                    if (edge_cnt[nei] == 1)
                        leaves.push(nei);
                }
            }
        }

        return {};
    }
};
