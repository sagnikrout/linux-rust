//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/sighandling.h
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
    pub fn signal_fault(regs: *mut pt_regs, frame: *mut void __user, where: *mut c_char);
}
extern "C" {
    pub fn ia32_setup_frame(ksig: *mut ksignal, regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn ia32_setup_rt_frame(ksig: *mut ksignal, regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn x64_setup_rt_frame(ksig: *mut ksignal, regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn x32_setup_rt_frame(ksig: *mut ksignal, regs: *mut pt_regs) -> c_int;
}
//
// To prevent immediate repeat of single step trap on return from SIGTRAP
// handler if the trap flag (TF) is set without an external debugger attached,
// clear the software event flag in the augmented SS, ensuring no single-step
// trap is pending upon ERETU completion.
//
// Note, this function should be called in sigreturn() before the original
// state is restored to make sure the TF is read from the entry frame.
//
// If the trap flag (TF) is set, i.e., the sigreturn() SYSCALL instruction
// is being single-stepped, do not clear the software event flag in the
// augmented SS, thus a debugger won't skip over the following instruction.
//

