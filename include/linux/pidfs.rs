//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pidfs.h
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
    pub fn pidfs_init() -> void __init;
}
extern "C" {
    pub fn pidfs_prepare_pid(pid: *mut pid);
}
extern "C" {
    pub fn pidfs_add_pid(pid: *mut pid) -> c_int;
}
extern "C" {
    pub fn pidfs_remove_pid(pid: *mut pid);
}
extern "C" {
    pub fn pidfs_exit(tsk: *mut task_struct);
}

extern "C" {
    pub fn pidfs_coredump(cprm: *const coredump_params);
}

extern "C" {
    pub fn pidfs_register_pid_gfp(pid: *mut pid, gfp: gfp_t) -> c_int;
}
//
// pidfs_register_pid - register a struct pid in pidfs
// @pid: pid to pin
//
// Register a struct pid in pidfs.
//
// Return: On success zero, on error a negative error code is returned.
//
extern "C" {
    pub fn pidfs_register_pid_gfp(_arg: pid, _arg: GFP_KERNEL) -> return;
}
extern "C" {
    pub fn pidfs_free_pid(pid: *mut pid);
}
