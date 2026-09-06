//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/ar9003_eeprom.h
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
// Copyright (c) 2010-2011 Atheros Communications Inc.
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

pub const AR9300_EEP_VER: c_uint = 0xD000;
pub const AR9300_EEP_VER_MINOR_MASK: c_uint = 0xFFF;
pub const AR9300_EEP_MINOR_VER_1: c_uint = 0x1;

// 16-bit offset location start of calibration struct
pub const AR9300_EEP_START_LOC: c_int = 256;
pub const AR9300_NUM_5G_CAL_PIERS: c_int = 8;
pub const AR9300_NUM_2G_CAL_PIERS: c_int = 3;
pub const AR9300_NUM_5G_20_TARGET_POWERS: c_int = 8;
pub const AR9300_NUM_5G_40_TARGET_POWERS: c_int = 8;
pub const AR9300_NUM_2G_CCK_TARGET_POWERS: c_int = 2;
pub const AR9300_NUM_2G_20_TARGET_POWERS: c_int = 3;
pub const AR9300_NUM_2G_40_TARGET_POWERS: c_int = 3;
// #define AR9300_NUM_CTLS              21
pub const AR9300_NUM_CTLS_5G: c_int = 9;
pub const AR9300_NUM_CTLS_2G: c_int = 12;
pub const AR9300_NUM_BAND_EDGES_5G: c_int = 8;
pub const AR9300_NUM_BAND_EDGES_2G: c_int = 4;
pub const AR9300_EEPMISC_WOW: c_uint = 0x02;
pub const AR9300_CUSTOMER_DATA_SIZE: c_int = 20;
pub const AR9300_MAX_CHAINS: c_int = 3;
pub const AR9300_ANT_16S: c_int = 25;
pub const AR9300_FUTURE_MODAL_SZ: c_int = 6;
pub const AR9300_PAPRD_RATE_MASK: c_uint = 0x01ffffff;
pub const AR9300_PAPRD_SCALE_1: c_uint = 0x0e000000;
pub const AR9300_PAPRD_SCALE_1_S: c_int = 25;
pub const AR9300_PAPRD_SCALE_2: c_uint = 0x70000000;
pub const AR9300_PAPRD_SCALE_2_S: c_int = 28;
pub const AR9300_EEP_ANTDIV_CONTROL_DEFAULT_VALUE: c_uint = 0xc9;
// Delta from which to start power to pdadc table
// This offset is used in both open loop and closed loop power control
// schemes. In open loop power control, it is not really needed, but for
// the "sake of consistency" it was kept. For certain AP designs, this
// value is overwritten by the value in the flag "pwrTableOffset" just
// before writing the pdadc vs pwr into the chip registers.
//
pub const AR9300_PWR_TABLE_OFFSET: c_int = 0;
// Noise power data definitions
// units are: 4 x dBm - NOISE_PWR_DATA_OFFSET
// (e.g. -25 = (-25/4 - 90) = -96.25 dBm)
// range (for 6 signed bits) is (-32 to 31) + offset => -122dBm to -59dBm
// resolution (2 bits) is 0.25dBm
//

// byte addressable

pub const AR9300_BASE_ADDR_4K: c_uint = 0xfff;
pub const AR9300_BASE_ADDR: c_uint = 0x3ff;
pub const AR9300_BASE_ADDR_512: c_uint = 0x1ff;
// AR5416_EEPMISC_BIG_ENDIAN not set indicates little endian
pub const AR9300_EEPMISC_LITTLE_ENDIAN: c_int = 0;

pub const AR9300_OTP_STATUS_TYPE: c_uint = 0x7;
pub const AR9300_OTP_STATUS_VALID: c_uint = 0x4;
pub const AR9300_OTP_STATUS_ACCESS_BUSY: c_uint = 0x2;
pub const AR9300_OTP_STATUS_SM_BUSY: c_uint = 0x1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum targetPowerHTRates {
    HT_TARGET_RATE_0_8_16,
    HT_TARGET_RATE_1_3_9_11_17_19,
    HT_TARGET_RATE_4,
    HT_TARGET_RATE_5,
    HT_TARGET_RATE_6,
    HT_TARGET_RATE_7,
    HT_TARGET_RATE_12,
    HT_TARGET_RATE_13,
    HT_TARGET_RATE_14,
    HT_TARGET_RATE_15,
    HT_TARGET_RATE_20,
    HT_TARGET_RATE_21,
    HT_TARGET_RATE_22,
    HT_TARGET_RATE_23
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum targetPowerLegacyRates {
    LEGACY_TARGET_RATE_6_24,
    LEGACY_TARGET_RATE_36,
    LEGACY_TARGET_RATE_48,
    LEGACY_TARGET_RATE_54
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum targetPowerCckRates {
    LEGACY_TARGET_RATE_1L_5L,
    LEGACY_TARGET_RATE_5S,
    LEGACY_TARGET_RATE_11L,
    LEGACY_TARGET_RATE_11S
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ar9300_Rates {
    ALL_TARGET_LEGACY_6_24,
    ALL_TARGET_LEGACY_36,
    ALL_TARGET_LEGACY_48,
    ALL_TARGET_LEGACY_54,
    ALL_TARGET_LEGACY_1L_5L,
    ALL_TARGET_LEGACY_5S,
    ALL_TARGET_LEGACY_11L,
    ALL_TARGET_LEGACY_11S,
    ALL_TARGET_HT20_0_8_16,
    ALL_TARGET_HT20_1_3_9_11_17_19,
    ALL_TARGET_HT20_4,
    ALL_TARGET_HT20_5,
    ALL_TARGET_HT20_6,
    ALL_TARGET_HT20_7,
    ALL_TARGET_HT20_12,
    ALL_TARGET_HT20_13,
    ALL_TARGET_HT20_14,
    ALL_TARGET_HT20_15,
    ALL_TARGET_HT20_20,
    ALL_TARGET_HT20_21,
    ALL_TARGET_HT20_22,
    ALL_TARGET_HT20_23,
    ALL_TARGET_HT40_0_8_16,
    ALL_TARGET_HT40_1_3_9_11_17_19,
    ALL_TARGET_HT40_4,
    ALL_TARGET_HT40_5,
    ALL_TARGET_HT40_6,
    ALL_TARGET_HT40_7,
    ALL_TARGET_HT40_12,
    ALL_TARGET_HT40_13,
    ALL_TARGET_HT40_14,
    ALL_TARGET_HT40_15,
    ALL_TARGET_HT40_20,
    ALL_TARGET_HT40_21,
    ALL_TARGET_HT40_22,
    ALL_TARGET_HT40_23,
    ar9300RateSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eepFlags {
    pub opFlags: u8,
    pub eepMisc: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CompressAlgorithm {
    _CompressNone = 0,
    _CompressLzma,
    _CompressPairs,
    _CompressBlock,
    _Compress4,
    _Compress5,
    _Compress6,
    _Compress7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9300_base_eep_hdr {
    pub regDmn: [__le16; 2],
// 4 bits tx and 4 bits rx
    pub txrxMask: u8,
    pub opCapFlags: eepFlags,
    pub rfSilent: u8,
    pub blueToothOptions: u8,
    pub deviceCap: u8,
// takes lower byte in eeprom location
    pub deviceType: u8,
// offset in dB to be added to beginning
// of pdadc table in calibration
//
    pub pwrTableOffset: i8,
    pub params_for_tuning_caps: [u8; 2],
//
// bit0 - enable tx temp comp
// bit1 - enable tx volt comp
// bit2 - enable fastClock - default to 1
// bit3 - enable doubling - default to 1
// bit4 - enable internal regulator - default to 1
//
    pub featureEnable: u8,
// misc flags: bit0 - turn down drivestrength
    pub miscConfiguration: u8,
    pub eepromWriteEnableGpio: u8,
    pub wlanDisableGpio: u8,
    pub wlanLedGpio: u8,
    pub rxBandSelectGpio: u8,
    pub txrxgain: u8,
// SW controlled internal regulator fields
    pub swreg: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9300_modal_eep_header {
// 4 idle, t1, t2, b (4 bits per setting)
    pub antCtrlCommon: __le32,
// 4 ra1l1, ra2l1, ra1l2, ra2l2, ra12
    pub antCtrlCommon2: __le32,
// 6 idle, t, r, rx1, rx12, b (2 bits each)
    pub antCtrlChain: [__le16; AR9300_MAX_CHAINS],
// 3 xatten1_db for AR9280 (0xa20c/b20c 5:0)
    pub xatten1DB: [u8; AR9300_MAX_CHAINS],
// 3  xatten1_margin for merlin (0xa20c/b20c 16:12
    pub xatten1Margin: [u8; AR9300_MAX_CHAINS],
    pub tempSlope: i8,
    pub voltSlope: i8,
// spur channels in usual fbin coding format
    pub spurChans: [u8; AR_EEPROM_MODAL_SPURS],
// 3  Check if the register is per chain
    pub noiseFloorThreshCh: [i8; AR9300_MAX_CHAINS],
    pub reserved: [u8; 11],
    pub quick_drop: i8,
    pub xpaBiasLvl: u8,
    pub txFrameToDataStart: u8,
    pub txFrameToPaOn: u8,
    pub txClip: u8,
    pub antennaGain: i8,
    pub switchSettling: u8,
    pub adcDesiredSize: i8,
    pub txEndToXpaOff: u8,
    pub txEndToRxOn: u8,
    pub txFrameToXpaOn: u8,
    pub thresh62: u8,
    pub papdRateMaskHt20: __le32,
    pub papdRateMaskHt40: __le32,
    pub switchcomspdt: __le16,
    pub xlna_bias_strength: u8,
    pub futureModal: [u8; 7],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9300_cal_data_per_freq_op_loop {
    pub refPower: i8,
// pdadc voltage at power measurement
    pub voltMeas: u8,
// pcdac used for power measurement
    pub tempMeas: u8,
// range is -60 to -127 create a mapping equation 1db resolution
    pub rxNoisefloorCal: i8,
// range is same as noisefloor
    pub rxNoisefloorPower: i8,
// temp measured when noisefloor cal was performed
    pub rxTempMeas: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_tgt_pow_legacy {
    pub tPow2x: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_tgt_pow_ht {
    pub tPow2x: [u8; 14],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_ctl_data_2g {
    pub ctlEdges: [u8; AR9300_NUM_BAND_EDGES_2G],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_ctl_data_5g {
    pub ctlEdges: [u8; AR9300_NUM_BAND_EDGES_5G],
    pub __packed: },
pub const MAX_BASE_EXTENSION_FUTURE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9300_BaseExtension_1 {
    pub ant_div_control: u8,
    pub future: [u8; MAX_BASE_EXTENSION_FUTURE],
//
// misc_enable:
//
// BIT 0   - TX Gain Cap enable.
// BIT 1   - Uncompressed Checksum enable.
// BIT 2/3 - MinCCApwr enable 2g/5g.
//
    pub misc_enable: u8,
    pub tempslopextension: [i8; 8],
    pub quick_drop_low: i8,
    pub quick_drop_high: i8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9300_BaseExtension_2 {
    pub tempSlopeLow: i8,
    pub tempSlopeHigh: i8,
    pub xatten1DBLow: [u8; AR9300_MAX_CHAINS],
    pub xatten1MarginLow: [u8; AR9300_MAX_CHAINS],
    pub xatten1DBHigh: [u8; AR9300_MAX_CHAINS],
    pub xatten1MarginHigh: [u8; AR9300_MAX_CHAINS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9300_eeprom {
    pub eepromVersion: u8,
    pub templateVersion: u8,
    pub macAddr: [u8; 6],
    pub custData: [u8; AR9300_CUSTOMER_DATA_SIZE],
    pub baseEepHeader: ar9300_base_eep_hdr,
    pub modalHeader2G: ar9300_modal_eep_header,
    pub base_ext1: ar9300_BaseExtension_1,
    pub calFreqPier2G: [u8; AR9300_NUM_2G_CAL_PIERS],
    pub calTarget_freqbin_Cck: [u8; AR9300_NUM_2G_CCK_TARGET_POWERS],
    pub calTarget_freqbin_2G: [u8; AR9300_NUM_2G_20_TARGET_POWERS],
    pub calTarget_freqbin_2GHT20: [u8; AR9300_NUM_2G_20_TARGET_POWERS],
    pub calTarget_freqbin_2GHT40: [u8; AR9300_NUM_2G_40_TARGET_POWERS],
    pub ctlIndex_2G: [u8; AR9300_NUM_CTLS_2G],
    pub ctl_freqbin_2G: [u8; AR9300_NUM_CTLS_2G][AR9300_NUM_BAND_EDGES_2G],
    pub ctlPowerData_2G: [cal_ctl_data_2g; AR9300_NUM_CTLS_2G],
    pub modalHeader5G: ar9300_modal_eep_header,
    pub base_ext2: ar9300_BaseExtension_2,
    pub calFreqPier5G: [u8; AR9300_NUM_5G_CAL_PIERS],
    pub calTarget_freqbin_5G: [u8; AR9300_NUM_5G_20_TARGET_POWERS],
    pub calTarget_freqbin_5GHT20: [u8; AR9300_NUM_5G_20_TARGET_POWERS],
    pub calTarget_freqbin_5GHT40: [u8; AR9300_NUM_5G_40_TARGET_POWERS],
    pub ctlIndex_5G: [u8; AR9300_NUM_CTLS_5G],
    pub ctl_freqbin_5G: [u8; AR9300_NUM_CTLS_5G][AR9300_NUM_BAND_EDGES_5G],
    pub ctlPowerData_5G: [cal_ctl_data_5g; AR9300_NUM_CTLS_5G],
    pub __packed: },
    pub ah): *mut s32 ar9003_hw_get_tx_gain_idx(struct ath_hw,
    pub ah): *mut s32 ar9003_hw_get_rx_gain_idx(struct ath_hw,
    pub is2ghz): *mut *mut u32 ar9003_hw_ant_ctrl_common_get(struct ath_hw ah, bool,
    pub is2ghz): *mut *mut u32 ar9003_hw_ant_ctrl_common_2_get(struct ath_hw ah, bool,
    pub is_2ghz): *mut *mut *mut u8 ar9003_get_spur_chan_ptr(struct ath_hw ah, bool,
    pub is2ghz): *mut *mut u32 ar9003_get_paprd_rate_mask_ht20(struct ath_hw ah, bool,
    pub is2ghz): *mut *mut u32 ar9003_get_paprd_rate_mask_ht40(struct ath_hw ah, bool,
    pub chan): *mut ath9k_channel,
    pub ah): *mut void ar9003_hw_internal_regulator_apply(struct ath_hw,
    pub pPwrArray): *mut *mut *mut int ar9003_hw_tx_power_regwrite(struct ath_hw ah, u8,
