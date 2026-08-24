class Solution {
public:
    int mostBooked(int n, vector<vector<int>>& meetings) {
        sort(meetings.begin(), meetings.end());
        priority_queue<pair<long long, int>, vector<pair<long long, int>>, greater<pair<long long, int>>> available;
        for (int i = 0; i < n; i++) {
            available.push({0, i});
        }
        vector<int> count(n);

        for (const auto& meeting : meetings) {
            int start = meeting[0], end = meeting[1];
            while (!available.empty() && available.top().first < start) {
                auto [end_time, room] = available.top();
                available.pop();
                available.push({start, room});
            }

            auto [end_time, room] = available.top();
            available.pop();
            available.push({end_time + (end - start), room});
            count[room]++;
        }

        return max_element(count.begin(), count.end()) - count.begin();
    }
};
