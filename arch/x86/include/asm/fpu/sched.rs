//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/fpu/sched.h
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
    pub fn save_fpregs_to_fpstate(fpu: *mut fpu);
}
extern "C" {
    pub fn fpu__drop(tsk: *mut task_struct);
}
extern "C" {
    pub fn fpu_flush_thread();
}
//
// FPU state switching for scheduling.
//
// switch_fpu() saves the old state and sets TIF_NEED_FPU_LOAD if
// TIF_NEED_FPU_LOAD is not set.  This is done within the context
// of the old process.
//
// Once TIF_NEED_FPU_LOAD is set, it is required to load the
// registers before returning to userland or using the content
// otherwise.
//
// The FPU context is only stored/restored for a user task and
// PF_KTHREAD is used to distinguish between kernel and user threads.
//
// The save operation preserved register state, so the
// fpu_fpregs_owner_ctx is still @old_fpu. Store the
// current CPU number in @old_fpu, so the next return
// to user space can avoid the FPU register restore
// when is returns on the same CPU and still owns the
// context. See fpregs_restore_userregs().
//
