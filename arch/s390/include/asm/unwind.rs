//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/unwind.h
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
// To use the stack unwinder it has to be initialized with unwind_start.
// There four combinations for task and regs:
// 1) task==NULL, regs==NULL: the unwind starts for the task that is currently
// running, sp/ip picked up from the CPU registers
// 2) task==NULL, regs!=NULL: the unwind starts from the sp/ip found in
// the struct pt_regs of an interrupt frame for the current task
// 3) task!=NULL, regs==NULL: the unwind starts for an inactive task with
// the sp picked up from task->thread.ksp and the ip picked up from the
// return address stored by __switch_to
// 4) task!=NULL, regs!=NULL: the sp/ip are picked up from the interrupt
// frame 'regs' of a inactive task
// If 'first_frame' is not zero unwind_start skips unwind frames until it
// reaches the specified stack pointer.
// The end of the unwinding is indicated with unwind_done, this can be true
// right after unwind_start, e.g. with first_frame!=0 that can not be found.
// unwind_next_frame skips to the next frame.
// Once the unwind is completed unwind_error() can be used to check if there
// has been a situation where the unwinder could not correctly understand
// the tasks call chain.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct unwind_state {
    pub stack_info: stack_info,
    pub stack_mask: c_ulong,
    pub task: *mut task_struct,
    pub regs: *mut pt_regs,
    pub ip: unsigned long sp,,
    pub graph_idx: c_int,
    pub kr_cur: *mut llist_node,
    pub reliable: bool,
    pub error: bool,
}

// Recover the return address modified by rethook and ftrace_graph.

extern "C" {
    pub fn unwind_next_frame(state: *mut unwind_state) -> bool;
}
extern "C" {
    pub fn unwind_get_return_address(state: *mut unwind_state) -> c_ulong;
}

