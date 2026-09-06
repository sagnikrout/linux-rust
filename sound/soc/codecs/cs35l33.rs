//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs35l33.h
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
// cs35l33.h -- CS35L33 ALSA SoC audio driver
//
// Copyright 2016 Cirrus Logic, Inc.
//
// Author: Paul Handrigan <paul.handrigan@cirrus.com>
//
pub const CS35L33_CHIP_ID: c_uint = 0x00035A33;
pub const CS35L33_DEVID_AB: c_uint = 0x01	/* Device ID A & B [RO] */;
pub const CS35L33_DEVID_CD: c_uint = 0x02	/* Device ID C & D [RO] */;
pub const CS35L33_DEVID_E: c_uint = 0x03	/* Device ID E [RO] */;
pub const CS35L33_FAB_ID: c_uint = 0x04	/* Fab ID [RO] */;
pub const CS35L33_REV_ID: c_uint = 0x05	/* Revision ID [RO] */;
pub const CS35L33_PWRCTL1: c_uint = 0x06	/* Power Ctl 1 */;
pub const CS35L33_PWRCTL2: c_uint = 0x07	/* Power Ctl 2 */;
pub const CS35L33_CLK_CTL: c_uint = 0x08	/* Clock Ctl */;
pub const CS35L33_BST_PEAK_CTL: c_uint = 0x09	/* Max Current for Boost */;
pub const CS35L33_PROTECT_CTL: c_uint = 0x0A	/* Amp Protection Parameters */;
pub const CS35L33_BST_CTL1: c_uint = 0x0B	/* Boost Converter CTL1 */;
pub const CS35L33_BST_CTL2: c_uint = 0x0C	/* Boost Converter CTL2 */;
pub const CS35L33_ADSP_CTL: c_uint = 0x0D	/* Serial Port Control */;
pub const CS35L33_ADC_CTL: c_uint = 0x0E	/* ADC Control */;
pub const CS35L33_DAC_CTL: c_uint = 0x0F	/* DAC Control */;
pub const CS35L33_DIG_VOL_CTL: c_uint = 0x10	/* Digital Volume CTL */;
pub const CS35L33_CLASSD_CTL: c_uint = 0x11	/* Class D Amp CTL */;
pub const CS35L33_AMP_CTL: c_uint = 0x12	/* Amp Gain/Protecton Release CTL */;
pub const CS35L33_INT_MASK_1: c_uint = 0x13	/* Interrupt Mask 1 */;
pub const CS35L33_INT_MASK_2: c_uint = 0x14	/* Interrupt Mask 2 */;
pub const CS35L33_INT_STATUS_1: c_uint = 0x15	/* Interrupt Status 1 [RO] */;
pub const CS35L33_INT_STATUS_2: c_uint = 0x16	/* Interrupt Status 2 [RO] */;
pub const CS35L33_DIAG_LOCK: c_uint = 0x17	/* Diagnostic Mode Register Lock */;
pub const CS35L33_DIAG_CTRL_1: c_uint = 0x18	/* Diagnostic Mode Register Control */;
pub const CS35L33_DIAG_CTRL_2: c_uint = 0x19	/* Diagnostic Mode Register Control 2 */;
pub const CS35L33_HG_MEMLDO_CTL: c_uint = 0x23	/* H/G Memory/LDO CTL */;
pub const CS35L33_HG_REL_RATE: c_uint = 0x24	/* H/G Release Rate */;
pub const CS35L33_LDO_DEL: c_uint = 0x25	/* LDO Entry Delay/VPhg Control 1 */;
pub const CS35L33_HG_HEAD: c_uint = 0x29	/* H/G Headroom */;
pub const CS35L33_HG_EN: c_uint = 0x2A	/* H/G Enable/VPhg CNT2 */;
pub const CS35L33_TX_VMON: c_uint = 0x2D	/* TDM TX Control 1 (VMON) */;
pub const CS35L33_TX_IMON: c_uint = 0x2E	/* TDM TX Control 2 (IMON) */;
pub const CS35L33_TX_VPMON: c_uint = 0x2F	/* TDM TX Control 3 (VPMON) */;
pub const CS35L33_TX_VBSTMON: c_uint = 0x30	/* TDM TX Control 4 (VBSTMON) */;
pub const CS35L33_TX_FLAG: c_uint = 0x31	/* TDM TX Control 5 (FLAG) */;
pub const CS35L33_TX_EN1: c_uint = 0x32	/* TDM TX Enable 1 */;
pub const CS35L33_TX_EN2: c_uint = 0x33	/* TDM TX Enable 2 */;
pub const CS35L33_TX_EN3: c_uint = 0x34	/* TDM TX Enable 3 */;
pub const CS35L33_TX_EN4: c_uint = 0x35	/* TDM TX Enable 4 */;
pub const CS35L33_RX_AUD: c_uint = 0x36	/* TDM RX Control 1 */;
pub const CS35L33_RX_SPLY: c_uint = 0x37	/* TDM RX Control 2 */;
pub const CS35L33_RX_ALIVE: c_uint = 0x38	/* TDM RX Control 3 */;
pub const CS35L33_BST_CTL4: c_uint = 0x39	/* Boost Converter Control 4 */;
pub const CS35L33_HG_STATUS: c_uint = 0x3F	/* H/G Status */;
pub const CS35L33_MAX_REGISTER: c_uint = 0x59;
pub const CS35L33_MCLK_5644: c_int = 5644800;
pub const CS35L33_MCLK_6144: c_int = 6144000;
pub const CS35L33_MCLK_6: c_int = 6000000;
pub const CS35L33_MCLK_11289: c_int = 11289600;
pub const CS35L33_MCLK_12: c_int = 12000000;
pub const CS35L33_MCLK_12288: c_int = 12288000;
// CS35L33_PWRCTL1

pub const CS35L33_PDN_ALL: c_int = 1;
// CS35L33_PWRCTL2
pub const CS35L33_PDN_VMON_SHIFT: c_int = 7;

pub const CS35L33_PDN_IMON_SHIFT: c_int = 6;

pub const CS35L33_PDN_VPMON_SHIFT: c_int = 5;

pub const CS35L33_PDN_VBSTMON_SHIFT: c_int = 4;

pub const CS35L33_SDOUT_3ST_I2S_SHIFT: c_int = 3;

pub const CS35L33_PDN_SDIN_SHIFT: c_int = 2;

pub const CS35L33_PDN_TDM_SHIFT: c_int = 1;

// CS35L33_CLK_CTL

pub const CS35L33_ADSP_FS: c_uint = 0xF;
// CS35L33_PROTECT_CTL

// CS35L33_BST_CTL1

pub const CS35L33_BST_CTL_MASK: c_uint = 0x3F;
// CS35L33_BST_CTL2

pub const CS35L33_VBST_SR_STEP: c_uint = 0x3;
// CS35L33_ADSP_CTL

pub const CS35L33_ALIVE_RATE: c_uint = 0x3;
// CS35L33_ADC_CTL

pub const CS35L33_IMON_SCALE: c_uint = 0xF;
// CS35L33_DAC_CTL

pub const CS35L33_DSR_RATE: c_uint = 0xF;
// CS35L33_CLASSD_CTL

pub const CS35L33_AMP_DRV_SEL_MASK: c_uint = 0x10;
pub const CS35L33_AMP_DRV_SEL_SHIFT: c_int = 4;

pub const CS35L33_GAIN_CHG_ZC_MASK: c_uint = 0x04;
pub const CS35L33_GAIN_CHG_ZC_SHIFT: c_int = 2;
pub const CS35L33_CLASS_D_CTL_MASK: c_uint = 0x3F;
// CS35L33_AMP_CTL
pub const CS35L33_AMP_GAIN: c_uint = 0xF0;

pub const CS35L33_OTE_RLS: c_int = 1;
// CS35L33_INT_MASK_1
pub const CS35L33_M_CAL_ERR_SHIFT: c_int = 6;

pub const CS35L33_M_ALIVE_ERR_SHIFT: c_int = 5;

pub const CS35L33_M_AMP_SHORT_SHIFT: c_int = 2;

pub const CS35L33_M_OTW_SHIFT: c_int = 1;

pub const CS35L33_M_OTE_SHIFT: c_int = 0;

// CS35L33_INT_STATUS_1

// CS35L33_INT_STATUS_2

pub const CS35L33_PDN_DONE: c_int = 1;
// CS35L33_BST_CTL4
pub const CS35L33_BST_RGS: c_uint = 0x70;
pub const CS35L33_BST_COEFF3: c_uint = 0xF;
// CS35L33_HG_MEMLDO_CTL
pub const CS35L33_MEM_DEPTH_SHIFT: c_int = 5;

pub const CS35L33_LDO_THLD_SHIFT: c_int = 1;

pub const CS35L33_LDO_DISABLE_SHIFT: c_int = 0;

// CS35L33_LDO_DEL
pub const CS35L33_VP_HG_VA_SHIFT: c_int = 5;

pub const CS35L33_LDO_ENTRY_DELAY_SHIFT: c_int = 2;

pub const CS35L33_VP_HG_RATE_SHIFT: c_int = 0;

// CS35L33_HG_HEAD
pub const CS35L33_HD_RM_SHIFT: c_int = 0;

// CS35L33_HG_EN
pub const CS35L33_CLASS_HG_ENA_SHIFT: c_int = 7;

pub const CS35L33_VP_HG_AUTO_SHIFT: c_int = 6;

pub const CS35L33_VP_HG_SHIFT: c_int = 0;

// CS35L33_{RX,TX}_X
pub const CS35L33_X_STATE_SHIFT: c_int = 7;

pub const CS35L33_X_LOC_SHIFT: c_int = 0;

// CS35L33_RX_AUD
pub const CS35L33_AUDIN_RX_DEPTH_SHIFT: c_int = 5;

