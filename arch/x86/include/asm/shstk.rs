//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/shstk.h
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
pub struct thread_shstk {
    pub base: u64,
    pub size: u64,
}

extern "C" {
    pub fn shstk_prctl(task: *mut task_struct, option: c_int, arg2: c_ulong) -> c_long;
}
extern "C" {
    pub fn reset_thread_features();
}
extern "C" {
    pub fn shstk_free(p: *mut task_struct);
}
extern "C" {
    pub fn setup_signal_shadow_stack(ksig: *mut ksignal) -> c_int;
}
extern "C" {
    pub fn restore_signal_shadow_stack() -> c_int;
}
extern "C" {
    pub fn shstk_update_last_frame(val: c_ulong) -> c_int;
}
extern "C" {
    pub fn shstk_is_enabled() -> bool;
}
extern "C" {
    pub fn shstk_pop(val: *mut u64) -> c_int;
}
extern "C" {
    pub fn shstk_push(val: u64) -> c_int;
}

