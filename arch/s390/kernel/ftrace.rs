//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/kernel/ftrace.h
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
pub struct ftrace_hotpatch_trampoline {
    pub brasl_opc: u16,
    pub brasl_disp: i32,
    pub 16: s16:,
    pub rest_of_intercepted_function: u64,
    pub interceptor: u64,
    pub __packed: },
    pub __ftrace_hotpatch_trampolines_start: [extern struct ftrace_hotpatch_trampoline; ],
    pub __ftrace_hotpatch_trampolines_end: [extern struct ftrace_hotpatch_trampoline; ],
    pub ftrace_shared_hotpatch_trampoline_br: [extern char; ],
    pub ftrace_shared_hotpatch_trampoline_br_end: [extern char; ],
    pub ftrace_shared_hotpatch_trampoline_exrl: [extern char; ],
    pub ftrace_shared_hotpatch_trampoline_exrl_end: [extern char; ],