//! Implementation of [`Processor`] and Intersection of control flow
//!
//! Here, the continuous operation of user apps in CPU is maintained,
//! the current running state of CPU is recorded,
//! and the replacement and transfer of control flow of different applications are executed.

use super::__switch;
use super::{fetch_task, TaskStatus};
use super::{ProcessControlBlock, TaskContext, TaskControlBlock};
use crate::config::MAX_SYSCALL_NUM;
use crate::sync::UPSafeCell;
use crate::trap::TrapContext;
use alloc::sync::Arc;
use lazy_static::*;

/// Processor management structure
pub struct Processor {
    current: Option<Arc<TaskControlBlock>>,

    ///The basic control flow of each core, helping to select and switch process
    idle_task_cx: TaskContext,
}

impl Processor {
    /// Create a new processor
    pub fn new() -> Self {
        Self {
            current: None,
            idle_task_cx: TaskContext::zero_init(),
        }
    }

    ///Get mutable reference to `idle_task_cx`
    fn get_idle_task_cx_ptr(&mut self) -> *mut TaskContext {
        &mut self.idle_task_cx as *mut _
    }

    ///Get current task in moving semanteme
    pub fn take_current(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.current.take()
    }

    ///Get current task in cloning semanteme
    pub fn current(&self) -> Option<Arc<TaskControlBlock>> {
        self.current.as_ref().map(Arc::clone)
    }
    /// increase the syscall times of current task
    fn increase_syscall_times(&self, syscall_id: usize) {
        // let mut inner = self.inner.exclusive_access();
        // let cur = inner.current_task;
        // inner.tasks[cur].syscall_times[syscall_id] += 1;
        // drop(inner);
        let current_task = self.current.as_ref().unwrap();
        let mut inner = current_task.inner_exclusive_access();
        inner.syscall_times[syscall_id] += 1;
        drop(inner);
    }

    /// get the syscall times
    fn get_syscall_times(&self) -> [u32; MAX_SYSCALL_NUM] {
        let current_task = self.current.as_ref().unwrap();
        let inner = current_task.inner_exclusive_access();
        inner.syscall_times
    }

    /// get the begin time of current task
    fn get_current_begin_time(&self) -> usize {
        let current_task = self.current.as_ref().unwrap();
        let inner = current_task.inner_exclusive_access();
        inner.begin_time
    }
}

lazy_static! {
    pub static ref PROCESSOR: UPSafeCell<Processor> = unsafe { UPSafeCell::new(Processor::new()) };
}

///The main part of process execution and scheduling
///Loop `fetch_task` to get the process that needs to run, and switch the process through `__switch`
pub fn run_tasks() {
    loop {
        let mut processor = PROCESSOR.exclusive_access();
        if let Some(task) = fetch_task() {
            let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
            // access coming task TCB exclusively
            let mut task_inner = task.inner_exclusive_access();
            let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
            task_inner.task_status = TaskStatus::Running;
            // release coming task_inner manually
            drop(task_inner);
            // release coming task TCB manually
            processor.current = Some(task);
            // release processor manually
            drop(processor);
            unsafe {
                __switch(idle_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            warn!("no tasks available in run_tasks");
        }
    }
}

/// Get current task through take, leaving a None in its place
pub fn take_current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.exclusive_access().take_current()
}

/// Get a copy of the current task
pub fn current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.exclusive_access().current()
}

/// get current process
pub fn current_process() -> Arc<ProcessControlBlock> {
    current_task().unwrap().process.upgrade().unwrap()
}

/// Get the current user token(addr of page table)
pub fn current_user_token() -> usize {
    let task = current_task().unwrap();
    task.get_user_token()
}

/// Get the mutable reference to trap context of current task
pub fn current_trap_cx() -> &'static mut TrapContext {
    current_task()
        .unwrap()
        .inner_exclusive_access()
        .get_trap_cx()
}

/// get the user virtual address of trap context
pub fn current_trap_cx_user_va() -> usize {
    current_task()
        .unwrap()
        .inner_exclusive_access()
        .res
        .as_ref()
        .unwrap()
        .trap_cx_user_va()
}

/// get the top addr of kernel stack
pub fn current_kstack_top() -> usize {
    current_task().unwrap().kstack.get_top()
}

/// Return to idle control flow for new scheduling
pub fn schedule(switched_task_cx_ptr: *mut TaskContext) {
    let mut processor = PROCESSOR.exclusive_access();
    let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
    drop(processor);
    unsafe {
        __switch(switched_task_cx_ptr, idle_task_cx_ptr);
    }
}

/// increase the syscall times of current task
pub fn increase_syscall_times(syscall_id: usize) {
    PROCESSOR.exclusive_access().increase_syscall_times(syscall_id);
}

/// get the syscall times
pub fn get_syscall_times() -> [u32; MAX_SYSCALL_NUM] {
    PROCESSOR.exclusive_access().get_syscall_times()
}

/// get the begin time of current task
pub fn get_current_begin_time() -> usize {
    PROCESSOR.exclusive_access().get_current_begin_time()
}
