//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt76x2/mt76x2.h
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
// Copyright (C) 2016 Felix Fietkau <nbd@nbd.name>
//

// Macro flag: #define __MT76x2_H

pub const MT7662_EEPROM_SIZE: c_int = 512;

extern "C" {
    pub fn mt76x2_register_device(dev: *mut mt76x02_dev) -> c_int;
}
extern "C" {
    pub fn mt76x2_resume_device(dev: *mut mt76x02_dev) -> c_int;
}
extern "C" {
    pub fn mt76x2_phy_power_on(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x2_stop_hardware(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x2_eeprom_init(dev: *mut mt76x02_dev) -> c_int;
}
extern "C" {
    pub fn mt76x2_apply_calibration_data(dev: *mut mt76x02_dev, channel: c_int) -> c_int;
}
extern "C" {
    pub fn mt76x2e_set_channel(phy: *mut mt76_phy) -> c_int;
}
extern "C" {
    pub fn mt76x2u_set_channel(phy: *mut mt76_phy) -> c_int;
}
extern "C" {
    pub fn mt76x2_phy_set_antenna(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x2_phy_start(dev: *mut mt76x02_dev) -> c_int;
}
extern "C" {
    pub fn mt76x2_phy_calibrate(work: *mut work_struct);
}
extern "C" {
    pub fn mt76x2_phy_set_txpower(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x2_mcu_init(dev: *mut mt76x02_dev) -> c_int;
}
extern "C" {
    pub fn mt76x2_cleanup(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x2_mac_reset(dev: *mut mt76x02_dev, hard: bool) -> c_int;
}
extern "C" {
    pub fn mt76x2_reset_wlan(dev: *mut mt76x02_dev, enable: bool);
}
extern "C" {
    pub fn mt76_write_mac_initvals(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x2_phy_tssi_compensate(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x2_apply_gain_adj(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x2_phy_update_channel_gain(dev: *mut mt76x02_dev);
}
