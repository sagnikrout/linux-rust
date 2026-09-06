//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/evergreen_reg.h
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
// Copyright 2010 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Alex Deucher
//
// trinity
pub const TN_SMC_IND_INDEX_0: c_uint = 0x200;
pub const TN_SMC_IND_DATA_0: c_uint = 0x204;
// evergreen
pub const EVERGREEN_PIF_PHY0_INDEX: c_uint = 0x8;
pub const EVERGREEN_PIF_PHY0_DATA: c_uint = 0xc;
pub const EVERGREEN_PIF_PHY1_INDEX: c_uint = 0x10;
pub const EVERGREEN_PIF_PHY1_DATA: c_uint = 0x14;
pub const EVERGREEN_MM_INDEX_HI: c_uint = 0x18;
pub const EVERGREEN_VGA_MEMORY_BASE_ADDRESS: c_uint = 0x310;
pub const EVERGREEN_VGA_MEMORY_BASE_ADDRESS_HIGH: c_uint = 0x324;
pub const EVERGREEN_D3VGA_CONTROL: c_uint = 0x3e0;
pub const EVERGREEN_D4VGA_CONTROL: c_uint = 0x3e4;
pub const EVERGREEN_D5VGA_CONTROL: c_uint = 0x3e8;
pub const EVERGREEN_D6VGA_CONTROL: c_uint = 0x3ec;
pub const EVERGREEN_P1PLL_SS_CNTL: c_uint = 0x414;
pub const EVERGREEN_P2PLL_SS_CNTL: c_uint = 0x454;

pub const EVERGREEN_AUDIO_PLL1_MUL: c_uint = 0x5b0;
pub const EVERGREEN_AUDIO_PLL1_DIV: c_uint = 0x5b4;
pub const EVERGREEN_AUDIO_PLL1_UNK: c_uint = 0x5bc;
pub const EVERGREEN_CG_IND_ADDR: c_uint = 0x8f8;
pub const EVERGREEN_CG_IND_DATA: c_uint = 0x8fc;
pub const EVERGREEN_AUDIO_ENABLE: c_uint = 0x5e78;
pub const EVERGREEN_AUDIO_VENDOR_ID: c_uint = 0x5ec0;
// GRPH blocks at 0x6800, 0x7400, 0x10000, 0x10c00, 0x11800, 0x12400
pub const EVERGREEN_GRPH_ENABLE: c_uint = 0x6800;
pub const EVERGREEN_GRPH_CONTROL: c_uint = 0x6804;

// 8 BPP

// 16 BPP

// 32 BPP

pub const EVERGREEN_GRPH_LUT_10BIT_BYPASS_CONTROL: c_uint = 0x6808;

pub const EVERGREEN_GRPH_SWAP_CONTROL: c_uint = 0x680c;

pub const EVERGREEN_GRPH_PRIMARY_SURFACE_ADDRESS: c_uint = 0x6810;
pub const EVERGREEN_GRPH_SECONDARY_SURFACE_ADDRESS: c_uint = 0x6814;

pub const EVERGREEN_GRPH_PITCH: c_uint = 0x6818;
pub const EVERGREEN_GRPH_PRIMARY_SURFACE_ADDRESS_HIGH: c_uint = 0x681c;
pub const EVERGREEN_GRPH_SECONDARY_SURFACE_ADDRESS_HIGH: c_uint = 0x6820;
pub const EVERGREEN_GRPH_SURFACE_OFFSET_X: c_uint = 0x6824;
pub const EVERGREEN_GRPH_SURFACE_OFFSET_Y: c_uint = 0x6828;
pub const EVERGREEN_GRPH_X_START: c_uint = 0x682c;
pub const EVERGREEN_GRPH_Y_START: c_uint = 0x6830;
pub const EVERGREEN_GRPH_X_END: c_uint = 0x6834;
pub const EVERGREEN_GRPH_Y_END: c_uint = 0x6838;
pub const EVERGREEN_GRPH_UPDATE: c_uint = 0x6844;

pub const EVERGREEN_GRPH_FLIP_CONTROL: c_uint = 0x6848;

// CUR blocks at 0x6998, 0x7598, 0x10198, 0x10d98, 0x11998, 0x12598
pub const EVERGREEN_CUR_CONTROL: c_uint = 0x6998;

pub const EVERGREEN_CUR_SURFACE_ADDRESS: c_uint = 0x699c;

pub const EVERGREEN_CUR_SIZE: c_uint = 0x69a0;
pub const EVERGREEN_CUR_SURFACE_ADDRESS_HIGH: c_uint = 0x69a4;
pub const EVERGREEN_CUR_POSITION: c_uint = 0x69a8;
pub const EVERGREEN_CUR_HOT_SPOT: c_uint = 0x69ac;
pub const EVERGREEN_CUR_COLOR1: c_uint = 0x69b0;
pub const EVERGREEN_CUR_COLOR2: c_uint = 0x69b4;
pub const EVERGREEN_CUR_UPDATE: c_uint = 0x69b8;

// LUT blocks at 0x69e0, 0x75e0, 0x101e0, 0x10de0, 0x119e0, 0x125e0
pub const EVERGREEN_DC_LUT_RW_MODE: c_uint = 0x69e0;
pub const EVERGREEN_DC_LUT_RW_INDEX: c_uint = 0x69e4;
pub const EVERGREEN_DC_LUT_SEQ_COLOR: c_uint = 0x69e8;
pub const EVERGREEN_DC_LUT_PWL_DATA: c_uint = 0x69ec;
pub const EVERGREEN_DC_LUT_30_COLOR: c_uint = 0x69f0;
pub const EVERGREEN_DC_LUT_VGA_ACCESS_ENABLE: c_uint = 0x69f4;
pub const EVERGREEN_DC_LUT_WRITE_EN_MASK: c_uint = 0x69f8;
pub const EVERGREEN_DC_LUT_AUTOFILL: c_uint = 0x69fc;
pub const EVERGREEN_DC_LUT_CONTROL: c_uint = 0x6a00;
pub const EVERGREEN_DC_LUT_BLACK_OFFSET_BLUE: c_uint = 0x6a04;
pub const EVERGREEN_DC_LUT_BLACK_OFFSET_GREEN: c_uint = 0x6a08;
pub const EVERGREEN_DC_LUT_BLACK_OFFSET_RED: c_uint = 0x6a0c;
pub const EVERGREEN_DC_LUT_WHITE_OFFSET_BLUE: c_uint = 0x6a10;
pub const EVERGREEN_DC_LUT_WHITE_OFFSET_GREEN: c_uint = 0x6a14;
pub const EVERGREEN_DC_LUT_WHITE_OFFSET_RED: c_uint = 0x6a18;
pub const EVERGREEN_DATA_FORMAT: c_uint = 0x6b00;

pub const EVERGREEN_DESKTOP_HEIGHT: c_uint = 0x6b04;
pub const EVERGREEN_VLINE_START_END: c_uint = 0x6b08;
pub const EVERGREEN_VLINE_STATUS: c_uint = 0x6bb8;

pub const EVERGREEN_VIEWPORT_START: c_uint = 0x6d70;
pub const EVERGREEN_VIEWPORT_SIZE: c_uint = 0x6d74;
// display controller offsets used for crtc/cur/lut/grph/viewport/etc.

// CRTC blocks at 0x6df0, 0x79f0, 0x105f0, 0x111f0, 0x11df0, 0x129f0
pub const EVERGREEN_CRTC_V_BLANK_START_END: c_uint = 0x6e34;
pub const EVERGREEN_CRTC_CONTROL: c_uint = 0x6e70;

pub const EVERGREEN_CRTC_BLANK_CONTROL: c_uint = 0x6e74;

pub const EVERGREEN_CRTC_STATUS: c_uint = 0x6e8c;

pub const EVERGREEN_CRTC_STATUS_POSITION: c_uint = 0x6e90;
pub const EVERGREEN_CRTC_STATUS_HV_COUNT: c_uint = 0x6ea0;
pub const EVERGREEN_CRTC_UPDATE_LOCK: c_uint = 0x6ed4;
pub const EVERGREEN_MASTER_UPDATE_LOCK: c_uint = 0x6ef4;
pub const EVERGREEN_MASTER_UPDATE_MODE: c_uint = 0x6ef8;
pub const EVERGREEN_DC_GPIO_HPD_MASK: c_uint = 0x64b0;
pub const EVERGREEN_DC_GPIO_HPD_A: c_uint = 0x64b4;
pub const EVERGREEN_DC_GPIO_HPD_EN: c_uint = 0x64b8;
pub const EVERGREEN_DC_GPIO_HPD_Y: c_uint = 0x64bc;
// HDMI blocks at 0x7030, 0x7c30, 0x10830, 0x11430, 0x12030, 0x12c30
pub const EVERGREEN_HDMI_BASE: c_uint = 0x7030;
// DIG block

pub const NI_DIG_FE_CNTL: c_uint = 0x7000;

pub const NI_DIG_BE_CNTL: c_uint = 0x7140;

pub const NI_DIG_BE_EN_CNTL: c_uint = 0x7144;

// Display Port block

pub const EVERGREEN_DP_VID_STREAM_CNTL: c_uint = 0x730C;

pub const EVERGREEN_DP_STEER_FIFO: c_uint = 0x7310;

pub const EVERGREEN_DP_SEC_CNTL: c_uint = 0x7280;

pub const EVERGREEN_DP_SEC_TIMESTAMP: c_uint = 0x72a4;

pub const EVERGREEN_DP_SEC_AUD_N: c_uint = 0x7294;

// DCIO_UNIPHY block

pub const NI_DCIO_UNIPHY0_PLL_CONTROL1: c_uint = 0x6618;

