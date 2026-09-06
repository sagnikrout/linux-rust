//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/asm-generic/poll.h
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
// These are specified by iBCS2
pub const POLLIN: c_uint = 0x0001;
pub const POLLPRI: c_uint = 0x0002;
pub const POLLOUT: c_uint = 0x0004;
pub const POLLERR: c_uint = 0x0008;
pub const POLLHUP: c_uint = 0x0010;
pub const POLLNVAL: c_uint = 0x0020;
// The rest seem to be more-or-less nonstandard. Check them!
pub const POLLRDNORM: c_uint = 0x0040;
pub const POLLRDBAND: c_uint = 0x0080;

pub const POLLWRNORM: c_uint = 0x0100;

pub const POLLWRBAND: c_uint = 0x0200;

pub const POLLMSG: c_uint = 0x0400;

pub const POLLREMOVE: c_uint = 0x1000;

pub const POLLRDHUP: c_uint = 0x2000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}
