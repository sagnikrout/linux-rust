//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/thread_info.h
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
// thread_info.h: common low-level thread information accessors
//
// Copyright (C) 2002  David Howells (dhowells@redhat.com)
// - Incorporating suggestions made by Linus Torvalds
//

//
// For CONFIG_THREAD_INFO_IN_TASK kernels we need <asm/current.h> for the
// definition of current, but for !CONFIG_THREAD_INFO_IN_TASK kernels,
// including <asm/current.h> can cause a circular dependency on some platforms.
//

//
// For per-arch arch_within_stack_frames() implementations, defined in
// asm/thread_info.h.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum syscall_work_bit {
    SYSCALL_WORK_BIT_SECCOMP,
    SYSCALL_WORK_BIT_SYSCALL_TRACEPOINT,
    SYSCALL_WORK_BIT_SYSCALL_TRACE,
    SYSCALL_WORK_BIT_SYSCALL_EMU,
    SYSCALL_WORK_BIT_SYSCALL_AUDIT,
    SYSCALL_WORK_BIT_SYSCALL_USER_DISPATCH,
    SYSCALL_WORK_BIT_SYSCALL_EXIT_TRAP,
    SYSCALL_WORK_BIT_SYSCALL_RSEQ_SLICE,
}

//
// flag set/clear/test wrappers
// - pass TIF_xxxx constants to these functions
//
extern "C" {
    pub fn test_and_set_bit(_arg: flag, )&ti->flags: *mut (unsigned long) -> return;
}
extern "C" {
    pub fn test_and_clear_bit(_arg: flag, )&ti->flags: *mut (unsigned long) -> return;
}
extern "C" {
    pub fn test_bit(_arg: flag, )&ti->flags: *mut (unsigned long) -> return;
}
//
// This may be used in noinstr code, and needs to be __always_inline to prevent
// inadvertent instrumentation.
//
extern "C" {
    pub fn READ_ONCE(_arg: ti->flags) -> return;
}

extern "C" {
    pub fn tif_test_bit(_arg: TIF_NEED_RESCHED) -> return;
}

extern "C" {
    pub fn arch_release_task_struct(tsk: *mut task_struct);
}

