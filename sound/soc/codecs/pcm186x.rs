//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/pcm186x.h
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
// Texas Instruments PCM186x Universal Audio ADC
//
// Copyright (C) 2015-2017 Texas Instruments Incorporated - https://www.ti.com
// Andreas Dannenberg <dannenberg@ti.com>
// Andrew F. Davis <afd@ti.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pcm186x_type {
    PCM1862,
    PCM1863,
    PCM1864,
    PCM1865,
}

pub const PCM186X_PAGE_LEN: c_uint = 0x0100;

// The page selection register address is the same on all pages
pub const PCM186X_PAGE: c_int = 0;
// Register Definitions - Page 0

// Register Definitions - Page 1

// Register Definitions - Page 3

// Register Definitions - Page 253

// PCM186X_PAGE
pub const PCM186X_RESET: c_uint = 0xfe;
// PCM186X_ADCX_INPUT_SEL_X

// PCM186X_PCM_CFG

pub const PCM186X_PCM_CFG_RX_WLEN_SHIFT: c_int = 6;
pub const PCM186X_PCM_CFG_RX_WLEN_32: c_uint = 0x00;
pub const PCM186X_PCM_CFG_RX_WLEN_24: c_uint = 0x01;
pub const PCM186X_PCM_CFG_RX_WLEN_20: c_uint = 0x02;
pub const PCM186X_PCM_CFG_RX_WLEN_16: c_uint = 0x03;

pub const PCM186X_PCM_CFG_TX_WLEN_SHIFT: c_int = 2;
pub const PCM186X_PCM_CFG_TX_WLEN_32: c_uint = 0x00;
pub const PCM186X_PCM_CFG_TX_WLEN_24: c_uint = 0x01;
pub const PCM186X_PCM_CFG_TX_WLEN_20: c_uint = 0x02;
pub const PCM186X_PCM_CFG_TX_WLEN_16: c_uint = 0x03;

pub const PCM186X_PCM_CFG_FMT_SHIFT: c_int = 0;
pub const PCM186X_PCM_CFG_FMT_I2S: c_uint = 0x00;
pub const PCM186X_PCM_CFG_FMT_LEFTJ: c_uint = 0x01;
pub const PCM186X_PCM_CFG_FMT_RIGHTJ: c_uint = 0x02;
pub const PCM186X_PCM_CFG_FMT_TDM: c_uint = 0x03;
// PCM186X_TDM_TX_SEL
pub const PCM186X_TDM_TX_SEL_2CH: c_uint = 0x00;
pub const PCM186X_TDM_TX_SEL_4CH: c_uint = 0x01;
pub const PCM186X_TDM_TX_SEL_6CH: c_uint = 0x02;
pub const PCM186X_TDM_TX_SEL_MASK: c_uint = 0x03;
// PCM186X_CLK_CTRL

// PCM186X_PLL_CTRL

// PCM186X_POWER_CTRL

// PCM186X_CLK_STATUS

// PCM186X_SUPPLY_STATUS

// PCM186X_MMAP_STAT_CTRL

