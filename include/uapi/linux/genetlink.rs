//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/genetlink.h
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

pub const GENL_MAX_ID: c_int = 1023;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genlmsghdr {
    pub cmd: __u8,
    pub version: __u8,
    pub reserved: __u16,
}

pub const GENL_ADMIN_PERM: c_uint = 0x01;
pub const GENL_CMD_CAP_DO: c_uint = 0x02;
pub const GENL_CMD_CAP_DUMP: c_uint = 0x04;
pub const GENL_CMD_CAP_HASPOL: c_uint = 0x08;
pub const GENL_UNS_ADMIN_PERM: c_uint = 0x10;
//
// List of reserved static generic netlink identifiers:
//

// must be last reserved + 1

//
// Controller
//

