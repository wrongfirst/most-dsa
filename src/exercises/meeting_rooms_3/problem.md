There are `n` rooms numbered from `0` to `n - 1`. You are given a 2D integer array `meetings` where `meetings[i] = [start[i], end[i]]` means that a meeting will be held during the half-closed time interval `[start[i], end[i])`. All the values of `start[i]` are **unique**.

Meetings are allocated to rooms in the following manner:

1. Each meeting will take place in the unused room with the **lowest** number.

2. If there are no available rooms, the meeting will be delayed until a room becomes free. The delayed meeting should have the **same** duration as the original meeting.

3. When a room becomes unused, meetings that have an earlier original **start** time should be given the room.

Return the **number** of the room that held the most meetings. If there are multiple rooms, return the room with the **lowest** number.

A half-closed interval `[a, b)` is the interval between `a` and `b` including `a` and not including `b`.

**Example 1:**

```java
Input: n = 2, meetings = [[1,10],[2,10],[3,10],[4,10]]

Output: 0
```

Explanation:
- At time 1, the room with number 0 is available, the first meeting is allocated to it.
- At time 2, the room with number 1 is available, the second meeting is allocated to it.
- At time 10, the rooms with numbers 0 and 1 becomes unused, the third meeting is allocated to room number 0 and fourth meeting is allocated to room number 1.

The room with most meetings held and have lowest number is 0.

**Example 2:**

```java
Input: n = 3, meetings = [[1,20],[2,10],[3,5],[6,8],[4,9]]

Output: 1
```

Explanation: 
- At time 1, all three rooms are not being used. The first meeting starts in room 0.
- At time 2, rooms 1 and 2 are not being used. The second meeting starts in room 1.
- At time 3, only room 2 is not being used. The third meeting starts in room 2.
- At time 4, all three rooms are being used. The fourth meeting is delayed.
- At time 5, the meeting in room 2 finishes. The fourth meeting starts in room 2 for the time period [5,10).
- At time 6, all three rooms are being used. The fifth meeting is delayed.
- At time 10, the meetings in rooms 1 and 2 finish. The fifth meeting starts in room 1 for the time period [10,12).

Room 0 held 1 meeting while rooms 1 and 2 each held 2 meetings, so we return 1 as it is lowest.

**Constraints:**
* `1 <= n <= 100`
* `1 <= meetings.length <= 100,000`
* `meetings[i].length == 2`
* `0 <= start[i] < end[i] <= 500,000`
* All the values of `start[i]` are **unique**.
