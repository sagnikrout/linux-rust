//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/switch_to.h
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

extern "C" {
    pub fn ret_from_fork_asm() -> asmlinkage void;
}
//
// This is the structure pointed to by thread.sp for an inactive task.  The
// order of the fields must match the code in __switch_to_asm().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inactive_task_frame {

    pub r15: c_ulong,
    pub r14: c_ulong,
    pub r13: c_ulong,
    pub r12: c_ulong,

    pub flags: c_ulong,
    pub si: c_ulong,
    pub di: c_ulong,

    pub bx: c_ulong,
//
// These two fields must be together.  They form a stack frame header,
// needed by get_frame_pointer().
//
    pub bp: c_ulong,
    pub ret_addr: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fork_frame {
    pub frame: inactive_task_frame,
    pub regs: pt_regs,
}

// Only happens when SEP is enabled, no need to test "SEP"arately:

// This is used when switching tasks or entering/exiting vm86 mode.
// sp0 always points to the entry trampoline stack, which is constant:

// Xen PV enters the kernel on the thread stack.

