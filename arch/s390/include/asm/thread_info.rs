//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/thread_info.h
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
// S390 version
// Copyright IBM Corp. 2002, 2006
// Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com)
//

//
// General size of kernel stacks
//

pub const THREAD_SIZE_ORDER: c_int = 4;

pub const THREAD_SIZE_ORDER: c_int = 2;

//
// low level task data that entry.S needs immediate access to
// - this struct should fit entirely inside of one cache line
// - this struct shares the supervisor stack pages
// - if the contents of this structure are changed, the assembly constants must also be changed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_info {
    pub /: *mut *mut unsigned long flags; / low level flags,
    pub /: *mut *mut unsigned long syscall_work; / SYSCALL_WORK_ flags,
    pub /: *mut *mut unsigned int cpu; / current CPU,
    pub /: *mut *mut unsigned char sie; / running in SIE context,
}

//
// macros/functions for gaining access to the thread information structure
//

extern "C" {
    pub fn arch_setup_new_exec();
}

//
// thread information flags bit numbers
//
// Tell the generic TIF infrastructure which special bits s390 supports
//
// Macro flag: #define HAVE_TIF_NEED_RESCHED_LAZY
// Macro flag: #define HAVE_TIF_RESTORE_SIGMASK
// Macro flag: #define HAVE_TIF_POLLING_NRFLAG

// Architecture specific bits

