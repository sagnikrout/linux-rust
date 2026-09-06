//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_rateest.h
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
pub enum xt_rateest_match_flags {
    XT_RATEEST_MATCH_INVERT	= 1<<0,
    XT_RATEEST_MATCH_ABS	= 1<<1,
    XT_RATEEST_MATCH_REL	= 1<<2,
    XT_RATEEST_MATCH_DELTA	= 1<<3,
    XT_RATEEST_MATCH_BPS	= 1<<4,
    XT_RATEEST_MATCH_PPS	= 1<<5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xt_rateest_match_mode {
    XT_RATEEST_MATCH_NONE,
    XT_RATEEST_MATCH_EQ,
    XT_RATEEST_MATCH_LT,
    XT_RATEEST_MATCH_GT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_rateest_match_info {
    pub name1: [c_char; IFNAMSIZ],
    pub name2: [c_char; IFNAMSIZ],
    pub flags: __u16,
    pub mode: __u16,
    pub bps1: __u32,
    pub pps1: __u32,
    pub bps2: __u32,
    pub pps2: __u32,
// Used internally by the kernel
    pub __attribute__((aligned(8))): *mut *mut xt_rateest est1,
    pub __attribute__((aligned(8))): *mut *mut xt_rateest est2,
}
