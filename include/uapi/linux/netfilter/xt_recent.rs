//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_recent.h
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
pub const _LINUX_NETFILTER_XT_RECENT_H: c_int = 1;

// Only allowed with --rcheck and --update

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_recent_mtinfo {
    pub seconds: __u32,
    pub hit_count: __u32,
    pub check_set: __u8,
    pub invert: __u8,
    pub name: [c_char; XT_RECENT_NAME_LEN],
    pub side: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_recent_mtinfo_v1 {
    pub seconds: __u32,
    pub hit_count: __u32,
    pub check_set: __u8,
    pub invert: __u8,
    pub name: [c_char; XT_RECENT_NAME_LEN],
    pub side: __u8,
    pub mask: nf_inet_addr,
}
