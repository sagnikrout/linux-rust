//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spi/mxs-spi.h
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
//
// include/linux/spi/mxs-spi.h
//
// Freescale i.MX233/i.MX28 SPI controller register definition
//
// Copyright 2008 Embedded Alley Solutions, Inc.
// Copyright 2009-2011 Freescale Semiconductor, Inc.
//

// SSP registers
pub const HW_SSP_CTRL0: c_uint = 0x000;

pub const BP_SSP_CTRL0_BUS_WIDTH: c_int = 22;

pub const BP_SSP_CTRL0_XFER_COUNT: c_int = 0;
pub const BM_SSP_CTRL0_XFER_COUNT: c_uint = 0xffff;
pub const HW_SSP_CMD0: c_uint = 0x010;

pub const BP_SSP_CMD0_BLOCK_SIZE: c_int = 16;

pub const BP_SSP_CMD0_BLOCK_COUNT: c_int = 8;

pub const BP_SSP_CMD0_CMD: c_int = 0;
pub const BM_SSP_CMD0_CMD: c_uint = 0xff;
pub const HW_SSP_CMD1: c_uint = 0x020;
pub const HW_SSP_XFER_SIZE: c_uint = 0x030;
pub const HW_SSP_BLOCK_SIZE: c_uint = 0x040;
pub const BP_SSP_BLOCK_SIZE_BLOCK_COUNT: c_int = 4;

pub const BP_SSP_BLOCK_SIZE_BLOCK_SIZE: c_int = 0;
pub const BM_SSP_BLOCK_SIZE_BLOCK_SIZE: c_uint = 0xf;

pub const BP_SSP_TIMING_TIMEOUT: c_int = 16;

pub const BP_SSP_TIMING_CLOCK_DIVIDE: c_int = 8;

pub const BP_SSP_TIMING_CLOCK_RATE: c_int = 0;
pub const BM_SSP_TIMING_CLOCK_RATE: c_uint = 0xff;

pub const BP_SSP_CTRL1_WORD_LENGTH: c_int = 4;

pub const BV_SSP_CTRL1_WORD_LENGTH__FOUR_BITS: c_uint = 0x3;
pub const BV_SSP_CTRL1_WORD_LENGTH__EIGHT_BITS: c_uint = 0x7;
pub const BV_SSP_CTRL1_WORD_LENGTH__SIXTEEN_BITS: c_uint = 0xF;
pub const BP_SSP_CTRL1_SSP_MODE: c_int = 0;
pub const BM_SSP_CTRL1_SSP_MODE: c_uint = 0xf;

pub const BV_SSP_CTRL1_SSP_MODE__SPI: c_uint = 0x0;
pub const BV_SSP_CTRL1_SSP_MODE__SSI: c_uint = 0x1;
pub const BV_SSP_CTRL1_SSP_MODE__SD_MMC: c_uint = 0x3;
pub const BV_SSP_CTRL1_SSP_MODE__MS: c_uint = 0x4;

pub const SSP_PIO_NUM: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxs_ssp_id {
    IMX23_SSP,
    IMX28_SSP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxs_ssp {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub clk_rate: c_uint,
    pub devid: mxs_ssp_id,
    pub dmach: *mut dma_chan,
    pub dma_dir: c_uint,
    pub slave_dirn: dma_transfer_direction,
    pub ssp_pio_words: [u32; SSP_PIO_NUM],
}

extern "C" {
    pub fn mxs_ssp_set_clk_rate(ssp: *mut mxs_ssp, rate: c_uint);
}
