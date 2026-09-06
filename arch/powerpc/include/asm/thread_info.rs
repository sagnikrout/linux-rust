//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/thread_info.h
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
// thread_info.h: PowerPC low-level thread information
// adapted from the i386 version by Paul Mackerras
//
// Copyright (C) 2002  David Howells (dhowells@redhat.com)
// - Incorporating suggestions made by Linus Torvalds and Dave Miller
//

//
// By aligning VMAP'd stacks to 2 * THREAD_SIZE, we can detect overflow by
// checking sp & (1 << THREAD_SHIFT), which we can do cheaply in the entry
// assembly.
//

//
// low level task data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_info {
    pub preemptable,: *mut *mut int preempt_count; / 0 =>,

    pub cpu: c_uint,

    pub /: *mut *mut unsigned long exit_flags; / Exit Flags for entry/exit,
    pub /: *mut *mut unsigned long syscall_work; / SYSCALL_WORK_ flags,
    pub /: *mut *mut unsigned long local_flags; / private flags for thread,

    pub livepatch_sp: *mut c_ulong,

    pub accounting: cpu_accounting_data,

    pub slb_preload_nr: c_uchar,
    pub slb_preload_tail: c_uchar,
    pub slb_preload_esid: [u32; SLB_PRELOAD_NR],
// low level flags - has atomic operations done on it
    pub ____cacheline_aligned_in_smp: unsigned long flags,
}

//
// macros/functions for gaining access to the thread information structure
//

// how to get the thread information struct from C
extern "C" {
    pub fn arch_dup_task_struct(dst: *mut task_struct, src: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn arch_setup_new_exec();
}

//
// thread information flag bit numbers
//

// as above, but as bit values

// Bits in local_flags
// Don't move TLF_NAPPING without adjusting the code in entry_32.S

//
// Walks up the stack frames to make sure that the specified object is
// entirely contained by a single stack frame.
//
// Returns:
// GOOD_FRAME	if within a frame
// BAD_STACK	if placed across a frame boundary (or outside stack)
//
// low -----------------------------------------------------------> high
// [backchain][metadata][params][local vars][saved registers][backchain]
// ^------------------------------------^
// |  allows copies only in this region |
// |                                    |
// params                               frame
// The metadata region contains the saved LR, CR etc.
//

