//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt76x02_phy.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (C) 2018 Lorenzo Bianconi <lorenzo.bianconi83@gmail.com>
//

// Macro flag: #define __MT76x02_PHY_H

extern "C" {
    pub fn mt76x02_add_rate_power_offset(r: *mut mt76x02_rate_power, offset: c_int);
}
extern "C" {
    pub fn mt76x02_phy_set_txpower(dev: *mut mt76x02_dev, txp_0: c_int, txp_2: c_int);
}
extern "C" {
    pub fn mt76x02_limit_rate_power(r: *mut mt76x02_rate_power, limit: c_int);
}
extern "C" {
    pub fn mt76x02_get_max_rate_power(r: *mut mt76x02_rate_power) -> c_int;
}
extern "C" {
    pub fn mt76x02_phy_set_rxpath(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x02_phy_set_txdac(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x02_phy_set_bw(dev: *mut mt76x02_dev, width: c_int, ctrl: u8);
}
extern "C" {
    pub fn mt76x02_phy_adjust_vga_gain(dev: *mut mt76x02_dev) -> bool;
}
extern "C" {
    pub fn mt76x02_init_agc_gain(dev: *mut mt76x02_dev);
}
