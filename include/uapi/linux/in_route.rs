//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/in_route.h
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
// IPv4 routing cache flags

// Obsolete flag. About to be deleted

pub const RTCF_NOTIFY: c_uint = 0x00010000;
pub const RTCF_DIRECTDST: c_uint = 0x00020000 /* unused */;
pub const RTCF_REDIRECTED: c_uint = 0x00040000;
pub const RTCF_TPROXY: c_uint = 0x00080000 /* unused */;
pub const RTCF_FAST: c_uint = 0x00200000 /* unused */;
pub const RTCF_MASQ: c_uint = 0x00400000 /* unused */;
pub const RTCF_SNAT: c_uint = 0x00800000 /* unused */;
pub const RTCF_DOREDIRECT: c_uint = 0x01000000;
pub const RTCF_DIRECTSRC: c_uint = 0x04000000;
pub const RTCF_DNAT: c_uint = 0x08000000;
pub const RTCF_BROADCAST: c_uint = 0x10000000;
pub const RTCF_MULTICAST: c_uint = 0x20000000;
pub const RTCF_REJECT: c_uint = 0x40000000 /* unused */;
pub const RTCF_LOCAL: c_uint = 0x80000000;

