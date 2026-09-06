//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/carl9170/eeprom.h
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


//
// Shared Atheros AR9170 Header
//
// EEPROM layout
//
// Copyright 2008, Johannes Berg <johannes@sipsolutions.net>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; see the file COPYING.  If not, see
// http://www.gnu.org/licenses/.
//
// This file incorporates work covered by the following copyright and
// permission notice:
// Copyright (c) 2007-2008 Atheros Communications, Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
pub const AR9170_EEPROM_START: c_uint = 0x1600;
pub const AR5416_MAX_CHAINS: c_int = 2;
pub const AR5416_MODAL_SPURS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_eeprom_modal {
    pub antCtrlChain: [__le32; AR5416_MAX_CHAINS],
    pub antCtrlCommon: __le32,
    pub antennaGainCh: [i8; AR5416_MAX_CHAINS],
    pub switchSettling: u8,
    pub txRxAttenCh: [u8; AR5416_MAX_CHAINS],
    pub rxTxMarginCh: [u8; AR5416_MAX_CHAINS],
    pub adcDesiredSize: i8,
    pub pgaDesiredSize: i8,
    pub xlnaGainCh: [u8; AR5416_MAX_CHAINS],
    pub txEndToXpaOff: u8,
    pub txEndToRxOn: u8,
    pub txFrameToXpaOn: u8,
    pub thresh62: u8,
    pub noiseFloorThreshCh: [i8; AR5416_MAX_CHAINS],
    pub xpdGain: u8,
    pub xpd: u8,
    pub iqCalICh: [i8; AR5416_MAX_CHAINS],
    pub iqCalQCh: [i8; AR5416_MAX_CHAINS],
    pub pdGainOverlap: u8,
    pub ob: u8,
    pub db: u8,
    pub xpaBiasLvl: u8,
    pub pwrDecreaseFor2Chain: u8,
    pub pwrDecreaseFor3Chain: u8,
    pub txFrameToDataStart: u8,
    pub txFrameToPaOn: u8,
    pub ht40PowerIncForPdadc: u8,
    pub bswAtten: [u8; AR5416_MAX_CHAINS],
    pub bswMargin: [u8; AR5416_MAX_CHAINS],
    pub swSettleHt40: u8,
    pub reserved: [u8; 22],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spur_channel {
    pub spurChan: __le16,
    pub spurRangeLow: u8,
    pub spurRangeHigh: u8,
    pub spur_channels: [} __packed; AR5416_MODAL_SPURS],
    pub __packed: },
pub const AR5416_NUM_PD_GAINS: c_int = 4;
pub const AR5416_PD_GAIN_ICEPTS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_calibration_data_per_freq {
    pub pwr_pdg: [u8; AR5416_NUM_PD_GAINS][AR5416_PD_GAIN_ICEPTS],
    pub vpd_pdg: [u8; AR5416_NUM_PD_GAINS][AR5416_PD_GAIN_ICEPTS],
    pub __packed: },
pub const AR5416_NUM_5G_CAL_PIERS: c_int = 8;
pub const AR5416_NUM_2G_CAL_PIERS: c_int = 4;
pub const AR5416_NUM_5G_TARGET_PWRS: c_int = 8;
pub const AR5416_NUM_2G_CCK_TARGET_PWRS: c_int = 3;
pub const AR5416_NUM_2G_OFDM_TARGET_PWRS: c_int = 4;
pub const AR5416_MAX_NUM_TGT_PWRS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_calibration_target_power_legacy {
    pub freq: u8,
    pub power: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_calibration_target_power_ht {
    pub freq: u8,
    pub power: [u8; 8],
    pub __packed: },
pub const AR5416_NUM_CTLS: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_calctl_edges {
    pub channel: u8,
pub const AR9170_CALCTL_EDGE_FLAGS: c_uint = 0xC0;
    pub power_flags: u8,
    pub __packed: },
pub const AR5416_NUM_BAND_EDGES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_calctl_data {
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_eeprom {
    pub length: __le16,
    pub checksum: __le16,
    pub version: __le16,
    pub operating_flags: u8,
pub const AR9170_OPFLAG_5GHZ: c_int = 1;
pub const AR9170_OPFLAG_2GHZ: c_int = 2;
    pub misc: u8,
    pub reg_domain: [__le16; 2],
    pub mac_address: [u8; 6],
    pub rx_mask: u8,
    pub tx_mask: u8,
    pub rf_silent: __le16,
    pub bluetooth_options: __le16,
    pub device_capabilities: __le16,
    pub build_number: __le32,
    pub deviceType: u8,
    pub reserved: [u8; 33],
    pub customer_data: [u8; 64],
    pub cal_freq_pier_5G: [u8; AR5416_NUM_5G_CAL_PIERS],
    pub cal_freq_pier_2G: [u8; AR5416_NUM_2G_CAL_PIERS],
// power calibration data
// conformance testing limits
    pub ctl_index: [u8; AR5416_NUM_CTLS],
    pub pad: u8,
    pub subsystem_id: __le16,
    pub __packed: },
pub const AR9170_LED_MODE_POWER_ON: c_uint = 0x0001;
pub const AR9170_LED_MODE_RESERVED: c_uint = 0x0002;
pub const AR9170_LED_MODE_DISABLE_STATE: c_uint = 0x0004;
pub const AR9170_LED_MODE_OFF_IN_PSM: c_uint = 0x0008;
// AR9170_LED_MODE BIT is set
pub const AR9170_LED_MODE_FREQUENCY_S: c_int = 4;
pub const AR9170_LED_MODE_FREQUENCY: c_uint = 0x0030;
pub const AR9170_LED_MODE_FREQUENCY_1HZ: c_uint = 0x0000;
pub const AR9170_LED_MODE_FREQUENCY_0_5HZ: c_uint = 0x0010;
pub const AR9170_LED_MODE_FREQUENCY_0_25HZ: c_uint = 0x0020;
pub const AR9170_LED_MODE_FREQUENCY_0_125HZ: c_uint = 0x0030;
// AR9170_LED_MODE BIT is not set
pub const AR9170_LED_MODE_CONN_STATE_S: c_int = 4;
pub const AR9170_LED_MODE_CONN_STATE: c_uint = 0x0030;
pub const AR9170_LED_MODE_CONN_STATE_FORCE_OFF: c_uint = 0x0000;
pub const AR9170_LED_MODE_CONN_STATE_FORCE_ON: c_uint = 0x0010;
// Idle off / Active on
pub const AR9170_LED_MODE_CONN_STATE_IOFF_AON: c_uint = 0x0020;
// Idle on / Active off
pub const AR9170_LED_MODE_CONN_STATE_ION_AOFF: c_uint = 0x0010;
pub const AR9170_LED_MODE_MODE: c_uint = 0x0040;
pub const AR9170_LED_MODE_RESERVED2: c_uint = 0x0080;
pub const AR9170_LED_MODE_TON_SCAN_S: c_int = 8;
pub const AR9170_LED_MODE_TON_SCAN: c_uint = 0x0f00;
pub const AR9170_LED_MODE_TOFF_SCAN_S: c_int = 12;
pub const AR9170_LED_MODE_TOFF_SCAN: c_uint = 0xf000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_led_mode {
    pub led: __le16,
}
