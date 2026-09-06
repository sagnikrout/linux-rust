//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_IDLETIMER.h
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


// SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note
//
// Header file for Xtables timer target module.
//
// Copyright (C) 2004, 2010 Nokia Corporation
// Written by Timo Teras <ext-timo.teras@nokia.com>
//
// Converted to x_tables and forward-ported to 2.6.34
// by Luciano Coelho <luciano.coelho@nokia.com>
//
// Contact: Luciano Coelho <luciano.coelho@nokia.com>
//

pub const MAX_IDLETIMER_LABEL_SIZE: c_int = 28;
pub const XT_IDLETIMER_ALARM: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idletimer_tg_info {
    pub timeout: __u32,
    pub label: [c_char; MAX_IDLETIMER_LABEL_SIZE],
// for kernel module internal use only
    pub __attribute__((aligned(8))): *mut *mut idletimer_tg timer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idletimer_tg_info_v1 {
    pub timeout: __u32,
    pub label: [c_char; MAX_IDLETIMER_LABEL_SIZE],
    pub /: *mut *mut __u8 send_nl_msg; / unused: for compatibility with Android,
    pub timer_type: __u8,
// for kernel module internal use only
    pub __attribute__((aligned(8))): *mut *mut idletimer_tg timer,
}
