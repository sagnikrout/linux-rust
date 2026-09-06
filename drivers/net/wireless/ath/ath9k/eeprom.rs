//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/eeprom.h
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
// Copyright (c) 2008-2011 Atheros Communications Inc.
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
pub const AR_EEPROM_MODAL_SPURS: c_int = 5;

// helpers to swap EEPROM fields, which are stored as __le16 or __le32. Since
// we are 100% sure about it we  these to u16/u32 for the swab calls to
// silence the sparse checks. These macros are used when we have a Big Endian
// EEPROM (according to AR5416_EEPMISC_BIG_ENDIAN) and need to convert the
// fields to __le16/__le32.
//

pub const AR5416_EEPROM_MAGIC: c_uint = 0x5aa5;

pub const AR5416_EEPROM_MAGIC: c_uint = 0xa55a;

pub const CTRY_DEBUG: c_uint = 0x1ff;
pub const CTRY_DEFAULT: c_int = 0;
pub const AR_EEPROM_EEPCAP_COMPRESS_DIS: c_uint = 0x0001;
pub const AR_EEPROM_EEPCAP_AES_DIS: c_uint = 0x0002;
pub const AR_EEPROM_EEPCAP_FASTFRAME_DIS: c_uint = 0x0004;
pub const AR_EEPROM_EEPCAP_BURST_DIS: c_uint = 0x0008;
pub const AR_EEPROM_EEPCAP_MAXQCU: c_uint = 0x01F0;
pub const AR_EEPROM_EEPCAP_MAXQCU_S: c_int = 4;
pub const AR_EEPROM_EEPCAP_HEAVY_CLIP_EN: c_uint = 0x0200;
pub const AR_EEPROM_EEPCAP_KC_ENTRIES: c_uint = 0xF000;
pub const AR_EEPROM_EEPCAP_KC_ENTRIES_S: c_int = 12;
pub const AR_EEPROM_EEREGCAP_EN_FCC_MIDBAND: c_uint = 0x0040;
pub const AR_EEPROM_EEREGCAP_EN_KK_U1_EVEN: c_uint = 0x0080;
pub const AR_EEPROM_EEREGCAP_EN_KK_U2: c_uint = 0x0100;
pub const AR_EEPROM_EEREGCAP_EN_KK_MIDBAND: c_uint = 0x0200;
pub const AR_EEPROM_EEREGCAP_EN_KK_U1_ODD: c_uint = 0x0400;
pub const AR_EEPROM_EEREGCAP_EN_KK_NEW_11A: c_uint = 0x0800;
pub const AR_EEPROM_EEREGCAP_EN_KK_U1_ODD_PRE4_0: c_uint = 0x4000;
pub const AR_EEPROM_EEREGCAP_EN_KK_NEW_11A_PRE4_0: c_uint = 0x8000;
pub const AR5416_EEPROM_MAGIC_OFFSET: c_uint = 0x0;
pub const AR5416_EEPROM_S: c_int = 2;
pub const AR5416_EEPROM_OFFSET: c_uint = 0x2000;
pub const AR5416_EEPROM_MAX: c_uint = 0xae0;

pub const SD_NO_CTL: c_uint = 0xE0;
pub const NO_CTL: c_uint = 0xff;
pub const CTL_MODE_M: c_uint = 0xf;
pub const CTL_11A: c_int = 0;
pub const CTL_11B: c_int = 1;
pub const CTL_11G: c_int = 2;
pub const CTL_2GHT20: c_int = 5;
pub const CTL_5GHT20: c_int = 6;
pub const CTL_2GHT40: c_int = 7;
pub const CTL_5GHT40: c_int = 8;

pub const SUB_NUM_CTL_MODES_AT_5G_40: c_int = 2;
pub const SUB_NUM_CTL_MODES_AT_2G_40: c_int = 3;

//
// For AR9285 and later chipsets, the following bits are not being programmed
// in EEPROM and so need to be enabled always.
//
// Bit 0: en_fcc_mid
// Bit 1: en_jap_mid
// Bit 2: en_fcc_dfs_ht40
// Bit 3: en_jap_ht40
// Bit 4: en_jap_dfs_ht40
//
pub const AR9285_RDEXT_DEFAULT: c_uint = 0x1F;

pub const EEP_RFSILENT_ENABLED: c_uint = 0x0001;
pub const EEP_RFSILENT_ENABLED_S: c_int = 0;
pub const EEP_RFSILENT_POLARITY: c_uint = 0x0002;
pub const EEP_RFSILENT_POLARITY_S: c_int = 1;

pub const EEP_RFSILENT_GPIO_SEL_S: c_int = 2;
pub const AR5416_OPFLAGS_11A: c_uint = 0x01;
pub const AR5416_OPFLAGS_11G: c_uint = 0x02;
pub const AR5416_OPFLAGS_N_5G_HT40: c_uint = 0x04;
pub const AR5416_OPFLAGS_N_2G_HT40: c_uint = 0x08;
pub const AR5416_OPFLAGS_N_5G_HT20: c_uint = 0x10;
pub const AR5416_OPFLAGS_N_2G_HT20: c_uint = 0x20;
pub const AR5416_EEP_NO_BACK_VER: c_uint = 0x1;
pub const AR5416_EEP_VER: c_uint = 0xE;
pub const AR5416_EEP_VER_MAJOR_SHIFT: c_int = 12;
pub const AR5416_EEP_VER_MAJOR_MASK: c_uint = 0xF000;
pub const AR5416_EEP_VER_MINOR_MASK: c_uint = 0x0FFF;
pub const AR5416_EEP_MINOR_VER_2: c_uint = 0x2;
pub const AR5416_EEP_MINOR_VER_3: c_uint = 0x3;
pub const AR5416_EEP_MINOR_VER_7: c_uint = 0x7;
pub const AR5416_EEP_MINOR_VER_9: c_uint = 0x9;
pub const AR5416_EEP_MINOR_VER_16: c_uint = 0x10;
pub const AR5416_EEP_MINOR_VER_17: c_uint = 0x11;
pub const AR5416_EEP_MINOR_VER_19: c_uint = 0x13;
pub const AR5416_EEP_MINOR_VER_20: c_uint = 0x14;
pub const AR5416_EEP_MINOR_VER_21: c_uint = 0x15;
pub const AR5416_EEP_MINOR_VER_22: c_uint = 0x16;
pub const AR5416_NUM_5G_CAL_PIERS: c_int = 8;
pub const AR5416_NUM_2G_CAL_PIERS: c_int = 4;
pub const AR5416_NUM_5G_20_TARGET_POWERS: c_int = 8;
pub const AR5416_NUM_5G_40_TARGET_POWERS: c_int = 8;
pub const AR5416_NUM_2G_CCK_TARGET_POWERS: c_int = 3;
pub const AR5416_NUM_2G_20_TARGET_POWERS: c_int = 4;
pub const AR5416_NUM_2G_40_TARGET_POWERS: c_int = 4;
pub const AR5416_NUM_CTLS: c_int = 24;
pub const AR5416_NUM_BAND_EDGES: c_int = 8;
pub const AR5416_NUM_PD_GAINS: c_int = 4;
pub const AR5416_PD_GAINS_IN_MASK: c_int = 4;
pub const AR5416_PD_GAIN_ICEPTS: c_int = 5;
pub const AR5416_NUM_PDADC_VALUES: c_int = 128;
pub const AR5416_BCHAN_UNUSED: c_uint = 0xFF;
pub const AR5416_MAX_PWR_RANGE_IN_HALF_DB: c_int = 64;
pub const AR5416_MAX_CHAINS: c_int = 3;
pub const AR9300_MAX_CHAINS: c_int = 3;

// Rx gain type values
pub const AR5416_EEP_RXGAIN_23DB_BACKOFF: c_int = 0;
pub const AR5416_EEP_RXGAIN_13DB_BACKOFF: c_int = 1;
pub const AR5416_EEP_RXGAIN_ORIG: c_int = 2;
// Tx gain type values
pub const AR5416_EEP_TXGAIN_ORIGINAL: c_int = 0;
pub const AR5416_EEP_TXGAIN_HIGH_POWER: c_int = 1;
// Endianness of EEPROM content
pub const AR5416_EEPMISC_BIG_ENDIAN: c_uint = 0x01;
pub const AR5416_EEP4K_START_LOC: c_int = 64;
pub const AR5416_EEP4K_NUM_2G_CAL_PIERS: c_int = 3;
pub const AR5416_EEP4K_NUM_2G_CCK_TARGET_POWERS: c_int = 3;
pub const AR5416_EEP4K_NUM_2G_20_TARGET_POWERS: c_int = 3;
pub const AR5416_EEP4K_NUM_2G_40_TARGET_POWERS: c_int = 3;
pub const AR5416_EEP4K_NUM_CTLS: c_int = 12;
pub const AR5416_EEP4K_NUM_BAND_EDGES: c_int = 4;
pub const AR5416_EEP4K_NUM_PD_GAINS: c_int = 2;
pub const AR5416_EEP4K_MAX_CHAINS: c_int = 1;
pub const AR9280_TX_GAIN_TABLE_SIZE: c_int = 22;
pub const AR9287_EEP_VER: c_uint = 0xE;
pub const AR9287_EEP_MINOR_VER_1: c_uint = 0x1;
pub const AR9287_EEP_MINOR_VER_2: c_uint = 0x2;
pub const AR9287_EEP_MINOR_VER_3: c_uint = 0x3;

pub const AR9287_EEP_START_LOC: c_int = 128;
pub const AR9287_HTC_EEP_START_LOC: c_int = 256;
pub const AR9287_NUM_2G_CAL_PIERS: c_int = 3;
pub const AR9287_NUM_2G_CCK_TARGET_POWERS: c_int = 3;
pub const AR9287_NUM_2G_20_TARGET_POWERS: c_int = 3;
pub const AR9287_NUM_2G_40_TARGET_POWERS: c_int = 3;
pub const AR9287_NUM_CTLS: c_int = 12;
pub const AR9287_NUM_BAND_EDGES: c_int = 4;
pub const AR9287_PD_GAIN_ICEPTS: c_int = 1;
pub const AR9287_EEPMISC_WOW: c_uint = 0x02;
pub const AR9287_MAX_CHAINS: c_int = 2;
pub const AR9287_ANT_16S: c_int = 32;
pub const AR9287_DATA_SZ: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eeprom_param {
    EEP_NFTHRESH_5,
    EEP_NFTHRESH_2,
    EEP_MAC_MSW,
    EEP_MAC_MID,
    EEP_MAC_LSW,
    EEP_REG_0,
    EEP_OP_CAP,
    EEP_OP_MODE,
    EEP_RF_SILENT,
    EEP_OB_5,
    EEP_DB_5,
    EEP_OB_2,
    EEP_DB_2,
    EEP_TX_MASK,
    EEP_RX_MASK,
    EEP_FSTCLK_5G,
    EEP_RXGAIN_TYPE,
    EEP_OL_PWRCTRL,
    EEP_TXGAIN_TYPE,
    EEP_RC_CHAIN_MASK,
    EEP_DAC_HPWR_5G,
    EEP_FRAC_N_5G,
    EEP_DEV_TYPE,
    EEP_TEMPSENSE_SLOPE,
    EEP_TEMPSENSE_SLOPE_PAL_ON,
    EEP_PWR_TABLE_OFFSET,
    EEP_PAPRD,
    EEP_MODAL_VER,
    EEP_ANT_DIV_CTL1,
    EEP_CHAIN_MASK_REDUCE,
    EEP_ANTENNA_GAIN_2G,
    EEP_ANTENNA_GAIN_5G,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ar5416_rates {
    rate6mb, rate9mb, rate12mb, rate18mb,
    rate24mb, rate36mb, rate48mb, rate54mb,
    rate1l, rate2l, rate2s, rate5_5l,
    rate5_5s, rate11l, rate11s, rateXr,
    rateHt20_0, rateHt20_1, rateHt20_2, rateHt20_3,
    rateHt20_4, rateHt20_5, rateHt20_6, rateHt20_7,
    rateHt40_0, rateHt40_1, rateHt40_2, rateHt40_3,
    rateHt40_4, rateHt40_5, rateHt40_6, rateHt40_7,
    rateDupCck, rateDupOfdm, rateExtCck, rateExtOfdm,
    Ar5416RateSize
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath9k_hal_freq_band {
    ATH9K_HAL_FREQ_BAND_5GHZ = 0,
    ATH9K_HAL_FREQ_BAND_2GHZ = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct base_eep_header {
    pub length: __le16,
    pub checksum: __le16,
    pub version: __le16,
    pub opCapFlags: u8,
    pub eepMisc: u8,
    pub regDmn: [__le16; 2],
    pub macAddr: [u8; 6],
    pub rxMask: u8,
    pub txMask: u8,
    pub rfSilent: __le16,
    pub blueToothOptions: __le16,
    pub deviceCap: __le16,
    pub binBuildNumber: __le32,
    pub deviceType: u8,
    pub pwdclkind: u8,
    pub fastClk5g: u8,
    pub divChain: u8,
    pub rxGainType: u8,
    pub dacHiPwrMode_5G: u8,
    pub openLoopPwrCntl: u8,
    pub dacLpMode: u8,
    pub txGainType: u8,
    pub rcChainMask: u8,
    pub desiredScaleCCK: u8,
    pub pwr_table_offset: u8,
    pub frac_n_5g: u8,
    pub futureBase_3: [u8; 21],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct base_eep_header_4k {
    pub length: __le16,
    pub checksum: __le16,
    pub version: __le16,
    pub opCapFlags: u8,
    pub eepMisc: u8,
    pub regDmn: [__le16; 2],
    pub macAddr: [u8; 6],
    pub rxMask: u8,
    pub txMask: u8,
    pub rfSilent: __le16,
    pub blueToothOptions: __le16,
    pub deviceCap: __le16,
    pub binBuildNumber: __le32,
    pub deviceType: u8,
    pub txGainType: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spur_chan {
    pub spurChan: __le16,
    pub spurRangeLow: u8,
    pub spurRangeHigh: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct modal_eep_header {
    pub antCtrlChain: [__le32; AR5416_MAX_CHAINS],
    pub antCtrlCommon: __le32,
    pub antennaGainCh: [u8; AR5416_MAX_CHAINS],
    pub switchSettling: u8,
    pub txRxAttenCh: [u8; AR5416_MAX_CHAINS],
    pub rxTxMarginCh: [u8; AR5416_MAX_CHAINS],
    pub adcDesiredSize: u8,
    pub pgaDesiredSize: u8,
    pub xlnaGainCh: [u8; AR5416_MAX_CHAINS],
    pub txEndToXpaOff: u8,
    pub txEndToRxOn: u8,
    pub txFrameToXpaOn: u8,
    pub thresh62: u8,
    pub noiseFloorThreshCh: [u8; AR5416_MAX_CHAINS],
    pub xpdGain: u8,
    pub xpd: u8,
    pub iqCalICh: [u8; AR5416_MAX_CHAINS],
    pub iqCalQCh: [u8; AR5416_MAX_CHAINS],
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
    pub xatten2Db: [u8; AR5416_MAX_CHAINS],
    pub xatten2Margin: [u8; AR5416_MAX_CHAINS],
    pub ob_ch1: u8,
    pub db_ch1: u8,
    pub lna_ctl: u8,
    pub miscBits: u8,
    pub xpaBiasLvlFreq: [__le16; 3],
    pub futureModal: [u8; 6],
    pub spurChans: [spur_chan; AR_EEPROM_MODAL_SPURS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct calDataPerFreqOpLoop {
    pub pwrPdg: [u8; 2][5],
    pub vpdPdg: [u8; 2][5],
    pub pcdac: [u8; 2][5],
    pub empty: [u8; 2][5],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct modal_eep_4k_header {
    pub antCtrlChain: [__le32; AR5416_EEP4K_MAX_CHAINS],
    pub antCtrlCommon: __le32,
    pub antennaGainCh: [u8; AR5416_EEP4K_MAX_CHAINS],
    pub switchSettling: u8,
    pub txRxAttenCh: [u8; AR5416_EEP4K_MAX_CHAINS],
    pub rxTxMarginCh: [u8; AR5416_EEP4K_MAX_CHAINS],
    pub adcDesiredSize: u8,
    pub pgaDesiredSize: u8,
    pub xlnaGainCh: [u8; AR5416_EEP4K_MAX_CHAINS],
    pub txEndToXpaOff: u8,
    pub txEndToRxOn: u8,
    pub txFrameToXpaOn: u8,
    pub thresh62: u8,
    pub noiseFloorThreshCh: [u8; AR5416_EEP4K_MAX_CHAINS],
    pub xpdGain: u8,
    pub xpd: u8,
    pub iqCalICh: [u8; AR5416_EEP4K_MAX_CHAINS],
    pub iqCalQCh: [u8; AR5416_EEP4K_MAX_CHAINS],
    pub pdGainOverlap: u8,

    pub ob_0:4: u8 ob_1:4,,
    pub db1_0:4: u8 db1_1:4,,

    pub ob_1:4: u8 ob_0:4,,
    pub db1_1:4: u8 db1_0:4,,

    pub xpaBiasLvl: u8,
    pub txFrameToDataStart: u8,
    pub txFrameToPaOn: u8,
    pub ht40PowerIncForPdadc: u8,
    pub bswAtten: [u8; AR5416_EEP4K_MAX_CHAINS],
    pub bswMargin: [u8; AR5416_EEP4K_MAX_CHAINS],
    pub swSettleHt40: u8,
    pub xatten2Db: [u8; AR5416_EEP4K_MAX_CHAINS],
    pub xatten2Margin: [u8; AR5416_EEP4K_MAX_CHAINS],
    pub db2_0:4: u8 db2_1:4,,

    pub db2_1:4: u8 db2_0:4,,

    pub version: u8,

    pub ob_2:4: u8 ob_3:4,,
    pub ob_4:4: u8 antdiv_ctl1:4,,
    pub db1_2:4: u8 db1_3:4,,
    pub db1_4:4: u8 antdiv_ctl2:4,,
    pub db2_3:4: u8 db2_2:4,,
    pub db2_4:4: u8 reserved:4,,

    pub ob_3:4: u8 ob_2:4,,
    pub antdiv_ctl1:4: u8 ob_4:4,,
    pub db1_3:4: u8 db1_2:4,,
    pub antdiv_ctl2:4: u8 db1_4:4,,
    pub db2_3:4: u8 db2_2:4,,
    pub reserved:4: u8 db2_4:4,,

    pub tx_diversity: u8,
    pub flc_pwr_thresh: u8,
    pub bb_scale_smrt_antenna: u8,
pub const EEP_4K_BB_DESIRED_SCALE_MASK: c_uint = 0x1f;
    pub futureModal: [u8; 1],
    pub spurChans: [spur_chan; AR_EEPROM_MODAL_SPURS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct base_eep_ar9287_header {
    pub length: __le16,
    pub checksum: __le16,
    pub version: __le16,
    pub opCapFlags: u8,
    pub eepMisc: u8,
    pub regDmn: [__le16; 2],
    pub macAddr: [u8; 6],
    pub rxMask: u8,
    pub txMask: u8,
    pub rfSilent: __le16,
    pub blueToothOptions: __le16,
    pub deviceCap: __le16,
    pub binBuildNumber: __le32,
    pub deviceType: u8,
    pub openLoopPwrCntl: u8,
    pub pwrTableOffset: i8,
    pub tempSensSlope: i8,
    pub tempSensSlopePalOn: i8,
    pub futureBase: [u8; 29],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct modal_eep_ar9287_header {
    pub antCtrlChain: [__le32; AR9287_MAX_CHAINS],
    pub antCtrlCommon: __le32,
    pub antennaGainCh: [i8; AR9287_MAX_CHAINS],
    pub switchSettling: u8,
    pub txRxAttenCh: [u8; AR9287_MAX_CHAINS],
    pub rxTxMarginCh: [u8; AR9287_MAX_CHAINS],
    pub adcDesiredSize: i8,
    pub txEndToXpaOff: u8,
    pub txEndToRxOn: u8,
    pub txFrameToXpaOn: u8,
    pub thresh62: u8,
    pub noiseFloorThreshCh: [i8; AR9287_MAX_CHAINS],
    pub xpdGain: u8,
    pub xpd: u8,
    pub iqCalICh: [i8; AR9287_MAX_CHAINS],
    pub iqCalQCh: [i8; AR9287_MAX_CHAINS],
    pub pdGainOverlap: u8,
    pub xpaBiasLvl: u8,
    pub txFrameToDataStart: u8,
    pub txFrameToPaOn: u8,
    pub ht40PowerIncForPdadc: u8,
    pub bswAtten: [u8; AR9287_MAX_CHAINS],
    pub bswMargin: [u8; AR9287_MAX_CHAINS],
    pub swSettleHt40: u8,
    pub version: u8,
    pub db1: u8,
    pub db2: u8,
    pub ob_cck: u8,
    pub ob_psk: u8,
    pub ob_qam: u8,
    pub ob_pal_off: u8,
    pub futureModal: [u8; 30],
    pub spurChans: [spur_chan; AR_EEPROM_MODAL_SPURS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_data_per_freq {
    pub pwrPdg: [u8; AR5416_NUM_PD_GAINS][AR5416_PD_GAIN_ICEPTS],
    pub vpdPdg: [u8; AR5416_NUM_PD_GAINS][AR5416_PD_GAIN_ICEPTS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_data_per_freq_4k {
    pub pwrPdg: [u8; AR5416_EEP4K_NUM_PD_GAINS][AR5416_PD_GAIN_ICEPTS],
    pub vpdPdg: [u8; AR5416_EEP4K_NUM_PD_GAINS][AR5416_PD_GAIN_ICEPTS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_target_power_leg {
    pub bChannel: u8,
    pub tPow2x: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_target_power_ht {
    pub bChannel: u8,
    pub tPow2x: [u8; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_ctl_edges {
    pub bChannel: u8,
    pub ctl: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_data_op_loop_ar9287 {
    pub pwrPdg: [u8; 2][5],
    pub vpdPdg: [u8; 2][5],
    pub pcdac: [u8; 2][5],
    pub empty: [u8; 2][5],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_data_per_freq_ar9287 {
    pub pwrPdg: [u8; AR5416_NUM_PD_GAINS][AR9287_PD_GAIN_ICEPTS],
    pub vpdPdg: [u8; AR5416_NUM_PD_GAINS][AR9287_PD_GAIN_ICEPTS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union cal_data_per_freq_ar9287_u {
    pub calDataOpen: cal_data_op_loop_ar9287,
    pub calDataClose: cal_data_per_freq_ar9287,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_ctl_data_ar9287 {
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_ctl_data {
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_ctl_data_4k {
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5416_eeprom_def {
    pub baseEepHeader: base_eep_header,
    pub custData: [u8; 64],
    pub modalHeader: [modal_eep_header; 2],
    pub calFreqPier5G: [u8; AR5416_NUM_5G_CAL_PIERS],
    pub calFreqPier2G: [u8; AR5416_NUM_2G_CAL_PIERS],
    pub ctlIndex: [u8; AR5416_NUM_CTLS],
    pub ctlData: [cal_ctl_data; AR5416_NUM_CTLS],
    pub padding: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5416_eeprom_4k {
    pub baseEepHeader: base_eep_header_4k,
    pub custData: [u8; 20],
    pub modalHeader: modal_eep_4k_header,
    pub calFreqPier2G: [u8; AR5416_EEP4K_NUM_2G_CAL_PIERS],
    pub ctlIndex: [u8; AR5416_EEP4K_NUM_CTLS],
    pub ctlData: [cal_ctl_data_4k; AR5416_EEP4K_NUM_CTLS],
    pub padding: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9287_eeprom {
    pub baseEepHeader: base_eep_ar9287_header,
    pub custData: [u8; AR9287_DATA_SZ],
    pub modalHeader: modal_eep_ar9287_header,
    pub calFreqPier2G: [u8; AR9287_NUM_2G_CAL_PIERS],
    pub ctlIndex: [u8; AR9287_NUM_CTLS],
    pub ctlData: [cal_ctl_data_ar9287; AR9287_NUM_CTLS],
    pub padding: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg_ext_bitmap {
    REG_EXT_FCC_MIDBAND = 0,
    REG_EXT_JAPAN_MIDBAND = 1,
    REG_EXT_FCC_DFS_HT40 = 2,
    REG_EXT_JAPAN_NONDFS_HT40 = 3,
    REG_EXT_JAPAN_DFS_HT40 = 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_country_entry {
    pub countryCode: u16,
    pub regDmnEnum: u16,
    pub regDmn5G: u16,
    pub regDmn2G: u16,
    pub isMultidomain: u8,
    pub iso: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeprom_ops {
    pub hw): *mut *mut int (check_eeprom)(struct ath_hw,
    pub param): *mut *mut *mut u32 (get_eeprom)(struct ath_hw hw, enum eeprom_param,
    pub hw): *mut *mut bool (fill_eeprom)(struct ath_hw,
    pub size): u32 len, u32,
    pub hw): *mut *mut int (get_eeprom_ver)(struct ath_hw,
    pub hw): *mut *mut int (get_eeprom_rev)(struct ath_hw,
    pub chan): *mut *mut *mut void (set_board_values)(struct ath_hw hw, struct ath9k_channel,
    pub chan): *mut *mut *mut void (set_addac)(struct ath_hw hw, struct ath9k_channel,
    pub test): u8 powerLimit, bool,
    pub is2GHz): *mut *mut *mut u16 (get_spur_channel)(struct ath_hw ah, u16 i, bool,
    pub ah): *mut *mut u8 (get_eepmisc)(struct ath_hw,
}

extern "C" {
    pub fn ath9k_hw_analog_shift_regwrite(ah: *mut ath_hw, reg: u32, val: u32);
}
extern "C" {
    pub fn ath9k_hw_nvram_read(ah: *mut ath_hw, off: u32, data: *mut u16) -> bool;
}
extern "C" {
    pub fn ath9k_hw_nvram_swap_data(ah: *mut ath_hw, swap_needed: *mut bool, size: c_int) -> c_int;
}
extern "C" {
    pub fn ath9k_hw_nvram_validate_checksum(ah: *mut ath_hw, size: c_int) -> bool;
}
extern "C" {
    pub fn ath9k_hw_nvram_check_version(ah: *mut ath_hw, version: c_int, minrev: c_int) -> bool;
}
extern "C" {
    pub fn ath9k_hw_update_regulatory_maxpower(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_eeprom_init(ah: *mut ath_hw) -> c_int;
}

