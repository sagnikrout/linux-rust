//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/thread_info.h
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
// thread_info.h: low-level thread information
//
// Copyright (C) 2002  David Howells (dhowells@redhat.com)
// - Incorporating suggestions made by Linus Torvalds and Dave Miller
//

//
// TOP_OF_KERNEL_STACK_PADDING is a number of unused bytes that we
// reserve at the top of the kernel stack.  We do it because of a nasty
// 32-bit corner case.  On x86_32, the hardware stack frame is
// variable-length.  Except for vm86 mode, struct pt_regs assumes a
// maximum-length frame.  If we enter from CPL 0, the top 8 bytes of
// pt_regs don't actually exist.  Ordinarily this doesn't matter, but it
// does in at least one case:
//
// If we take an NMI early enough in SYSENTER, then we can end up with
// pt_regs that extends above sp0.  On the way out, in the espfix code,
// we can read the saved SS value, but that value will be above sp0.
// Without this offset, that can result in a page fault.  (We are
// careful that, in this case, the value we read doesn't matter.)
//
// In vm86 mode, the hardware frame is much longer still, so add 16
// bytes to make room for the real-mode segments.
//
// x86-64 has a fixed-length stack frame, but it depends on whether
// or not FRED is enabled. Future versions of FRED might make this
// dynamic, but for now it is always 2 words longer.
//

//
// low level task data that entry.S needs immediate access to
// - this struct should fit entirely inside of one cache line
// - this struct shares the supervisor stack pages
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_info {
    pub /: *mut *mut unsigned long flags; / low level flags,
    pub /: *mut *mut unsigned long syscall_work; / SYSCALL_WORK_ flags,
    pub /: *mut *mut u32 status; / thread synchronous flags,

    pub /: *mut *mut u32 cpu; / current CPU,

}

//
// Tell the generic TIF infrastructure which bits x86 supports
//
// Macro flag: #define HAVE_TIF_NEED_RESCHED_LAZY
// Macro flag: #define HAVE_TIF_POLLING_NRFLAG
// Macro flag: #define HAVE_TIF_SINGLESTEP

// Architecture specific TIF space starts at 16

// flags to check in __switch_to()

//
// Avoid calls to __switch_to_xtra() on UP as STIBP is not evaluated.
//

//
// macros/functions for gaining access to the thread information structure
//
// preempt_count needs to be 1 initially, until the scheduler is functional.
//
// Walks up the stack frames to make sure that the specified object is
// entirely contained by a single stack frame.
//
// Returns:
// GOOD_FRAME	if within a frame
// BAD_STACK	if placed across a frame boundary (or outside stack)
// NOT_STACK	unable to determine (no frame pointers, etc)
//
// This function reads pointers from the stack and dereferences them. The
// pointers may not have their KMSAN shadow set up properly, which may result
// in false positive reports. Disable instrumentation to avoid those.
//

//
// low ----------------------------------------------> high
// [saved bp][saved ip][args][local vars][saved bp][saved ip]
// ^----------------^
// allow copies only within here
//
// If obj + len extends past the last frame, this
// check won't pass and the next frame will be 0,
// causing us to bail out and correctly report
// the copy as invalid.
//

//
// Thread-synchronous status.
//
// This is different from the flags in that nobody else
// ever touches our thread-synchronous status, so we don't
// have to worry about atomic accesses.
//
pub const TS_COMPAT: c_uint = 0x0002	/* 32bit syscall active (64BIT)*/;

pub const TS_I386_REGS_POKED: c_uint = 0x0004	/* regs poked by 32-bit ptracer */;

extern "C" {
    pub fn arch_setup_new_exec();
}

