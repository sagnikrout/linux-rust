//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/st/cw1200/pm.h
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
// Mac80211 power management interface for ST-Ericsson CW1200 mac80211 drivers
//
// Copyright (c) 2011, ST-Ericsson
// Author: Dmitry Tarnyagin <dmitry.tarnyagin@lockless.no>
//

// Macro flag: #define PM_H_INCLUDED
// ********************************************************************
// mac80211 API
// extern */  struct cw1200_common;
// private */ struct cw1200_suspend_state;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cw1200_pm_state {
    pub suspend_state: *mut cw1200_suspend_state,
    pub stay_awake: timer_list,
    pub pm_dev: *mut platform_device,
    pub /: *mut *mut spinlock_t lock; / Protect access,
}

extern "C" {
    pub fn cw1200_pm_deinit(pm: *mut cw1200_pm_state);
}
extern "C" {
    pub fn cw1200_can_suspend(priv: *mut cw1200_common) -> c_int;
}
extern "C" {
    pub fn cw1200_wow_resume(hw: *mut ieee80211_hw) -> c_int;
}

