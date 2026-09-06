//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/thread_info.h
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
// thread_info.h: LoongArch low-level thread information
//
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

//
// low level task data that entry.S needs immediate access to
// - this struct should fit entirely inside of one cache line
// - this struct shares the supervisor stack pages
// - if the contents of this structure are changed, the assembly constants
// must also be changed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_info {
    pub /: *mut *mut unsigned long flags; / low level flags,
    pub /: *mut *mut unsigned long tp_value; / thread pointer,
    pub /: *mut *mut __u32 cpu; / current CPU,
    pub /: *mut *mut int preempt_count; / 0 => preemptible, <0 => BUG,
    pub regs: *mut pt_regs,
    pub /: *mut *mut unsigned long syscall; / syscall number,
    pub /: *mut *mut unsigned long syscall_work; / SYSCALL_WORK_ flags,
}

//
// macros/functions for gaining access to the thread information structure
//

extern "C" {
    pub fn __asm__(_arg: "$sp") -> register unsigned long current_stack_pointer;
}

// thread information allocation

//
// thread information flags
// - these are process state flags that various assembly files may need to
// access
// - pending work-to-be-done flags are in LSW
// - other flags in MSW
//
// Tell the generic TIF infrastructure which special bits loongarch supports
//
// Macro flag: #define HAVE_TIF_NEED_RESCHED_LAZY
// Macro flag: #define HAVE_TIF_RESTORE_SIGMASK

// Architecture specific bits

