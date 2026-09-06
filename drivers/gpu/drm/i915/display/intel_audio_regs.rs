//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_audio_regs.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2022 Intel Corporation
//

pub const _IBX_HDMIW_HDMIEDID_A: c_uint = 0xE2050;
pub const _IBX_HDMIW_HDMIEDID_B: c_uint = 0xE2150;

pub const _IBX_AUD_CNTL_ST_A: c_uint = 0xE20B4;
pub const _IBX_AUD_CNTL_ST_B: c_uint = 0xE21B4;

pub const _CPT_HDMIW_HDMIEDID_A: c_uint = 0xE5050;
pub const _CPT_HDMIW_HDMIEDID_B: c_uint = 0xE5150;

pub const _CPT_AUD_CNTL_ST_A: c_uint = 0xE50B4;
pub const _CPT_AUD_CNTL_ST_B: c_uint = 0xE51B4;

pub const _IBX_AUD_CONFIG_A: c_uint = 0xe2000;
pub const _IBX_AUD_CONFIG_B: c_uint = 0xe2100;

pub const _CPT_AUD_CONFIG_A: c_uint = 0xe5000;
pub const _CPT_AUD_CONFIG_B: c_uint = 0xe5100;

pub const _HSW_AUD_CONFIG_A: c_uint = 0x65000;
pub const _HSW_AUD_CONFIG_B: c_uint = 0x65100;

pub const _HSW_AUD_MISC_CTRL_A: c_uint = 0x65010;
pub const _HSW_AUD_MISC_CTRL_B: c_uint = 0x65110;

pub const _HSW_AUD_M_CTS_ENABLE_A: c_uint = 0x65028;
pub const _HSW_AUD_M_CTS_ENABLE_B: c_uint = 0x65128;

pub const _HSW_AUD_DIP_ELD_CTRL_ST_A: c_uint = 0x650b4;
pub const _HSW_AUD_DIP_ELD_CTRL_ST_B: c_uint = 0x651b4;

// Audio Digital Converter
pub const _HSW_AUD_DIG_CNVT_1: c_uint = 0x65080;
pub const _HSW_AUD_DIG_CNVT_2: c_uint = 0x65180;

pub const DIP_PORT_SEL_MASK: c_uint = 0x3;
pub const _HSW_AUD_EDID_DATA_A: c_uint = 0x65050;
pub const _HSW_AUD_EDID_DATA_B: c_uint = 0x65150;

pub const _AUD_TCA_DP_2DOT0_CTRL: c_uint = 0x650bc;
pub const _AUD_TCB_DP_2DOT0_CTRL: c_uint = 0x651bc;

// Display Audio Config Reg

pub const HBLANK_START_COUNT_8: c_int = 0;
pub const HBLANK_START_COUNT_16: c_int = 1;
pub const HBLANK_START_COUNT_32: c_int = 2;
pub const HBLANK_START_COUNT_64: c_int = 3;
pub const HBLANK_START_COUNT_96: c_int = 4;
pub const HBLANK_START_COUNT_128: c_int = 5;
// LPE Audio

pub const I915_HDMI_LPE_AUDIO_SIZE: c_uint = 0x1000;

pub const _VLV_AUD_PORT_EN_B_DBG: c_uint = 0x62F20;
pub const _VLV_AUD_PORT_EN_C_DBG: c_uint = 0x62F30;
pub const _VLV_AUD_PORT_EN_D_DBG: c_uint = 0x62F34;

