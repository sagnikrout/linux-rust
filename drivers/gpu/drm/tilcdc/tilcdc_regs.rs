//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tilcdc/tilcdc_regs.h
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
// Copyright (C) 2012 Texas Instruments
// Author: Rob Clark <robdclark@gmail.com>
//
// LCDC register definitions, based on da8xx-fb

// LCDC Status Register

// LCDC DMA Control Register

pub const LCDC_DMA_BURST_1: c_uint = 0x0;
pub const LCDC_DMA_BURST_2: c_uint = 0x1;
pub const LCDC_DMA_BURST_4: c_uint = 0x2;
pub const LCDC_DMA_BURST_8: c_uint = 0x3;
pub const LCDC_DMA_BURST_16: c_uint = 0x4;

// LCDC Control Register

pub const LCDC_RASTER_MODE: c_uint = 0x01;
// LCDC Raster Control Register

pub const PALETTE_AND_DATA: c_uint = 0x00;
pub const PALETTE_ONLY: c_uint = 0x01;
pub const DATA_ONLY: c_uint = 0x02;

pub const LCDC_V2_LPP_B10: c_int = 26;

// LCDC Raster Timing 2 Register

// LCDC Block
pub const LCDC_PID_REG: c_uint = 0x0;
pub const LCDC_CTRL_REG: c_uint = 0x4;
pub const LCDC_STAT_REG: c_uint = 0x8;
pub const LCDC_RASTER_CTRL_REG: c_uint = 0x28;
pub const LCDC_RASTER_TIMING_0_REG: c_uint = 0x2c;
pub const LCDC_RASTER_TIMING_1_REG: c_uint = 0x30;
pub const LCDC_RASTER_TIMING_2_REG: c_uint = 0x34;
pub const LCDC_DMA_CTRL_REG: c_uint = 0x40;
pub const LCDC_DMA_FB_BASE_ADDR_0_REG: c_uint = 0x44;
pub const LCDC_DMA_FB_CEILING_ADDR_0_REG: c_uint = 0x48;
pub const LCDC_DMA_FB_BASE_ADDR_1_REG: c_uint = 0x4c;
pub const LCDC_DMA_FB_CEILING_ADDR_1_REG: c_uint = 0x50;
// Interrupt Registers available only in Version 2
pub const LCDC_RAW_STAT_REG: c_uint = 0x58;
pub const LCDC_MASKED_STAT_REG: c_uint = 0x5c;
pub const LCDC_INT_ENABLE_SET_REG: c_uint = 0x60;
pub const LCDC_INT_ENABLE_CLR_REG: c_uint = 0x64;
pub const LCDC_END_OF_INT_IND_REG: c_uint = 0x68;
// Clock registers available only on Version 2
pub const LCDC_CLK_ENABLE_REG: c_uint = 0x6c;
pub const LCDC_CLK_RESET_REG: c_uint = 0x70;

//
// Helpers:
//

// This compiles to strd (=64-bit write) on ARM7
// (volatile u64  *)addr = __cpu_to_le64(data);

extern "C" {
    pub fn ioread32(reg: priv->mmio +) -> return;
}
// the register to read/clear irqstatus differs between v1 and v2 of the IP
extern "C" {
    pub fn tilcdc_read(_arg: dev, _arg: tilcdc_irqstatus_reg(dev)) -> return;
}
