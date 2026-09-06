//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/usb-musb-ux500.h
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
// Copyright (C) ST-Ericsson SA 2011
//
// Author: Mian Yousaf Kaukab <mian.yousaf.kaukab@stericsson.com>
//

pub const UX500_MUSB_DMA_NUM_RX_TX_CHANNELS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ux500_musb_board_data {
    pub dma_rx_param_array: *mut c_void,
    pub dma_tx_param_array: *mut c_void,
    pub filter_param): *mut *mut *mut bool (dma_filter)(struct dma_chan chan, void,
}
