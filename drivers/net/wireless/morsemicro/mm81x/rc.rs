//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/morsemicro/mm81x/rc.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2017-2026 Morse Micro
//

pub const INIT_MAX_RATES_NUM: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_rc {
// Serialise rate control queue manipulation and timer functions
    pub lock: spinlock_t,
    pub stas: list_head,
    pub timer: timer_list,
    pub work: work_struct,
    pub mors: *mut mm81x,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_rc_sta {
    pub tb: *mut mmrc_table,
    pub list: list_head,
    pub last_update: c_ulong,
}

extern "C" {
    pub fn mm81x_rc_init(mors: *mut mm81x);
}
extern "C" {
    pub fn mm81x_rc_deinit(mors: *mut mm81x);
}
extern "C" {
    pub fn mm81x_rc_sta_remove(mors: *mut mm81x, sta: *mut ieee80211_sta);
}
