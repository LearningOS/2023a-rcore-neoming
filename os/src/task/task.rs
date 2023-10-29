//! Types related to task management

use crate::config::MAX_SYSCALL_NUM;
use crate::timer::get_time_ms;

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// first lanuch time
    pub first_lanuch_time: Option<usize>,
    /// syscall times
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
}

impl TaskControlBlock {
    /// update syscall times
    pub fn update_syscall_times(&mut self, syscall_id: usize) {
        self.syscall_times[syscall_id] += 1;
    }

    /// get syscall times
    pub fn get_syscall_times(&self) -> [u32; MAX_SYSCALL_NUM] {
        self.syscall_times
    }

    /// record first launch time
    pub fn record_first_launch_time(&mut self) {
        if self.first_lanuch_time == None {
            self.first_lanuch_time = Some(get_time_ms());
        }
    }

    /// get task running time
    pub fn get_task_real_runtime(&self) -> usize {
        if let Some(time_begin) = self.first_lanuch_time {
            get_time_ms().checked_sub(time_begin).unwrap()
        } else {
            0
        }
    }
}
/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
