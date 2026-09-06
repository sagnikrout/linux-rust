//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/utsname.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
pub const __OLD_UTS_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct oldold_utsname {
    pub sysname: [c_char; 9],
    pub nodename: [c_char; 9],
    pub release: [c_char; 9],
    pub version: [c_char; 9],
    pub machine: [c_char; 9],
}

pub const __NEW_UTS_LEN: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct old_utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct new_utsname {
    pub 1]: char sysname[__NEW_UTS_LEN +,
    pub 1]: char nodename[__NEW_UTS_LEN +,
    pub 1]: char release[__NEW_UTS_LEN +,
    pub 1]: char version[__NEW_UTS_LEN +,
    pub 1]: char machine[__NEW_UTS_LEN +,
    pub 1]: char domainname[__NEW_UTS_LEN +,
}
