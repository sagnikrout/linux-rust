//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/exynos-gsc/gsc-regs.h
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
// Copyright (c) 2011 - 2012 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Register definition file for Samsung G-Scaler driver
//
// G-Scaler enable
pub const GSC_ENABLE: c_uint = 0x00;

// G-Scaler S/W reset
pub const GSC_SW_RESET: c_uint = 0x04;

// G-Scaler IRQ
pub const GSC_IRQ: c_uint = 0x08;

// G-Scaler input control
pub const GSC_IN_CON: c_uint = 0x10;

// G-Scaler source image size
pub const GSC_SRCIMG_SIZE: c_uint = 0x14;

// G-Scaler source image offset
pub const GSC_SRCIMG_OFFSET: c_uint = 0x18;

// G-Scaler cropped source image size
pub const GSC_CROPPED_SIZE: c_uint = 0x1c;

// G-Scaler output control
pub const GSC_OUT_CON: c_uint = 0x20;

// G-Scaler scaled destination image size
pub const GSC_SCALED_SIZE: c_uint = 0x24;

// G-Scaler pre scale ratio
pub const GSC_PRE_SCALE_RATIO: c_uint = 0x28;

// G-Scaler main scale horizontal ratio
pub const GSC_MAIN_H_RATIO: c_uint = 0x2c;

// G-Scaler main scale vertical ratio
pub const GSC_MAIN_V_RATIO: c_uint = 0x30;

// G-Scaler destination image size
pub const GSC_DSTIMG_SIZE: c_uint = 0x40;

// G-Scaler destination image offset
pub const GSC_DSTIMG_OFFSET: c_uint = 0x44;

// G-Scaler input y address mask
pub const GSC_IN_BASE_ADDR_Y_MASK: c_uint = 0x4c;
// G-Scaler input y base address

// G-Scaler input cb address mask
pub const GSC_IN_BASE_ADDR_CB_MASK: c_uint = 0x7c;
// G-Scaler input cb base address

// G-Scaler input cr address mask
pub const GSC_IN_BASE_ADDR_CR_MASK: c_uint = 0xac;
// G-Scaler input cr base address

// G-Scaler output y address mask
pub const GSC_OUT_BASE_ADDR_Y_MASK: c_uint = 0x10c;
// G-Scaler output y base address

// G-Scaler output cb address mask
pub const GSC_OUT_BASE_ADDR_CB_MASK: c_uint = 0x15c;
// G-Scaler output cb base address

// G-Scaler output cr address mask
pub const GSC_OUT_BASE_ADDR_CR_MASK: c_uint = 0x1ac;
// G-Scaler output cr base address

