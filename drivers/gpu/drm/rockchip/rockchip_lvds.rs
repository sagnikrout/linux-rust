//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/rockchip/rockchip_lvds.h
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
// Copyright (C) Rockchip Electronics Co., Ltd.
// Author:
// Sandy Huang <hjc@rock-chips.com>
// Mark Yao <mark.yao@rock-chips.com>
//

pub const RK3288_LVDS_CH0_REG0: c_uint = 0x00;

pub const RK3288_LVDS_CH0_REG1: c_uint = 0x04;

pub const RK3288_LVDS_CH0_REG2: c_uint = 0x08;

pub const RK3288_LVDS_CH0_REG3: c_uint = 0x0c;
pub const RK3288_LVDS_CH0_REG3_PLL_FBDIV_MASK: c_uint = 0xff;
pub const RK3288_LVDS_CH0_REG4: c_uint = 0x10;

pub const RK3288_LVDS_CH0_REG5: c_uint = 0x14;

pub const RK3288_LVDS_CFG_REGC: c_uint = 0x30;
pub const RK3288_LVDS_CFG_REGC_PLL_ENABLE: c_uint = 0x00;
pub const RK3288_LVDS_CFG_REGC_PLL_DISABLE: c_uint = 0xff;
pub const RK3288_LVDS_CH0_REGD: c_uint = 0x34;
pub const RK3288_LVDS_CH0_REGD_PLL_PREDIV_MASK: c_uint = 0x1f;
pub const RK3288_LVDS_CH0_REG20: c_uint = 0x80;
pub const RK3288_LVDS_CH0_REG20_MSB: c_uint = 0x45;
pub const RK3288_LVDS_CH0_REG20_LSB: c_uint = 0x44;
pub const RK3288_LVDS_CFG_REG21: c_uint = 0x84;
pub const RK3288_LVDS_CFG_REG21_TX_ENABLE: c_uint = 0x92;
pub const RK3288_LVDS_CFG_REG21_TX_DISABLE: c_uint = 0x00;
pub const RK3288_LVDS_CH1_OFFSET: c_uint = 0x100;
pub const RK3288_LVDS_GRF_SOC_CON6: c_uint = 0x025C;
pub const RK3288_LVDS_GRF_SOC_CON7: c_uint = 0x0260;
// fbdiv value is split over 2 registers, with bit8 in reg2

pub const LVDS_VESA_24: c_int = 0;
pub const LVDS_JEIDA_24: c_int = 1;
pub const LVDS_VESA_18: c_int = 2;
pub const LVDS_JEIDA_18: c_int = 3;
pub const PX30_LVDS_GRF_PD_VO_CON0: c_uint = 0x434;

pub const PX30_LVDS_GRF_PD_VO_CON1: c_uint = 0x438;

