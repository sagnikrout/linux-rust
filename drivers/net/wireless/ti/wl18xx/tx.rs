//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl18xx/tx.h
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
// This file is part of wl18xx
//
// Copyright (C) 2011 Texas Instruments. All rights reserved.
//

pub const WL18XX_TX_HW_BLOCK_SPARE: c_int = 1;
// for special cases - namely, TKIP and GEM
pub const WL18XX_TX_HW_EXTRA_BLOCK_SPARE: c_int = 2;
pub const WL18XX_TX_HW_BLOCK_SIZE: c_int = 268;
pub const WL18XX_TX_STATUS_DESC_ID_MASK: c_uint = 0x7F;
pub const WL18XX_TX_STATUS_STAT_BIT_IDX: c_int = 7;
// Indicates this TX HW frame is not padded to SDIO block size

//
// The FW uses a special bit to indicate a wide channel should be used in
// the rate policy.
//

extern "C" {
    pub fn wl18xx_tx_immediate_complete(wl: *mut wl1271);
}
