//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_connbytes.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xt_connbytes_what {
    XT_CONNBYTES_PKTS,
    XT_CONNBYTES_BYTES,
    XT_CONNBYTES_AVGPKT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xt_connbytes_direction {
    XT_CONNBYTES_DIR_ORIGINAL,
    XT_CONNBYTES_DIR_REPLY,
    XT_CONNBYTES_DIR_BOTH,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_connbytes_info {
    pub /: *mut *mut __aligned_u64 from; / count to be matched,
    pub /: *mut *mut __aligned_u64 to; / count to be matched,
    pub count: },
    pub /: *mut *mut __u8 what; / ipt_connbytes_what,
    pub /: *mut *mut __u8 direction; / ipt_connbytes_direction,
}
