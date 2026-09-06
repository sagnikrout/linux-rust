//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/microchip/wilc1000/cfg80211.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2012 - 2018 Microchip Technology Inc., and its subsidiaries.
// All rights reserved.
//

extern "C" {
    pub fn wilc_cfg80211_register(wilc: *mut wilc) -> c_int;
}
extern "C" {
    pub fn wilc_deinit_host_int(net: *mut net_device);
}
extern "C" {
    pub fn wilc_init_host_int(net: *mut net_device) -> c_int;
}
extern "C" {
    pub fn wilc_wfi_monitor_rx(mon_dev: *mut net_device, buff: *mut u8, size: u32);
}
extern "C" {
    pub fn wilc_wfi_deinit_mon_interface(wl: *mut wilc, rtnl_locked: bool);
}
extern "C" {
    pub fn wlan_deinit_locks(wilc: *mut wilc);
}
