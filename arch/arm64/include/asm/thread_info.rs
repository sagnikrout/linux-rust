//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/thread_info.h
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
// Based on arch/arm/include/asm/thread_info.h
//
// Copyright (C) 2002 Russell King.
// Copyright (C) 2012 ARM Ltd.
//

//
// low level task data that entry.S needs immediate access to.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_info {
    pub /: *mut *mut unsigned long flags; / low level flags,

    pub /: *mut *mut u64 ttbr0; / saved TTBR0_EL1,

    pub /: *mut *mut u64 preempt_count; / 0 => preemptible, <0 => bug,

    pub need_resched: u32,
    pub count: u32,

    pub count: u32,
    pub need_resched: u32,

    pub preempt: },
}

extern "C" {
    pub fn arch_setup_new_exec();
}

pub const TIF_FREEZE: c_int = 19;
pub const TIF_RESTORE_SIGMASK: c_int = 20;
pub const TIF_SINGLESTEP: c_int = 21;

// Macro flag: #define INIT_SCS

