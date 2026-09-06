//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_statistic.h
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
pub enum xt_statistic_mode {
    XT_STATISTIC_MODE_RANDOM,
    XT_STATISTIC_MODE_NTH,
    __XT_STATISTIC_MODE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xt_statistic_flags {
    XT_STATISTIC_INVERT		= 0x1,
}

pub const XT_STATISTIC_MASK: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_statistic_info {
    pub mode: __u16,
    pub flags: __u16,
    pub probability: __u32,
    pub random: },
    pub every: __u32,
    pub packet: __u32,
    pub /: *mut *mut __u32 count; / unused,
    pub nth: },
    pub u: },
    pub __attribute__((aligned(8))): *mut *mut xt_statistic_priv master,
}
