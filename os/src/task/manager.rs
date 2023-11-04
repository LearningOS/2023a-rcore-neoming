//!Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;
///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        // find min stride
        let mut min_stride = usize::MAX;
        for tcb in self.ready_queue.iter() {
            let cur_stride = tcb.inner_exclusive_access().stride;
            if min_stride > cur_stride {
                min_stride = cur_stride;
            }
            // debug!(
            //     "task {}, stride {}",
            //     tcb.getpid(),
            //     tcb.inner_exclusive_access().stride
            // );
        }

        // debug!("Now TaskNum {}", self.ready_queue.len());
        // return tcb whose stirde equal to min stride
        for _i in 0..self.ready_queue.len() {
            let cur_tcb = self.ready_queue.pop_front().unwrap();
            if cur_tcb.inner_exclusive_access().stride == min_stride {
                cur_tcb.inner_exclusive_access().update_stride();
                // debug!("Find min stride {}", min_stride);
                return Some(cur_tcb);
            } else {
                self.ready_queue.push_back(cur_tcb);
            }
        }
        None
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task {}", task.getpid());
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}
