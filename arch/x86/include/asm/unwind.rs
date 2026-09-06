//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/unwind.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unwind_state {
    pub stack_info: stack_info,
    pub stack_mask: c_ulong,
    pub task: *mut task_struct,
    pub graph_idx: c_int,

    pub kr_cur: *mut llist_node,

    pub error: bool,

    pub full_regs: bool signal,,
    pub ip: unsigned long sp, bp,,
    pub prev_regs: *mut *mut pt_regs regs,,

    pub got_irq: bool,
    pub ip: *mut *mut *mut unsigned long bp, orig_sp,,
//
// If non-NULL: The current frame is incomplete and doesn't contain a
// valid BP. When looking for the next frame, use this instead of the
// non-existent saved BP.
//
    pub next_bp: *mut c_ulong,
    pub regs: *mut pt_regs,

    pub sp: *mut c_ulong,

}

extern "C" {
    pub fn unwind_next_frame(state: *mut unwind_state) -> bool;
}
extern "C" {
    pub fn unwind_get_return_address(state: *mut unwind_state) -> c_ulong;
}

//
// If 'partial' returns true, only the iret frame registers are valid.
//

// partial = !state->full_regs;

// partial = false;

extern "C" {
    pub fn unwind_init();
}

// Recover the return address modified by rethook and ftrace_graph.
extern "C" {
    pub fn unwind_recover_rethook(_arg: state, _arg: ret, _arg: addr_p) -> return;
}
//
// This disables KASAN checking when reading a value from another task's stack,
// since the other task could be running on another CPU and could have poisoned
// the stack in the meantime.
//

