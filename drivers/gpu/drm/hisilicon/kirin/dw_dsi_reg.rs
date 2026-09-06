//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/hisilicon/kirin/dw_dsi_reg.h
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
// Copyright (c) 2016 Linaro Limited.
// Copyright (c) 2014-2016 HiSilicon Limited.
//

//
// regs
//
pub const PWR_UP: c_uint = 0x04  /* Core power-up */;
pub const RESET: c_int = 0;

pub const PHY_IF_CFG: c_uint = 0xA4  /* D-PHY interface configuration */;
pub const CLKMGR_CFG: c_uint = 0x08  /* the internal clock dividers */;
pub const PHY_RSTZ: c_uint = 0xA0  /* D-PHY reset control */;

pub const PHY_TST_CTRL0: c_uint = 0xB4  /* D-PHY test interface control 0 */;
pub const PHY_TST_CTRL1: c_uint = 0xB8  /* D-PHY test interface control 1 */;
pub const CLK_TLPX: c_uint = 0x10;
pub const CLK_THS_PREPARE: c_uint = 0x11;
pub const CLK_THS_ZERO: c_uint = 0x12;
pub const CLK_THS_TRAIL: c_uint = 0x13;
pub const CLK_TWAKEUP: c_uint = 0x14;

pub const PHY_CFG_I: c_uint = 0x60;
pub const PHY_CFG_PLL_I: c_uint = 0x63;
pub const PHY_CFG_PLL_II: c_uint = 0x64;
pub const PHY_CFG_PLL_III: c_uint = 0x65;
pub const PHY_CFG_PLL_IV: c_uint = 0x66;
pub const PHY_CFG_PLL_V: c_uint = 0x67;
pub const DPI_COLOR_CODING: c_uint = 0x10  /* DPI color coding */;
pub const DPI_CFG_POL: c_uint = 0x14  /* DPI polarity configuration */;
pub const VID_HSA_TIME: c_uint = 0x48  /* Horizontal Sync Active time */;
pub const VID_HBP_TIME: c_uint = 0x4C  /* Horizontal Back Porch time */;
pub const VID_HLINE_TIME: c_uint = 0x50  /* Line time */;
pub const VID_VSA_LINES: c_uint = 0x54  /* Vertical Sync Active period */;
pub const VID_VBP_LINES: c_uint = 0x58  /* Vertical Back Porch period */;
pub const VID_VFP_LINES: c_uint = 0x5C  /* Vertical Front Porch period */;
pub const VID_VACTIVE_LINES: c_uint = 0x60  /* Vertical resolution */;
pub const VID_PKT_SIZE: c_uint = 0x3C  /* Video packet size */;
pub const VID_MODE_CFG: c_uint = 0x38  /* Video mode configuration */;
pub const PHY_TMR_CFG: c_uint = 0x9C  /* Data lanes timing configuration */;
pub const BTA_TO_CNT: c_uint = 0x8C  /* Response timeout definition */;
pub const PHY_TMR_LPCLK_CFG: c_uint = 0x98  /* clock lane timing configuration */;
pub const CLK_DATA_TMR_CFG: c_uint = 0xCC;
pub const LPCLK_CTRL: c_uint = 0x94  /* Low-power in clock lane */;

pub const MODE_CFG: c_uint = 0x34  /* Video or Command mode selection */;
pub const PHY_STATUS: c_uint = 0xB0  /* D-PHY PPI status interface */;
pub const PHY_STOP_WAIT_TIME: c_uint = 0x30;
//
// regs relevant enum
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpi_color_coding {
    DSI_24BITS_1 = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsi_video_mode_type {
    DSI_NON_BURST_SYNC_PULSES = 0,
    DSI_NON_BURST_SYNC_EVENTS,
    DSI_BURST_SYNC_PULSES_1,
    DSI_BURST_SYNC_PULSES_2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsi_work_mode {
    DSI_VIDEO_MODE = 0,
    DSI_COMMAND_MODE
}

//
// Register Write/Read Helper functions
//
