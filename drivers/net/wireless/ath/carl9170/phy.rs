//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/carl9170/phy.h
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
// PHY register map
//
// Copyright (c) 2008-2009 Atheros Communications Inc.
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

pub const AR9170_PHY_TEST_AGC_CLR: c_uint = 0x10000000;
pub const AR9170_PHY_TEST_RFSILENT_BB: c_uint = 0x00002000;

pub const AR9170_PHY_TURBO_FC_TURBO_MODE: c_uint = 0x00000001;
pub const AR9170_PHY_TURBO_FC_TURBO_SHORT: c_uint = 0x00000002;
pub const AR9170_PHY_TURBO_FC_DYN2040_EN: c_uint = 0x00000004;
pub const AR9170_PHY_TURBO_FC_DYN2040_PRI_ONLY: c_uint = 0x00000008;
pub const AR9170_PHY_TURBO_FC_DYN2040_PRI_CH: c_uint = 0x00000010;
// For 25 MHz channel spacing -- not used but supported by hw
pub const AR9170_PHY_TURBO_FC_DYN2040_EXT_CH: c_uint = 0x00000020;
pub const AR9170_PHY_TURBO_FC_HT_EN: c_uint = 0x00000040;
pub const AR9170_PHY_TURBO_FC_SHORT_GI_40: c_uint = 0x00000080;
pub const AR9170_PHY_TURBO_FC_WALSH: c_uint = 0x00000100;
pub const AR9170_PHY_TURBO_FC_SINGLE_HT_LTF1: c_uint = 0x00000200;
pub const AR9170_PHY_TURBO_FC_ENABLE_DAC_FIFO: c_uint = 0x00000800;

pub const AR9170_PHY_TIMING2_USE_FORCE: c_uint = 0x00001000;
pub const AR9170_PHY_TIMING2_FORCE: c_uint = 0x00000fff;
pub const AR9170_PHY_TIMING2_FORCE_S: c_int = 0;

pub const AR9170_PHY_TIMING3_DSC_EXP: c_uint = 0x0001e000;
pub const AR9170_PHY_TIMING3_DSC_EXP_S: c_int = 13;
pub const AR9170_PHY_TIMING3_DSC_MAN: c_uint = 0xfffe0000;
pub const AR9170_PHY_TIMING3_DSC_MAN_S: c_int = 17;

pub const AR9170_PHY_CHIP_ID_REV_0: c_uint = 0x80;
pub const AR9170_PHY_CHIP_ID_REV_1: c_uint = 0x81;
pub const AR9170_PHY_CHIP_ID_9160_REV_0: c_uint = 0xb0;

pub const AR9170_PHY_ACTIVE_EN: c_uint = 0x00000001;
pub const AR9170_PHY_ACTIVE_DIS: c_uint = 0x00000000;

pub const AR9170_PHY_RF_CTL2_TX_END_DATA_START: c_uint = 0x000000ff;
pub const AR9170_PHY_RF_CTL2_TX_END_DATA_START_S: c_int = 0;
pub const AR9170_PHY_RF_CTL2_TX_END_PA_ON: c_uint = 0x0000ff00;
pub const AR9170_PHY_RF_CTL2_TX_END_PA_ON_S: c_int = 8;

pub const AR9170_PHY_RF_CTL3_TX_END_TO_A2_RX_ON: c_uint = 0x00ff0000;
pub const AR9170_PHY_RF_CTL3_TX_END_TO_A2_RX_ON_S: c_int = 16;

pub const AR9170_PHY_ADC_CTL_OFF_INBUFGAIN: c_uint = 0x00000003;
pub const AR9170_PHY_ADC_CTL_OFF_INBUFGAIN_S: c_int = 0;
pub const AR9170_PHY_ADC_CTL_OFF_PWDDAC: c_uint = 0x00002000;
pub const AR9170_PHY_ADC_CTL_OFF_PWDBANDGAP: c_uint = 0x00004000;
pub const AR9170_PHY_ADC_CTL_OFF_PWDADC: c_uint = 0x00008000;
pub const AR9170_PHY_ADC_CTL_ON_INBUFGAIN: c_uint = 0x00030000;
pub const AR9170_PHY_ADC_CTL_ON_INBUFGAIN_S: c_int = 16;

pub const AR9170_PHY_ADC_SCTL_SEL_INTERNAL_ADDAC: c_uint = 0x00000000;
pub const AR9170_PHY_ADC_SCTL_SEL_EXTERNAL_RADIO: c_uint = 0x00000001;

pub const AR9170_PHY_RF_CTL4_TX_END_XPAB_OFF: c_uint = 0xff000000;
pub const AR9170_PHY_RF_CTL4_TX_END_XPAB_OFF_S: c_int = 24;
pub const AR9170_PHY_RF_CTL4_TX_END_XPAA_OFF: c_uint = 0x00ff0000;
pub const AR9170_PHY_RF_CTL4_TX_END_XPAA_OFF_S: c_int = 16;
pub const AR9170_PHY_RF_CTL4_FRAME_XPAB_ON: c_uint = 0x0000ff00;
pub const AR9170_PHY_RF_CTL4_FRAME_XPAB_ON_S: c_int = 8;
pub const AR9170_PHY_RF_CTL4_FRAME_XPAA_ON: c_uint = 0x000000ff;
pub const AR9170_PHY_RF_CTL4_FRAME_XPAA_ON_S: c_int = 0;

pub const AR9170_PHY_SETTLING_SWITCH: c_uint = 0x00003f80;
pub const AR9170_PHY_SETTLING_SWITCH_S: c_int = 7;

pub const AR9170_PHY_RXGAIN_TXRX_ATTEN: c_uint = 0x0003f000;
pub const AR9170_PHY_RXGAIN_TXRX_ATTEN_S: c_int = 12;
pub const AR9170_PHY_RXGAIN_TXRX_RF_MAX: c_uint = 0x007c0000;
pub const AR9170_PHY_RXGAIN_TXRX_RF_MAX_S: c_int = 18;

pub const AR9170_PHY_DESIRED_SZ_ADC: c_uint = 0x000000ff;
pub const AR9170_PHY_DESIRED_SZ_ADC_S: c_int = 0;
pub const AR9170_PHY_DESIRED_SZ_PGA: c_uint = 0x0000ff00;
pub const AR9170_PHY_DESIRED_SZ_PGA_S: c_int = 8;
pub const AR9170_PHY_DESIRED_SZ_TOT_DES: c_uint = 0x0ff00000;
pub const AR9170_PHY_DESIRED_SZ_TOT_DES_S: c_int = 20;

pub const AR9170_PHY_FIND_SIG_FIRSTEP: c_uint = 0x0003f000;
pub const AR9170_PHY_FIND_SIG_FIRSTEP_S: c_int = 12;
pub const AR9170_PHY_FIND_SIG_FIRPWR: c_uint = 0x03fc0000;
pub const AR9170_PHY_FIND_SIG_FIRPWR_S: c_int = 18;

pub const AR9170_PHY_AGC_CTL1_COARSE_LOW: c_uint = 0x00007f80;
pub const AR9170_PHY_AGC_CTL1_COARSE_LOW_S: c_int = 7;
pub const AR9170_PHY_AGC_CTL1_COARSE_HIGH: c_uint = 0x003f8000;
pub const AR9170_PHY_AGC_CTL1_COARSE_HIGH_S: c_int = 15;

pub const AR9170_PHY_AGC_CONTROL_CAL: c_uint = 0x00000001;
pub const AR9170_PHY_AGC_CONTROL_NF: c_uint = 0x00000002;
pub const AR9170_PHY_AGC_CONTROL_ENABLE_NF: c_uint = 0x00008000;
pub const AR9170_PHY_AGC_CONTROL_FLTR_CAL: c_uint = 0x00010000;
pub const AR9170_PHY_AGC_CONTROL_NO_UPDATE_NF: c_uint = 0x00020000;

pub const AR9170_PHY_CCA_MIN_PWR: c_uint = 0x0ff80000;
pub const AR9170_PHY_CCA_MIN_PWR_S: c_int = 19;
pub const AR9170_PHY_CCA_THRESH62: c_uint = 0x0007f000;
pub const AR9170_PHY_CCA_THRESH62_S: c_int = 12;

pub const AR9170_PHY_SFCORR_M2COUNT_THR: c_uint = 0x0000001f;
pub const AR9170_PHY_SFCORR_M2COUNT_THR_S: c_int = 0;
pub const AR9170_PHY_SFCORR_M1_THRESH: c_uint = 0x00fe0000;
pub const AR9170_PHY_SFCORR_M1_THRESH_S: c_int = 17;
pub const AR9170_PHY_SFCORR_M2_THRESH: c_uint = 0x7f000000;
pub const AR9170_PHY_SFCORR_M2_THRESH_S: c_int = 24;

pub const AR9170_PHY_SFCORR_LOW_USE_SELF_CORR_LOW: c_uint = 0x00000001;
pub const AR9170_PHY_SFCORR_LOW_M2COUNT_THR_LOW: c_uint = 0x00003f00;
pub const AR9170_PHY_SFCORR_LOW_M2COUNT_THR_LOW_S: c_int = 8;
pub const AR9170_PHY_SFCORR_LOW_M1_THRESH_LOW: c_uint = 0x001fc000;
pub const AR9170_PHY_SFCORR_LOW_M1_THRESH_LOW_S: c_int = 14;
pub const AR9170_PHY_SFCORR_LOW_M2_THRESH_LOW: c_uint = 0x0fe00000;
pub const AR9170_PHY_SFCORR_LOW_M2_THRESH_LOW_S: c_int = 21;

pub const AR9170_PHY_PLL_CTL_40: c_uint = 0xaa;
pub const AR9170_PHY_PLL_CTL_40_5413: c_uint = 0x04;
pub const AR9170_PHY_PLL_CTL_44: c_uint = 0xab;
pub const AR9170_PHY_PLL_CTL_44_2133: c_uint = 0xeb;
pub const AR9170_PHY_PLL_CTL_40_2133: c_uint = 0xea;

// analogue power on time (100ns)

pub const AR9170_PHY_RX_DELAY_DELAY: c_uint = 0x00003fff;

pub const AR9170_PHY_TIMING_CTRL4_IQCORR_Q_Q_COFF: c_uint = 0x01f;
pub const AR9170_PHY_TIMING_CTRL4_IQCORR_Q_Q_COFF_S: c_int = 0;
pub const AR9170_PHY_TIMING_CTRL4_IQCORR_Q_I_COFF: c_uint = 0x7e0;
pub const AR9170_PHY_TIMING_CTRL4_IQCORR_Q_I_COFF_S: c_int = 5;
pub const AR9170_PHY_TIMING_CTRL4_IQCORR_ENABLE: c_uint = 0x800;
pub const AR9170_PHY_TIMING_CTRL4_IQCAL_LOG_COUNT_MAX: c_uint = 0xf000;
pub const AR9170_PHY_TIMING_CTRL4_IQCAL_LOG_COUNT_MAX_S: c_int = 12;
pub const AR9170_PHY_TIMING_CTRL4_DO_IQCAL: c_uint = 0x10000;
pub const AR9170_PHY_TIMING_CTRL4_ENABLE_SPUR_RSSI: c_uint = 0x80000000;
pub const AR9170_PHY_TIMING_CTRL4_ENABLE_SPUR_FILTER: c_uint = 0x40000000;
pub const AR9170_PHY_TIMING_CTRL4_ENABLE_CHAN_MASK: c_uint = 0x20000000;
pub const AR9170_PHY_TIMING_CTRL4_ENABLE_PILOT_MASK: c_uint = 0x10000000;

pub const AR9170_PHY_TIMING5_CYCPWR_THR1: c_uint = 0x000000fe;
pub const AR9170_PHY_TIMING5_CYCPWR_THR1_S: c_int = 1;

pub const AR9170_PHY_POWER_TX_RATE_MAX_TPC_ENABLE: c_uint = 0x00000040;

pub const AR9170_PHY_FRAME_CTL_TX_CLIP: c_uint = 0x00000038;
pub const AR9170_PHY_FRAME_CTL_TX_CLIP_S: c_int = 3;

pub const AR9170_PHY_SPUR_REG_MASK_RATE_CNTL_S: c_int = 18;
pub const AR9170_PHY_SPUR_REG_ENABLE_MASK_PPM: c_uint = 0x20000;

pub const AR9170_PHY_SPUR_REG_MASK_RATE_SELECT_S: c_int = 9;
pub const AR9170_PHY_SPUR_REG_ENABLE_VIT_SPUR_RSSI: c_uint = 0x100;
pub const AR9170_PHY_SPUR_REG_SPUR_RSSI_THRESH: c_uint = 0x7f;
pub const AR9170_PHY_SPUR_REG_SPUR_RSSI_THRESH_S: c_int = 0;

pub const AR9170_PHY_RADAR_EXT_ENA: c_uint = 0x00004000;

pub const AR9170_PHY_RADAR_0_ENA: c_uint = 0x00000001;
pub const AR9170_PHY_RADAR_0_FFT_ENA: c_uint = 0x80000000;
// inband pulse threshold
pub const AR9170_PHY_RADAR_0_INBAND: c_uint = 0x0000003e;
pub const AR9170_PHY_RADAR_0_INBAND_S: c_int = 1;
// pulse RSSI threshold
pub const AR9170_PHY_RADAR_0_PRSSI: c_uint = 0x00000fc0;
pub const AR9170_PHY_RADAR_0_PRSSI_S: c_int = 6;
// pulse height threshold
pub const AR9170_PHY_RADAR_0_HEIGHT: c_uint = 0x0003f000;
pub const AR9170_PHY_RADAR_0_HEIGHT_S: c_int = 12;
// radar RSSI threshold
pub const AR9170_PHY_RADAR_0_RRSSI: c_uint = 0x00fc0000;
pub const AR9170_PHY_RADAR_0_RRSSI_S: c_int = 18;
// radar firepower threshold
pub const AR9170_PHY_RADAR_0_FIRPWR: c_uint = 0x7f000000;
pub const AR9170_PHY_RADAR_0_FIRPWR_S: c_int = 24;

pub const AR9170_PHY_RADAR_1_RELPWR_ENA: c_uint = 0x00800000;
pub const AR9170_PHY_RADAR_1_USE_FIR128: c_uint = 0x00400000;
pub const AR9170_PHY_RADAR_1_RELPWR_THRESH: c_uint = 0x003f0000;
pub const AR9170_PHY_RADAR_1_RELPWR_THRESH_S: c_int = 16;
pub const AR9170_PHY_RADAR_1_BLOCK_CHECK: c_uint = 0x00008000;
pub const AR9170_PHY_RADAR_1_MAX_RRSSI: c_uint = 0x00004000;
pub const AR9170_PHY_RADAR_1_RELSTEP_CHECK: c_uint = 0x00002000;
pub const AR9170_PHY_RADAR_1_RELSTEP_THRESH: c_uint = 0x00001f00;
pub const AR9170_PHY_RADAR_1_RELSTEP_THRESH_S: c_int = 8;
pub const AR9170_PHY_RADAR_1_MAXLEN: c_uint = 0x000000ff;
pub const AR9170_PHY_RADAR_1_MAXLEN_S: c_int = 0;

pub const AR9170_PHY_SIGMA_DELTA_ADC_SEL: c_uint = 0x00000003;
pub const AR9170_PHY_SIGMA_DELTA_ADC_SEL_S: c_int = 0;
pub const AR9170_PHY_SIGMA_DELTA_FILT2: c_uint = 0x000000f8;
pub const AR9170_PHY_SIGMA_DELTA_FILT2_S: c_int = 3;
pub const AR9170_PHY_SIGMA_DELTA_FILT1: c_uint = 0x00001f00;
pub const AR9170_PHY_SIGMA_DELTA_FILT1_S: c_int = 8;
pub const AR9170_PHY_SIGMA_DELTA_ADC_CLIP: c_uint = 0x01ffe000;
pub const AR9170_PHY_SIGMA_DELTA_ADC_CLIP_S: c_int = 13;

pub const AR9170_PHY_RESTART_DIV_GC: c_uint = 0x001c0000;
pub const AR9170_PHY_RESTART_DIV_GC_S: c_int = 18;

pub const AR9170_PHY_RFBUS_REQ_EN: c_uint = 0x00000001;

pub const AR9170_PHY_TIMING8_PILOT_MASK_2: c_uint = 0x000fffff;
pub const AR9170_PHY_TIMING8_PILOT_MASK_2_S: c_int = 0;

pub const AR9170_PHY_BIN_MASK2_4_MASK_4: c_uint = 0x00003fff;
pub const AR9170_PHY_BIN_MASK2_4_MASK_4_S: c_int = 0;

pub const AR9170_PHY_TIMING10_PILOT_MASK_2: c_uint = 0x000fffff;
pub const AR9170_PHY_TIMING10_PILOT_MASK_2_S: c_int = 0;

pub const AR9170_PHY_TIMING11_SPUR_DELTA_PHASE: c_uint = 0x000fffff;
pub const AR9170_PHY_TIMING11_SPUR_DELTA_PHASE_S: c_int = 0;
pub const AR9170_PHY_TIMING11_SPUR_FREQ_SD: c_uint = 0x3ff00000;
pub const AR9170_PHY_TIMING11_SPUR_FREQ_SD_S: c_int = 20;
pub const AR9170_PHY_TIMING11_USE_SPUR_IN_AGC: c_uint = 0x40000000;
pub const AR9170_PHY_TIMING11_USE_SPUR_IN_SELFCOR: c_uint = 0x80000000;

pub const AR9170_PHY_NEW_ADC_GAIN_CORR_ENABLE: c_uint = 0x40000000;
pub const AR9170_PHY_NEW_ADC_DC_OFFSET_CORR_ENABLE: c_uint = 0x80000000;

pub const AR9170_PHY_9285_ANT_DIV_CTL_ALL: c_uint = 0x7f000000;
pub const AR9170_PHY_9285_ANT_DIV_CTL: c_uint = 0x01000000;
pub const AR9170_PHY_9285_ANT_DIV_CTL_S: c_int = 24;
pub const AR9170_PHY_9285_ANT_DIV_ALT_LNACONF: c_uint = 0x06000000;
pub const AR9170_PHY_9285_ANT_DIV_ALT_LNACONF_S: c_int = 25;
pub const AR9170_PHY_9285_ANT_DIV_MAIN_LNACONF: c_uint = 0x18000000;
pub const AR9170_PHY_9285_ANT_DIV_MAIN_LNACONF_S: c_int = 27;
pub const AR9170_PHY_9285_ANT_DIV_ALT_GAINTB: c_uint = 0x20000000;
pub const AR9170_PHY_9285_ANT_DIV_ALT_GAINTB_S: c_int = 29;
pub const AR9170_PHY_9285_ANT_DIV_MAIN_GAINTB: c_uint = 0x40000000;
pub const AR9170_PHY_9285_ANT_DIV_MAIN_GAINTB_S: c_int = 30;
pub const AR9170_PHY_9285_ANT_DIV_LNA1: c_int = 2;
pub const AR9170_PHY_9285_ANT_DIV_LNA2: c_int = 1;
pub const AR9170_PHY_9285_ANT_DIV_LNA1_PLUS_LNA2: c_int = 3;
pub const AR9170_PHY_9285_ANT_DIV_LNA1_MINUS_LNA2: c_int = 0;
pub const AR9170_PHY_9285_ANT_DIV_GAINTB_0: c_int = 0;
pub const AR9170_PHY_9285_ANT_DIV_GAINTB_1: c_int = 1;

pub const AR9170_PHY_REG_EXT_CCA0_THRESH62: c_uint = 0x000000ff;
pub const AR9170_PHY_REG_EXT_CCA0_THRESH62_S: c_int = 0;

pub const AR9170_PHY_EXT_CCA_CYCPWR_THR1: c_uint = 0x0000fe00;
pub const AR9170_PHY_EXT_CCA_CYCPWR_THR1_S: c_int = 9;
pub const AR9170_PHY_EXT_CCA_THRESH62: c_uint = 0x007f0000;
pub const AR9170_PHY_EXT_CCA_THRESH62_S: c_int = 16;
pub const AR9170_PHY_EXT_CCA_MIN_PWR: c_uint = 0xff800000;
pub const AR9170_PHY_EXT_CCA_MIN_PWR_S: c_int = 23;

pub const AR9170_PHY_SFCORR_EXT_M1_THRESH: c_uint = 0x0000007f;
pub const AR9170_PHY_SFCORR_EXT_M1_THRESH_S: c_int = 0;
pub const AR9170_PHY_SFCORR_EXT_M2_THRESH: c_uint = 0x00003f80;
pub const AR9170_PHY_SFCORR_EXT_M2_THRESH_S: c_int = 7;
pub const AR9170_PHY_SFCORR_EXT_M1_THRESH_LOW: c_uint = 0x001fc000;
pub const AR9170_PHY_SFCORR_EXT_M1_THRESH_LOW_S: c_int = 14;
pub const AR9170_PHY_SFCORR_EXT_M2_THRESH_LOW: c_uint = 0x0fe00000;
pub const AR9170_PHY_SFCORR_EXT_M2_THRESH_LOW_S: c_int = 21;
pub const AR9170_PHY_SFCORR_SPUR_SUBCHNL_SD_S: c_int = 28;

pub const AR9170_PHY_HALFGI_DSC_MAN: c_uint = 0x0007fff0;
pub const AR9170_PHY_HALFGI_DSC_MAN_S: c_int = 4;
pub const AR9170_PHY_HALFGI_DSC_EXP: c_uint = 0x0000000f;
pub const AR9170_PHY_HALFGI_DSC_EXP_S: c_int = 0;

pub const AR9170_PHY_CHAN_INFO_MEMORY_CAPTURE_MASK: c_uint = 0x0001;

pub const AR9170_PHY_RIFS_INIT_DELAY: c_uint = 0x03ff0000;

pub const AR9170_PHY_CALMODE_IQ: c_uint = 0x00000000;
pub const AR9170_PHY_CALMODE_ADC_GAIN: c_uint = 0x00000001;
pub const AR9170_PHY_CALMODE_ADC_DC_PER: c_uint = 0x00000002;
pub const AR9170_PHY_CALMODE_ADC_DC_INIT: c_uint = 0x00000003;

pub const AR9170_PHY_RFBUS_GRANT_EN: c_uint = 0x00000001;

pub const AR9170_PHY_CHAN_INFO_GAIN_DIFF_UPPER_LIMIT: c_int = 320;

pub const AR9170_PHY_MODE_ASYNCFIFO: c_uint = 0x80;
pub const AR9170_PHY_MODE_AR2133: c_uint = 0x08;
pub const AR9170_PHY_MODE_AR5111: c_uint = 0x00;
pub const AR9170_PHY_MODE_AR5112: c_uint = 0x08;
pub const AR9170_PHY_MODE_DYNAMIC: c_uint = 0x04;
pub const AR9170_PHY_MODE_RF2GHZ: c_uint = 0x02;
pub const AR9170_PHY_MODE_RF5GHZ: c_uint = 0x00;
pub const AR9170_PHY_MODE_CCK: c_uint = 0x01;
pub const AR9170_PHY_MODE_OFDM: c_uint = 0x00;
pub const AR9170_PHY_MODE_DYN_CCK_DISABLE: c_uint = 0x100;

pub const AR9170_PHY_CCK_TX_CTRL_JAPAN: c_uint = 0x00000010;
pub const AR9170_PHY_CCK_TX_CTRL_TX_DAC_SCALE_CCK: c_uint = 0x0000000c;
pub const AR9170_PHY_CCK_TX_CTRL_TX_DAC_SCALE_CCK_S: c_int = 2;

pub const AR9170_PHY_CCK_DETECT_WEAK_SIG_THR_CCK: c_uint = 0x0000003f;
pub const AR9170_PHY_CCK_DETECT_WEAK_SIG_THR_CCK_S: c_int = 0;
// [12:6] settling time for antenna switch
pub const AR9170_PHY_CCK_DETECT_ANT_SWITCH_TIME: c_uint = 0x00001fc0;
pub const AR9170_PHY_CCK_DETECT_ANT_SWITCH_TIME_S: c_int = 6;
pub const AR9170_PHY_CCK_DETECT_BB_ENABLE_ANT_FAST_DIV: c_uint = 0x2000;
pub const AR9170_PHY_CCK_DETECT_BB_ENABLE_ANT_FAST_DIV_S: c_int = 13;

pub const AR9170_PHY_GAIN_2GHZ_RXTX_MARGIN: c_uint = 0x00fc0000;
pub const AR9170_PHY_GAIN_2GHZ_RXTX_MARGIN_S: c_int = 18;
pub const AR9170_PHY_GAIN_2GHZ_BSW_MARGIN: c_uint = 0x00003c00;
pub const AR9170_PHY_GAIN_2GHZ_BSW_MARGIN_S: c_int = 10;
pub const AR9170_PHY_GAIN_2GHZ_BSW_ATTEN: c_uint = 0x0000001f;
pub const AR9170_PHY_GAIN_2GHZ_BSW_ATTEN_S: c_int = 0;
pub const AR9170_PHY_GAIN_2GHZ_XATTEN2_MARGIN: c_uint = 0x003e0000;
pub const AR9170_PHY_GAIN_2GHZ_XATTEN2_MARGIN_S: c_int = 17;
pub const AR9170_PHY_GAIN_2GHZ_XATTEN1_MARGIN: c_uint = 0x0001f000;
pub const AR9170_PHY_GAIN_2GHZ_XATTEN1_MARGIN_S: c_int = 12;
pub const AR9170_PHY_GAIN_2GHZ_XATTEN2_DB: c_uint = 0x00000fc0;
pub const AR9170_PHY_GAIN_2GHZ_XATTEN2_DB_S: c_int = 6;
pub const AR9170_PHY_GAIN_2GHZ_XATTEN1_DB: c_uint = 0x0000003f;
pub const AR9170_PHY_GAIN_2GHZ_XATTEN1_DB_S: c_int = 0;

pub const AR9170_PHY_CCK_RXCTRL4_FREQ_EST_SHORT: c_uint = 0x01f80000;
pub const AR9170_PHY_CCK_RXCTRL4_FREQ_EST_SHORT_S: c_int = 19;

pub const AR9170_REG_DAG_CTRLCCK_EN_RSSI_THR: c_uint = 0x00000200;
pub const AR9170_REG_DAG_CTRLCCK_RSSI_THR: c_uint = 0x0001fc00;
pub const AR9170_REG_DAG_CTRLCCK_RSSI_THR_S: c_int = 10;

pub const AR9170_FORCE_CLKEN_CCK_MRC_MUX: c_uint = 0x00000040;

pub const AR9170_PHY_TPCRG1_NUM_PD_GAIN: c_uint = 0x0000c000;
pub const AR9170_PHY_TPCRG1_NUM_PD_GAIN_S: c_int = 14;
pub const AR9170_PHY_TPCRG1_PD_GAIN_1: c_uint = 0x00030000;
pub const AR9170_PHY_TPCRG1_PD_GAIN_1_S: c_int = 16;
pub const AR9170_PHY_TPCRG1_PD_GAIN_2: c_uint = 0x000c0000;
pub const AR9170_PHY_TPCRG1_PD_GAIN_2_S: c_int = 18;
pub const AR9170_PHY_TPCRG1_PD_GAIN_3: c_uint = 0x00300000;
pub const AR9170_PHY_TPCRG1_PD_GAIN_3_S: c_int = 20;
pub const AR9170_PHY_TPCRG1_PD_CAL_ENABLE: c_uint = 0x00400000;
pub const AR9170_PHY_TPCRG1_PD_CAL_ENABLE_S: c_int = 22;

pub const AR9170_PHY_TX_PWRCTRL_PD_AVG_VALID: c_uint = 0x00000001;
pub const AR9170_PHY_TX_PWRCTRL_PD_AVG_VALID_S: c_int = 0;
pub const AR9170_PHY_TX_PWRCTRL_PD_AVG_OUT: c_uint = 0x000001fe;
pub const AR9170_PHY_TX_PWRCTRL_PD_AVG_OUT_S: c_int = 1;

pub const AR9170_PHY_ANALOG_SWAP_AB: c_uint = 0x0001;
pub const AR9170_PHY_ANALOG_SWAP_ALT_CHAIN: c_uint = 0x00000040;

pub const AR9170_PHY_TPCRG5_PD_GAIN_OVERLAP: c_uint = 0x0000000f;
pub const AR9170_PHY_TPCRG5_PD_GAIN_OVERLAP_S: c_int = 0;
pub const AR9170_PHY_TPCRG5_PD_GAIN_BOUNDARY_1: c_uint = 0x000003f0;
pub const AR9170_PHY_TPCRG5_PD_GAIN_BOUNDARY_1_S: c_int = 4;
pub const AR9170_PHY_TPCRG5_PD_GAIN_BOUNDARY_2: c_uint = 0x0000fc00;
pub const AR9170_PHY_TPCRG5_PD_GAIN_BOUNDARY_2_S: c_int = 10;
pub const AR9170_PHY_TPCRG5_PD_GAIN_BOUNDARY_3: c_uint = 0x003f0000;
pub const AR9170_PHY_TPCRG5_PD_GAIN_BOUNDARY_3_S: c_int = 16;
pub const AR9170_PHY_TPCRG5_PD_GAIN_BOUNDARY_4: c_uint = 0x0fc00000;
pub const AR9170_PHY_TPCRG5_PD_GAIN_BOUNDARY_4_S: c_int = 22;

pub const AR9170_PHY_TX_PWRCTRL_ERR_EST_MODE: c_uint = 0x03000000;
pub const AR9170_PHY_TX_PWRCTRL_ERR_EST_MODE_S: c_int = 24;

pub const AR9170_PHY_TX_PWRCTRL_INIT_TX_GAIN: c_uint = 0x01f80000;
pub const AR9170_PHY_TX_PWRCTRL_INIT_TX_GAIN_S: c_int = 19;

pub const AR9170_PHY_TX_DESIRED_SCALE_CCK: c_uint = 0x00007c00;
pub const AR9170_PHY_TX_DESIRED_SCALE_CCK_S: c_int = 10;
pub const AR9170_PHY_TX_PWRCTRL9_RES_DC_REMOVAL: c_uint = 0x80000000;
pub const AR9170_PHY_TX_PWRCTRL9_RES_DC_REMOVAL_S: c_int = 31;

pub const AR9170_PHY_TX_GAIN: c_uint = 0x0007f000;
pub const AR9170_PHY_TX_GAIN_S: c_int = 12;
// Carrier leak calibration control, do it after AGC calibration

pub const AR9170_PHY_CL_CAL_ENABLE: c_uint = 0x00000002;
pub const AR9170_PHY_CL_CAL_PARALLEL_CAL_ENABLE: c_uint = 0x00000001;

pub const AR9170_PHY_TX_CHX_PWRCTRL_OLPC_TEMP_COMP: c_uint = 0x0000fc00;
pub const AR9170_PHY_TX_CHX_PWRCTRL_OLPC_TEMP_COMP_S: c_int = 10;

pub const AR9170_PHY_FORCE_XPA_CFG: c_uint = 0x000000001;
pub const AR9170_PHY_FORCE_XPA_CFG_S: c_int = 0;

pub const AR9170_PHY_CH1_CCA_MIN_PWR: c_uint = 0x0ff80000;
pub const AR9170_PHY_CH1_CCA_MIN_PWR_S: c_int = 19;

pub const AR9170_PHY_CH2_CCA_MIN_PWR: c_uint = 0x0ff80000;
pub const AR9170_PHY_CH2_CCA_MIN_PWR_S: c_int = 19;

pub const AR9170_PHY_CH1_EXT_CCA_MIN_PWR: c_uint = 0xff800000;
pub const AR9170_PHY_CH1_EXT_CCA_MIN_PWR_S: c_int = 23;

pub const AR9170_PHY_CH2_EXT_CCA_MIN_PWR: c_uint = 0xff800000;
pub const AR9170_PHY_CH2_EXT_CCA_MIN_PWR_S: c_int = 23;
