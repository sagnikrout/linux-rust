//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7603/eeprom.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7603_eeprom_field {
    MT_EE_CHIP_ID =				0x000,
    MT_EE_VERSION =				0x002,
    MT_EE_MAC_ADDR =			0x004,
    MT_EE_NIC_CONF_0 =			0x034,
    MT_EE_NIC_CONF_1 =			0x036,
    MT_EE_NIC_CONF_2 =			0x042,

    MT_EE_XTAL_TRIM_1 =			0x03a,

    MT_EE_RSSI_OFFSET_2G =			0x046,
    MT_EE_WIFI_RF_SETTING =			0x048,
    MT_EE_RSSI_OFFSET_5G =			0x04a,

    MT_EE_TX_POWER_DELTA_BW40 =		0x050,
    MT_EE_TX_POWER_DELTA_BW80 =		0x052,

    MT_EE_TX_POWER_EXT_PA_5G =		0x054,

    MT_EE_TEMP_SENSOR_CAL =			0x055,

    MT_EE_TX_POWER_0_START_2G =		0x056,
    MT_EE_TX_POWER_1_START_2G =		0x05c,

// used as byte arrays
pub const MT_TX_POWER_GROUP_SIZE_5G: c_int = 5;
pub const MT_TX_POWER_GROUPS_5G: c_int = 6;
    MT_EE_TX_POWER_0_START_5G =		0x062,

    MT_EE_TX_POWER_0_GRP3_TX_POWER_DELTA =	0x074,
    MT_EE_TX_POWER_0_GRP4_TSSI_SLOPE =	0x076,

    MT_EE_TX_POWER_1_START_5G =		0x080,

    MT_EE_TX_POWER_CCK =			0x0a0,
    MT_EE_TX_POWER_OFDM_2G_6M =		0x0a2,
    MT_EE_TX_POWER_OFDM_2G_24M =		0x0a4,
    MT_EE_TX_POWER_OFDM_2G_54M =		0x0a6,
    MT_EE_TX_POWER_HT_BPSK_QPSK =		0x0a8,
    MT_EE_TX_POWER_HT_16_64_QAM =		0x0aa,
    MT_EE_TX_POWER_HT_64_QAM =		0x0ac,

    MT_EE_ELAN_RX_MODE_GAIN =		0x0c0,
    MT_EE_ELAN_RX_MODE_NF =			0x0c1,
    MT_EE_ELAN_RX_MODE_P1DB =		0x0c2,

    MT_EE_ELAN_BYPASS_MODE_GAIN =		0x0c3,
    MT_EE_ELAN_BYPASS_MODE_NF =		0x0c4,
    MT_EE_ELAN_BYPASS_MODE_P1DB =		0x0c5,

    MT_EE_STEP_NUM_NEG_6_7 =		0x0c6,
    MT_EE_STEP_NUM_NEG_4_5 =		0x0c8,
    MT_EE_STEP_NUM_NEG_2_3 =		0x0ca,
    MT_EE_STEP_NUM_NEG_0_1 =		0x0cc,

    MT_EE_REF_STEP_24G =			0x0ce,

    MT_EE_STEP_NUM_PLUS_1_2 =		0x0d0,
    MT_EE_STEP_NUM_PLUS_3_4 =		0x0d2,
    MT_EE_STEP_NUM_PLUS_5_6 =		0x0d4,
    MT_EE_STEP_NUM_PLUS_7 =			0x0d6,

    MT_EE_CP_FT_VERSION =			0x0f0,

    MT_EE_TX_POWER_TSSI_OFF =		0x0f2,

    MT_EE_XTAL_FREQ_OFFSET =		0x0f4,
    MT_EE_XTAL_TRIM_2_COMP =		0x0f5,
    MT_EE_XTAL_TRIM_3_COMP =		0x0f6,
    MT_EE_XTAL_WF_RFCAL =			0x0f7,

    __MT_EE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7603_eeprom_source {
    MT_EE_SRC_PROM,
    MT_EE_SRC_EFUSE,
    MT_EE_SRC_FLASH,
}

