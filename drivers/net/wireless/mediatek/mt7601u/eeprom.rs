//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt7601u/eeprom.h
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
//
pub const MT7601U_EE_MAX_VER: c_uint = 0x0d;
pub const MT7601U_EEPROM_SIZE: c_int = 256;
pub const MT7601U_DEFAULT_TX_POWER: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_eeprom_field {
    MT_EE_CHIP_ID =				0x00,
    MT_EE_VERSION_FAE =			0x02,
    MT_EE_VERSION_EE =			0x03,
    MT_EE_MAC_ADDR =			0x04,
    MT_EE_NIC_CONF_0 =			0x34,
    MT_EE_NIC_CONF_1 =			0x36,
    MT_EE_COUNTRY_REGION =			0x39,
    MT_EE_FREQ_OFFSET =			0x3a,
    MT_EE_NIC_CONF_2 =			0x42,

    MT_EE_LNA_GAIN =			0x44,
    MT_EE_RSSI_OFFSET =			0x46,

    MT_EE_TX_POWER_DELTA_BW40 =		0x50,
    MT_EE_TX_POWER_OFFSET =			0x52,

    MT_EE_TX_TSSI_SLOPE =			0x6e,
    MT_EE_TX_TSSI_OFFSET_GROUP =		0x6f,
    MT_EE_TX_TSSI_OFFSET =			0x76,

    MT_EE_TX_TSSI_TARGET_POWER =		0xd0,
    MT_EE_REF_TEMP =			0xd1,
    MT_EE_FREQ_OFFSET_COMPENSATION =	0xdb,
    MT_EE_TX_POWER_BYRATE_BASE =		0xde,

    MT_EE_USAGE_MAP_START =			0x1e0,
    MT_EE_USAGE_MAP_END =			0x1fc,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7601u_eeprom_access_modes {
    MT_EE_READ = 0,
    MT_EE_PHYSICAL_READ = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_per_rate {
    pub /: *mut *mut u8 raw; / validated s6 value,
    pub /: *mut *mut s8 bw20; / sign-extended int,
    pub /: *mut *mut s8 bw40; / sign-extended int,
}

// Power per rate - one value per two rates
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7601u_rate_power {
    pub cck: [power_per_rate; 2],
    pub ofdm: [power_per_rate; 4],
    pub ht: [power_per_rate; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_channel_bounds {
    pub start: u8,
    pub num: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7601u_eeprom_params {
    pub tssi_enabled: bool,
    pub rf_freq_off: u8,
    pub rssi_offset: [i8; 2],
    pub ref_temp: i8,
    pub lna_gain: i8,
    pub chan_pwr: [u8; 14],
    pub power_rate_table: mt7601u_rate_power,
    pub real_cck_bw20: [i8; 2],
// TSSI stuff - only with internal TX ALC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tssi_data {
    pub tx0_delta_offset: c_int,
    pub slope: u8,
    pub offset: [u8; 3],
    pub tssi_data: },
    pub reg: reg_channel_bounds,
}

extern "C" {
    pub fn mt7601u_eeprom_init(dev: *mut mt7601u_dev) -> c_int;
}
