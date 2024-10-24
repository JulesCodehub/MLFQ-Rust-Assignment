// src/mlfq.rs

#[derive(Clone)]
pub struct Process {
    pub id: u32,
    pub priority: usize,  // Represents the current queue index
    pub remaining_time: u32,
    pub total_executed_time: u32,
}

pub struct MLFQ {
    queues: Vec<Vec<Process>>,
    num_levels: usize,
    time_quanta: Vec<u32>,
    current_time: u32,
}

impl MLFQ {
    pub fn new(num_levels: usize, time_quanta: Vec<u32>) -> Self {
        MLFQ {
            queues: vec![Vec::new(); num_levels],
            num_levels,
            time_quanta,
            current_time: 0,
        }
    }

    // Exercise 1: Queue Management
    pub fn add_process(&mut self, process: Process) {
        
        // 0. Edge case: determine if the process is already completed, if so automatically return and ignore everything else
        if process.remaining_time == 0 {
            return
        }

        // 1. given the process, read its priority so we can see what queue it belongs in.
        let prio = process.priority; //make sure this value is immutable

        match prio {
            1..=num_levels => queues[prio-1].add(process);
            _ => queues[0].add(process); // 2. Edge case: if bounds is outside of range 1...num_levels, we need to put it in the 0th queue
        }
    }

    // Exercise 2: Process Execution
    pub fn execute_process(&mut self, queue_index: usize) {

        // 1. given queue_index, determine which level and actual index in the queues this process falls in
        let (level, idx) = find_index(self, queue_index);

        // 2. Edge case: if this index does not exist, we need to return as there is nothing to process
        if level == -1 {
            return
        }

        // 3. using this proper index lets copy the information to preform some math on it locally
        let proc_time = queue[level][idx].remaining_time;
        let t_q = time_quanta[level];

        // 4. determine how long the process runs for
        mut time_ran: i32;
        mut job_left: i32;

        if proc_time == 0 {
            time_ran = 0;
            job_left = 0;
        } else {
            if proc_time > t_q {
                time_ran = t_q;
            } else {
                time_ran = proc_time;
            }
            // time_ran = min(proc_time, t_q);
            job_left = proc_time - time_ran;
        }

        // 5. update the process information based on the math
        queue[level][idx].remaining_time = job_left;
        queue[level][idx].total_executed_time += time_ran;

        // 6. if the process does not have anything left to run, eject it
        if job_left == 0 {
            queue[level][idx].remove();
        }

        // 7. if instead the process ran all of its time quanta, we need to move it to the next lower queue
        // 8. Edge case: if the process is already in the lowest queue, we dont need to do anything
        // We will use this as a condition to ignore execution for this whole block
        if time_ran >= t_q && level < num_levels {
            queue[level][idx].priority += 1;
            queue[num_levels].add(queue[level][idx].remove());
        }

        // 9. update the global time
        current_time += time_ran;
    }

    // Exercise 3: Priority Boost
    pub fn priority_boost(&mut self) {

        // 1. Move any processes not in the 0th level to the 0th level in the most elegant way possible
        for i in 1..nums_levels {
            while !queue[i].isempty() {
                queue[0].add(queue[i][0].remove()));
            }
        }

        // 2. Assign every process to have 0 priority
        for p in queues[0] {
            p.priority = 0;
        }
    }

    // Simulate time passing and trigger a boost if needed
    pub fn update_time(&mut self, elapsed_time: u32) {
        self.current_time += elapsed_time;
        let boost_interval = 100;
        if self.current_time % boost_interval == 0 {
            self.priority_boost();
        }
    }

    // Helper function to convert queue_index to real index
    pub fn find_index(&mut self, queue_index: usize) -> (usize, usize) {

        let mut level = 0;
        let mut idx = queue_index;

        for i in 0...num_levels {
            let len = nums[i].len();
            let c = idx % len;
                if c == idx {
                    idx = c;
                    break;
                }
                idx -= len;
                level += 1;
            }
        }

        (level, idx)
    }
}

// Automated Test Cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        
        let process1 = Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 };
        let process2 = Process { id: 2, priority: 1, remaining_time: 5, total_executed_time: 0 };
        let process3 = Process { id: 3, priority: 5, remaining_time: 8, total_executed_time: 0 };

        mlfq.add_process(process1);
        mlfq.add_process(process2);
        mlfq.add_process(process3);

        assert_eq!(mlfq.queues[0].len(), 1);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[2].len(), 1);
    }

    #[test]
    fn test_execute_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[0].push(Process { id: 1, priority: 0, remaining_time: 5, total_executed_time: 0 });

        mlfq.execute_process(0);

        assert_eq!(mlfq.queues[0].len(), 0);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[1][0].remaining_time, 3);
        assert_eq!(mlfq.queues[1][0].total_executed_time, 2);
    }

    #[test]
    fn test_priority_boost() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        mlfq.queues[2].push(Process { id: 2, priority: 2, remaining_time: 3, total_executed_time: 7 });

        mlfq.update_time(100); // Should trigger priority boost

        assert_eq!(mlfq.queues[0].len(), 2);
        assert_eq!(mlfq.queues[1].len(), 0);
        assert_eq!(mlfq.queues[2].len(), 0);
    }

    #[test]
    fn test_boost_does_not_occur_prematurely() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        
        mlfq.update_time(50); // No boost should happen

        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[0].len(), 0);
    }
}