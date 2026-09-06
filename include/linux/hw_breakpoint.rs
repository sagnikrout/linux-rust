//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hw_breakpoint.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bp_type_idx {
    TYPE_INST	= 0,

    TYPE_DATA	= 0,

    TYPE_DATA	= 1,

    TYPE_MAX
}

extern "C" {
    pub fn init_hw_breakpoint() -> int __init;
}
//
// As it's for in-kernel or ptrace use, we want it to be pinned
// and to call its callback every hits.
//
// FIXME: only change from the attr, and don't unregister
//
// Kernel breakpoints are not associated with any particular thread.
//
extern "C" {
    pub fn register_perf_hw_breakpoint(bp: *mut perf_event) -> c_int;
}
extern "C" {
    pub fn unregister_hw_breakpoint(bp: *mut perf_event);
}
extern "C" {
    pub fn unregister_wide_hw_breakpoint(cpu_events: *mut *mut perf_event  __percpu);
}
extern "C" {
    pub fn hw_breakpoint_is_used() -> bool;
}
extern "C" {
    pub fn dbg_reserve_bp_slot(bp: *mut perf_event) -> c_int;
}
extern "C" {
    pub fn dbg_release_bp_slot(bp: *mut perf_event) -> c_int;
}
extern "C" {
    pub fn reserve_bp_slot(bp: *mut perf_event) -> c_int;
}
extern "C" {
    pub fn release_bp_slot(bp: *mut perf_event);
}
extern "C" {
    pub fn flush_ptrace_hw_breakpoint(tsk: *mut task_struct);
}

