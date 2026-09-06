//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/exynos/regs-gsc.h
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
// linux/drivers/gpu/drm/exynos/regs-gsc.h
//
// Copyright (c) 2012 Samsung Electronics Co., Ltd.
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
pub const GSC_CROPPED_SIZE: c_uint = 0x1C;

// G-Scaler output control
pub const GSC_OUT_CON: c_uint = 0x20;

// G-Scaler scaled destination image size
pub const GSC_SCALED_SIZE: c_uint = 0x24;

// G-Scaler pre scale ratio
pub const GSC_PRE_SCALE_RATIO: c_uint = 0x28;

// G-Scaler main scale horizontal ratio
pub const GSC_MAIN_H_RATIO: c_uint = 0x2C;

// G-Scaler main scale vertical ratio
pub const GSC_MAIN_V_RATIO: c_uint = 0x30;

// G-Scaler input chrominance stride
pub const GSC_IN_CHROM_STRIDE: c_uint = 0x3C;

// G-Scaler destination image size
pub const GSC_DSTIMG_SIZE: c_uint = 0x40;

// G-Scaler destination image offset
pub const GSC_DSTIMG_OFFSET: c_uint = 0x44;

// G-Scaler output chrominance stride
pub const GSC_OUT_CHROM_STRIDE: c_uint = 0x48;

// G-Scaler input y address mask
pub const GSC_IN_BASE_ADDR_Y_MASK: c_uint = 0x4C;
// G-Scaler input y base address

// G-Scaler input y base current address

// G-Scaler input cb address mask
pub const GSC_IN_BASE_ADDR_CB_MASK: c_uint = 0x7C;
// G-Scaler input cb base address

// G-Scaler input cb base current address

// G-Scaler input cr address mask
pub const GSC_IN_BASE_ADDR_CR_MASK: c_uint = 0xAC;
// G-Scaler input cr base address

// G-Scaler input cr base current address

// G-Scaler input address mask

// G-Scaler output y address mask
pub const GSC_OUT_BASE_ADDR_Y_MASK: c_uint = 0x10C;
// G-Scaler output y base address

// G-Scaler output cb address mask
pub const GSC_OUT_BASE_ADDR_CB_MASK: c_uint = 0x15C;
// G-Scaler output cb base address

// G-Scaler output cr address mask
pub const GSC_OUT_BASE_ADDR_CR_MASK: c_uint = 0x1AC;
// G-Scaler output cr base address

// G-Scaler output address mask

// G-Scaler horizontal scaling filter

// G-Scaler vertical scaling filter

// G-Scaler BUS control
pub const GSC_BUSCON: c_uint = 0xA78;

// G-Scaler V position
pub const GSC_VPOSITION: c_uint = 0xA7C;

// G-Scaler clock initial count
pub const GSC_CLK_INIT_COUNT: c_uint = 0xC00;

// G-Scaler clock snoop count
pub const GSC_CLK_SNOOP_COUNT: c_uint = 0xC04;

// SYSCON. GSCBLK_CFG
pub const SYSREG_GSCBLK_CFG1: c_uint = 0x0224;

pub const SYSREG_GSCBLK_CFG2: c_uint = 0x2000;

