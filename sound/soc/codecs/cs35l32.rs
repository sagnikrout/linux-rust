//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs35l32.h
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
// cs35l32.h -- CS35L32 ALSA SoC audio driver
//
// Copyright 2014 CirrusLogic, Inc.
//
// Author: Brian Austin <brian.austin@cirrus.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l32_platform_data {
// Low Battery Threshold
    pub batt_thresh: c_uint,
// Low Battery Recovery
    pub batt_recov: c_uint,
// LED Current Management
    pub led_mng: c_uint,
// Audio Gain w/ LED
    pub audiogain_mng: c_uint,
// Boost Management
    pub boost_mng: c_uint,
// Data CFG for DUAL device
    pub sdout_datacfg: c_uint,
// SDOUT Sharing
    pub sdout_share: c_uint,
}

pub const CS35L32_CHIP_ID: c_uint = 0x00035A32;
pub const CS35L32_DEVID_AB: c_uint = 0x01	/* Device ID A & B [RO] */;
pub const CS35L32_DEVID_CD: c_uint = 0x02    /* Device ID C & D [RO] */;
pub const CS35L32_DEVID_E: c_uint = 0x03    /* Device ID E [RO] */;
pub const CS35L32_FAB_ID: c_uint = 0x04	/* Fab ID [RO] */;
pub const CS35L32_REV_ID: c_uint = 0x05	/* Revision ID [RO] */;
pub const CS35L32_PWRCTL1: c_uint = 0x06    /* Power Ctl 1 */;
pub const CS35L32_PWRCTL2: c_uint = 0x07    /* Power Ctl 2 */;
pub const CS35L32_CLK_CTL: c_uint = 0x08	/* Clock Ctl */;
pub const CS35L32_BATT_THRESHOLD: c_uint = 0x09	/* Low Battery Threshold */;
pub const CS35L32_VMON: c_uint = 0x0A	/* Voltage Monitor [RO] */;
pub const CS35L32_BST_CPCP_CTL: c_uint = 0x0B	/* Conv Peak Curr Protection CTL */;
pub const CS35L32_IMON_SCALING: c_uint = 0x0C	/* IMON Scaling */;
pub const CS35L32_AUDIO_LED_MNGR: c_uint = 0x0D	/* Audio/LED Pwr Manager */;
pub const CS35L32_ADSP_CTL: c_uint = 0x0F	/* Serial Port Control */;
pub const CS35L32_CLASSD_CTL: c_uint = 0x10	/* Class D Amp CTL */;
pub const CS35L32_PROTECT_CTL: c_uint = 0x11	/* Protection Release CTL */;
pub const CS35L32_INT_MASK_1: c_uint = 0x12	/* Interrupt Mask 1 */;
pub const CS35L32_INT_MASK_2: c_uint = 0x13	/* Interrupt Mask 2 */;
pub const CS35L32_INT_MASK_3: c_uint = 0x14	/* Interrupt Mask 3 */;
pub const CS35L32_INT_STATUS_1: c_uint = 0x15	/* Interrupt Status 1 [RO] */;
pub const CS35L32_INT_STATUS_2: c_uint = 0x16	/* Interrupt Status 2 [RO] */;
pub const CS35L32_INT_STATUS_3: c_uint = 0x17	/* Interrupt Status 3 [RO] */;
pub const CS35L32_LED_STATUS: c_uint = 0x18	/* LED Lighting Status [RO] */;
pub const CS35L32_FLASH_MODE: c_uint = 0x19	/* LED Flash Mode Current */;
pub const CS35L32_MOVIE_MODE: c_uint = 0x1A	/* LED Movie Mode Current */;
pub const CS35L32_FLASH_TIMER: c_uint = 0x1B	/* LED Flash Timer */;
pub const CS35L32_FLASH_INHIBIT: c_uint = 0x1C	/* LED Flash Inhibit Current */;
pub const CS35L32_MAX_REGISTER: c_uint = 0x1C;
pub const CS35L32_MCLK_DIV2: c_uint = 0x01;
pub const CS35L32_MCLK_RATIO: c_uint = 0x01;
pub const CS35L32_MCLKDIS: c_uint = 0x80;
pub const CS35L32_PDN_ALL: c_uint = 0x01;
pub const CS35L32_PDN_AMP: c_uint = 0x80;
pub const CS35L32_PDN_BOOST: c_uint = 0x04;
pub const CS35L32_PDN_IMON: c_uint = 0x40;
pub const CS35L32_PDN_VMON: c_uint = 0x80;
pub const CS35L32_PDN_VPMON: c_uint = 0x20;
pub const CS35L32_PDN_ADSP: c_uint = 0x08;
pub const CS35L32_MCLK_DIV2_MASK: c_uint = 0x40;
pub const CS35L32_MCLK_RATIO_MASK: c_uint = 0x01;
pub const CS35L32_MCLK_MASK: c_uint = 0x41;
pub const CS35L32_ADSP_MASTER_MASK: c_uint = 0x40;
pub const CS35L32_BOOST_MASK: c_uint = 0x03;
pub const CS35L32_GAIN_MGR_MASK: c_uint = 0x08;
pub const CS35L32_ADSP_SHARE_MASK: c_uint = 0x08;
pub const CS35L32_ADSP_DATACFG_MASK: c_uint = 0x30;
pub const CS35L32_SDOUT_3ST: c_uint = 0x08;
pub const CS35L32_BATT_REC_MASK: c_uint = 0x0E;
pub const CS35L32_BATT_THRESH_MASK: c_uint = 0x30;

