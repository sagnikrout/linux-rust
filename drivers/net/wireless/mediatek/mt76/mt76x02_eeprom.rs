//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt76x02_eeprom.h
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
// Copyright (C) 2018 Lorenzo Bianconi <lorenzo.bianconi83@gmail.com>
//

// Macro flag: #define __MT76x02_EEPROM_H

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76x02_eeprom_field {
    MT_EE_CHIP_ID =				0x000,
    MT_EE_VERSION =				0x002,
    MT_EE_MAC_ADDR =			0x004,
    MT_EE_PCI_ID =				0x00A,
    MT_EE_ANTENNA =				0x022,
    MT_EE_CFG1_INIT =			0x024,
    MT_EE_NIC_CONF_0 =			0x034,
    MT_EE_NIC_CONF_1 =			0x036,
    MT_EE_COUNTRY_REGION_5GHZ =		0x038,
    MT_EE_COUNTRY_REGION_2GHZ =		0x039,
    MT_EE_FREQ_OFFSET =			0x03a,
    MT_EE_NIC_CONF_2 =			0x042,

    MT_EE_XTAL_TRIM_1 =			0x03a,
    MT_EE_XTAL_TRIM_2 =			0x09e,

    MT_EE_LNA_GAIN =			0x044,
    MT_EE_RSSI_OFFSET_2G_0 =		0x046,
    MT_EE_RSSI_OFFSET_2G_1 =		0x048,
    MT_EE_LNA_GAIN_5GHZ_1 =			0x049,
    MT_EE_RSSI_OFFSET_5G_0 =		0x04a,
    MT_EE_RSSI_OFFSET_5G_1 =		0x04c,
    MT_EE_LNA_GAIN_5GHZ_2 =			0x04d,

    MT_EE_TX_POWER_DELTA_BW40 =		0x050,
    MT_EE_TX_POWER_DELTA_BW80 =		0x052,

    MT_EE_TX_POWER_EXT_PA_5G =		0x054,

    MT_EE_TX_POWER_0_START_2G =		0x056,
    MT_EE_TX_POWER_1_START_2G =		0x05c,

// used as byte arrays
pub const MT_TX_POWER_GROUP_SIZE_5G: c_int = 5;
pub const MT_TX_POWER_GROUPS_5G: c_int = 6;
    MT_EE_TX_POWER_0_START_5G =		0x062,
    MT_EE_TSSI_SLOPE_2G =			0x06e,

    MT_EE_TX_POWER_0_GRP3_TX_POWER_DELTA =	0x074,
    MT_EE_TX_POWER_0_GRP4_TSSI_SLOPE =	0x076,

    MT_EE_TX_POWER_1_START_5G =		0x080,

    MT_EE_TX_POWER_CCK =			0x0a0,
    MT_EE_TX_POWER_OFDM_2G_6M =		0x0a2,
    MT_EE_TX_POWER_OFDM_2G_24M =		0x0a4,
    MT_EE_TX_POWER_OFDM_5G_6M =		0x0b2,
    MT_EE_TX_POWER_OFDM_5G_24M =		0x0b4,
    MT_EE_TX_POWER_HT_MCS0 =		0x0a6,
    MT_EE_TX_POWER_HT_MCS4 =		0x0a8,
    MT_EE_TX_POWER_HT_MCS8 =		0x0aa,
    MT_EE_TX_POWER_HT_MCS12 =		0x0ac,
    MT_EE_TX_POWER_VHT_MCS8 =		0x0be,

    MT_EE_2G_TARGET_POWER =			0x0d0,
    MT_EE_TEMP_OFFSET =			0x0d1,
    MT_EE_5G_TARGET_POWER =			0x0d2,
    MT_EE_TSSI_BOUND1 =			0x0d4,
    MT_EE_TSSI_BOUND2 =			0x0d6,
    MT_EE_TSSI_BOUND3 =			0x0d8,
    MT_EE_TSSI_BOUND4 =			0x0da,
    MT_EE_FREQ_OFFSET_COMPENSATION =	0x0db,
    MT_EE_TSSI_BOUND5 =			0x0dc,
    MT_EE_TX_POWER_BYRATE_BASE =		0x0de,

    MT_EE_TSSI_SLOPE_5G =			0x0f0,
    MT_EE_RF_TEMP_COMP_SLOPE_5G =		0x0f2,
    MT_EE_RF_TEMP_COMP_SLOPE_2G =		0x0f4,

    MT_EE_RF_2G_TSSI_OFF_TXPOWER =		0x0f6,
    MT_EE_RF_2G_RX_HIGH_GAIN =		0x0f8,
    MT_EE_RF_5G_GRP0_1_RX_HIGH_GAIN =	0x0fa,
    MT_EE_RF_5G_GRP2_3_RX_HIGH_GAIN =	0x0fc,
    MT_EE_RF_5G_GRP4_5_RX_HIGH_GAIN =	0x0fe,

    MT_EE_BT_RCAL_RESULT =			0x138,
    MT_EE_BT_VCDL_CALIBRATION =		0x13c,
    MT_EE_BT_PMUCFG =			0x13e,

    MT_EE_USAGE_MAP_START =			0x1e0,
    MT_EE_USAGE_MAP_END =			0x1fc,

    __MT_EE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76x02_eeprom_modes {
    MT_EE_READ,
    MT_EE_PHYSICAL_READ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76x02_board_type {
    BOARD_TYPE_2GHZ = 1,
    BOARD_TYPE_5GHZ = 2,
}

extern "C" {
    pub fn mt76x02_sign_extend_optional(_arg: val, _arg: 7) -> return;
}
extern "C" {
    pub fn get_unaligned_le16(field: dev->mt76.eeprom.data +) -> return;
}
extern "C" {
    pub fn mt76x02_ext_pa_enabled(dev: *mut mt76x02_dev, band: nl80211_band) -> bool;
}
extern "C" {
    pub fn mt76x02_eeprom_parse_hw_cap(dev: *mut mt76x02_dev);
}
