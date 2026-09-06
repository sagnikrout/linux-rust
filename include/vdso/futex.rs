//! Automatically rewritten from C Header to Rust Module
//! Source: include/vdso/futex.h
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
// __vdso_futex_robust_list64_try_unlock - Try to unlock an uncontended robust futex
// with a 64-bit pending op pointer
// @lock:	Pointer to the futex lock object
// @tid:	The TID of the calling task
// @pop:	Pointer to the task's robust_list_head::list_pending_op
//
// Return: The content of *@lock. On success this is the same as @tid.
//
// The function implements:
// if (atomic_try_cmpxchg(lock, &tid, 0))
// *op = NULL;
// return tid;
//
// There is a race between a successful unlock and clearing the pending op
// pointer in the robust list head. If the calling task is interrupted in the
// race window and has to handle a (fatal) signal on return to user space then
// the kernel handles the clearing of @pending_op before attempting to deliver
// the signal. That ensures that a task cannot exit with a potentially invalid
// pending op pointer.
//
// User space uses it in the following way:
//
// if (__vdso_futex_robust_list64_try_unlock(lock, tid, &pending_op) != tid)
// err = sys_futex($OP | FUTEX_ROBUST_UNLOCK,....);
//
// If the unlock attempt fails due to the FUTEX_WAITERS bit set in the lock,
// then the syscall does the unlock, clears the pending op pointer and wakes the
// requested number of waiters.
//
extern "C" {
    pub fn __vdso_futex_robust_list64_try_unlock(lock: *mut __u32, tid: __u32, pop: *mut __u64) -> __u32;
}
//
// __vdso_futex_robust_list32_try_unlock - Try to unlock an uncontended robust futex
// with a 32-bit pending op pointer
// @lock:	Pointer to the futex lock object
// @tid:	The TID of the calling task
// @pop:	Pointer to the task's robust_list_head::list_pending_op
//
// Return: The content of *@lock. On success this is the same as @tid.
//
// Same as __vdso_futex_robust_list64_try_unlock() just with a 32-bit @pop pointer.
//
extern "C" {
    pub fn __vdso_futex_robust_list32_try_unlock(lock: *mut __u32, tid: __u32, pop: *mut __u32) -> __u32;
}
