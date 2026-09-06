//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7996/eeprom.h
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
// Copyright (C) 2022 MediaTek Inc.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7996_eeprom_field {
    MT_EE_CHIP_ID =		0x000,
    MT_EE_VERSION =		0x002,
    MT_EE_MAC_ADDR =	0x004,
    MT_EE_MAC_ADDR2 =	0x00a,
    MT_EE_WIFI_CONF =	0x190,
    MT_EE_MAC_ADDR3 =	0x2c0,
    MT_EE_RATE_DELTA_2G =	0x1400,
    MT_EE_RATE_DELTA_5G =	0x147d,
    MT_EE_RATE_DELTA_6G =	0x154a,
    MT_EE_TX0_POWER_2G =	0x1300,
    MT_EE_TX0_POWER_5G =	0x1301,
    MT_EE_TX0_POWER_6G =	0x1310,

    __MT_EE_MAX =	0x1dff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7996_eeprom_band {
    MT_EE_BAND_SEL_DEFAULT,
    MT_EE_BAND_SEL_2GHZ,
    MT_EE_BAND_SEL_5GHZ,
    MT_EE_BAND_SEL_6GHZ,
}

extern "C" {
    pub fn DIV_ROUND_UP(29: channel -, _arg: 32) -> return;
}
