//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tas2783.h
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


// SPDX-License-Identifier: GPL-2.0
//
// ALSA SoC Texas Instruments TAS2783 Audio Smart Amplifier
//
// Copyright (C) 2025 Texas Instruments Incorporated
// https://www.ti.com
//
// The TAS2783 driver implements a flexible and configurable
// algo coefficient setting for single TAS2783 chips.
//
// Author: Niranjan H Y <niranjanhy@ti.com>
// Author: Baojun Xu <baojun.xu@ti.com>
//

// book, page, register

// Volume control

// Calibration data

// TAS2783 SDCA Control - function number
pub const FUNC_NUM_SMART_AMP: c_uint = 0x01;
// TAS2783 SDCA entity
pub const TAS2783_SDCA_ENT_FU21: c_uint = 0x01;
pub const TAS2783_SDCA_ENT_FU23: c_uint = 0x02;
pub const TAS2783_SDCA_ENT_FU26: c_uint = 0x03;
pub const TAS2783_SDCA_ENT_XU22: c_uint = 0x04;
pub const TAS2783_SDCA_ENT_CS24: c_uint = 0x05;
pub const TAS2783_SDCA_ENT_CS21: c_uint = 0x06;
pub const TAS2783_SDCA_ENT_CS25: c_uint = 0x07;
pub const TAS2783_SDCA_ENT_CS26: c_uint = 0x08;
pub const TAS2783_SDCA_ENT_CS28: c_uint = 0x09;
pub const TAS2783_SDCA_ENT_PDE23: c_uint = 0x0C;
pub const TAS2783_SDCA_ENT_UDMPU23: c_uint = 0x0E;
pub const TAS2783_SDCA_ENT_SAPU29: c_uint = 0x0F;
pub const TAS2783_SDCA_ENT_PPU21: c_uint = 0x10;
pub const TAS2783_SDCA_ENT_PPU26: c_uint = 0x11;
pub const TAS2783_SDCA_ENT_TG23: c_uint = 0x12;
pub const TAS2783_SDCA_ENT_IT21: c_uint = 0x13;
pub const TAS2783_SDCA_ENT_IT29: c_uint = 0x14;
pub const TAS2783_SDCA_ENT_IT26: c_uint = 0x15;
pub const TAS2783_SDCA_ENT_IT28: c_uint = 0x16;
pub const TAS2783_SDCA_ENT_OT24: c_uint = 0x17;
pub const TAS2783_SDCA_ENT_OT23: c_uint = 0x18;
pub const TAS2783_SDCA_ENT_OT25: c_uint = 0x19;
pub const TAS2783_SDCA_ENT_OT28: c_uint = 0x1A;
pub const TAS2783_SDCA_ENT_MU26: c_uint = 0x1b;
pub const TAS2783_SDCA_ENT_OT127: c_uint = 0x1E;
pub const TAS2783_SDCA_ENT_FU127: c_uint = 0x1F;
pub const TAS2783_SDCA_ENT_CS127: c_uint = 0x20;
pub const TAS2783_SDCA_ENT_MFPU21: c_uint = 0x22;
pub const TAS2783_SDCA_ENT_MFPU26: c_uint = 0x23;
// TAS2783 SDCA control
pub const TAS2783_SDCA_CTL_REQ_POW_STATE: c_uint = 0x01;
pub const TAS2783_SDCA_CTL_FU_MUTE: c_uint = 0x01;
pub const TAS2783_SDCA_CTL_UDMPU_CLUSTER: c_uint = 0x10;
pub const TAS2783_DEVICE_CHANNEL_LEFT: c_int = 1;
pub const TAS2783_DEVICE_CHANNEL_RIGHT: c_int = 2;
pub const TAS2783_SDCA_POW_STATE_ON: c_int = 0;
pub const TAS2783_SDCA_POW_STATE_OFF: c_int = 3;
// calibration data

pub const TAS2783_CALIB_MAX_SPK_COUNT: c_int = 8;
pub const TAS2783_CALIB_HDR_SZ: c_int = 12;
pub const TAS2783_CALIB_CRC_SZ: c_int = 4;

