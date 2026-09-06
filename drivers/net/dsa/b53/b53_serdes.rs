//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/b53/b53_serdes.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Northstar Plus switch SerDes/SGMII PHY definitions
//
// Copyright (C) 2018 Florian Fainelli <f.fainelli@gmail.com>
//

// Non-standard page used to access SerDes PHY registers on NorthStar Plus
pub const B53_SERDES_PAGE: c_uint = 0x16;
pub const B53_SERDES_BLKADDR: c_uint = 0x3e;
pub const B53_SERDES_LANE: c_uint = 0x3c;
pub const B53_SERDES_ID0: c_uint = 0x20;
pub const SERDES_ID0_MODEL_MASK: c_uint = 0x3f;
pub const SERDES_ID0_REV_NUM_SHIFT: c_int = 11;
pub const SERDES_ID0_REV_NUM_MASK: c_uint = 0x7;
pub const SERDES_ID0_REV_LETTER_SHIFT: c_int = 14;

pub const B53_SERDES_DIGITAL_STATUS: c_uint = 0x28;
// SERDES_DIGITAL_CONTROL1

// SERDES_DIGITAL_CONTROL2

// SERDES_DIGITAL_CONTROL3

pub const FIFO_ELAST_TX_RX_SHIFT: c_int = 1;
pub const FIFO_ELAST_TX_RX_5K: c_int = 0;
pub const FIFO_ELAST_TX_RX_10K: c_int = 1;
pub const FIFO_ELAST_TX_RX_13_5K: c_int = 2;
pub const FIFO_ELAST_TX_RX_18_5K: c_int = 3;

// SERDES_DIGITAL_STATUS

pub const SPEED_STATUS_SHIFT: c_int = 3;
pub const SPEED_STATUS_10: c_int = 0;
pub const SPEED_STATUS_100: c_int = 1;
pub const SPEED_STATUS_1000: c_int = 2;
pub const SPEED_STATUS_2500: c_int = 3;

// Block offsets
pub const SERDES_DIGITAL_BLK: c_uint = 0x8300;
pub const SERDES_ID0: c_uint = 0x8310;
pub const SERDES_MII_BLK: c_uint = 0xffe0;
pub const SERDES_XGXSBLK0_BLOCKADDRESS: c_uint = 0xffd0;

extern "C" {
    pub fn b53_serdes_init(dev: *mut b53_device, port: c_int) -> c_int;
}

