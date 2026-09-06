//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7915/eeprom.h
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
// Copyright (C) 2020 MediaTek Inc.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_data {
    pub count: u8,
    pub offset: [u16; 60],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7915_eeprom_field {
    MT_EE_CHIP_ID =		0x000,
    MT_EE_VERSION =		0x002,
    MT_EE_MAC_ADDR =	0x004,
    MT_EE_MAC_ADDR2 =	0x00a,
    MT_EE_DDIE_FT_VERSION =	0x050,
    MT_EE_DO_PRE_CAL =	0x062,
    MT_EE_WIFI_CONF =	0x190,
    MT_EE_DO_PRE_CAL_V2 =	0x19a,
    MT_EE_RATE_DELTA_2G =	0x252,
    MT_EE_RATE_DELTA_5G =	0x29d,
    MT_EE_TX0_POWER_2G =	0x2fc,
    MT_EE_TX0_POWER_5G =	0x34b,
    MT_EE_RATE_DELTA_2G_V2 = 0x7d3,
    MT_EE_RATE_DELTA_5G_V2 = 0x81e,
    MT_EE_RATE_DELTA_6G_V2 = 0x884, /* 6g fields only appear in eeprom v2 */
    MT_EE_TX0_POWER_2G_V2 =	0x441,
    MT_EE_TX0_POWER_5G_V2 =	0x445,
    MT_EE_TX0_POWER_6G_V2 =	0x465,
    MT_EE_ADIE_FT_VERSION =	0x9a0,

    __MT_EE_MAX =		0xe00,
    __MT_EE_MAX_V2 =	0x1000,
// 0xe10 ~ 0x5780 used to save group cal data
    MT_EE_PRECAL =		0xe10,
    MT_EE_PRECAL_V2 =	0x1010
}

pub const MT_EE_CAL_UNIT: c_int = 1024;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7915_adie_sku {
    MT7976_ONE_ADIE_DBDC = 0x7,
    MT7975_ONE_ADIE	= 0x8,
    MT7976_ONE_ADIE	= 0xa,
    MT7975_DUAL_ADIE = 0xd,
    MT7976_DUAL_ADIE = 0xf,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7915_eeprom_band {
    MT_EE_BAND_SEL_DEFAULT,
    MT_EE_BAND_SEL_5GHZ,
    MT_EE_BAND_SEL_2GHZ,
    MT_EE_BAND_SEL_DUAL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7915_sku_rate_group {
    SKU_CCK,
    SKU_OFDM,
    SKU_HT_BW20,
    SKU_HT_BW40,
    SKU_VHT_BW20,
    SKU_VHT_BW40,
    SKU_VHT_BW80,
    SKU_VHT_BW160,
    SKU_HE_RU26,
    SKU_HE_RU52,
    SKU_HE_RU106,
    SKU_HE_RU242,
    SKU_HE_RU484,
    SKU_HE_RU996,
    SKU_HE_RU2x996,
    MAX_SKU_RATE_GROUP_NUM,
}

extern "C" {
    pub fn DIV_ROUND_UP(29: channel -, _arg: 32) -> return;
}
