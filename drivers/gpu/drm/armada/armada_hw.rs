//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/armada/armada_hw.h
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
// Copyright (C) 2012 Russell King
// Rewritten from the dovefb driver, and Armada510 manuals.
//
// Note: the following registers are written from IRQ context:
// LCD_SPU_V_PORCH, LCD_SPU_ADV_REG, LCD_SPUT_V_H_TOTAL
// LCD_SPU_DMA_START_ADDR_[YUV][01], LCD_SPU_DMA_PITCH_YC,
// LCD_SPU_DMA_PITCH_UV, LCD_SPU_DMA_OVSA_HPXL_VLN,
// LCD_SPU_DMA_HPXL_VLN, LCD_SPU_DZM_HPXL_VLN, LCD_SPU_DMA_CTRL0
//
// For LCD_SPU_ADV_REG
// LCD_CFG_RDREG4F - Armada 510 only

// For LCD_SPU_DMA_CTRL0

// For LCD_SPU_DMA_CTRL1

// For LCD_SPU_SRAM_CTRL
// For LCD_SPU_SRAM_PARA1
// For LCD_CFG_SCLK_DIV
// Armada 510
// Armada 16x
// For LCD_SPU_DUMB_CTRL
// For LCD_SPU_IOPAD_CONTROL
pub const IOPAD_DUMB24: c_uint = 0x0;
// For LCD_SPU_IRQ_ENA
// For LCD_SPU_IRQ_ISR
