//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt76x0/mt76x0.h
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
// Copyright (C) 2014 Felix Fietkau <nbd@openwrt.org>
// Copyright (C) 2015 Jakub Kicinski <kubakici@wp.pl>
// Copyright (C) 2018 Stanislaw Gruszka <stf_xl@wp.pl>
//

pub const MT_USB_AGGR_TIMEOUT: c_uint = 0x80 /* * 33ns */;
// Init
extern "C" {
    pub fn mt76x0_init_hardware(dev: *mut mt76x02_dev) -> c_int;
}
extern "C" {
    pub fn mt76x0_register_device(dev: *mut mt76x02_dev) -> c_int;
}
extern "C" {
    pub fn mt76x0_chip_onoff(dev: *mut mt76x02_dev, enable: bool, reset: bool);
}
extern "C" {
    pub fn mt76x0_mac_stop(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x0_config(hw: *mut ieee80211_hw, radio_idx: c_int, changed: u32) -> c_int;
}
extern "C" {
    pub fn mt76x0_set_channel(mphy: *mut mt76_phy) -> c_int;
}
// PHY
extern "C" {
    pub fn mt76x0_phy_init(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x0_phy_wait_bbp_ready(dev: *mut mt76x02_dev) -> c_int;
}
extern "C" {
    pub fn mt76x0_phy_set_txpower(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x0_phy_calibrate(dev: *mut mt76x02_dev, power_on: bool);
}
