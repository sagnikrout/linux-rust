//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/ar9003_phy.h
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
// Copyright (c) 2010-2011 Atheros Communications, Inc.
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
// Channel Register Map
//
pub const AR_CHAN_BASE: c_uint = 0x9800;

pub const AR_PHY_TIMING_CONTROL4_DO_GAIN_DC_IQ_CAL_SHIFT: c_int = 16;
pub const AR_PHY_TIMING11_SPUR_FREQ_SD: c_uint = 0x3FF00000;
pub const AR_PHY_TIMING11_SPUR_FREQ_SD_S: c_int = 20;
pub const AR_PHY_TIMING11_SPUR_DELTA_PHASE: c_uint = 0x000FFFFF;
pub const AR_PHY_TIMING11_SPUR_DELTA_PHASE_S: c_int = 0;
pub const AR_PHY_TIMING11_USE_SPUR_FILTER_IN_AGC: c_uint = 0x40000000;
pub const AR_PHY_TIMING11_USE_SPUR_FILTER_IN_AGC_S: c_int = 30;
pub const AR_PHY_TIMING11_USE_SPUR_FILTER_IN_SELFCOR: c_uint = 0x80000000;
pub const AR_PHY_TIMING11_USE_SPUR_FILTER_IN_SELFCOR_S: c_int = 31;
pub const AR_PHY_SPUR_REG_ENABLE_NF_RSSI_SPUR_MIT: c_uint = 0x4000000;
pub const AR_PHY_SPUR_REG_ENABLE_NF_RSSI_SPUR_MIT_S: c_int = 26;
pub const AR_PHY_SPUR_REG_ENABLE_MASK_PPM: c_uint = 0x20000     /* bins move with freq offset */;
pub const AR_PHY_SPUR_REG_ENABLE_MASK_PPM_S: c_int = 17;
pub const AR_PHY_SPUR_REG_SPUR_RSSI_THRESH: c_uint = 0x000000FF;
pub const AR_PHY_SPUR_REG_SPUR_RSSI_THRESH_S: c_int = 0;
pub const AR_PHY_SPUR_REG_EN_VIT_SPUR_RSSI: c_uint = 0x00000100;
pub const AR_PHY_SPUR_REG_EN_VIT_SPUR_RSSI_S: c_int = 8;
pub const AR_PHY_SPUR_REG_MASK_RATE_CNTL: c_uint = 0x03FC0000;
pub const AR_PHY_SPUR_REG_MASK_RATE_CNTL_S: c_int = 18;
pub const AR_PHY_RX_IQCAL_CORR_B0_LOOPBACK_IQCORR_EN: c_uint = 0x20000000;
pub const AR_PHY_RX_IQCAL_CORR_B0_LOOPBACK_IQCORR_EN_S: c_int = 29;
pub const AR_PHY_TX_IQCAL_CONTROL_3_IQCORR_EN: c_uint = 0x80000000;
pub const AR_PHY_TX_IQCAL_CONTROL_3_IQCORR_EN_S: c_int = 31;

// The following registers changed position from AR9300 1.0 to AR9300 2.0

//
// Channel Field Definitions
//
pub const AR_PHY_TIMING2_USE_FORCE_PPM: c_uint = 0x00001000;
pub const AR_PHY_TIMING2_FORCE_PPM_VAL: c_uint = 0x00000fff;
pub const AR_PHY_TIMING3_DSC_MAN: c_uint = 0xFFFE0000;
pub const AR_PHY_TIMING3_DSC_MAN_S: c_int = 17;
pub const AR_PHY_TIMING3_DSC_EXP: c_uint = 0x0001E000;
pub const AR_PHY_TIMING3_DSC_EXP_S: c_int = 13;
pub const AR_PHY_TIMING4_IQCAL_LOG_COUNT_MAX: c_uint = 0xF000;
pub const AR_PHY_TIMING4_IQCAL_LOG_COUNT_MAX_S: c_int = 12;
pub const AR_PHY_TIMING4_DO_CAL: c_uint = 0x10000;
pub const AR_PHY_TIMING4_ENABLE_PILOT_MASK: c_uint = 0x10000000;
pub const AR_PHY_TIMING4_ENABLE_PILOT_MASK_S: c_int = 28;
pub const AR_PHY_TIMING4_ENABLE_CHAN_MASK: c_uint = 0x20000000;
pub const AR_PHY_TIMING4_ENABLE_CHAN_MASK_S: c_int = 29;
pub const AR_PHY_TIMING4_ENABLE_SPUR_FILTER: c_uint = 0x40000000;
pub const AR_PHY_TIMING4_ENABLE_SPUR_FILTER_S: c_int = 30;
pub const AR_PHY_TIMING4_ENABLE_SPUR_RSSI: c_uint = 0x80000000;
pub const AR_PHY_TIMING4_ENABLE_SPUR_RSSI_S: c_int = 31;
pub const AR_PHY_NEW_ADC_GAIN_CORR_ENABLE: c_uint = 0x40000000;
pub const AR_PHY_NEW_ADC_DC_OFFSET_CORR_ENABLE: c_uint = 0x80000000;
pub const AR_PHY_SFCORR_LOW_USE_SELF_CORR_LOW: c_uint = 0x00000001;
pub const AR_PHY_SFCORR_LOW_M2COUNT_THR_LOW: c_uint = 0x00003F00;
pub const AR_PHY_SFCORR_LOW_M2COUNT_THR_LOW_S: c_int = 8;
pub const AR_PHY_SFCORR_LOW_M1_THRESH_LOW: c_uint = 0x001FC000;
pub const AR_PHY_SFCORR_LOW_M1_THRESH_LOW_S: c_int = 14;
pub const AR_PHY_SFCORR_LOW_M2_THRESH_LOW: c_uint = 0x0FE00000;
pub const AR_PHY_SFCORR_LOW_M2_THRESH_LOW_S: c_int = 21;
pub const AR_PHY_SFCORR_M2COUNT_THR: c_uint = 0x0000001F;
pub const AR_PHY_SFCORR_M2COUNT_THR_S: c_int = 0;
pub const AR_PHY_SFCORR_M1_THRESH: c_uint = 0x00FE0000;
pub const AR_PHY_SFCORR_M1_THRESH_S: c_int = 17;
pub const AR_PHY_SFCORR_M2_THRESH: c_uint = 0x7F000000;
pub const AR_PHY_SFCORR_M2_THRESH_S: c_int = 24;
pub const AR_PHY_SFCORR_EXT_M1_THRESH: c_uint = 0x0000007F;
pub const AR_PHY_SFCORR_EXT_M1_THRESH_S: c_int = 0;
pub const AR_PHY_SFCORR_EXT_M2_THRESH: c_uint = 0x00003F80;
pub const AR_PHY_SFCORR_EXT_M2_THRESH_S: c_int = 7;
pub const AR_PHY_SFCORR_EXT_M1_THRESH_LOW: c_uint = 0x001FC000;
pub const AR_PHY_SFCORR_EXT_M1_THRESH_LOW_S: c_int = 14;
pub const AR_PHY_SFCORR_EXT_M2_THRESH_LOW: c_uint = 0x0FE00000;
pub const AR_PHY_SFCORR_EXT_M2_THRESH_LOW_S: c_int = 21;
pub const AR_PHY_SFCORR_EXT_SPUR_SUBCHANNEL_SD: c_uint = 0x10000000;
pub const AR_PHY_SFCORR_EXT_SPUR_SUBCHANNEL_SD_S: c_int = 28;
pub const AR_PHY_SFCORR_SPUR_SUBCHNL_SD_S: c_int = 28;
pub const AR_PHY_EXT_CCA_THRESH62: c_uint = 0x007F0000;
pub const AR_PHY_EXT_CCA_THRESH62_S: c_int = 16;
pub const AR_PHY_EXTCHN_PWRTHR1_ANT_DIV_ALT_ANT_MINGAINIDX: c_uint = 0x0000FF00;
pub const AR_PHY_EXTCHN_PWRTHR1_ANT_DIV_ALT_ANT_MINGAINIDX_S: c_int = 8;
pub const AR_PHY_EXT_MINCCA_PWR: c_uint = 0x01FF0000;
pub const AR_PHY_EXT_MINCCA_PWR_S: c_int = 16;
pub const AR_PHY_EXT_CYCPWR_THR1: c_uint = 0x0000FE00L;
pub const AR_PHY_EXT_CYCPWR_THR1_S: c_int = 9;
pub const AR_PHY_TIMING5_CYCPWR_THR1: c_uint = 0x000000FE;
pub const AR_PHY_TIMING5_CYCPWR_THR1_S: c_int = 1;
pub const AR_PHY_TIMING5_CYCPWR_THR1_ENABLE: c_uint = 0x00000001;
pub const AR_PHY_TIMING5_CYCPWR_THR1_ENABLE_S: c_int = 0;
pub const AR_PHY_TIMING5_CYCPWR_THR1A: c_uint = 0x007F0000;
pub const AR_PHY_TIMING5_CYCPWR_THR1A_S: c_int = 16;

pub const AR_PHY_TIMING5_RSSI_THR1A_S: c_int = 16;

pub const AR_PHY_RADAR_0_ENA: c_uint = 0x00000001;
pub const AR_PHY_RADAR_0_FFT_ENA: c_uint = 0x80000000;
pub const AR_PHY_RADAR_0_INBAND: c_uint = 0x0000003e;
pub const AR_PHY_RADAR_0_INBAND_S: c_int = 1;
pub const AR_PHY_RADAR_0_PRSSI: c_uint = 0x00000FC0;
pub const AR_PHY_RADAR_0_PRSSI_S: c_int = 6;
pub const AR_PHY_RADAR_0_HEIGHT: c_uint = 0x0003F000;
pub const AR_PHY_RADAR_0_HEIGHT_S: c_int = 12;
pub const AR_PHY_RADAR_0_RRSSI: c_uint = 0x00FC0000;
pub const AR_PHY_RADAR_0_RRSSI_S: c_int = 18;
pub const AR_PHY_RADAR_0_FIRPWR: c_uint = 0x7F000000;
pub const AR_PHY_RADAR_0_FIRPWR_S: c_int = 24;
pub const AR_PHY_RADAR_1_RELPWR_ENA: c_uint = 0x00800000;
pub const AR_PHY_RADAR_1_USE_FIR128: c_uint = 0x00400000;
pub const AR_PHY_RADAR_1_RELPWR_THRESH: c_uint = 0x003F0000;
pub const AR_PHY_RADAR_1_RELPWR_THRESH_S: c_int = 16;
pub const AR_PHY_RADAR_1_BLOCK_CHECK: c_uint = 0x00008000;
pub const AR_PHY_RADAR_1_MAX_RRSSI: c_uint = 0x00004000;
pub const AR_PHY_RADAR_1_RELSTEP_CHECK: c_uint = 0x00002000;
pub const AR_PHY_RADAR_1_RELSTEP_THRESH: c_uint = 0x00001F00;
pub const AR_PHY_RADAR_1_RELSTEP_THRESH_S: c_int = 8;
pub const AR_PHY_RADAR_1_MAXLEN: c_uint = 0x000000FF;
pub const AR_PHY_RADAR_1_MAXLEN_S: c_int = 0;
pub const AR_PHY_RADAR_EXT_ENA: c_uint = 0x00004000;
pub const AR_PHY_RADAR_DC_PWR_THRESH: c_uint = 0x007f8000;
pub const AR_PHY_RADAR_DC_PWR_THRESH_S: c_int = 15;
pub const AR_PHY_RADAR_LB_DC_CAP: c_uint = 0x7f800000;
pub const AR_PHY_RADAR_LB_DC_CAP_S: c_int = 23;

pub const AR_PHY_FIND_SIG_LOW_FIRSTEP_LOW_S: c_int = 6;

pub const AR_PHY_FIND_SIG_LOW_FIRPWR_S: c_int = 12;
pub const AR_PHY_FIND_SIG_LOW_FIRPWR_SIGN_BIT: c_int = 19;
pub const AR_PHY_FIND_SIG_LOW_RELSTEP: c_uint = 0x1f;
pub const AR_PHY_FIND_SIG_LOW_RELSTEP_S: c_int = 0;
pub const AR_PHY_FIND_SIG_LOW_RELSTEP_SIGN_BIT: c_int = 5;
pub const AR_PHY_CHAN_INFO_TAB_S2_READ: c_uint = 0x00000008;
pub const AR_PHY_CHAN_INFO_TAB_S2_READ_S: c_int = 3;
pub const AR_PHY_RX_IQCAL_CORR_IQCORR_Q_Q_COFF: c_uint = 0x0000007F;
pub const AR_PHY_RX_IQCAL_CORR_IQCORR_Q_Q_COFF_S: c_int = 0;
pub const AR_PHY_RX_IQCAL_CORR_IQCORR_Q_I_COFF: c_uint = 0x00003F80;
pub const AR_PHY_RX_IQCAL_CORR_IQCORR_Q_I_COFF_S: c_int = 7;
pub const AR_PHY_RX_IQCAL_CORR_IQCORR_ENABLE: c_uint = 0x00004000;
pub const AR_PHY_RX_IQCAL_CORR_LOOPBACK_IQCORR_Q_Q_COFF: c_uint = 0x003f8000;
pub const AR_PHY_RX_IQCAL_CORR_LOOPBACK_IQCORR_Q_Q_COFF_S: c_int = 15;
pub const AR_PHY_RX_IQCAL_CORR_LOOPBACK_IQCORR_Q_I_COFF: c_uint = 0x1fc00000;
pub const AR_PHY_RX_IQCAL_CORR_LOOPBACK_IQCORR_Q_I_COFF_S: c_int = 22;
//
// MRC Register Map
//
pub const AR_MRC_BASE: c_uint = 0x9c00;

pub const AR_PHY_PILOT_SPUR_MASK_CF_PILOT_MASK_IDX_A: c_uint = 0x00000FE0;
pub const AR_PHY_PILOT_SPUR_MASK_CF_PILOT_MASK_IDX_A_S: c_int = 5;
pub const AR_PHY_PILOT_SPUR_MASK_CF_PILOT_MASK_A: c_uint = 0x1F;
pub const AR_PHY_PILOT_SPUR_MASK_CF_PILOT_MASK_A_S: c_int = 0;
pub const AR_PHY_PILOT_SPUR_MASK_CF_PILOT_MASK_IDX_B: c_uint = 0x00FE0000;
pub const AR_PHY_PILOT_SPUR_MASK_CF_PILOT_MASK_IDX_B_S: c_int = 17;
pub const AR_PHY_PILOT_SPUR_MASK_CF_PILOT_MASK_B: c_uint = 0x0001F000;
pub const AR_PHY_PILOT_SPUR_MASK_CF_PILOT_MASK_B_S: c_int = 12;
pub const AR_PHY_CHAN_SPUR_MASK_CF_CHAN_MASK_IDX_A: c_uint = 0x00000FE0;
pub const AR_PHY_CHAN_SPUR_MASK_CF_CHAN_MASK_IDX_A_S: c_int = 5;
pub const AR_PHY_CHAN_SPUR_MASK_CF_CHAN_MASK_A: c_uint = 0x1F;
pub const AR_PHY_CHAN_SPUR_MASK_CF_CHAN_MASK_A_S: c_int = 0;
pub const AR_PHY_CHAN_SPUR_MASK_CF_CHAN_MASK_IDX_B: c_uint = 0x00FE0000;
pub const AR_PHY_CHAN_SPUR_MASK_CF_CHAN_MASK_IDX_B_S: c_int = 17;
pub const AR_PHY_CHAN_SPUR_MASK_CF_CHAN_MASK_B: c_uint = 0x0001F000;
pub const AR_PHY_CHAN_SPUR_MASK_CF_CHAN_MASK_B_S: c_int = 12;
//
// MRC Field Definitions
//
pub const AR_PHY_SGI_DSC_MAN: c_uint = 0x0007FFF0;
pub const AR_PHY_SGI_DSC_MAN_S: c_int = 4;
pub const AR_PHY_SGI_DSC_EXP: c_uint = 0x0000000F;
pub const AR_PHY_SGI_DSC_EXP_S: c_int = 0;
//
// BBB Register Map
//
pub const AR_BBB_BASE: c_uint = 0x9d00;
//
// AGC Register Map
//
pub const AR_AGC_BASE: c_uint = 0x9e00;

//
// Antenna Diversity  settings
//

pub const AR_ANT_DIV_CTRL_ALL: c_uint = 0x7e000000;
pub const AR_ANT_DIV_CTRL_ALL_S: c_int = 25;
pub const AR_ANT_DIV_ENABLE: c_uint = 0x1000000;
pub const AR_ANT_DIV_ENABLE_S: c_int = 24;
pub const AR_PHY_ANT_FAST_DIV_BIAS: c_uint = 0x00007e00;
pub const AR_PHY_ANT_FAST_DIV_BIAS_S: c_int = 9;
pub const AR_PHY_ANT_SW_RX_PROT: c_uint = 0x00800000;
pub const AR_PHY_ANT_SW_RX_PROT_S: c_int = 23;
pub const AR_PHY_ANT_DIV_LNADIV: c_uint = 0x01000000;
pub const AR_PHY_ANT_DIV_LNADIV_S: c_int = 24;
pub const AR_PHY_ANT_DIV_ALT_LNACONF: c_uint = 0x06000000;
pub const AR_PHY_ANT_DIV_ALT_LNACONF_S: c_int = 25;
pub const AR_PHY_ANT_DIV_MAIN_LNACONF: c_uint = 0x18000000;
pub const AR_PHY_ANT_DIV_MAIN_LNACONF_S: c_int = 27;
pub const AR_PHY_ANT_DIV_ALT_GAINTB: c_uint = 0x20000000;
pub const AR_PHY_ANT_DIV_ALT_GAINTB_S: c_int = 29;
pub const AR_PHY_ANT_DIV_MAIN_GAINTB: c_uint = 0x40000000;
pub const AR_PHY_ANT_DIV_MAIN_GAINTB_S: c_int = 30;

pub const AR_FAST_DIV_ENABLE: c_uint = 0x2000;
pub const AR_FAST_DIV_ENABLE_S: c_int = 13;

pub const AR_PHY_CCK_SPUR_MIT_SPUR_RSSI_THR: c_uint = 0x000001fe;
pub const AR_PHY_CCK_SPUR_MIT_SPUR_RSSI_THR_S: c_int = 1;
pub const AR_PHY_CCK_SPUR_MIT_SPUR_FILTER_TYPE: c_uint = 0x60000000;
pub const AR_PHY_CCK_SPUR_MIT_SPUR_FILTER_TYPE_S: c_int = 29;
pub const AR_PHY_CCK_SPUR_MIT_USE_CCK_SPUR_MIT: c_uint = 0x00000001;
pub const AR_PHY_CCK_SPUR_MIT_USE_CCK_SPUR_MIT_S: c_int = 0;
pub const AR_PHY_CCK_SPUR_MIT_CCK_SPUR_FREQ: c_uint = 0x1ffffe00;
pub const AR_PHY_CCK_SPUR_MIT_CCK_SPUR_FREQ_S: c_int = 9;

pub const AR_PHY_MRC_CCK_ENABLE: c_uint = 0x00000001;
pub const AR_PHY_MRC_CCK_ENABLE_S: c_int = 0;
pub const AR_PHY_MRC_CCK_MUX_REG: c_uint = 0x00000002;
pub const AR_PHY_MRC_CCK_MUX_REG_S: c_int = 1;

pub const AR9300_EXT_LNA_CTL_GPIO_AR9485: c_int = 9;
//
// AGC Field Definitions
//
pub const AR_PHY_EXT_ATTEN_CTL_RXTX_MARGIN: c_uint = 0x00FC0000;
pub const AR_PHY_EXT_ATTEN_CTL_RXTX_MARGIN_S: c_int = 18;
pub const AR_PHY_EXT_ATTEN_CTL_BSW_MARGIN: c_uint = 0x00003C00;
pub const AR_PHY_EXT_ATTEN_CTL_BSW_MARGIN_S: c_int = 10;
pub const AR_PHY_EXT_ATTEN_CTL_BSW_ATTEN: c_uint = 0x0000001F;
pub const AR_PHY_EXT_ATTEN_CTL_BSW_ATTEN_S: c_int = 0;
pub const AR_PHY_EXT_ATTEN_CTL_XATTEN2_MARGIN: c_uint = 0x003E0000;
pub const AR_PHY_EXT_ATTEN_CTL_XATTEN2_MARGIN_S: c_int = 17;
pub const AR_PHY_EXT_ATTEN_CTL_XATTEN1_MARGIN: c_uint = 0x0001F000;
pub const AR_PHY_EXT_ATTEN_CTL_XATTEN1_MARGIN_S: c_int = 12;
pub const AR_PHY_EXT_ATTEN_CTL_XATTEN2_DB: c_uint = 0x00000FC0;
pub const AR_PHY_EXT_ATTEN_CTL_XATTEN2_DB_S: c_int = 6;
pub const AR_PHY_EXT_ATTEN_CTL_XATTEN1_DB: c_uint = 0x0000003F;
pub const AR_PHY_EXT_ATTEN_CTL_XATTEN1_DB_S: c_int = 0;
pub const AR_PHY_RXGAIN_TXRX_ATTEN: c_uint = 0x0003F000;
pub const AR_PHY_RXGAIN_TXRX_ATTEN_S: c_int = 12;
pub const AR_PHY_RXGAIN_TXRX_RF_MAX: c_uint = 0x007C0000;
pub const AR_PHY_RXGAIN_TXRX_RF_MAX_S: c_int = 18;
pub const AR9280_PHY_RXGAIN_TXRX_ATTEN: c_uint = 0x00003F80;
pub const AR9280_PHY_RXGAIN_TXRX_ATTEN_S: c_int = 7;
pub const AR9280_PHY_RXGAIN_TXRX_MARGIN: c_uint = 0x001FC000;
pub const AR9280_PHY_RXGAIN_TXRX_MARGIN_S: c_int = 14;
pub const AR_PHY_SETTLING_SWITCH: c_uint = 0x00003F80;
pub const AR_PHY_SETTLING_SWITCH_S: c_int = 7;
pub const AR_PHY_DESIRED_SZ_ADC: c_uint = 0x000000FF;
pub const AR_PHY_DESIRED_SZ_ADC_S: c_int = 0;
pub const AR_PHY_DESIRED_SZ_PGA: c_uint = 0x0000FF00;
pub const AR_PHY_DESIRED_SZ_PGA_S: c_int = 8;
pub const AR_PHY_DESIRED_SZ_TOT_DES: c_uint = 0x0FF00000;
pub const AR_PHY_DESIRED_SZ_TOT_DES_S: c_int = 20;
pub const AR_PHY_MINCCA_PWR: c_uint = 0x1FF00000;
pub const AR_PHY_MINCCA_PWR_S: c_int = 20;
pub const AR_PHY_CCA_THRESH62: c_uint = 0x0007F000;
pub const AR_PHY_CCA_THRESH62_S: c_int = 12;
pub const AR9280_PHY_MINCCA_PWR: c_uint = 0x1FF00000;
pub const AR9280_PHY_MINCCA_PWR_S: c_int = 20;
pub const AR9280_PHY_CCA_THRESH62: c_uint = 0x000FF000;
pub const AR9280_PHY_CCA_THRESH62_S: c_int = 12;
pub const AR_PHY_EXT_CCA0_THRESH62: c_uint = 0x000000FF;
pub const AR_PHY_EXT_CCA0_THRESH62_S: c_int = 0;
pub const AR_PHY_EXT_CCA0_THRESH62_1: c_uint = 0x000001FF;
pub const AR_PHY_EXT_CCA0_THRESH62_1_S: c_int = 0;
pub const AR_PHY_CCK_DETECT_WEAK_SIG_THR_CCK: c_uint = 0x0000003F;
pub const AR_PHY_CCK_DETECT_WEAK_SIG_THR_CCK_S: c_int = 0;
pub const AR_PHY_CCK_DETECT_ANT_SWITCH_TIME: c_uint = 0x00001FC0;
pub const AR_PHY_CCK_DETECT_ANT_SWITCH_TIME_S: c_int = 6;
pub const AR_PHY_CCK_DETECT_BB_ENABLE_ANT_FAST_DIV: c_uint = 0x2000;
pub const AR_PHY_DAG_CTRLCCK_EN_RSSI_THR: c_uint = 0x00000200;
pub const AR_PHY_DAG_CTRLCCK_EN_RSSI_THR_S: c_int = 9;
pub const AR_PHY_DAG_CTRLCCK_RSSI_THR: c_uint = 0x0001FC00;
pub const AR_PHY_DAG_CTRLCCK_RSSI_THR_S: c_int = 10;
pub const AR_PHY_RIFS_INIT_DELAY: c_uint = 0x3ff0000;
pub const AR_PHY_AGC_QUICK_DROP: c_uint = 0x03c00000;
pub const AR_PHY_AGC_QUICK_DROP_S: c_int = 22;
pub const AR_PHY_AGC_COARSE_LOW: c_uint = 0x00007F80;
pub const AR_PHY_AGC_COARSE_LOW_S: c_int = 7;
pub const AR_PHY_AGC_COARSE_HIGH: c_uint = 0x003F8000;
pub const AR_PHY_AGC_COARSE_HIGH_S: c_int = 15;
pub const AR_PHY_AGC_COARSE_PWR_CONST: c_uint = 0x0000007F;
pub const AR_PHY_AGC_COARSE_PWR_CONST_S: c_int = 0;
pub const AR_PHY_FIND_SIG_FIRSTEP: c_uint = 0x0003F000;
pub const AR_PHY_FIND_SIG_FIRSTEP_S: c_int = 12;
pub const AR_PHY_FIND_SIG_FIRPWR: c_uint = 0x03FC0000;
pub const AR_PHY_FIND_SIG_FIRPWR_S: c_int = 18;
pub const AR_PHY_FIND_SIG_FIRPWR_SIGN_BIT: c_int = 25;

pub const AR_PHY_FIND_SIG_RELPWR_S: c_int = 6;
pub const AR_PHY_FIND_SIG_RELPWR_SIGN_BIT: c_int = 11;
pub const AR_PHY_FIND_SIG_RELSTEP: c_uint = 0x1f;
pub const AR_PHY_FIND_SIG_RELSTEP_S: c_int = 0;
pub const AR_PHY_FIND_SIG_RELSTEP_SIGN_BIT: c_int = 5;
pub const AR_PHY_RESTART_ENABLE_DIV_M2FLAG: c_uint = 0x00200000;
pub const AR_PHY_RESTART_ENABLE_DIV_M2FLAG_S: c_int = 21;
pub const AR_PHY_RESTART_DIV_GC: c_uint = 0x001C0000;
pub const AR_PHY_RESTART_DIV_GC_S: c_int = 18;
pub const AR_PHY_RESTART_ENA: c_uint = 0x01;
pub const AR_PHY_DC_RESTART_DIS: c_uint = 0x40000000;
pub const AR_PHY_TPC_OLPC_GAIN_DELTA_PAL_ON: c_uint = 0xFF000000;
pub const AR_PHY_TPC_OLPC_GAIN_DELTA_PAL_ON_S: c_int = 24;
pub const AR_PHY_TPC_OLPC_GAIN_DELTA: c_uint = 0x00FF0000;
pub const AR_PHY_TPC_OLPC_GAIN_DELTA_S: c_int = 16;
pub const AR_PHY_TPC_6_ERROR_EST_MODE: c_uint = 0x03000000;
pub const AR_PHY_TPC_6_ERROR_EST_MODE_S: c_int = 24;
//
// SM Register Map
//
pub const AR_SM_BASE: c_uint = 0xa200;

pub const AR_PHY_FLC_PWR_THRESH: c_int = 7;
pub const AR_PHY_FLC_PWR_THRESH_S: c_int = 0;
pub const AR_PHY_FRAME_CTL_CF_OVERLAP_WINDOW: c_int = 3;
pub const AR_PHY_FRAME_CTL_CF_OVERLAP_WINDOW_S: c_int = 0;
pub const AR_PHY_SPUR_MASK_A_CF_PUNC_MASK_IDX_A: c_uint = 0x0001FC00;
pub const AR_PHY_SPUR_MASK_A_CF_PUNC_MASK_IDX_A_S: c_int = 10;
pub const AR_PHY_SPUR_MASK_A_CF_PUNC_MASK_A: c_uint = 0x3FF;
pub const AR_PHY_SPUR_MASK_A_CF_PUNC_MASK_A_S: c_int = 0;

pub const AR_PHY_TEST_BBB_OBS_SEL: c_uint = 0x780000;
pub const AR_PHY_TEST_BBB_OBS_SEL_S: c_int = 19;
pub const AR_PHY_TEST_RX_OBS_SEL_BIT5_S: c_int = 23;

pub const AR_PHY_TEST_CHAIN_SEL: c_uint = 0xC0000000;
pub const AR_PHY_TEST_CHAIN_SEL_S: c_int = 30;

pub const AR_PHY_TEST_CTL_TSTDAC_EN: c_uint = 0x1;
pub const AR_PHY_TEST_CTL_TSTDAC_EN_S: c_int = 0;
pub const AR_PHY_TEST_CTL_TX_OBS_SEL: c_uint = 0x1C;
pub const AR_PHY_TEST_CTL_TX_OBS_SEL_S: c_int = 2;
pub const AR_PHY_TEST_CTL_TX_OBS_MUX_SEL: c_uint = 0x60;
pub const AR_PHY_TEST_CTL_TX_OBS_MUX_SEL_S: c_int = 5;
pub const AR_PHY_TEST_CTL_TSTADC_EN: c_uint = 0x100;
pub const AR_PHY_TEST_CTL_TSTADC_EN_S: c_int = 8;
pub const AR_PHY_TEST_CTL_RX_OBS_SEL: c_uint = 0x3C00;
pub const AR_PHY_TEST_CTL_RX_OBS_SEL_S: c_int = 10;
pub const AR_PHY_TEST_CTL_DEBUGPORT_SEL: c_uint = 0xe0000000;
pub const AR_PHY_TEST_CTL_DEBUGPORT_SEL_S: c_int = 29;

pub const AR_PHY_CHAN_INFO_MEMORY_CHANINFOMEM_S2_READ: c_uint = 0x00000008;
pub const AR_PHY_CHAN_INFO_MEMORY_CHANINFOMEM_S2_READ_S: c_int = 3;

pub const AR_PHY_TPC_1_FORCED_DAC_GAIN: c_uint = 0x0000003e;
pub const AR_PHY_TPC_1_FORCED_DAC_GAIN_S: c_int = 1;
pub const AR_PHY_TPC_1_FORCE_DAC_GAIN: c_uint = 0x00000001;
pub const AR_PHY_TPC_1_FORCE_DAC_GAIN_S: c_int = 0;

pub const AR_PHY_TPC_11_OLPC_GAIN_DELTA: c_uint = 0x00ff0000;
pub const AR_PHY_TPC_11_OLPC_GAIN_DELTA_S: c_int = 16;

pub const AR_PHY_TPC_12_DESIRED_SCALE_HT40_5: c_uint = 0x3e000000;
pub const AR_PHY_TPC_12_DESIRED_SCALE_HT40_5_S: c_int = 25;

pub const AR_PHY_TPC_18_THERM_CAL_VALUE: c_uint = 0x000000ff;
pub const AR_PHY_TPC_18_THERM_CAL_VALUE_S: c_int = 0;
pub const AR_PHY_TPC_18_VOLT_CAL_VALUE: c_uint = 0x0000ff00;
pub const AR_PHY_TPC_18_VOLT_CAL_VALUE_S: c_int = 8;

pub const AR_PHY_TPC_19_ALPHA_VOLT: c_uint = 0x001f0000;
pub const AR_PHY_TPC_19_ALPHA_VOLT_S: c_int = 16;
pub const AR_PHY_TPC_19_ALPHA_THERM: c_uint = 0xff;
pub const AR_PHY_TPC_19_ALPHA_THERM_S: c_int = 0;

pub const AR_PHY_TX_FORCED_GAIN_FORCE_TX_GAIN: c_uint = 0x00000001;
pub const AR_PHY_TX_FORCED_GAIN_FORCE_TX_GAIN_S: c_int = 0;
pub const AR_PHY_TX_FORCED_GAIN_FORCED_TXBB1DBGAIN: c_uint = 0x0000000e;
pub const AR_PHY_TX_FORCED_GAIN_FORCED_TXBB1DBGAIN_S: c_int = 1;
pub const AR_PHY_TX_FORCED_GAIN_FORCED_TXBB6DBGAIN: c_uint = 0x00000030;
pub const AR_PHY_TX_FORCED_GAIN_FORCED_TXBB6DBGAIN_S: c_int = 4;
pub const AR_PHY_TX_FORCED_GAIN_FORCED_TXMXRGAIN: c_uint = 0x000003c0;
pub const AR_PHY_TX_FORCED_GAIN_FORCED_TXMXRGAIN_S: c_int = 6;
pub const AR_PHY_TX_FORCED_GAIN_FORCED_PADRVGNA: c_uint = 0x00003c00;
pub const AR_PHY_TX_FORCED_GAIN_FORCED_PADRVGNA_S: c_int = 10;
pub const AR_PHY_TX_FORCED_GAIN_FORCED_PADRVGNB: c_uint = 0x0003c000;
pub const AR_PHY_TX_FORCED_GAIN_FORCED_PADRVGNB_S: c_int = 14;
pub const AR_PHY_TX_FORCED_GAIN_FORCED_PADRVGNC: c_uint = 0x003c0000;
pub const AR_PHY_TX_FORCED_GAIN_FORCED_PADRVGNC_S: c_int = 18;
pub const AR_PHY_TX_FORCED_GAIN_FORCED_PADRVGND: c_uint = 0x00c00000;
pub const AR_PHY_TX_FORCED_GAIN_FORCED_PADRVGND_S: c_int = 22;
pub const AR_PHY_TX_FORCED_GAIN_FORCED_ENABLE_PAL: c_uint = 0x01000000;
pub const AR_PHY_TX_FORCED_GAIN_FORCED_ENABLE_PAL_S: c_int = 24;

pub const AR_PHY_BB_THERM_ADC_1_INIT_THERM: c_uint = 0x000000ff;
pub const AR_PHY_BB_THERM_ADC_1_INIT_THERM_S: c_int = 0;

pub const AR_PHY_BB_THERM_ADC_3_THERM_ADC_SCALE_GAIN: c_uint = 0x0001ff00;
pub const AR_PHY_BB_THERM_ADC_3_THERM_ADC_SCALE_GAIN_S: c_int = 8;
pub const AR_PHY_BB_THERM_ADC_3_THERM_ADC_OFFSET: c_uint = 0x000000ff;
pub const AR_PHY_BB_THERM_ADC_3_THERM_ADC_OFFSET_S: c_int = 0;

pub const AR_PHY_BB_THERM_ADC_4_LATEST_THERM_VALUE: c_uint = 0x000000ff;
pub const AR_PHY_BB_THERM_ADC_4_LATEST_THERM_VALUE_S: c_int = 0;
pub const AR_PHY_BB_THERM_ADC_4_LATEST_VOLT_VALUE: c_uint = 0x0000ff00;
pub const AR_PHY_BB_THERM_ADC_4_LATEST_VOLT_VALUE_S: c_int = 8;
pub const AR_PHY_65NM_CH0_TXRF3: c_uint = 0x16048;
pub const AR_PHY_65NM_CH0_TXRF3_CAPDIV2G: c_uint = 0x0000001e;
pub const AR_PHY_65NM_CH0_TXRF3_CAPDIV2G_S: c_int = 1;
pub const AR_PHY_65NM_CH0_SYNTH4: c_uint = 0x1608c;

pub const AR_PHY_65NM_CH0_SYNTH7: c_uint = 0x16098;
pub const AR_PHY_65NM_CH0_SYNTH12: c_uint = 0x160ac;
pub const AR_PHY_65NM_CH0_BIAS1: c_uint = 0x160c0;
pub const AR_PHY_65NM_CH0_BIAS2: c_uint = 0x160c4;
pub const AR_PHY_65NM_CH0_BIAS4: c_uint = 0x160cc;
pub const AR_PHY_65NM_CH0_RXTX2: c_uint = 0x16104;
pub const AR_PHY_65NM_CH1_RXTX2: c_uint = 0x16504;
pub const AR_PHY_65NM_CH2_RXTX2: c_uint = 0x16904;
pub const AR_PHY_65NM_CH0_RXTX4: c_uint = 0x1610c;
pub const AR_PHY_65NM_CH1_RXTX4: c_uint = 0x1650c;
pub const AR_PHY_65NM_CH2_RXTX4: c_uint = 0x1690c;
pub const AR_PHY_65NM_CH0_BB1: c_uint = 0x16140;
pub const AR_PHY_65NM_CH0_BB2: c_uint = 0x16144;
pub const AR_PHY_65NM_CH0_BB3: c_uint = 0x16148;
pub const AR_PHY_65NM_CH1_BB1: c_uint = 0x16540;
pub const AR_PHY_65NM_CH1_BB2: c_uint = 0x16544;
pub const AR_PHY_65NM_CH1_BB3: c_uint = 0x16548;
pub const AR_PHY_65NM_CH2_BB1: c_uint = 0x16940;
pub const AR_PHY_65NM_CH2_BB2: c_uint = 0x16944;
pub const AR_PHY_65NM_CH2_BB3: c_uint = 0x16948;
pub const AR_PHY_65NM_CH0_SYNTH12_VREFMUL3: c_uint = 0x00780000;
pub const AR_PHY_65NM_CH0_SYNTH12_VREFMUL3_S: c_int = 19;
pub const AR_PHY_65NM_CH0_RXTX2_SYNTHON_MASK: c_uint = 0x00000004;
pub const AR_PHY_65NM_CH0_RXTX2_SYNTHON_MASK_S: c_int = 2;
pub const AR_PHY_65NM_CH0_RXTX2_SYNTHOVR_MASK: c_uint = 0x00000008;
pub const AR_PHY_65NM_CH0_RXTX2_SYNTHOVR_MASK_S: c_int = 3;

pub const AR_CH0_THERM_XPABIASLVL_MSB: c_uint = 0x3;
pub const AR_CH0_THERM_XPABIASLVL_MSB_S: c_int = 0;
pub const AR_CH0_THERM_XPASHORT2GND: c_uint = 0x4;
pub const AR_CH0_THERM_XPASHORT2GND_S: c_int = 2;
pub const AR_CH0_THERM_LOCAL: c_uint = 0x80000000;
pub const AR_CH0_THERM_START: c_uint = 0x20000000;
pub const AR_CH0_THERM_SAR_ADC_OUT: c_uint = 0x0000ff00;
pub const AR_CH0_THERM_SAR_ADC_OUT_S: c_int = 8;

pub const AR_CH0_XTAL_CAPINDAC: c_uint = 0x7f000000;
pub const AR_CH0_XTAL_CAPINDAC_S: c_int = 24;
pub const AR_CH0_XTAL_CAPOUTDAC: c_uint = 0x00fe0000;
pub const AR_CH0_XTAL_CAPOUTDAC_S: c_int = 17;

pub const AR_PHY_PMU1_PWD: c_uint = 0x1;
pub const AR_PHY_PMU1_PWD_S: c_int = 0;

pub const AR_PHY_PMU2_PGM: c_uint = 0x00200000;
pub const AR_PHY_PMU2_PGM_S: c_int = 21;
pub const AR_PHY_RX1DB_BIQUAD_LONG_SHIFT: c_uint = 0x00380000;
pub const AR_PHY_RX1DB_BIQUAD_LONG_SHIFT_S: c_int = 19;
pub const AR_PHY_RX6DB_BIQUAD_LONG_SHIFT: c_uint = 0x00c00000;
pub const AR_PHY_RX6DB_BIQUAD_LONG_SHIFT_S: c_int = 22;
pub const AR_PHY_LNAGAIN_LONG_SHIFT: c_uint = 0xe0000000;
pub const AR_PHY_LNAGAIN_LONG_SHIFT_S: c_int = 29;
pub const AR_PHY_MXRGAIN_LONG_SHIFT: c_uint = 0x03000000;
pub const AR_PHY_MXRGAIN_LONG_SHIFT_S: c_int = 24;
pub const AR_PHY_VGAGAIN_LONG_SHIFT: c_uint = 0x1c000000;
pub const AR_PHY_VGAGAIN_LONG_SHIFT_S: c_int = 26;
pub const AR_PHY_SCFIR_GAIN_LONG_SHIFT: c_uint = 0x00000001;
pub const AR_PHY_SCFIR_GAIN_LONG_SHIFT_S: c_int = 0;
pub const AR_PHY_MANRXGAIN_LONG_SHIFT: c_uint = 0x00000002;
pub const AR_PHY_MANRXGAIN_LONG_SHIFT_S: c_int = 1;
//
// SM Field Definitions
//
pub const AR_PHY_CL_CAL_ENABLE: c_uint = 0x00000002;
pub const AR_PHY_PARALLEL_CAL_ENABLE: c_uint = 0x00000001;
pub const AR_PHY_TPCRG1_PD_CAL_ENABLE: c_uint = 0x00400000;
pub const AR_PHY_TPCRG1_PD_CAL_ENABLE_S: c_int = 22;
pub const AR_PHY_ADDAC_PARACTL_OFF_PWDADC: c_uint = 0x00008000;
pub const AR_PHY_FCAL20_CAP_STATUS_0: c_uint = 0x01f00000;
pub const AR_PHY_FCAL20_CAP_STATUS_0_S: c_int = 20;
pub const AR_PHY_RFBUS_REQ_EN: c_uint = 0x00000001  /* request for RF bus */;
pub const AR_PHY_RFBUS_GRANT_EN: c_uint = 0x00000001  /* RF bus granted */;
pub const AR_PHY_GC_TURBO_MODE: c_uint = 0x00000001  /* set turbo mode bits */;
pub const AR_PHY_GC_TURBO_SHORT: c_uint = 0x00000002  /* set short symbols to turbo mode setting */;
pub const AR_PHY_GC_DYN2040_EN: c_uint = 0x00000004  /* enable dyn 20/40 mode */;
pub const AR_PHY_GC_DYN2040_PRI_ONLY: c_uint = 0x00000008  /* dyn 20/40 - primary only */;
pub const AR_PHY_GC_DYN2040_PRI_CH: c_uint = 0x00000010  /* dyn 20/40 - primary ch offset (0=+10MHz, 1=-10MHz)*/;
pub const AR_PHY_GC_DYN2040_PRI_CH_S: c_int = 4;
pub const AR_PHY_GC_DYN2040_EXT_CH: c_uint = 0x00000020  /* dyn 20/40 - ext ch spacing (0=20MHz/ 1=25MHz) */;
pub const AR_PHY_GC_HT_EN: c_uint = 0x00000040  /* ht enable */;
pub const AR_PHY_GC_SHORT_GI_40: c_uint = 0x00000080  /* allow short GI for HT 40 */;
pub const AR_PHY_GC_WALSH: c_uint = 0x00000100  /* walsh spatial spreading for 2 chains,2 streams TX */;
pub const AR_PHY_GC_SINGLE_HT_LTF1: c_uint = 0x00000200  /* single length (4us) 1st HT long training symbol */;
pub const AR_PHY_GC_GF_DETECT_EN: c_uint = 0x00000400  /* enable Green Field detection. Only affects rx, not tx */;
pub const AR_PHY_GC_ENABLE_DAC_FIFO: c_uint = 0x00000800  /* fifo between bb and dac */;
pub const AR_PHY_RX_DELAY_DELAY: c_uint = 0x00003FFF  /* delay from wakeup to rx ena */;
pub const AR_PHY_CALMODE_IQ: c_uint = 0x00000000;
pub const AR_PHY_CALMODE_ADC_GAIN: c_uint = 0x00000001;
pub const AR_PHY_CALMODE_ADC_DC_PER: c_uint = 0x00000002;
pub const AR_PHY_CALMODE_ADC_DC_INIT: c_uint = 0x00000003;
pub const AR_PHY_SWAP_ALT_CHAIN: c_uint = 0x00000040;
pub const AR_PHY_MODE_OFDM: c_uint = 0x00000000;
pub const AR_PHY_MODE_CCK: c_uint = 0x00000001;
pub const AR_PHY_MODE_DYNAMIC: c_uint = 0x00000004;
pub const AR_PHY_MODE_DYNAMIC_S: c_int = 2;
pub const AR_PHY_MODE_HALF: c_uint = 0x00000020;
pub const AR_PHY_MODE_QUARTER: c_uint = 0x00000040;
pub const AR_PHY_MAC_CLK_MODE: c_uint = 0x00000080;
pub const AR_PHY_MODE_DYN_CCK_DISABLE: c_uint = 0x00000100;
pub const AR_PHY_MODE_SVD_HALF: c_uint = 0x00000200;
pub const AR_PHY_ACTIVE_EN: c_uint = 0x00000001;
pub const AR_PHY_ACTIVE_DIS: c_uint = 0x00000000;
pub const AR_PHY_FORCE_XPA_CFG: c_uint = 0x000000001;
pub const AR_PHY_FORCE_XPA_CFG_S: c_int = 0;
pub const AR_PHY_XPA_TIMING_CTL_TX_END_XPAB_OFF: c_uint = 0xFF000000;
pub const AR_PHY_XPA_TIMING_CTL_TX_END_XPAB_OFF_S: c_int = 24;
pub const AR_PHY_XPA_TIMING_CTL_TX_END_XPAA_OFF: c_uint = 0x00FF0000;
pub const AR_PHY_XPA_TIMING_CTL_TX_END_XPAA_OFF_S: c_int = 16;
pub const AR_PHY_XPA_TIMING_CTL_FRAME_XPAB_ON: c_uint = 0x0000FF00;
pub const AR_PHY_XPA_TIMING_CTL_FRAME_XPAB_ON_S: c_int = 8;
pub const AR_PHY_XPA_TIMING_CTL_FRAME_XPAA_ON: c_uint = 0x000000FF;
pub const AR_PHY_XPA_TIMING_CTL_FRAME_XPAA_ON_S: c_int = 0;
pub const AR_PHY_TX_END_TO_A2_RX_ON: c_uint = 0x00FF0000;
pub const AR_PHY_TX_END_TO_A2_RX_ON_S: c_int = 16;
pub const AR_PHY_TX_END_DATA_START: c_uint = 0x000000FF;
pub const AR_PHY_TX_END_DATA_START_S: c_int = 0;
pub const AR_PHY_TX_END_PA_ON: c_uint = 0x0000FF00;
pub const AR_PHY_TX_END_PA_ON_S: c_int = 8;
pub const AR_PHY_TPCRG5_PD_GAIN_OVERLAP: c_uint = 0x0000000F;
pub const AR_PHY_TPCRG5_PD_GAIN_OVERLAP_S: c_int = 0;
pub const AR_PHY_TPCRG5_PD_GAIN_BOUNDARY_1: c_uint = 0x000003F0;
pub const AR_PHY_TPCRG5_PD_GAIN_BOUNDARY_1_S: c_int = 4;
pub const AR_PHY_TPCRG5_PD_GAIN_BOUNDARY_2: c_uint = 0x0000FC00;
pub const AR_PHY_TPCRG5_PD_GAIN_BOUNDARY_2_S: c_int = 10;
pub const AR_PHY_TPCRG5_PD_GAIN_BOUNDARY_3: c_uint = 0x003F0000;
pub const AR_PHY_TPCRG5_PD_GAIN_BOUNDARY_3_S: c_int = 16;
pub const AR_PHY_TPCRG5_PD_GAIN_BOUNDARY_4: c_uint = 0x0FC00000;
pub const AR_PHY_TPCRG5_PD_GAIN_BOUNDARY_4_S: c_int = 22;
pub const AR_PHY_TPCRG1_NUM_PD_GAIN: c_uint = 0x0000c000;
pub const AR_PHY_TPCRG1_NUM_PD_GAIN_S: c_int = 14;
pub const AR_PHY_TPCRG1_PD_GAIN_1: c_uint = 0x00030000;
pub const AR_PHY_TPCRG1_PD_GAIN_1_S: c_int = 16;
pub const AR_PHY_TPCRG1_PD_GAIN_2: c_uint = 0x000C0000;
pub const AR_PHY_TPCRG1_PD_GAIN_2_S: c_int = 18;
pub const AR_PHY_TPCRG1_PD_GAIN_3: c_uint = 0x00300000;
pub const AR_PHY_TPCRG1_PD_GAIN_3_S: c_int = 20;
pub const AR_PHY_TPCGR1_FORCED_DAC_GAIN: c_uint = 0x0000003e;
pub const AR_PHY_TPCGR1_FORCED_DAC_GAIN_S: c_int = 1;
pub const AR_PHY_TPCGR1_FORCE_DAC_GAIN: c_uint = 0x00000001;
pub const AR_PHY_TXGAIN_FORCE: c_uint = 0x00000001;
pub const AR_PHY_TXGAIN_FORCE_S: c_int = 0;
pub const AR_PHY_TXGAIN_FORCED_PADVGNRA: c_uint = 0x00003c00;
pub const AR_PHY_TXGAIN_FORCED_PADVGNRA_S: c_int = 10;
pub const AR_PHY_TXGAIN_FORCED_PADVGNRB: c_uint = 0x0003c000;
pub const AR_PHY_TXGAIN_FORCED_PADVGNRB_S: c_int = 14;
pub const AR_PHY_TXGAIN_FORCED_PADVGNRD: c_uint = 0x00c00000;
pub const AR_PHY_TXGAIN_FORCED_PADVGNRD_S: c_int = 22;
pub const AR_PHY_TXGAIN_FORCED_TXMXRGAIN: c_uint = 0x000003c0;
pub const AR_PHY_TXGAIN_FORCED_TXMXRGAIN_S: c_int = 6;
pub const AR_PHY_TXGAIN_FORCED_TXBB1DBGAIN: c_uint = 0x0000000e;
pub const AR_PHY_TXGAIN_FORCED_TXBB1DBGAIN_S: c_int = 1;
pub const AR_PHY_POWER_TX_RATE_MAX: c_uint = 0x993c;
pub const AR_PHY_POWER_TX_RATE_MAX_TPC_ENABLE: c_uint = 0x00000040;
pub const PHY_AGC_CLR: c_uint = 0x10000000;
pub const RFSILENT_BB: c_uint = 0x00002000;
pub const AR_PHY_CHAN_INFO_GAIN_DIFF_PPM_MASK: c_uint = 0xFFF;
pub const AR_PHY_CHAN_INFO_GAIN_DIFF_PPM_SIGNED_BIT: c_uint = 0x800;
pub const AR_PHY_CHAN_INFO_GAIN_DIFF_UPPER_LIMIT: c_int = 320;
pub const AR_PHY_CHAN_INFO_MEMORY_CAPTURE_MASK: c_uint = 0x0001;
pub const AR_PHY_RX_DELAY_DELAY: c_uint = 0x00003FFF;
pub const AR_PHY_CCK_TX_CTRL_JAPAN: c_uint = 0x00000010;
pub const AR_PHY_SPECTRAL_SCAN_ENABLE: c_uint = 0x00000001;
pub const AR_PHY_SPECTRAL_SCAN_ENABLE_S: c_int = 0;
pub const AR_PHY_SPECTRAL_SCAN_ACTIVE: c_uint = 0x00000002;
pub const AR_PHY_SPECTRAL_SCAN_ACTIVE_S: c_int = 1;
pub const AR_PHY_SPECTRAL_SCAN_FFT_PERIOD: c_uint = 0x000000F0;
pub const AR_PHY_SPECTRAL_SCAN_FFT_PERIOD_S: c_int = 4;
pub const AR_PHY_SPECTRAL_SCAN_PERIOD: c_uint = 0x0000FF00;
pub const AR_PHY_SPECTRAL_SCAN_PERIOD_S: c_int = 8;
pub const AR_PHY_SPECTRAL_SCAN_COUNT: c_uint = 0x0FFF0000;
pub const AR_PHY_SPECTRAL_SCAN_COUNT_S: c_int = 16;
pub const AR_PHY_SPECTRAL_SCAN_SHORT_REPEAT: c_uint = 0x10000000;
pub const AR_PHY_SPECTRAL_SCAN_SHORT_REPEAT_S: c_int = 28;
pub const AR_PHY_SPECTRAL_SCAN_PRIORITY: c_uint = 0x20000000;
pub const AR_PHY_SPECTRAL_SCAN_PRIORITY_S: c_int = 29;
pub const AR_PHY_SPECTRAL_SCAN_USE_ERR5: c_uint = 0x40000000;
pub const AR_PHY_SPECTRAL_SCAN_USE_ERR5_S: c_int = 30;
pub const AR_PHY_SPECTRAL_SCAN_COMPRESSED_RPT: c_uint = 0x80000000;
pub const AR_PHY_SPECTRAL_SCAN_COMPRESSED_RPT_S: c_int = 31;
pub const AR_PHY_CHANNEL_STATUS_RX_CLEAR: c_uint = 0x00000004;
pub const AR_PHY_RTT_CTRL_ENA_RADIO_RETENTION: c_uint = 0x00000001;
pub const AR_PHY_RTT_CTRL_ENA_RADIO_RETENTION_S: c_int = 0;
pub const AR_PHY_RTT_CTRL_RESTORE_MASK: c_uint = 0x0000007E;
pub const AR_PHY_RTT_CTRL_RESTORE_MASK_S: c_int = 1;
pub const AR_PHY_RTT_CTRL_FORCE_RADIO_RESTORE: c_uint = 0x00000080;
pub const AR_PHY_RTT_CTRL_FORCE_RADIO_RESTORE_S: c_int = 7;
pub const AR_PHY_RTT_SW_RTT_TABLE_ACCESS: c_uint = 0x00000001;
pub const AR_PHY_RTT_SW_RTT_TABLE_ACCESS_S: c_int = 0;
pub const AR_PHY_RTT_SW_RTT_TABLE_WRITE: c_uint = 0x00000002;
pub const AR_PHY_RTT_SW_RTT_TABLE_WRITE_S: c_int = 1;
pub const AR_PHY_RTT_SW_RTT_TABLE_ADDR: c_uint = 0x0000001C;
pub const AR_PHY_RTT_SW_RTT_TABLE_ADDR_S: c_int = 2;
pub const AR_PHY_RTT_SW_RTT_TABLE_DATA: c_uint = 0xFFFFFFF0;
pub const AR_PHY_RTT_SW_RTT_TABLE_DATA_S: c_int = 4;
pub const AR_PHY_TX_IQCAL_CONTROL_0_ENABLE_TXIQ_CAL: c_uint = 0x80000000;
pub const AR_PHY_TX_IQCAL_CONTROL_0_ENABLE_TXIQ_CAL_S: c_int = 31;
pub const AR_PHY_TX_IQCAL_CONTROL_1_IQCORR_I_Q_COFF_DELPT: c_uint = 0x01fc0000;
pub const AR_PHY_TX_IQCAL_CONTROL_1_IQCORR_I_Q_COFF_DELPT_S: c_int = 18;
pub const AR_PHY_TX_IQCAL_START_DO_CAL: c_uint = 0x00000001;
pub const AR_PHY_TX_IQCAL_START_DO_CAL_S: c_int = 0;
pub const AR_PHY_TX_IQCAL_STATUS_FAILED: c_uint = 0x00000001;
pub const AR_PHY_CALIBRATED_GAINS_0: c_uint = 0x3e;
pub const AR_PHY_CALIBRATED_GAINS_0_S: c_int = 1;
pub const AR_PHY_TX_IQCAL_CORR_COEFF_00_COEFF_TABLE: c_uint = 0x00003fff;
pub const AR_PHY_TX_IQCAL_CORR_COEFF_00_COEFF_TABLE_S: c_int = 0;
pub const AR_PHY_TX_IQCAL_CORR_COEFF_01_COEFF_TABLE: c_uint = 0x0fffc000;
pub const AR_PHY_TX_IQCAL_CORR_COEFF_01_COEFF_TABLE_S: c_int = 14;
pub const AR_PHY_65NM_CH0_RXTX4_THERM_ON: c_uint = 0x10000000;
pub const AR_PHY_65NM_CH0_RXTX4_THERM_ON_S: c_int = 28;
pub const AR_PHY_65NM_CH0_RXTX4_THERM_ON_OVR: c_uint = 0x20000000;
pub const AR_PHY_65NM_CH0_RXTX4_THERM_ON_OVR_S: c_int = 29;
pub const AR_PHY_65NM_RXTX4_XLNA_BIAS: c_uint = 0xC0000000;
pub const AR_PHY_65NM_RXTX4_XLNA_BIAS_S: c_int = 30;
//
// Channel 1 Register Map
//
pub const AR_CHAN1_BASE: c_uint = 0xa800;

//
// Channel 1 Field Definitions
//
pub const AR_PHY_CH1_EXT_MINCCA_PWR: c_uint = 0x01FF0000;
pub const AR_PHY_CH1_EXT_MINCCA_PWR_S: c_int = 16;
//
// AGC 1 Register Map
//
pub const AR_AGC1_BASE: c_uint = 0xae00;

//
// AGC 1 Field Definitions
//
pub const AR_PHY_CH1_MINCCA_PWR: c_uint = 0x1FF00000;
pub const AR_PHY_CH1_MINCCA_PWR_S: c_int = 20;
//
// SM 1 Register Map
//
pub const AR_SM1_BASE: c_uint = 0xb200;

pub const AR_PHY_TPC_19_B1_ALPHA_THERM: c_uint = 0xff;
pub const AR_PHY_TPC_19_B1_ALPHA_THERM_S: c_int = 0;

//
// Channel 2 Register Map
//
pub const AR_CHAN2_BASE: c_uint = 0xb800;

//
// Channel 2 Field Definitions
//
pub const AR_PHY_CH2_EXT_MINCCA_PWR: c_uint = 0x01FF0000;
pub const AR_PHY_CH2_EXT_MINCCA_PWR_S: c_int = 16;
//
// AGC 2 Register Map
//
pub const AR_AGC2_BASE: c_uint = 0xbe00;

//
// AGC 2 Field Definitions
//
pub const AR_PHY_CH2_MINCCA_PWR: c_uint = 0x1FF00000;
pub const AR_PHY_CH2_MINCCA_PWR_S: c_int = 20;
//
// SM 2 Register Map
//
pub const AR_SM2_BASE: c_uint = 0xc200;

pub const AR_PHY_TX_IQCAL_STATUS_B2_FAILED: c_uint = 0x00000001;
// GLB Registers
pub const AR_GLB_BASE: c_uint = 0x20000;

//
// Misc helper defines
//

pub const AR_PHY_WATCHDOG_NON_IDLE_ENABLE: c_uint = 0x00000001;
pub const AR_PHY_WATCHDOG_IDLE_ENABLE: c_uint = 0x00000002;
pub const AR_PHY_WATCHDOG_IDLE_MASK: c_uint = 0xFFFF0000;
pub const AR_PHY_WATCHDOG_NON_IDLE_MASK: c_uint = 0x0000FFFC;
pub const AR_PHY_WATCHDOG_RST_ENABLE: c_uint = 0x00000002;
pub const AR_PHY_WATCHDOG_IRQ_ENABLE: c_uint = 0x00000004;
pub const AR_PHY_WATCHDOG_CNTL2_MASK: c_uint = 0xFFFFFFF9;
pub const AR_PHY_WATCHDOG_INFO: c_uint = 0x00000007;
pub const AR_PHY_WATCHDOG_INFO_S: c_int = 0;
pub const AR_PHY_WATCHDOG_DET_HANG: c_uint = 0x00000008;
pub const AR_PHY_WATCHDOG_DET_HANG_S: c_int = 3;
pub const AR_PHY_WATCHDOG_RADAR_SM: c_uint = 0x000000F0;
pub const AR_PHY_WATCHDOG_RADAR_SM_S: c_int = 4;
pub const AR_PHY_WATCHDOG_RX_OFDM_SM: c_uint = 0x00000F00;
pub const AR_PHY_WATCHDOG_RX_OFDM_SM_S: c_int = 8;
pub const AR_PHY_WATCHDOG_RX_CCK_SM: c_uint = 0x0000F000;
pub const AR_PHY_WATCHDOG_RX_CCK_SM_S: c_int = 12;
pub const AR_PHY_WATCHDOG_TX_OFDM_SM: c_uint = 0x000F0000;
pub const AR_PHY_WATCHDOG_TX_OFDM_SM_S: c_int = 16;
pub const AR_PHY_WATCHDOG_TX_CCK_SM: c_uint = 0x00F00000;
pub const AR_PHY_WATCHDOG_TX_CCK_SM_S: c_int = 20;
pub const AR_PHY_WATCHDOG_AGC_SM: c_uint = 0x0F000000;
pub const AR_PHY_WATCHDOG_AGC_SM_S: c_int = 24;
pub const AR_PHY_WATCHDOG_SRCH_SM: c_uint = 0xF0000000;
pub const AR_PHY_WATCHDOG_SRCH_SM_S: c_int = 28;
pub const AR_PHY_WATCHDOG_STATUS_CLR: c_uint = 0x00000008;
//
// PAPRD registers
//

pub const AR_PHY_PAPRD_AM2AM_MASK: c_uint = 0x01ffffff;
pub const AR_PHY_PAPRD_AM2AM_MASK_S: c_int = 0;

pub const AR_PHY_PAPRD_AM2PM_MASK: c_uint = 0x01ffffff;
pub const AR_PHY_PAPRD_AM2PM_MASK_S: c_int = 0;

pub const AR_PHY_PAPRD_HT40_MASK: c_uint = 0x01ffffff;
pub const AR_PHY_PAPRD_HT40_MASK_S: c_int = 0;

pub const AR_PHY_PAPRD_CTRL0_PAPRD_ENABLE: c_uint = 0x00000001;
pub const AR_PHY_PAPRD_CTRL0_PAPRD_ENABLE_S: c_int = 0;
pub const AR_PHY_PAPRD_CTRL0_USE_SINGLE_TABLE_MASK: c_uint = 0x00000002;
pub const AR_PHY_PAPRD_CTRL0_USE_SINGLE_TABLE_MASK_S: c_int = 1;
pub const AR_PHY_PAPRD_CTRL0_PAPRD_MAG_THRSH: c_uint = 0xf8000000;
pub const AR_PHY_PAPRD_CTRL0_PAPRD_MAG_THRSH_S: c_int = 27;

pub const AR_PHY_PAPRD_CTRL1_ADAPTIVE_SCALING_ENA: c_uint = 0x00000001;
pub const AR_PHY_PAPRD_CTRL1_ADAPTIVE_SCALING_ENA_S: c_int = 0;
pub const AR_PHY_PAPRD_CTRL1_ADAPTIVE_AM2AM_ENABLE: c_uint = 0x00000002;
pub const AR_PHY_PAPRD_CTRL1_ADAPTIVE_AM2AM_ENABLE_S: c_int = 1;
pub const AR_PHY_PAPRD_CTRL1_ADAPTIVE_AM2PM_ENABLE: c_uint = 0x00000004;
pub const AR_PHY_PAPRD_CTRL1_ADAPTIVE_AM2PM_ENABLE_S: c_int = 2;
pub const AR_PHY_PAPRD_CTRL1_PAPRD_POWER_AT_AM2AM_CAL: c_uint = 0x000001f8;
pub const AR_PHY_PAPRD_CTRL1_PAPRD_POWER_AT_AM2AM_CAL_S: c_int = 3;
pub const AR_PHY_PAPRD_CTRL1_PA_GAIN_SCALE_FACT_MASK: c_uint = 0x0001fe00;
pub const AR_PHY_PAPRD_CTRL1_PA_GAIN_SCALE_FACT_MASK_S: c_int = 9;
pub const AR_PHY_PAPRD_CTRL1_PAPRD_MAG_SCALE_FACT: c_uint = 0x0ffe0000;
pub const AR_PHY_PAPRD_CTRL1_PAPRD_MAG_SCALE_FACT_S: c_int = 17;

pub const AR_PHY_PAPRD_TRAINER_CNTL1_CF_CF_PAPRD_TRAIN_ENABLE: c_uint = 0x00000001;
pub const AR_PHY_PAPRD_TRAINER_CNTL1_CF_CF_PAPRD_TRAIN_ENABLE_S: c_int = 0;
pub const AR_PHY_PAPRD_TRAINER_CNTL1_CF_PAPRD_AGC2_SETTLING: c_uint = 0x0000007e;
pub const AR_PHY_PAPRD_TRAINER_CNTL1_CF_PAPRD_AGC2_SETTLING_S: c_int = 1;
pub const AR_PHY_PAPRD_TRAINER_CNTL1_CF_PAPRD_IQCORR_ENABLE: c_uint = 0x00000100;
pub const AR_PHY_PAPRD_TRAINER_CNTL1_CF_PAPRD_IQCORR_ENABLE_S: c_int = 8;
pub const AR_PHY_PAPRD_TRAINER_CNTL1_CF_PAPRD_RX_BB_GAIN_FORCE: c_uint = 0x00000200;
pub const AR_PHY_PAPRD_TRAINER_CNTL1_CF_PAPRD_RX_BB_GAIN_FORCE_S: c_int = 9;
pub const AR_PHY_PAPRD_TRAINER_CNTL1_CF_PAPRD_TX_GAIN_FORCE: c_uint = 0x00000400;
pub const AR_PHY_PAPRD_TRAINER_CNTL1_CF_PAPRD_TX_GAIN_FORCE_S: c_int = 10;
pub const AR_PHY_PAPRD_TRAINER_CNTL1_CF_PAPRD_LB_ENABLE: c_uint = 0x00000800;
pub const AR_PHY_PAPRD_TRAINER_CNTL1_CF_PAPRD_LB_ENABLE_S: c_int = 11;
pub const AR_PHY_PAPRD_TRAINER_CNTL1_CF_PAPRD_LB_SKIP: c_uint = 0x0003f000;
pub const AR_PHY_PAPRD_TRAINER_CNTL1_CF_PAPRD_LB_SKIP_S: c_int = 12;

pub const AR_PHY_PAPRD_TRAINER_CNTL2_CF_PAPRD_INIT_RX_BB_GAIN: c_uint = 0xFFFFFFFF;
pub const AR_PHY_PAPRD_TRAINER_CNTL2_CF_PAPRD_INIT_RX_BB_GAIN_S: c_int = 0;

pub const AR_PHY_PAPRD_TRAINER_CNTL3_CF_PAPRD_ADC_DESIRED_SIZE: c_uint = 0x0000003f;
pub const AR_PHY_PAPRD_TRAINER_CNTL3_CF_PAPRD_ADC_DESIRED_SIZE_S: c_int = 0;
pub const AR_PHY_PAPRD_TRAINER_CNTL3_CF_PAPRD_QUICK_DROP: c_uint = 0x00000fc0;
pub const AR_PHY_PAPRD_TRAINER_CNTL3_CF_PAPRD_QUICK_DROP_S: c_int = 6;
pub const AR_PHY_PAPRD_TRAINER_CNTL3_CF_PAPRD_MIN_LOOPBACK_DEL: c_uint = 0x0001f000;
pub const AR_PHY_PAPRD_TRAINER_CNTL3_CF_PAPRD_MIN_LOOPBACK_DEL_S: c_int = 12;
pub const AR_PHY_PAPRD_TRAINER_CNTL3_CF_PAPRD_NUM_CORR_STAGES: c_uint = 0x000e0000;
pub const AR_PHY_PAPRD_TRAINER_CNTL3_CF_PAPRD_NUM_CORR_STAGES_S: c_int = 17;
pub const AR_PHY_PAPRD_TRAINER_CNTL3_CF_PAPRD_COARSE_CORR_LEN: c_uint = 0x00f00000;
pub const AR_PHY_PAPRD_TRAINER_CNTL3_CF_PAPRD_COARSE_CORR_LEN_S: c_int = 20;
pub const AR_PHY_PAPRD_TRAINER_CNTL3_CF_PAPRD_FINE_CORR_LEN: c_uint = 0x0f000000;
pub const AR_PHY_PAPRD_TRAINER_CNTL3_CF_PAPRD_FINE_CORR_LEN_S: c_int = 24;
pub const AR_PHY_PAPRD_TRAINER_CNTL3_CF_PAPRD_BBTXMIX_DISABLE: c_uint = 0x20000000;
pub const AR_PHY_PAPRD_TRAINER_CNTL3_CF_PAPRD_BBTXMIX_DISABLE_S: c_int = 29;

pub const AR_PHY_PAPRD_TRAINER_CNTL4_CF_PAPRD_NUM_TRAIN_SAMPLES: c_uint = 0x03ff0000;
pub const AR_PHY_PAPRD_TRAINER_CNTL4_CF_PAPRD_NUM_TRAIN_SAMPLES_S: c_int = 16;
pub const AR_PHY_PAPRD_TRAINER_CNTL4_CF_PAPRD_SAFETY_DELTA: c_uint = 0x0000f000;
pub const AR_PHY_PAPRD_TRAINER_CNTL4_CF_PAPRD_SAFETY_DELTA_S: c_int = 12;
pub const AR_PHY_PAPRD_TRAINER_CNTL4_CF_PAPRD_MIN_CORR: c_uint = 0x00000fff;
pub const AR_PHY_PAPRD_TRAINER_CNTL4_CF_PAPRD_MIN_CORR_S: c_int = 0;

pub const AR_PHY_PAPRD_PRE_POST_SCALING: c_uint = 0x3FFFF;
pub const AR_PHY_PAPRD_PRE_POST_SCALING_S: c_int = 0;

pub const AR_PHY_PAPRD_TRAINER_STAT1_PAPRD_TRAIN_DONE: c_uint = 0x00000001;
pub const AR_PHY_PAPRD_TRAINER_STAT1_PAPRD_TRAIN_DONE_S: c_int = 0;
pub const AR_PHY_PAPRD_TRAINER_STAT1_PAPRD_TRAIN_INCOMPLETE: c_uint = 0x00000002;
pub const AR_PHY_PAPRD_TRAINER_STAT1_PAPRD_TRAIN_INCOMPLETE_S: c_int = 1;
pub const AR_PHY_PAPRD_TRAINER_STAT1_PAPRD_CORR_ERR: c_uint = 0x00000004;
pub const AR_PHY_PAPRD_TRAINER_STAT1_PAPRD_CORR_ERR_S: c_int = 2;
pub const AR_PHY_PAPRD_TRAINER_STAT1_PAPRD_TRAIN_ACTIVE: c_uint = 0x00000008;
pub const AR_PHY_PAPRD_TRAINER_STAT1_PAPRD_TRAIN_ACTIVE_S: c_int = 3;
pub const AR_PHY_PAPRD_TRAINER_STAT1_PAPRD_RX_GAIN_IDX: c_uint = 0x000001f0;
pub const AR_PHY_PAPRD_TRAINER_STAT1_PAPRD_RX_GAIN_IDX_S: c_int = 4;
pub const AR_PHY_PAPRD_TRAINER_STAT1_PAPRD_AGC2_PWR: c_uint = 0x0001fe00;
pub const AR_PHY_PAPRD_TRAINER_STAT1_PAPRD_AGC2_PWR_S: c_int = 9;

pub const AR_PHY_PAPRD_TRAINER_STAT2_PAPRD_FINE_VAL: c_uint = 0x0000ffff;
pub const AR_PHY_PAPRD_TRAINER_STAT2_PAPRD_FINE_VAL_S: c_int = 0;
pub const AR_PHY_PAPRD_TRAINER_STAT2_PAPRD_COARSE_IDX: c_uint = 0x001f0000;
pub const AR_PHY_PAPRD_TRAINER_STAT2_PAPRD_COARSE_IDX_S: c_int = 16;
pub const AR_PHY_PAPRD_TRAINER_STAT2_PAPRD_FINE_IDX: c_uint = 0x00600000;
pub const AR_PHY_PAPRD_TRAINER_STAT2_PAPRD_FINE_IDX_S: c_int = 21;

pub const AR_PHY_PAPRD_TRAINER_STAT3_PAPRD_TRAIN_SAMPLES_CNT: c_uint = 0x000fffff;
pub const AR_PHY_PAPRD_TRAINER_STAT3_PAPRD_TRAIN_SAMPLES_CNT_S: c_int = 0;

pub const AR_PHY_PA_GAIN123_PA_GAIN1: c_uint = 0x3FF;
pub const AR_PHY_PA_GAIN123_PA_GAIN1_S: c_int = 0;

pub const AR_PHY_POWERTX_RATE5_POWERTXHT20_0: c_uint = 0x3F;
pub const AR_PHY_POWERTX_RATE5_POWERTXHT20_0_S: c_int = 0;

pub const AR_PHY_POWERTX_RATE6_POWERTXHT20_5: c_uint = 0x3F00;
pub const AR_PHY_POWERTX_RATE6_POWERTXHT20_5_S: c_int = 8;

pub const AR_PHY_POWERTX_RATE8_POWERTXHT40_5: c_uint = 0x3F00;
pub const AR_PHY_POWERTX_RATE8_POWERTXHT40_5_S: c_int = 8;
pub const AR_PHY_CL_TAB_CL_GAIN_MOD: c_uint = 0x1f;
pub const AR_PHY_CL_TAB_CL_GAIN_MOD_S: c_int = 0;
pub const AR_BTCOEX_WL_LNADIV: c_uint = 0x1a64;
pub const AR_BTCOEX_WL_LNADIV_PREDICTED_PERIOD: c_uint = 0x00003FFF;
pub const AR_BTCOEX_WL_LNADIV_PREDICTED_PERIOD_S: c_int = 0;
pub const AR_BTCOEX_WL_LNADIV_DPDT_IGNORE_PRIORITY: c_uint = 0x00004000;
pub const AR_BTCOEX_WL_LNADIV_DPDT_IGNORE_PRIORITY_S: c_int = 14;
pub const AR_BTCOEX_WL_LNADIV_FORCE_ON: c_uint = 0x00008000;
pub const AR_BTCOEX_WL_LNADIV_FORCE_ON_S: c_int = 15;
pub const AR_BTCOEX_WL_LNADIV_MODE_OPTION: c_uint = 0x00030000;
pub const AR_BTCOEX_WL_LNADIV_MODE_OPTION_S: c_int = 16;
pub const AR_BTCOEX_WL_LNADIV_MODE: c_uint = 0x007c0000;
pub const AR_BTCOEX_WL_LNADIV_MODE_S: c_int = 18;
pub const AR_BTCOEX_WL_LNADIV_ALLOWED_TX_ANTDIV_WL_TX_REQ: c_uint = 0x00800000;
pub const AR_BTCOEX_WL_LNADIV_ALLOWED_TX_ANTDIV_WL_TX_REQ_S: c_int = 23;
pub const AR_BTCOEX_WL_LNADIV_DISABLE_TX_ANTDIV_ENABLE: c_uint = 0x01000000;
pub const AR_BTCOEX_WL_LNADIV_DISABLE_TX_ANTDIV_ENABLE_S: c_int = 24;
pub const AR_BTCOEX_WL_LNADIV_CONTINUOUS_BT_ACTIVE_PROTECT: c_uint = 0x02000000;
pub const AR_BTCOEX_WL_LNADIV_CONTINUOUS_BT_ACTIVE_PROTECT_S: c_int = 25;
pub const AR_BTCOEX_WL_LNADIV_BT_INACTIVE_THRESHOLD: c_uint = 0xFC000000;
pub const AR_BTCOEX_WL_LNADIV_BT_INACTIVE_THRESHOLD_S: c_int = 26;
// Manual Peak detector calibration
pub const AR_PHY_65NM_BASE: c_uint = 0x16000;

pub const AR_PHY_65NM_RXRF_GAINSTAGES_RX_OVERRIDE: c_uint = 0x80000000;
pub const AR_PHY_65NM_RXRF_GAINSTAGES_RX_OVERRIDE_S: c_int = 31;
pub const AR_PHY_65NM_RXRF_GAINSTAGES_LNAON_CALDC: c_uint = 0x00000002;
pub const AR_PHY_65NM_RXRF_GAINSTAGES_LNAON_CALDC_S: c_int = 1;
pub const AR_PHY_65NM_RXRF_GAINSTAGES_LNA2G_GAIN_OVR: c_uint = 0x70000000;
pub const AR_PHY_65NM_RXRF_GAINSTAGES_LNA2G_GAIN_OVR_S: c_int = 28;
pub const AR_PHY_65NM_RXRF_GAINSTAGES_LNA5G_GAIN_OVR: c_uint = 0x03800000;
pub const AR_PHY_65NM_RXRF_GAINSTAGES_LNA5G_GAIN_OVR_S: c_int = 23;

pub const AR_PHY_65NM_RXTX2_RXON_OVR: c_uint = 0x00001000;
pub const AR_PHY_65NM_RXTX2_RXON_OVR_S: c_int = 12;
pub const AR_PHY_65NM_RXTX2_RXON: c_uint = 0x00000800;
pub const AR_PHY_65NM_RXTX2_RXON_S: c_int = 11;

pub const AR_PHY_65NM_RXRF_AGC_AGC_OVERRIDE: c_uint = 0x80000000;
pub const AR_PHY_65NM_RXRF_AGC_AGC_OVERRIDE_S: c_int = 31;
pub const AR_PHY_65NM_RXRF_AGC_AGC_ON_OVR: c_uint = 0x40000000;
pub const AR_PHY_65NM_RXRF_AGC_AGC_ON_OVR_S: c_int = 30;
pub const AR_PHY_65NM_RXRF_AGC_AGC_CAL_OVR: c_uint = 0x20000000;
pub const AR_PHY_65NM_RXRF_AGC_AGC_CAL_OVR_S: c_int = 29;
pub const AR_PHY_65NM_RXRF_AGC_AGC2G_DBDAC_OVR: c_uint = 0x1E000000;
pub const AR_PHY_65NM_RXRF_AGC_AGC2G_DBDAC_OVR_S: c_int = 25;
pub const AR_PHY_65NM_RXRF_AGC_AGC5G_DBDAC_OVR: c_uint = 0x00078000;
pub const AR_PHY_65NM_RXRF_AGC_AGC5G_DBDAC_OVR_S: c_int = 15;
pub const AR_PHY_65NM_RXRF_AGC_AGC2G_CALDAC_OVR: c_uint = 0x01F80000;
pub const AR_PHY_65NM_RXRF_AGC_AGC2G_CALDAC_OVR_S: c_int = 19;
pub const AR_PHY_65NM_RXRF_AGC_AGC5G_CALDAC_OVR: c_uint = 0x00007e00;
pub const AR_PHY_65NM_RXRF_AGC_AGC5G_CALDAC_OVR_S: c_int = 9;
pub const AR_PHY_65NM_RXRF_AGC_AGC_OUT: c_uint = 0x00000004;
pub const AR_PHY_65NM_RXRF_AGC_AGC_OUT_S: c_int = 2;

