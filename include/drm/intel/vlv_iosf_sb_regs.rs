//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/intel/vlv_iosf_sb_regs.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vlv_iosf_sb_unit {
    VLV_IOSF_SB_BUNIT,
    VLV_IOSF_SB_CCK,
    VLV_IOSF_SB_CCU,
    VLV_IOSF_SB_DPIO,
    VLV_IOSF_SB_DPIO_2,
    VLV_IOSF_SB_FLISDSI,
    VLV_IOSF_SB_GPIO,
    VLV_IOSF_SB_NC,
    VLV_IOSF_SB_PUNIT,
}

// See configdb bunit SB addr map
pub const BUNIT_REG_BISOC: c_uint = 0x11;
// PUNIT_REG_*SSPM0

// PUNIT_REG_*SSPM1
pub const SSPM1_FREQSTAT_SHIFT: c_int = 24;

pub const SSPM1_FREQGUAR_SHIFT: c_int = 8;

pub const SSPM1_FREQ_SHIFT: c_int = 0;

pub const PUNIT_REG_VEDSSPM0: c_uint = 0x32;
pub const PUNIT_REG_VEDSSPM1: c_uint = 0x33;
pub const PUNIT_REG_DSPSSPM: c_uint = 0x36;
pub const DSPFREQSTAT_SHIFT_CHV: c_int = 24;

pub const DSPFREQGUAR_SHIFT_CHV: c_int = 8;

pub const DSPFREQSTAT_SHIFT: c_int = 30;

pub const DSPFREQGUAR_SHIFT: c_int = 14;

pub const PUNIT_REG_ISPSSPM0: c_uint = 0x39;
pub const PUNIT_REG_ISPSSPM1: c_uint = 0x3a;
pub const PUNIT_REG_PWRGT_CTRL: c_uint = 0x60;
pub const PUNIT_REG_PWRGT_STATUS: c_uint = 0x61;

pub const PUNIT_PWGT_IDX_RENDER: c_int = 0;
pub const PUNIT_PWGT_IDX_MEDIA: c_int = 1;
pub const PUNIT_PWGT_IDX_DISP2D: c_int = 3;
pub const PUNIT_PWGT_IDX_DPIO_CMN_BC: c_int = 5;
pub const PUNIT_PWGT_IDX_DPIO_TX_B_LANES_01: c_int = 6;
pub const PUNIT_PWGT_IDX_DPIO_TX_B_LANES_23: c_int = 7;
pub const PUNIT_PWGT_IDX_DPIO_TX_C_LANES_01: c_int = 8;
pub const PUNIT_PWGT_IDX_DPIO_TX_C_LANES_23: c_int = 9;
pub const PUNIT_PWGT_IDX_DPIO_RX0: c_int = 10;
pub const PUNIT_PWGT_IDX_DPIO_RX1: c_int = 11;
pub const PUNIT_PWGT_IDX_DPIO_CMN_D: c_int = 12;
pub const PUNIT_REG_GPU_LFM: c_uint = 0xd3;
pub const PUNIT_REG_GPU_FREQ_REQ: c_uint = 0xd4;
pub const PUNIT_REG_GPU_FREQ_STS: c_uint = 0xd8;

pub const PUNIT_REG_MEDIA_TURBO_FREQ_REQ: c_uint = 0xdc;
pub const PUNIT_REG_CZ_TIMESTAMP: c_uint = 0xce;
pub const PUNIT_FUSE_BUS2: c_uint = 0xf6 /* bits 47:40 */;
pub const PUNIT_FUSE_BUS1: c_uint = 0xf5 /* bits 55:48 */;
pub const FB_GFX_FMAX_AT_VMAX_FUSE: c_uint = 0x136;
pub const FB_GFX_FREQ_FUSE_MASK: c_uint = 0xff;
pub const FB_GFX_FMAX_AT_VMAX_2SS4EU_FUSE_SHIFT: c_int = 24;
pub const FB_GFX_FMAX_AT_VMAX_2SS6EU_FUSE_SHIFT: c_int = 16;
pub const FB_GFX_FMAX_AT_VMAX_2SS8EU_FUSE_SHIFT: c_int = 8;
pub const FB_GFX_FMIN_AT_VMIN_FUSE: c_uint = 0x137;
pub const FB_GFX_FMIN_AT_VMIN_FUSE_SHIFT: c_int = 8;
pub const PUNIT_REG_DDR_SETUP2: c_uint = 0x139;

pub const PUNIT_GPU_STATUS_REG: c_uint = 0xdb;
pub const PUNIT_GPU_STATUS_MAX_FREQ_SHIFT: c_int = 16;
pub const PUNIT_GPU_STATUS_MAX_FREQ_MASK: c_uint = 0xff;
pub const PUNIT_GPU_STATIS_GFX_MIN_FREQ_SHIFT: c_int = 8;
pub const PUNIT_GPU_STATUS_GFX_MIN_FREQ_MASK: c_uint = 0xff;
pub const PUNIT_GPU_DUTYCYCLE_REG: c_uint = 0xdf;
pub const PUNIT_GPU_DUTYCYCLE_RPE_FREQ_SHIFT: c_int = 8;
pub const PUNIT_GPU_DUTYCYCLE_RPE_FREQ_MASK: c_uint = 0xff;
pub const IOSF_NC_FB_GFX_FREQ_FUSE: c_uint = 0x1c;
pub const FB_GFX_MAX_FREQ_FUSE_SHIFT: c_int = 3;
pub const FB_GFX_MAX_FREQ_FUSE_MASK: c_uint = 0x000007f8;
pub const FB_GFX_FGUARANTEED_FREQ_FUSE_SHIFT: c_int = 11;
pub const FB_GFX_FGUARANTEED_FREQ_FUSE_MASK: c_uint = 0x0007f800;
pub const IOSF_NC_FB_GFX_FMAX_FUSE_HI: c_uint = 0x34;
pub const FB_FMAX_VMIN_FREQ_HI_MASK: c_uint = 0x00000007;
pub const IOSF_NC_FB_GFX_FMAX_FUSE_LO: c_uint = 0x30;
pub const FB_FMAX_VMIN_FREQ_LO_SHIFT: c_int = 27;
pub const FB_FMAX_VMIN_FREQ_LO_MASK: c_uint = 0xf8000000;
pub const VLV_TURBO_SOC_OVERRIDE: c_uint = 0x04;
pub const VLV_OVERRIDE_EN: c_int = 1;

// vlv2 north clock has
pub const CCK_FUSE_REG: c_uint = 0x8;
pub const CCK_FUSE_HPLL_FREQ_MASK: c_uint = 0x3;
pub const CCK_REG_DSI_PLL_FUSE: c_uint = 0x44;
pub const CCK_REG_DSI_PLL_CONTROL: c_uint = 0x48;

pub const DSI_PLL_P1_POST_DIV_SHIFT: c_int = 17;

pub const CCK_REG_DSI_PLL_DIVIDER: c_uint = 0x4c;

pub const DSI_PLL_FRAC_COUNTER_SHIFT: c_int = 27;

pub const DSI_PLL_USYNC_CNT_SHIFT: c_int = 18;

pub const DSI_PLL_N1_DIV_SHIFT: c_int = 16;

pub const DSI_PLL_M1_DIV_SHIFT: c_int = 0;

pub const CCK_CZ_CLOCK_CONTROL: c_uint = 0x62;
pub const CCK_GPLL_CLOCK_CONTROL: c_uint = 0x67;
pub const CCK_DISPLAY_CLOCK_CONTROL: c_uint = 0x6b;
pub const CCK_DISPLAY_REF_CLOCK_CONTROL: c_uint = 0x6c;

pub const CCK_FREQUENCY_STATUS_SHIFT: c_int = 8;

