//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs35l34.h
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
// cs35l34.h -- CS35L34 ALSA SoC audio driver
//
// Copyright 2016 Cirrus Logic, Inc.
//
// Author: Paul Handrigan <Paul.Handrigan@cirrus.com>
//
pub const CS35L34_CHIP_ID: c_uint = 0x00035A34;
pub const CS35L34_DEVID_AB: c_uint = 0x01	/* Device ID A & B [RO] */;
pub const CS35L34_DEVID_CD: c_uint = 0x02    /* Device ID C & D [RO] */;
pub const CS35L34_DEVID_E: c_uint = 0x03    /* Device ID E [RO] */;
pub const CS35L34_FAB_ID: c_uint = 0x04	/* Fab ID [RO] */;
pub const CS35L34_REV_ID: c_uint = 0x05	/* Revision ID [RO] */;
pub const CS35L34_PWRCTL1: c_uint = 0x06    /* Power Ctl 1 */;
pub const CS35L34_PWRCTL2: c_uint = 0x07    /* Power Ctl 2 */;
pub const CS35L34_PWRCTL3: c_uint = 0x08	/* Power Ctl 3 */;
pub const CS35L34_ADSP_CLK_CTL: c_uint = 0x0A	/* (ADSP) Clock Ctl */;
pub const CS35L34_MCLK_CTL: c_uint = 0x0B	/* Master Clocking Ctl */;
pub const CS35L34_AMP_INP_DRV_CTL: c_uint = 0x14	/* Amp Input Drive Ctl */;
pub const CS35L34_AMP_DIG_VOL_CTL: c_uint = 0x15	/* Amplifier Dig Volume Ctl */;
pub const CS35L34_AMP_DIG_VOL: c_uint = 0x16	/* Amplifier Dig Volume */;
pub const CS35L34_AMP_ANLG_GAIN_CTL: c_uint = 0x17	/* Amplifier Analog Gain Ctl */;
pub const CS35L34_PROTECT_CTL: c_uint = 0x18	/* Amp Gain - Prot Ctl Param */;
pub const CS35L34_AMP_KEEP_ALIVE_CTL: c_uint = 0x1A	/* Amplifier Keep Alive Ctl */;
pub const CS35L34_BST_CVTR_V_CTL: c_uint = 0x1D	/* Boost Conv Voltage Ctl */;
pub const CS35L34_BST_PEAK_I: c_uint = 0x1E	/* Boost Conv Peak Current */;
pub const CS35L34_BST_RAMP_CTL: c_uint = 0x20	/* Boost Conv Soft Ramp Ctl */;
pub const CS35L34_BST_CONV_COEF_1: c_uint = 0x21	/* Boost Conv Coefficients 1 */;
pub const CS35L34_BST_CONV_COEF_2: c_uint = 0x22	/* Boost Conv Coefficients 2 */;
pub const CS35L34_BST_CONV_SLOPE_COMP: c_uint = 0x23	/* Boost Conv Slope Comp */;
pub const CS35L34_BST_CONV_SW_FREQ: c_uint = 0x24	/* Boost Conv L BST SW Freq */;
pub const CS35L34_CLASS_H_CTL: c_uint = 0x30	/* CLS H Control */;
pub const CS35L34_CLASS_H_HEADRM_CTL: c_uint = 0x31	/* CLS H Headroom Ctl */;
pub const CS35L34_CLASS_H_RELEASE_RATE: c_uint = 0x32	/* CLS H Release Rate */;
pub const CS35L34_CLASS_H_FET_DRIVE_CTL: c_uint = 0x33	/* CLS H Weak FET Drive Ctl */;
pub const CS35L34_CLASS_H_STATUS: c_uint = 0x38	/* CLS H Status */;
pub const CS35L34_VPBR_CTL: c_uint = 0x3A	/* VPBR Ctl */;
pub const CS35L34_VPBR_VOL_CTL: c_uint = 0x3B	/* VPBR Volume Ctl */;
pub const CS35L34_VPBR_TIMING_CTL: c_uint = 0x3C	/* VPBR Timing Ctl */;
pub const CS35L34_PRED_MAX_ATTEN_SPK_LOAD: c_uint = 0x40	/* PRD Max Atten / Spkr Load */;
pub const CS35L34_PRED_BROWNOUT_THRESH: c_uint = 0x41	/* PRD Brownout Threshold */;
pub const CS35L34_PRED_BROWNOUT_VOL_CTL: c_uint = 0x42	/* PRD Brownout Volume Ctl */;
pub const CS35L34_PRED_BROWNOUT_RATE_CTL: c_uint = 0x43	/* PRD Brownout Rate Ctl */;
pub const CS35L34_PRED_WAIT_CTL: c_uint = 0x44	/* PRD Wait Ctl */;
pub const CS35L34_PRED_ZVP_INIT_IMP_CTL: c_uint = 0x46	/* PRD ZVP Initial Imp Ctl */;
pub const CS35L34_PRED_MAN_SAFE_VPI_CTL: c_uint = 0x47	/* PRD Manual Safe VPI Ctl */;
pub const CS35L34_VPBR_ATTEN_STATUS: c_uint = 0x4B	/* VPBR Attenuation Status */;
pub const CS35L34_PRED_BRWNOUT_ATT_STATUS: c_uint = 0x4C	/* PRD Brownout Atten Status */;
pub const CS35L34_SPKR_MON_CTL: c_uint = 0x4E	/* Speaker Monitoring Ctl */;
pub const CS35L34_ADSP_I2S_CTL: c_uint = 0x50	/* ADSP I2S Ctl */;
pub const CS35L34_ADSP_TDM_CTL: c_uint = 0x51	/* ADSP TDM Ctl */;
pub const CS35L34_TDM_TX_CTL_1_VMON: c_uint = 0x52	/* TDM TX Ctl 1 (VMON) */;
pub const CS35L34_TDM_TX_CTL_2_IMON: c_uint = 0x53	/* TDM TX Ctl 2 (IMON) */;
pub const CS35L34_TDM_TX_CTL_3_VPMON: c_uint = 0x54	/* TDM TX Ctl 3 (VPMON) */;
pub const CS35L34_TDM_TX_CTL_4_VBSTMON: c_uint = 0x55	/* TDM TX Ctl 4 (VBSTMON) */;
pub const CS35L34_TDM_TX_CTL_5_FLAG1: c_uint = 0x56	/* TDM TX Ctl 5 (FLAG1) */;
pub const CS35L34_TDM_TX_CTL_6_FLAG2: c_uint = 0x57	/* TDM TX Ctl 6 (FLAG2) */;
pub const CS35L34_TDM_TX_SLOT_EN_1: c_uint = 0x5A	/* TDM TX Slot Enable */;
pub const CS35L34_TDM_TX_SLOT_EN_2: c_uint = 0x5B	/* TDM TX Slot Enable */;
pub const CS35L34_TDM_TX_SLOT_EN_3: c_uint = 0x5C	/* TDM TX Slot Enable */;
pub const CS35L34_TDM_TX_SLOT_EN_4: c_uint = 0x5D	/* TDM TX Slot Enable */;
pub const CS35L34_TDM_RX_CTL_1_AUDIN: c_uint = 0x5E	/* TDM RX Ctl 1 */;
pub const CS35L34_TDM_RX_CTL_3_ALIVE: c_uint = 0x60	/* TDM RX Ctl 3 (ALIVE) */;
pub const CS35L34_MULT_DEV_SYNCH1: c_uint = 0x62	/* Multidevice Synch */;
pub const CS35L34_MULT_DEV_SYNCH2: c_uint = 0x63	/* Multidevice Synch 2 */;
pub const CS35L34_PROT_RELEASE_CTL: c_uint = 0x64	/* Protection Release Ctl */;
pub const CS35L34_DIAG_MODE_REG_LOCK: c_uint = 0x68	/* Diagnostic Mode Reg Lock */;
pub const CS35L34_DIAG_MODE_CTL_1: c_uint = 0x69	/* Diagnostic Mode Ctl 1 */;
pub const CS35L34_DIAG_MODE_CTL_2: c_uint = 0x6A	/* Diagnostic Mode Ctl 2 */;
pub const CS35L34_INT_MASK_1: c_uint = 0x70	/* Interrupt Mask 1 */;
pub const CS35L34_INT_MASK_2: c_uint = 0x71	/* Interrupt Mask 2 */;
pub const CS35L34_INT_MASK_3: c_uint = 0x72	/* Interrupt Mask 3 */;
pub const CS35L34_INT_MASK_4: c_uint = 0x73	/* Interrupt Mask 4 */;
pub const CS35L34_INT_STATUS_1: c_uint = 0x74	/* Interrupt Status 1 */;
pub const CS35L34_INT_STATUS_2: c_uint = 0x75	/* Interrupt Status 2 */;
pub const CS35L34_INT_STATUS_3: c_uint = 0x76	/* Interrupt Status 3 */;
pub const CS35L34_INT_STATUS_4: c_uint = 0x77	/* Interrupt Status 4 */;
pub const CS35L34_OTP_TRIM_STATUS: c_uint = 0x7E	/* OTP Trim Status */;
pub const CS35L34_MAX_REGISTER: c_uint = 0x7F;
pub const CS35L34_REGISTER_COUNT: c_uint = 0x4E;
pub const CS35L34_MCLK_5644: c_int = 5644800;
pub const CS35L34_MCLK_6144: c_int = 6144000;
pub const CS35L34_MCLK_6: c_int = 6000000;
pub const CS35L34_MCLK_11289: c_int = 11289600;
pub const CS35L34_MCLK_12: c_int = 12000000;
pub const CS35L34_MCLK_12288: c_int = 12288000;
// CS35L34_PWRCTL1

pub const CS35L34_PDN_ALL: c_int = 1;
// CS35L34_PWRCTL2

pub const CS35L34_PDN_AMP: c_int = 1;
// CS35L34_PWRCTL3

// Tristate the ADSP SDOUT when in I2C mode

pub const CS35L34_PDN_TDM: c_int = 1;
// CS35L34_ADSP_CLK_CTL
pub const CS35L34_ADSP_RATE: c_uint = 0xF;

// CS35L34_MCLK_CTL

pub const CS35L34_MCLK_RATE_MASK: c_uint = 0x7;
pub const CS35L34_MCLK_RATE_6P1440: c_uint = 0x2;
pub const CS35L34_MCLK_RATE_6P0000: c_uint = 0x1;
pub const CS35L34_MCLK_RATE_5P6448: c_uint = 0x0;

pub const CS35L34_ADSP_FS: c_uint = 0xF;
// CS35L34_AMP_INP_DRV_CTL

pub const CS35L34_DRV_STR: c_int = 1;
// CS35L34_AMP_DIG_VOL_CTL
pub const CS35L34_AMP_DSR_RATE_MASK: c_uint = 0xF0;

pub const CS35L34_INV: c_int = 1;
// CS35L34_PROTECT_CTL
pub const CS35L34_OTW_ATTN_MASK: c_uint = 0xC;
pub const CS35L34_OTW_THRD_MASK: c_uint = 0x3;

pub const CS35L34_GAIN_ZC_MASK: c_uint = 0x10;
pub const CS35L34_GAIN_ZC_SHIFT: c_int = 4;
// CS35L34_AMP_KEEP_ALIVE_CTL

// CS35L34_BST_CVTR_V_CTL
pub const CS35L34_BST_CVTL_MASK: c_uint = 0x3F;
// CS35L34_BST_PEAK_I
pub const CS35L34_BST_PEAK_MASK: c_uint = 0x3F;
// CS35L34_ADSP_I2S_CTL
pub const CS35L34_I2S_LOC_MASK: c_uint = 0xC;
pub const CS35L34_I2S_LOC_SHIFT: c_int = 2;
// CS35L34_MULT_DEV_SYNCH2
pub const CS35L34_SYNC2_MASK: c_uint = 0xF;
// CS35L34_PROT_RELEASE_CTL

pub const CS35L34_OTE_RLS: c_int = 1;
// CS35L34_INT_MASK_1
pub const CS35L34_M_CAL_ERR_SHIFT: c_int = 7;

pub const CS35L34_M_ALIVE_ERR_SHIFT: c_int = 5;

pub const CS35L34_M_ADSP_CLK_SHIFT: c_int = 4;

pub const CS35L34_M_MCLK_SHIFT: c_int = 3;

pub const CS35L34_M_AMP_SHORT_SHIFT: c_int = 2;

pub const CS35L34_M_OTW_SHIFT: c_int = 1;

pub const CS35L34_M_OTE_SHIFT: c_int = 0;

// CS35L34_INT_MASK_2
pub const CS35L34_M_PDN_DONE_SHIFT: c_int = 4;

pub const CS35L34_M_PRED_SHIFT: c_int = 3;

pub const CS35L34_M_PRED_CLR_SHIFT: c_int = 2;

pub const CS35L34_M_VPBR_SHIFT: c_int = 1;

pub const CS35L34_M_VPBR_CLR_SHIFT: c_int = 0;

// CS35L34_INT_MASK_3
pub const CS35L34_M_BST_HIGH_SHIFT: c_int = 4;

pub const CS35L34_M_BST_HIGH_FLAG_SHIFT: c_int = 3;

pub const CS35L34_M_BST_IPK_FLAG_SHIFT: c_int = 2;

pub const CS35L34_M_LBST_SHORT_SHIFT: c_int = 0;

// CS35L34_INT_MASK_4
pub const CS35L34_M_VMON_OVFL_SHIFT: c_int = 3;

pub const CS35L34_M_IMON_OVFL_SHIFT: c_int = 2;

pub const CS35L34_M_VPMON_OVFL_SHIFT: c_int = 1;

pub const CS35L34_M_VBSTMON_OVFL_SHIFT: c_int = 1;

// CS35L34_INT_1

// CS35L34_INT_2

// CS35L34_INT_3

// CS35L34_INT_4

// CS35L34_{RX,TX}_X
pub const CS35L34_X_STATE_SHIFT: c_int = 7;

pub const CS35L34_X_LOC_SHIFT: c_int = 0;

