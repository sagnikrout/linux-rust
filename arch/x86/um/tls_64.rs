//! Automatically rewritten from C to Rust
//! Source: arch/x86/um/tls_64.c
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

#[no_mangle]
pub unsafe extern "C" fn clear_flushed_tls(task: *mut task_struct) {
    void clear_flushed_tls(struct task_struct *task)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn arch_set_tls(t: *mut task_struct, tls: c_ulong) -> c_int {
    int arch_set_tls(struct task_struct *t, unsigned long tls)
    {
//
// If CLONE_SETTLS is set, we need to save the thread id
// so it can be set during context switches.
//
    t.thread.regs.regs.gp[FS_BASE / sizeof(unsigned long)] = tls;
    return 0;
    }
