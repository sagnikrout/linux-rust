//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/include/asm/stacktrace.h
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
pub struct stack_frame {
    pub next_frame: *mut stack_frame,
    pub return_address: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stacktrace_ops {
    pub reliable): *mut *mut *mut void (address)(void data, unsigned long address, int,
}

extern "C" {
    pub fn KSTK_EBP(_arg: task) -> return;
}

// get_stack_pointer(struct task_struct *task, struct pt_regs *segv_regs)
extern "C" {
    pub fn dump_trace(tsk: *mut task_struct, ops: *const stacktrace_ops, data: *mut c_void);
}
