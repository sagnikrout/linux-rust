//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/task_stack.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0
//
// task->stack (kernel stack) handling interfaces:
//

//
// When accessing the stack of a non-current task that might exit, use
// try_get_task_stack() instead.  task_stack_page will return a pointer
// that could get freed out from under you.
//

// task_thread_info(p) = *task_thread_info(org);
//
// Return the address of the last usable long on the stack.
//
// When the stack grows down, this is just above the thread
// info struct. Going any lower will corrupt the threadinfo.
//
// When the stack grows up, this is the highest address.
// Beyond that position, we corrupt data on the next page.
//

extern "C" {
    pub fn put_task_stack(tsk: *mut task_struct);
}

extern "C" {
    pub fn task_stack_page(_arg: tsk) -> return;
}

extern "C" {
    pub fn exit_task_stack_account(tsk: *mut task_struct);
}

extern "C" {
    pub fn thread_stack_cache_init();
}

extern "C" {
    pub fn stack_not_used(p: *mut task_struct) -> c_ulong;
}

extern "C" {
    pub fn set_task_stack_end_magic(tsk: *mut task_struct);
}
// Reliable end of stack detection:
// Some APM bios versions misalign the stack
//
