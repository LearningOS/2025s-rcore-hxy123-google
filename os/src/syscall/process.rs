//! Process management syscalls
//use core::mem;

use crate::{ task::{change_program_brk, exit_current_and_run_next, select_mmap, select_munmap, suspend_current_and_run_next,
vaddr_to_paddr_r,vaddr_to_paddr_w,get_current_syscall_time}};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
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
    -1
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    match _trace_request{
        0 => unsafe {
           //(_id as *const u8 ).read_volatile() as isize
           let p_addr=vaddr_to_paddr_r(_id);
           if p_addr==-1{
            return -1;
           }
           let p_addr_u8=p_addr as *const u8;
           p_addr_u8.read_volatile() as isize
        },
        1 => unsafe {
            let p_addr=vaddr_to_paddr_w(_id);
            if p_addr==-1{
            return -1;
           }
            (p_addr as *mut u8 ).write_volatile(_data as u8);
            0
        },
        2=> {get_current_syscall_time(_id) as isize},
        _ => {
            
            -1
        }

    }
    
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    //trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    select_mmap(_start, _len, _port)
    
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    select_munmap(_start, _len)
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
