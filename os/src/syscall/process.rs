//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,
        current_user_token, get_current_begin_time, get_syscall_times, alloc_framed_area,dealloc_framed_area
    },
    timer::{get_time_ms,get_time_us,},
    mm::{translated_byte_buffer, MapPermission},
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
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
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let token = current_user_token();
    let mut buf = translated_byte_buffer(token, _ts as *const u8, core::mem::size_of::<TimeVal>());
    if buf.len() == 0 {
        return -1;
    }
    let sec = get_time_ms() / 1000;
    let usec = get_time_us() % 1000000;
    unsafe {
        let ts = buf[0].as_mut_ptr() as *mut TimeVal;
        (*ts).sec = sec;
        (*ts).usec = usec;
    }
    
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let token = current_user_token();
    let mut buf = translated_byte_buffer(token, _ti as *const u8, core::mem::size_of::<TaskInfo>());
    if buf.len() == 0 {
        return -1;
    }
    let syscall_time = get_syscall_times();
    let running_time = get_time_ms() - get_current_begin_time();
    unsafe {
        let ti = buf[0].as_mut_ptr() as *mut TaskInfo;
        (*ti).status = TaskStatus::Running;
        (*ti).syscall_times = syscall_time;
        (*ti).time = running_time;
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if _port & 0x7 == 0 || _port & !0x7 != 0 {
        return -1;
    }
    let permision = MapPermission::from_bits_truncate((_port as u8) << 1) | MapPermission::U; 
    if alloc_framed_area(_start, _start + _len, permision) == true {
        0
    } else {
        -1
    }
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    if dealloc_framed_area(_start, _start + _len){
        0
    } else {
        -1
    }
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
