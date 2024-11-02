//! Process management syscalls
use core::borrow::BorrowMut;

use alloc::sync::Arc;

use crate::{
    config::{MAX_SYSCALL_NUM,PAGE_SIZE},
    loader::get_app_data_by_name,
    mm::{translated_refmut, translated_str},
    task::{
        add_task, current_task, current_user_token, exit_current_and_run_next,
        suspend_current_and_run_next, TaskStatus, get_current_taskinfo,task_map, task_unmap, 
        TaskControlBlock,
    }, 
    timer::get_time_us,
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
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("kernel:pid[{}] sys_exit", current_task().unwrap().pid.0);
    exit_current_and_run_next(exit_code);
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel:pid[{}] sys_yield", current_task().unwrap().pid.0);
    suspend_current_and_run_next();
    0
}

/// get the pid of current task
pub fn sys_getpid() -> isize {
    trace!("kernel: sys_getpid pid:{}", current_task().unwrap().pid.0);
    current_task().unwrap().pid.0 as isize
}

/// create a new process
pub fn sys_fork() -> isize {
    trace!("kernel:pid[{}] sys_fork", current_task().unwrap().pid.0);
    let current_task = current_task().unwrap();
    let new_task = current_task.fork();
    let new_pid = new_task.pid.0;
    // modify trap context of new_task, because it returns immediately after switching
    let trap_cx = new_task.inner_exclusive_access().get_trap_cx();
    // we do not have to move to next instruction since we have done it before
    // for child process, fork returns 0
    trap_cx.x[10] = 0;
    // add new task to scheduler
    add_task(new_task);
    new_pid as isize
}

/// execute a new program
pub fn sys_exec(path: *const u8) -> isize {
    trace!("kernel:pid[{}] sys_exec", current_task().unwrap().pid.0);
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        let task = current_task().unwrap();
        task.exec(data);
        0
    } else {
        -1
    }
}

/// If there is not a child process whose pid is same as given, return -1.
/// Else if there is a child process but it is still running, return -2.
pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    trace!("kernel::pid[{}] sys_waitpid [{}]", current_task().unwrap().pid.0, pid);
    let task = current_task().unwrap();
    // find a child process

    // ---- access current PCB exclusively
    let mut inner = task.inner_exclusive_access();
    if !inner
        .children
        .iter()
        .any(|p| pid == -1 || pid as usize == p.getpid())
    {
        return -1;
        // ---- release current PCB
    }
    let pair = inner.children.iter().enumerate().find(|(_, p)| {
        // ++++ temporarily access child PCB exclusively
        p.inner_exclusive_access().is_zombie() && (pid == -1 || pid as usize == p.getpid())
        // ++++ release child PCB
    });
    if let Some((idx, _)) = pair {
        let child = inner.children.remove(idx);
        // confirm that child will be deallocated after being removed from children list
        assert_eq!(Arc::strong_count(&child), 1);
        let found_pid = child.getpid();
        // ++++ temporarily access child PCB exclusively
        let exit_code = child.inner_exclusive_access().exit_code;
        // ++++ release child PCB
        *translated_refmut(inner.memory_set.token(), exit_code_ptr) = exit_code;
        found_pid as isize
    } else {
        -2
    }
    // ---- release current PCB automatically
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!(
        "kernel:pid[{}] sys_get_time NOT IMPLEMENTED",
        current_task().unwrap().pid.0
    );
    let time = get_time_us();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    *translated_refmut(inner.memory_set.token(), ts) = TimeVal {
        sec: time / 1000000,
        usec: time % 1000000,
    };
    //the inner is dropped to release the lock on the task,but it is not immplemented up
    drop(inner); 
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!(
        "kernel:pid[{}] sys_task_info",
        current_task().unwrap().pid.0
    );
    let taskinfo = get_current_taskinfo(); //get taskinfo struct
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    *translated_refmut(inner.memory_set.token(), ti) = taskinfo;
    drop(inner);
    0
}

/// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!(
        "kernel:pid[{}] sys_mmap",
        current_task().unwrap().pid.0
    );
    // check if start is page aligned and port is valid
    if start % PAGE_SIZE != 0 || port & 0x7 ==0 || port & (!(0x7)) != 0 {
        return -1;
    }
    // start,end,port are valid
    task_map(start, len, port)
}

/// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!(
        "kernel:pid[{}] sys_munmap ",
        current_task().unwrap().pid.0
    );
    task_unmap(start, len)
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel:pid[{}] sys_sbrk", current_task().unwrap().pid.0);
    if let Some(old_brk) = current_task().unwrap().change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}

/// YOUR JOB: Implement spawn.
/// HINT: fork + exec =/= spawn
/// need to return 0 in the son process???
pub fn sys_spawn(path: *const u8) -> isize {
    trace!(
        "kernel:pid[{}] sys_spawn ",
        current_task().unwrap().pid.0
    );
    // get the ppn and path
    let path = translated_str(current_user_token(), path);
    
    //get elf data
    let elf_data = match get_app_data_by_name(path.as_str()) {
        Some(data) =>data,
        None => return -1,
    };

    let new_task =Arc::new( TaskControlBlock::new(elf_data));
    let new_pid = new_task.getpid();
    
    //==change the parent and child relationship==
    let current_task = current_task().unwrap();
    let mut par_inner = current_task.inner_exclusive_access();
    par_inner.children.push(new_task.clone());
    // set the parent of new_task
    let par = Some(Arc::downgrade(&current_task));
    let mut new_task_inner =new_task.inner_exclusive_access();
    new_task_inner.parent = par;
    drop(new_task_inner);
    //drop parent
    drop(par_inner);

    // add new task to scheduler
    add_task(new_task);
    new_pid as isize
}

/// YOUR JOB: Set task priority.
pub fn sys_set_priority(prio: isize) -> isize {
    trace!(
        "kernel:pid[{}] sys_set_priority ",
        current_task().unwrap().pid.0
    );
    if prio <= 1 {
        return -1;
    } 
    let mut task = current_task().unwrap();
    let task = task.borrow_mut();
    task.set_priority(prio as usize);
    prio as isize
}
