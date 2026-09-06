//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_NFLOG.h
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

pub const XT_NFLOG_DEFAULT_GROUP: c_uint = 0x1;
pub const XT_NFLOG_DEFAULT_THRESHOLD: c_int = 0;
pub const XT_NFLOG_MASK: c_uint = 0x1;
// This flag indicates that 'len' field in xt_nflog_info is set
pub const XT_NFLOG_F_COPY_LEN: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_nflog_info {
// 'len' will be used iff you set XT_NFLOG_F_COPY_LEN in flags
    pub len: __u32,
    pub group: __u16,
    pub threshold: __u16,
    pub flags: __u16,
    pub pad: __u16,
    pub prefix: [c_char; 64],
}
