//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/socket.h
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
//
// Desired design of maximum size and alignment (see RFC2553)
//

pub type __kernel_sa_family_t = c_ushort;
//
// The definition uses anonymous union and struct in order to control the
// default alignment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __kernel_sockaddr_storage {
    pub /: *mut *mut __kernel_sa_family_t ss_family; / address family,
// Following field(s) are implementation specific
    pub short)]: char __data[_K_SS_MAXSIZE - sizeof(unsigned,
// space to achieve desired size,
// _SS_MAXSIZE value minus size of ss_family
}

pub const SOCK_SNDBUF_LOCK: c_int = 1;
pub const SOCK_RCVBUF_LOCK: c_int = 2;

pub const SOCK_TXREHASH_DEFAULT: c_int = 255;
pub const SOCK_TXREHASH_DISABLED: c_int = 0;
pub const SOCK_TXREHASH_ENABLED: c_int = 1;
