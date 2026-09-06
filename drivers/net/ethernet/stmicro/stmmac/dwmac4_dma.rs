//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac4_dma.h
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
// DWMAC4 DMA Header file.
//
// Copyright (C) 2007-2015  STMicroelectronics Ltd
//
// Author: Alexandre Torgue <alexandre.torgue@st.com>
//
// Define the max channel number used for tx (also rx).
// dwmac4 accepts up to 8 channels for TX (and also 8 channels for RX
//
pub const DMA_CHANNEL_NB_MAX: c_int = 1;
pub const DMA_BUS_MODE: c_uint = 0x00001000;

pub const DMA_BUS_MODE_INTM_MODE1: c_uint = 0x1;

pub const DMA_SYS_BUS_MODE: c_uint = 0x00001004;

pub const DMA_STATUS: c_uint = 0x00001008;
pub const DMA_AXI_BUS_MODE: c_uint = 0x00001028;

pub const DMA_TBS_CTRL: c_uint = 0x00001050;

// Following DMA defines are channel-oriented
pub const DMA_CHAN_BASE_ADDR: c_uint = 0x00001100;
pub const DMA_CHAN_BASE_OFFSET: c_uint = 0x80;

// DMA default interrupt mask for 4.00

// DMA default interrupt mask for 4.10a

// Interrupt status per channel

extern "C" {
    pub fn dwmac4_dma_reset(ioaddr: *mut void __iomem) -> c_int;
}
