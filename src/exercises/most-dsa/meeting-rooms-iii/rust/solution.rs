use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut meetings = meetings;
        meetings.sort();

        let mut count = vec![0usize; n];
        // Min-heap of (end_time, room_id)
        let mut available: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();

        for i in 0..n {
            available.push(Reverse((0, i)));
        }

        for meeting in meetings {
            let start = meeting[0] as i64;
            let end = meeting[1] as i64;
            let duration = end - start;

            // Move all rooms that become free before this meeting starts to available with start time
            while let Some(&Reverse((end_time, room))) = available.peek() {
                if end_time < start {
                    available.pop();
                    available.push(Reverse((start, room)));
                } else {
                    break;
                }
            }

            // Get the earliest available room
            let Reverse((end_time, room)) = available.pop().unwrap();
            let new_end = end_time + duration;
            available.push(Reverse((new_end, room)));
            count[room] += 1;
        }

        // Find the room with the most meetings, tie-break by smallest room number
        let mut max_count = 0;
        let mut result = 0;
        for i in 0..n {
            if count[i] > max_count {
                max_count = count[i];
                result = i;
            }
        }
        result as i32
    }
}
