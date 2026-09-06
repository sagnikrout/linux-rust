//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7615/eeprom.h
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
// Copyright (C) 2019 MediaTek Inc.

pub const MT7615_EEPROM_DCOC_SIZE: c_int = 256;
pub const MT7615_EEPROM_DCOC_COUNT: c_int = 34;

pub const MT7615_EEPROM_TXDPD_SIZE: c_int = 216;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7615_eeprom_field {
    MT_EE_CHIP_ID =				0x000,
    MT_EE_VERSION =				0x002,
    MT_EE_MAC_ADDR =			0x004,
    MT_EE_NIC_CONF_0 =			0x034,
    MT_EE_NIC_CONF_1 =			0x036,
    MT_EE_WIFI_CONF =			0x03e,
    MT_EE_CALDATA_FLASH =			0x052,
    MT_EE_TX0_2G_TARGET_POWER =		0x058,
    MT_EE_TX0_5G_G0_TARGET_POWER =		0x070,
    MT7663_EE_5G_RATE_POWER =		0x089,
    MT_EE_TX1_5G_G0_TARGET_POWER =		0x098,
    MT_EE_2G_RATE_POWER =			0x0be,
    MT_EE_5G_RATE_POWER =			0x0d5,
    MT7663_EE_TX0_2G_TARGET_POWER =		0x0e3,
    MT_EE_EXT_PA_2G_TARGET_POWER =		0x0f2,
    MT_EE_EXT_PA_5G_TARGET_POWER =		0x0f3,
    MT_EE_TX2_5G_G0_TARGET_POWER =		0x142,
    MT_EE_TX3_5G_G0_TARGET_POWER =		0x16a,
    MT7663_EE_HW_CONF1 =			0x1b0,
    MT7663_EE_TX0_5G_G0_TARGET_POWER =	0x245,
    MT7663_EE_TX1_5G_G0_TARGET_POWER =	0x2b5,

    MT7615_EE_MAX =				0x3bf,
    MT7622_EE_MAX =				0x3db,
    MT7663_EE_MAX =				0x600,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7615_eeprom_band {
    MT_EE_DUAL_BAND,
    MT_EE_5GHZ,
    MT_EE_2GHZ,
    MT_EE_DBDC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7615_channel_group {
    MT_CH_5G_JAPAN,
    MT_CH_5G_UNII_1,
    MT_CH_5G_UNII_2A,
    MT_CH_5G_UNII_2B,
    MT_CH_5G_UNII_2E_1,
    MT_CH_5G_UNII_2E_2,
    MT_CH_5G_UNII_2E_3,
    MT_CH_5G_UNII_3,
    __MT_CH_MAX
}
