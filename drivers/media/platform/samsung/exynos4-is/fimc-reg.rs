//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/exynos4-is/fimc-reg.h
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
// Samsung camera host interface (FIMC) registers definition
//
// Copyright (C) 2010 - 2012 Samsung Electronics Co., Ltd.
//

// Input source format
pub const FIMC_REG_CISRCFMT: c_uint = 0x00;

// Window offset
pub const FIMC_REG_CIWDOFST: c_uint = 0x04;

// Global control
pub const FIMC_REG_CIGCTRL: c_uint = 0x08;

pub const FIMC_REG_CIGCTRL_TESTPAT_SHIFT: c_int = 27;

// 0 - selects Writeback A (LCD), 1 - selects Writeback B (LCD/ISP)

// 0 - ITU601; 1 - ITU709

// Window offset 2
pub const FIMC_REG_CIWDOFST2: c_uint = 0x14;

// Output DMA Y/Cb/Cr plane start addresses

// Target image format
pub const FIMC_REG_CITRGFMT: c_uint = 0x48;

pub const FIMC_REG_CITRGFMT_FLIP_SHIFT: c_int = 14;

// Output DMA control
pub const FIMC_REG_CIOCTRL: c_uint = 0x4c;

pub const FIMC_REG_CIOCTRL_ORDER2P_SHIFT: c_int = 24;

// Pre-scaler control 1
pub const FIMC_REG_CISCPRERATIO: c_uint = 0x50;
pub const FIMC_REG_CISCPREDST: c_uint = 0x54;
// Main scaler control
pub const FIMC_REG_CISCCTRL: c_uint = 0x58;

// Target area
pub const FIMC_REG_CITAREA: c_uint = 0x5c;
pub const FIMC_REG_CITAREA_MASK: c_uint = 0x0fffffff;
// General status
pub const FIMC_REG_CISTATUS: c_uint = 0x64;

pub const FIMC_REG_CISTATUS_FRAMECNT_SHIFT: c_int = 26;

// Indexes to the last and the currently processed buffer.
pub const FIMC_REG_CISTATUS2: c_uint = 0x68;
// Image capture control
pub const FIMC_REG_CIIMGCPT: c_uint = 0xc0;

// Frame capture sequence
pub const FIMC_REG_CICPTSEQ: c_uint = 0xc4;
// Image effect
pub const FIMC_REG_CIIMGEFF: c_uint = 0xd0;

// Input DMA Y/Cb/Cr plane start address 0/1

// Real input DMA image size
pub const FIMC_REG_CIREAL_ISIZE: c_uint = 0xf8;

// Input DMA control
pub const FIMC_REG_MSCTRL: c_uint = 0xfc;

pub const FIMC_REG_MSCTRL_2P_IN_ORDER_SHIFT: c_int = 16;

pub const FIMC_REG_MSCTRL_FLIP_SHIFT: c_int = 13;

pub const FIMC_REG_MSCTRL_ORDER422_SHIFT: c_int = 4;

// Output DMA Y/Cb/Cr offset
pub const FIMC_REG_CIOYOFF: c_uint = 0x168;
pub const FIMC_REG_CIOCBOFF: c_uint = 0x16c;
pub const FIMC_REG_CIOCROFF: c_uint = 0x170;
// Input DMA Y/Cb/Cr offset
pub const FIMC_REG_CIIYOFF: c_uint = 0x174;
pub const FIMC_REG_CIICBOFF: c_uint = 0x178;
pub const FIMC_REG_CIICROFF: c_uint = 0x17c;
// Input DMA original image size
pub const FIMC_REG_ORGISIZE: c_uint = 0x180;
// Output DMA original image size
pub const FIMC_REG_ORGOSIZE: c_uint = 0x184;
// Real output DMA image size (extension register)
pub const FIMC_REG_CIEXTEN: c_uint = 0x188;

pub const FIMC_REG_CIEXTEN_MVRATIO_EXT_MASK: c_uint = 0x3f;
pub const FIMC_REG_CIDMAPARAM: c_uint = 0x18c;

// MIPI CSI image format
pub const FIMC_REG_CSIIMGFMT: c_uint = 0x194;
pub const FIMC_REG_CSIIMGFMT_YCBCR422_8BIT: c_uint = 0x1e;
pub const FIMC_REG_CSIIMGFMT_RAW8: c_uint = 0x2a;
pub const FIMC_REG_CSIIMGFMT_RAW10: c_uint = 0x2b;
pub const FIMC_REG_CSIIMGFMT_RAW12: c_uint = 0x2c;
// User defined formats. x = 0...16.

// Output frame buffer sequence mask
pub const FIMC_REG_CIFCNTSEQ: c_uint = 0x1fc;
// SYSREG ISP Writeback register address offsets
pub const SYSREG_ISPBLK: c_uint = 0x020c;

pub const SYSREG_CAMBLK: c_uint = 0x0218;

//
// Function declarations
//
extern "C" {
    pub fn fimc_hw_reset(fimc: *mut fimc_dev);
}
extern "C" {
    pub fn fimc_hw_set_rotation(ctx: *mut fimc_ctx);
}
extern "C" {
    pub fn fimc_hw_set_target_format(ctx: *mut fimc_ctx);
}
extern "C" {
    pub fn fimc_hw_set_out_dma(ctx: *mut fimc_ctx);
}
extern "C" {
    pub fn fimc_hw_en_lastirq(fimc: *mut fimc_dev, enable: c_int);
}
extern "C" {
    pub fn fimc_hw_en_irq(fimc: *mut fimc_dev, enable: c_int);
}
extern "C" {
    pub fn fimc_hw_set_prescaler(ctx: *mut fimc_ctx);
}
extern "C" {
    pub fn fimc_hw_set_mainscaler(ctx: *mut fimc_ctx);
}
extern "C" {
    pub fn fimc_hw_enable_capture(ctx: *mut fimc_ctx);
}
extern "C" {
    pub fn fimc_hw_set_effect(ctx: *mut fimc_ctx);
}
extern "C" {
    pub fn fimc_hw_set_rgb_alpha(ctx: *mut fimc_ctx);
}
extern "C" {
    pub fn fimc_hw_set_in_dma(ctx: *mut fimc_ctx);
}
extern "C" {
    pub fn fimc_hw_set_input_path(ctx: *mut fimc_ctx);
}
extern "C" {
    pub fn fimc_hw_set_output_path(ctx: *mut fimc_ctx);
}
extern "C" {
    pub fn fimc_hw_set_input_addr(fimc: *mut fimc_dev, addr: *const fimc_addr);
}
extern "C" {
    pub fn fimc_hw_set_camera_offset(fimc: *mut fimc_dev, f: *const fimc_frame);
}
extern "C" {
    pub fn fimc_hw_clear_irq(dev: *mut fimc_dev);
}
extern "C" {
    pub fn fimc_hw_enable_scaler(dev: *mut fimc_dev, on: bool);
}
extern "C" {
    pub fn fimc_hw_activate_input_dma(dev: *mut fimc_dev, on: bool);
}
extern "C" {
    pub fn fimc_hw_disable_capture(dev: *mut fimc_dev);
}
extern "C" {
    pub fn fimc_hw_get_frame_index(dev: *mut fimc_dev) -> i32;
}
extern "C" {
    pub fn fimc_hw_get_prev_frame_index(dev: *mut fimc_dev) -> i32;
}
extern "C" {
    pub fn fimc_hw_camblk_cfg_writeback(fimc: *mut fimc_dev) -> c_int;
}
extern "C" {
    pub fn fimc_activate_capture(ctx: *mut fimc_ctx);
}
extern "C" {
    pub fn fimc_deactivate_capture(fimc: *mut fimc_dev);
}
//
// fimc_hw_set_dma_seq - configure output DMA buffer sequence
// @dev: fimc device
// @mask: bitmask for the DMA output buffer registers, set to 0 to skip buffer
// This function masks output DMA ring buffers, it allows to select which of
// the 32 available output buffer address registers will be used by the DMA
// engine.
//
