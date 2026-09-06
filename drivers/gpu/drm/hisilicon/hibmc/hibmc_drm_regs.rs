//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/hisilicon/hibmc/hibmc_drm_regs.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Hisilicon Hibmc SoC drm driver
//
// Based on the bochs drm driver.
//
// Copyright (c) 2016 Huawei Limited.
//
// Author:
// Rongrong Zou <zourongrong@huawei.com>
// Rongrong Zou <zourongrong@gmail.com>
// Jianhua Li <lijianhua@huawei.com>
//
// register definition
pub const HIBMC_MISC_CTRL: c_uint = 0x4;

pub const HIBMC_MSCCTL_LOCALMEM_RESET_MASK: c_uint = 0x40;
pub const HIBMC_CURRENT_GATE: c_uint = 0x000040;

pub const HIBMC_CURR_GATE_DISPLAY_MASK: c_uint = 0x4;

pub const HIBMC_CURR_GATE_LOCALMEM_MASK: c_uint = 0x2;
pub const HIBMC_MODE0_GATE: c_uint = 0x000044;
pub const HIBMC_MODE1_GATE: c_uint = 0x000048;
pub const HIBMC_POWER_MODE_CTRL: c_uint = 0x00004C;

pub const HIBMC_PW_MODE_CTL_OSC_INPUT_MASK: c_uint = 0x8;

pub const HIBMC_PW_MODE_CTL_MODE_MASK: c_uint = 0x03;
pub const HIBMC_PW_MODE_CTL_MODE_SHIFT: c_int = 0;
pub const HIBMC_PW_MODE_CTL_MODE_MODE0: c_int = 0;
pub const HIBMC_PW_MODE_CTL_MODE_MODE1: c_int = 1;
pub const HIBMC_PW_MODE_CTL_MODE_SLEEP: c_int = 2;
pub const HIBMC_PANEL_PLL_CTRL: c_uint = 0x00005C;
pub const HIBMC_CRT_PLL_CTRL: c_uint = 0x000060;

pub const HIBMC_PLL_CTRL_BYPASS_MASK: c_uint = 0x40000;

pub const HIBMC_PLL_CTRL_POWER_MASK: c_uint = 0x20000;

pub const HIBMC_PLL_CTRL_INPUT_MASK: c_uint = 0x10000;

pub const HIBMC_PLL_CTRL_POD_MASK: c_uint = 0xC000;

pub const HIBMC_PLL_CTRL_OD_MASK: c_uint = 0x3000;

pub const HIBMC_PLL_CTRL_N_MASK: c_uint = 0xF00;

pub const HIBMC_PLL_CTRL_M_MASK: c_uint = 0xFF;
pub const HIBMC_CRT_DISP_CTL: c_uint = 0x80200;

pub const HIBMC_CRT_DISP_CTL_DPMS_MASK: c_uint = 0xc0000000;
pub const HIBMC_CRT_DPMS_ON: c_int = 0;
pub const HIBMC_CRT_DPMS_OFF: c_int = 3;

pub const HIBMC_CRT_DISP_CTL_CRTSELECT_MASK: c_uint = 0x2000000;
pub const HIBMC_CRTSELECT_CRT: c_int = 1;

pub const HIBMC_CRT_DISP_CTL_CLOCK_PHASE_MASK: c_uint = 0x4000;

pub const HIBMC_CRT_DISP_CTL_VSYNC_PHASE_MASK: c_uint = 0x2000;

pub const HIBMC_CRT_DISP_CTL_HSYNC_PHASE_MASK: c_uint = 0x1000;

pub const HIBMC_CRT_DISP_CTL_TIMING_MASK: c_uint = 0x100;

pub const HIBMC_CTL_DISP_CTL_GAMMA_MASK: c_uint = 0x08;

pub const HIBMC_CRT_DISP_CTL_PLANE_MASK: c_int = 4;

pub const HIBMC_CRT_DISP_CTL_FORMAT_MASK: c_uint = 0x03;
pub const HIBMC_CRT_FB_ADDRESS: c_uint = 0x080204;
pub const HIBMC_CRT_FB_WIDTH: c_uint = 0x080208;

pub const HIBMC_CRT_FB_WIDTH_WIDTH_MASK: c_uint = 0x3FFF0000;

pub const HIBMC_CRT_FB_WIDTH_OFFS_MASK: c_uint = 0x3FFF;
pub const HIBMC_CRT_HORZ_TOTAL: c_uint = 0x08020C;

pub const HIBMC_CRT_HORZ_TOTAL_TOTAL_MASK: c_uint = 0xFFF0000;

pub const HIBMC_CRT_HORZ_TOTAL_DISP_END_MASK: c_uint = 0xFFF;
pub const HIBMC_CRT_HORZ_SYNC: c_uint = 0x080210;

pub const HIBMC_CRT_HORZ_SYNC_WIDTH_MASK: c_uint = 0xFF0000;

pub const HIBMC_CRT_HORZ_SYNC_START_MASK: c_uint = 0xFFF;
pub const HIBMC_CRT_VERT_TOTAL: c_uint = 0x080214;

pub const HIBMC_CRT_VERT_TOTAL_TOTAL_MASK: c_uint = 0x7FFF0000;

pub const HIBMC_CRT_VERT_TOTAL_DISP_END_MASK: c_uint = 0x7FF;
pub const HIBMC_CRT_VERT_SYNC: c_uint = 0x080218;

pub const HIBMC_CRT_VERT_SYNC_HEIGHT_MASK: c_uint = 0x3F0000;

pub const HIBMC_CRT_VERT_SYNC_START_MASK: c_uint = 0x7FF;
// Auto Centering
pub const HIBMC_CRT_AUTO_CENTERING_TL: c_uint = 0x080280;

pub const HIBMC_CRT_AUTO_CENTERING_TL_TOP_MASK: c_uint = 0x7FF0000;

pub const HIBMC_CRT_AUTO_CENTERING_TL_LEFT_MASK: c_uint = 0x7FF;
pub const HIBMC_CRT_AUTO_CENTERING_BR: c_uint = 0x080284;

pub const HIBMC_CRT_AUTO_CENTERING_BR_BOTTOM_MASK: c_uint = 0x7FF0000;

pub const HIBMC_CRT_AUTO_CENTERING_BR_RIGHT_MASK: c_uint = 0x7FF;
// register to control panel output
pub const HIBMC_DISPLAY_CONTROL_HISILE: c_uint = 0x80288;

pub const HIBMC_RAW_INTERRUPT: c_uint = 0x80290;

pub const HIBMC_RAW_INTERRUPT_VBLANK_MASK: c_uint = 0x4;
pub const HIBMC_RAW_INTERRUPT_EN: c_uint = 0x80298;

pub const HIBMC_RAW_INTERRUPT_EN_VBLANK_MASK: c_uint = 0x4;
// register and values for PLL control
pub const CRT_PLL1_HS: c_uint = 0x802a8;

pub const CRT_PLL1_HS_25MHZ: c_uint = 0x23d40f02;
pub const CRT_PLL1_HS_40MHZ: c_uint = 0x23940801;
pub const CRT_PLL1_HS_65MHZ: c_uint = 0x23940d01;
pub const CRT_PLL1_HS_78MHZ: c_uint = 0x23540F82;
pub const CRT_PLL1_HS_74MHZ: c_uint = 0x23941dc2;
pub const CRT_PLL1_HS_80MHZ: c_uint = 0x23941001;
pub const CRT_PLL1_HS_80MHZ_1152: c_uint = 0x23540fc2;
pub const CRT_PLL1_HS_106MHZ: c_uint = 0x237C1641;
pub const CRT_PLL1_HS_108MHZ: c_uint = 0x23b41b01;
pub const CRT_PLL1_HS_162MHZ: c_uint = 0x23480681;
pub const CRT_PLL1_HS_148MHZ: c_uint = 0x23541dc2;
pub const CRT_PLL1_HS_193MHZ: c_uint = 0x234807c1;
pub const CRT_PLL2_HS: c_uint = 0x802ac;
pub const CRT_PLL2_HS_25MHZ: c_uint = 0x206B851E;
pub const CRT_PLL2_HS_40MHZ: c_uint = 0x30000000;
pub const CRT_PLL2_HS_65MHZ: c_uint = 0x40000000;
pub const CRT_PLL2_HS_78MHZ: c_uint = 0x50E147AE;
pub const CRT_PLL2_HS_74MHZ: c_uint = 0x602B6AE7;
pub const CRT_PLL2_HS_80MHZ: c_uint = 0x70000000;
pub const CRT_PLL2_HS_106MHZ: c_uint = 0x0075c28f;
pub const CRT_PLL2_HS_108MHZ: c_uint = 0x80000000;
pub const CRT_PLL2_HS_162MHZ: c_uint = 0xA0000000;
pub const CRT_PLL2_HS_148MHZ: c_uint = 0xB0CCCCCD;
pub const CRT_PLL2_HS_193MHZ: c_uint = 0xC0872B02;
pub const HIBMC_CRT_PALETTE: c_uint = 0x80C00;

