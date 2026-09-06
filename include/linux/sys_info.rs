//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sys_info.h
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
// SYS_INFO_PANIC_CONSOLE_REPLAY is for panic case only, as it needs special
// handling which only fits panic case.
//
pub const SYS_INFO_TASKS: c_uint = 0x00000001;
pub const SYS_INFO_MEM: c_uint = 0x00000002;
pub const SYS_INFO_TIMERS: c_uint = 0x00000004;
pub const SYS_INFO_LOCKS: c_uint = 0x00000008;
pub const SYS_INFO_FTRACE: c_uint = 0x00000010;
pub const SYS_INFO_PANIC_CONSOLE_REPLAY: c_uint = 0x00000020;
pub const SYS_INFO_ALL_BT: c_uint = 0x00000040;
pub const SYS_INFO_BLOCKED_TASKS: c_uint = 0x00000080;
extern "C" {
    pub fn sys_info(si_mask: c_ulong);
}
extern "C" {
    pub fn sys_info_parse_param(str: *mut c_char) -> c_ulong;
}

