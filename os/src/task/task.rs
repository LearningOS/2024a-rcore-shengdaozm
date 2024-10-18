//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;

/// Task information
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize, 
}

impl TaskInfo {
    /// TaskInfo defeat value
    pub fn new() -> Self {
        Self {
            status: TaskStatus::Running, //this is the default status,no need to set it here
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
        }
    }
}

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The task information
    pub task_info: TaskInfo,
    /// the time the task start
    pub start_time: usize  //just get the start time.
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
