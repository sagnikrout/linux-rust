//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/trace/ftrace_internal.h
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
    pub fn __register_ftrace_function(ops: *mut ftrace_ops) -> c_int;
}
extern "C" {
    pub fn __unregister_ftrace_function(ops: *mut ftrace_ops) -> c_int;
}

extern "C" {
    pub fn ftrace_startup(ops: *mut ftrace_ops, command: c_int) -> c_int;
}
extern "C" {
    pub fn ftrace_shutdown(ops: *mut ftrace_ops, command: c_int) -> c_int;
}
extern "C" {
    pub fn ftrace_ops_test(ops: *mut ftrace_ops, ip: c_ulong, regs: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ftrace_startup_subops(ops: *mut ftrace_ops, subops: *mut ftrace_ops, command: c_int) -> c_int;
}
extern "C" {
    pub fn ftrace_shutdown_subops(ops: *mut ftrace_ops, subops: *mut ftrace_ops, command: c_int) -> c_int;
}

// Keep as macros so we do not need to define the commands

extern "C" {
    pub fn fgraph_update_pid_func();
}

