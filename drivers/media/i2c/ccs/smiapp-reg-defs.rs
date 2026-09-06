//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/ccs/smiapp-reg-defs.h
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
// drivers/media/i2c/smiapp/smiapp-reg-defs.h
//
// Generic driver for MIPI CCS/SMIA/SMIA++ compliant camera sensors
//
// Copyright (C) 2020 Intel Corporation
// Copyright (C) 2011--2012 Nokia Corporation
// Contact: Sakari Ailus <sakari.ailus@iki.fi>
//

// Register addresses

// Register bit definitions

pub const SMIAPP_CSI_SIGNALLING_MODE_CCP2_DATA_CLOCK: c_int = 0;
pub const SMIAPP_CSI_SIGNALLING_MODE_CCP2_DATA_STROBE: c_int = 1;
pub const SMIAPP_CSI_SIGNALLING_MODE_CSI2: c_int = 2;
pub const SMIAPP_DPHY_CTRL_AUTOMATIC: c_int = 0;
// DPHY control based on REQUESTED_LINK_BIT_RATE_MBPS
pub const SMIAPP_DPHY_CTRL_UI: c_int = 1;
pub const SMIAPP_DPHY_CTRL_REGISTER: c_int = 2;
pub const SMIAPP_COMPRESSION_MODE_SIMPLE_PREDICTOR: c_int = 1;
pub const SMIAPP_COMPRESSION_MODE_ADVANCED_PREDICTOR: c_int = 2;
pub const SMIAPP_MODE_SELECT_SOFTWARE_STANDBY: c_int = 0;
pub const SMIAPP_MODE_SELECT_STREAMING: c_int = 1;
pub const SMIAPP_SCALING_MODE_NONE: c_int = 0;
pub const SMIAPP_SCALING_MODE_HORIZONTAL: c_int = 1;
pub const SMIAPP_SCALING_MODE_BOTH: c_int = 2;
pub const SMIAPP_SCALING_CAPABILITY_NONE: c_int = 0;
pub const SMIAPP_SCALING_CAPABILITY_HORIZONTAL: c_int = 1;

// digital crop right before scaler
pub const SMIAPP_DIGITAL_CROP_CAPABILITY_NONE: c_int = 0;
pub const SMIAPP_DIGITAL_CROP_CAPABILITY_INPUT_CROP: c_int = 1;
pub const SMIAPP_DIGITAL_GAIN_CAPABILITY_PER_CHANNEL: c_int = 1;
pub const SMIAPP_BINNING_CAPABILITY_NO: c_int = 0;
pub const SMIAPP_BINNING_CAPABILITY_YES: c_int = 1;
// Maximum number of binning subtypes
pub const SMIAPP_BINNING_SUBTYPES: c_int = 253;
pub const SMIAPP_PIXEL_ORDER_GRBG: c_int = 0;
pub const SMIAPP_PIXEL_ORDER_RGGB: c_int = 1;
pub const SMIAPP_PIXEL_ORDER_BGGR: c_int = 2;
pub const SMIAPP_PIXEL_ORDER_GBRG: c_int = 3;
pub const SMIAPP_DATA_FORMAT_MODEL_TYPE_NORMAL: c_int = 1;
pub const SMIAPP_DATA_FORMAT_MODEL_TYPE_EXTENDED: c_int = 2;
pub const SMIAPP_DATA_FORMAT_MODEL_TYPE_NORMAL_N: c_int = 8;
pub const SMIAPP_DATA_FORMAT_MODEL_TYPE_EXTENDED_N: c_int = 16;
pub const SMIAPP_FRAME_FORMAT_MODEL_TYPE_2BYTE: c_uint = 0x01;
pub const SMIAPP_FRAME_FORMAT_MODEL_TYPE_4BYTE: c_uint = 0x02;
pub const SMIAPP_FRAME_FORMAT_MODEL_SUBTYPE_NROWS_MASK: c_uint = 0x0f;
pub const SMIAPP_FRAME_FORMAT_MODEL_SUBTYPE_NCOLS_MASK: c_uint = 0xf0;
pub const SMIAPP_FRAME_FORMAT_MODEL_SUBTYPE_NCOLS_SHIFT: c_int = 4;
pub const SMIAPP_FRAME_FORMAT_DESC_2_PIXELCODE_MASK: c_uint = 0xf000;
pub const SMIAPP_FRAME_FORMAT_DESC_2_PIXELCODE_SHIFT: c_int = 12;
pub const SMIAPP_FRAME_FORMAT_DESC_2_PIXELS_MASK: c_uint = 0x0fff;
pub const SMIAPP_FRAME_FORMAT_DESC_4_PIXELCODE_MASK: c_uint = 0xf0000000;
pub const SMIAPP_FRAME_FORMAT_DESC_4_PIXELCODE_SHIFT: c_int = 28;
pub const SMIAPP_FRAME_FORMAT_DESC_4_PIXELS_MASK: c_uint = 0x0000ffff;
pub const SMIAPP_FRAME_FORMAT_DESC_PIXELCODE_EMBEDDED: c_int = 1;
pub const SMIAPP_FRAME_FORMAT_DESC_PIXELCODE_DUMMY: c_int = 2;
pub const SMIAPP_FRAME_FORMAT_DESC_PIXELCODE_BLACK: c_int = 3;
pub const SMIAPP_FRAME_FORMAT_DESC_PIXELCODE_DARK: c_int = 4;
pub const SMIAPP_FRAME_FORMAT_DESC_PIXELCODE_VISIBLE: c_int = 5;
pub const SMIAPP_FAST_STANDBY_CTRL_COMPLETE_FRAMES: c_int = 0;
pub const SMIAPP_FAST_STANDBY_CTRL_IMMEDIATE: c_int = 1;
// Scaling N factor
pub const SMIAPP_SCALE_N: c_int = 16;
