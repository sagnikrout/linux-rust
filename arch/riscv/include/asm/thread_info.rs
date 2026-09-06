//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/thread_info.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2009 Chen Liqin <liqin.chen@sunplusct.com>
// Copyright (C) 2012 Regents of the University of California
// Copyright (C) 2017 SiFive
//

// thread information allocation

pub const KASAN_STACK_ORDER: c_int = 1;

pub const KASAN_STACK_ORDER: c_int = 0;

//
// By aligning VMAP'd stacks to 2 * THREAD_SIZE, we can detect overflow by
// checking sp & (1 << THREAD_SHIFT), which we can do cheaply in the entry
// assembly.
//

//
// low level task data that entry.S needs immediate access to
// - this struct should fit entirely inside of one cache line
// - if the members of this struct changes, the assembly constants
// in asm-offsets.c must be updated accordingly
// - thread_info is included in task_struct at an offset of 0.  This means that
// tp points to both thread_info and task_struct.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_info {
    pub /: *mut *mut unsigned long flags; / low level flags,
    pub /: *mut *mut int preempt_count; / 0=>preemptible, <0=>BUG,
//
// These stack pointers are overwritten on every system call or
// exception.  SP is also saved to the stack it can be recovered when
// overwritten.
//
    pub /: *mut *mut long kernel_sp; / Kernel stack pointer,
    pub /: *mut *mut long user_sp; / User stack pointer,
    pub cpu: c_int,
    pub /: *mut *mut unsigned long syscall_work; / SYSCALL_WORK_ flags,

    pub scs_base: *mut c_void,
    pub scs_sp: *mut c_void,

//
// Used in handle_exception() to save a0, a1 and a2 before knowing if we
// can access the kernel stack.
//
    pub a2: unsigned long a0, a1,,

    pub user_cfi_state: cfi_state,

}

// Macro flag: #define INIT_SCS

//
// macros/functions for gaining access to the thread information structure
//
// preempt_count needs to be 1 initially, until the scheduler is functional.
//

extern "C" {
    pub fn arch_release_task_struct(tsk: *mut task_struct);
}
extern "C" {
    pub fn arch_dup_task_struct(dst: *mut task_struct, src: *mut task_struct) -> c_int;
}

//
// thread information flags
// - these are process state flags that various assembly files may need to
// access
// - pending work-to-be-done flags are in lowest half-word
// - other flags in upper half-word(s)
//
// Tell the generic TIF infrastructure which bits riscv supports
//
// Macro flag: #define HAVE_TIF_NEED_RESCHED_LAZY
// Macro flag: #define HAVE_TIF_RESTORE_SIGMASK

