// src/mlfq.rs

// Supress warnings
#![allow(dead_code)]

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
    pub fn add_process(&mut self, mut process: Process) {
        // Check to see if process's priority is within the range
        if process.priority >= self.num_levels {
            // If priority is not in range, move to lowest priority queue
            process.priority = self.num_levels - 1;
        }

        // Add the process to queue based on priority
        self.queues[process.priority].push(process);
    }

    // Exercise 2: Process Execution
    pub fn execute_process(&mut self, queue_index: usize) {
        // Check if queue is empty
        if self.queues[queue_index].is_empty() {
            return;
        }

        // Get time for this queue
        let time_quantum = self.time_quanta[queue_index];

        // Remove the process from the front of the queue
        let mut process = self.queues[queue_index].remove(0);

        // Execution time calculation
        let execution_time = std::cmp::min(process.remaining_time, time_quantum);

        // Update process times
        process.remaining_time -= execution_time;
        process.total_executed_time += execution_time;

        // Update MLFQ current time
        self.current_time += execution_time;

        if process.remaining_time > 0 {
            // Process is not completed
            if execution_time == time_quantum {
                // Full time is used, so move process to lower priority queue
                let new_priority = std::cmp::min(process.priority + 1, self.num_levels - 1);
                process.priority = new_priority;
                self.queues[new_priority].push(process);
            } else {
                // Full time is NOT used, keep process in the same queue
                self.queues[queue_index].push(process);
            }
        }
    }

    // Exercise 3: Priority Boost
    pub fn priority_boost(&mut self) {
        // Boost all processes from lower queues to the highest queue
        for i in 1..self.num_levels {
            while let Some(mut process) = self.queues[i].pop() {
                process.priority = 0;
                self.queues[0].push(process);
            }
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