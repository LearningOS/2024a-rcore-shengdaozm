//! Process management syscalls
use crate::{
    config::{MAX_SYSCALL_NUM,PAGE_SIZE},
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,
        get_current_task_info,current_user_token,task_mmap,task_unmap,
    }, 
    timer::get_time_us,
    mm::page_table::get_datawrite_phys_addr,
};

#[repr(C)]
#[derive(Debug)]
/// Time value structure
pub struct TimeVal {
    /// Seconds
    pub sec: usize,
    /// Microseconds
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
/// taskinfo struct
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
/// I get it finally.the physical address of the TimeVal may be into two pages,not contiguous. 
/// if we use the virtual address to access the TimeVal, we may access the wrong page.
/// so we can use the physical address to make Timeval into conectted physical address.
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let time = get_time_us();
    *get_datawrite_phys_addr(current_user_token(),ts) = TimeVal {sec: time / 1000000, usec: time % 1000000};
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    *get_datawrite_phys_addr(current_user_token(),ti) = get_current_task_info();
    0
}

/// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap");
    // check if start is page aligned and port is valid
    if start % PAGE_SIZE != 0 || port & 0x7 ==0 || port & (!(0x7)) != 0 {
        return -1;
    }
    // start,end,port are valid
    task_mmap(start, len, port)
}

/// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap");
    task_unmap(start, len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
