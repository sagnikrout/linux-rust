//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/ingenic/ingenic-drm.h
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
// Ingenic JZ47xx KMS driver - Register definitions and private API
//
// Copyright (C) 2020, Paul Cercueil <paul@crapouillou.net>

pub const JZ_REG_LCD_CFG: c_uint = 0x00;
pub const JZ_REG_LCD_VSYNC: c_uint = 0x04;
pub const JZ_REG_LCD_HSYNC: c_uint = 0x08;
pub const JZ_REG_LCD_VAT: c_uint = 0x0C;
pub const JZ_REG_LCD_DAH: c_uint = 0x10;
pub const JZ_REG_LCD_DAV: c_uint = 0x14;
pub const JZ_REG_LCD_PS: c_uint = 0x18;
pub const JZ_REG_LCD_CLS: c_uint = 0x1C;
pub const JZ_REG_LCD_SPL: c_uint = 0x20;
pub const JZ_REG_LCD_REV: c_uint = 0x24;
pub const JZ_REG_LCD_CTRL: c_uint = 0x30;
pub const JZ_REG_LCD_STATE: c_uint = 0x34;
pub const JZ_REG_LCD_IID: c_uint = 0x38;
pub const JZ_REG_LCD_DA0: c_uint = 0x40;
pub const JZ_REG_LCD_SA0: c_uint = 0x44;
pub const JZ_REG_LCD_FID0: c_uint = 0x48;
pub const JZ_REG_LCD_CMD0: c_uint = 0x4C;
pub const JZ_REG_LCD_DA1: c_uint = 0x50;
pub const JZ_REG_LCD_SA1: c_uint = 0x54;
pub const JZ_REG_LCD_FID1: c_uint = 0x58;
pub const JZ_REG_LCD_CMD1: c_uint = 0x5C;
pub const JZ_REG_LCD_RGBC: c_uint = 0x90;
pub const JZ_REG_LCD_OSDC: c_uint = 0x100;
pub const JZ_REG_LCD_OSDCTRL: c_uint = 0x104;
pub const JZ_REG_LCD_OSDS: c_uint = 0x108;
pub const JZ_REG_LCD_BGC: c_uint = 0x10c;
pub const JZ_REG_LCD_KEY0: c_uint = 0x110;
pub const JZ_REG_LCD_KEY1: c_uint = 0x114;
pub const JZ_REG_LCD_ALPHA: c_uint = 0x118;
pub const JZ_REG_LCD_IPUR: c_uint = 0x11c;
pub const JZ_REG_LCD_XYP0: c_uint = 0x120;
pub const JZ_REG_LCD_XYP1: c_uint = 0x124;
pub const JZ_REG_LCD_SIZE0: c_uint = 0x128;
pub const JZ_REG_LCD_SIZE1: c_uint = 0x12c;
pub const JZ_REG_LCD_PCFG: c_uint = 0x2c0;

pub const JZ_LCD_CFG_MODE_GENERIC_16BIT: c_int = 0;

pub const JZ_LCD_CFG_MODE_SPECIAL_TFT_1: c_int = 1;
pub const JZ_LCD_CFG_MODE_SPECIAL_TFT_2: c_int = 2;
pub const JZ_LCD_CFG_MODE_SPECIAL_TFT_3: c_int = 3;
pub const JZ_LCD_CFG_MODE_TV_OUT_P: c_int = 4;
pub const JZ_LCD_CFG_MODE_TV_OUT_I: c_int = 6;
pub const JZ_LCD_CFG_MODE_SINGLE_COLOR_STN: c_int = 8;
pub const JZ_LCD_CFG_MODE_SINGLE_MONOCHROME_STN: c_int = 9;
pub const JZ_LCD_CFG_MODE_DUAL_COLOR_STN: c_int = 10;
pub const JZ_LCD_CFG_MODE_DUAL_MONOCHROME_STN: c_int = 11;
pub const JZ_LCD_CFG_MODE_8BIT_SERIAL: c_int = 12;
pub const JZ_LCD_CFG_MODE_LCM: c_int = 13;
pub const JZ_LCD_VSYNC_VPS_OFFSET: c_int = 16;
pub const JZ_LCD_VSYNC_VPE_OFFSET: c_int = 0;
pub const JZ_LCD_HSYNC_HPS_OFFSET: c_int = 16;
pub const JZ_LCD_HSYNC_HPE_OFFSET: c_int = 0;
pub const JZ_LCD_VAT_HT_OFFSET: c_int = 16;
pub const JZ_LCD_VAT_VT_OFFSET: c_int = 0;
pub const JZ_LCD_DAH_HDS_OFFSET: c_int = 16;
pub const JZ_LCD_DAH_HDE_OFFSET: c_int = 0;
pub const JZ_LCD_DAV_VDS_OFFSET: c_int = 16;
pub const JZ_LCD_DAV_VDE_OFFSET: c_int = 0;

pub const JZ_LCD_CTRL_BPP_1: c_uint = 0x0;
pub const JZ_LCD_CTRL_BPP_2: c_uint = 0x1;
pub const JZ_LCD_CTRL_BPP_4: c_uint = 0x2;
pub const JZ_LCD_CTRL_BPP_8: c_uint = 0x3;
pub const JZ_LCD_CTRL_BPP_15_16: c_uint = 0x4;
pub const JZ_LCD_CTRL_BPP_18_24: c_uint = 0x5;
pub const JZ_LCD_CTRL_BPP_24_COMP: c_uint = 0x6;
pub const JZ_LCD_CTRL_BPP_30: c_uint = 0x7;

pub const JZ_LCD_SYNC_MASK: c_uint = 0x3ff;

pub const JZ_LCD_OSDCTRL_BPP_15_16: c_uint = 0x4;
pub const JZ_LCD_OSDCTRL_BPP_18_24: c_uint = 0x5;
pub const JZ_LCD_OSDCTRL_BPP_24_COMP: c_uint = 0x6;
pub const JZ_LCD_OSDCTRL_BPP_30: c_uint = 0x7;

pub const JZ_LCD_IPUR_IPUR_LSB: c_int = 0;
pub const JZ_LCD_XYP01_XPOS_LSB: c_int = 0;
pub const JZ_LCD_XYP01_YPOS_LSB: c_int = 16;
pub const JZ_LCD_SIZE01_WIDTH_LSB: c_int = 0;
pub const JZ_LCD_SIZE01_HEIGHT_LSB: c_int = 16;
pub const JZ_LCD_DESSIZE_ALPHA_OFFSET: c_int = 24;

pub const JZ_LCD_CPOS_COEFFICIENT_OFFSET: c_int = 24;
pub const JZ_LCD_CPOS_COEFFICIENT_0: c_int = 0;
pub const JZ_LCD_CPOS_COEFFICIENT_1: c_int = 1;
pub const JZ_LCD_CPOS_COEFFICIENT_ALPHA1: c_int = 2;
pub const JZ_LCD_CPOS_COEFFICIENT_1_ALPHA1: c_int = 3;

pub const JZ_LCD_PCFG_THRESHOLD2_OFFSET: c_int = 18;
pub const JZ_LCD_PCFG_THRESHOLD1_OFFSET: c_int = 9;
pub const JZ_LCD_PCFG_THRESHOLD0_OFFSET: c_int = 0;
extern "C" {
    pub fn ingenic_drm_plane_disable(dev: *mut device, plane: *mut drm_plane);
}
extern "C" {
    pub fn ingenic_drm_map_noncoherent(dev: *const device) -> bool;
}
