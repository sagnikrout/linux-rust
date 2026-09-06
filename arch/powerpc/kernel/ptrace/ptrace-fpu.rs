//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/ptrace/ptrace-fpu.c
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


// SPDX-License-Identifier: GPL-2.0-or-later

#[no_mangle]
pub unsafe extern "C" fn ptrace_get_fpr(child: *mut task_struct, index: c_int, data: *mut c_ulong) -> c_int {
    int ptrace_get_fpr(struct task_struct *child, int index, unsigned long *data)
    {

    let mut fpidx: c_uint = index - PT_FPR0;

    if (index > PT_FPSCR)
    return -EIO;

    flush_fp_to_thread(child);
    if (fpidx < (PT_FPSCR - PT_FPR0)) {
    if (IS_ENABLED(CONFIG_PPC32))
// On 32-bit the index we are passed refers to 32-bit words
// data = ((u32 *)child->thread.fp_state.fpr)[fpidx];
    else
    memcpy(data, &child.thread.TS_FPR(fpidx), sizeof(long));
    } else
// data = child->thread.fp_state.fpscr;

// data = 0;

    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ptrace_put_fpr(child: *mut task_struct, index: c_int, data: c_ulong) -> c_int {
    int ptrace_put_fpr(struct task_struct *child, int index, unsigned long data)
    {

    let mut fpidx: c_uint = index - PT_FPR0;

    if (index > PT_FPSCR)
    return -EIO;

    flush_fp_to_thread(child);
    if (fpidx < (PT_FPSCR - PT_FPR0)) {
    if (IS_ENABLED(CONFIG_PPC32))
// On 32-bit the index we are passed refers to 32-bit words
    ((u32 *)child.thread.fp_state.fpr)[fpidx] = data;
    else
    memcpy(&child.thread.TS_FPR(fpidx), &data, sizeof(long));
    } else
    child.thread.fp_state.fpscr = data;

    return 0;
    }
