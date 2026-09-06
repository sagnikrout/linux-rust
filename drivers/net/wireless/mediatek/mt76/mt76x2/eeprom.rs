//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt76x2/eeprom.h
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

// Macro flag: #define __MT76x2_EEPROM_H

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76x2_cal_channel_group {
    MT_CH_5G_JAPAN,
    MT_CH_5G_UNII_1,
    MT_CH_5G_UNII_2,
    MT_CH_5G_UNII_2E_1,
    MT_CH_5G_UNII_2E_2,
    MT_CH_5G_UNII_3,
    __MT_CH_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x2_tx_power_info {
    pub target_power: u8,
    pub delta_bw40: i8,
    pub delta_bw80: i8,
    pub tssi_slope: i8,
    pub tssi_offset: i8,
    pub target_power: i8,
    pub delta: i8,
    pub chain: [}; MT_MAX_CHAINS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x2_temp_comp {
    pub temp_25_ref: u8,
    pub /: *mut *mut int lower_bound; / J,
    pub /: *mut *mut int upper_bound; / J,
    pub /: *mut *mut unsigned int high_slope; / J / dB,
    pub /: *mut *mut unsigned int low_slope; / J / dB,
}

extern "C" {
    pub fn mt76x2_get_temp_comp(dev: *mut mt76x02_dev, t: *mut mt76x2_temp_comp) -> c_int;
}
extern "C" {
    pub fn mt76x2_read_rx_gain(dev: *mut mt76x02_dev);
}
