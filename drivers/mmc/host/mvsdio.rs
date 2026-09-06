//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/host/mvsdio.h
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
// Copyright (C) 2008 Marvell Semiconductors, All Rights Reserved.
//
// Clock rates
//
pub const MVSD_CLOCKRATE_MAX: c_int = 50000000;
pub const MVSD_BASE_DIV_MAX: c_uint = 0x7ff;
//
// Register offsets
//
pub const MVSD_SYS_ADDR_LOW: c_uint = 0x000;
pub const MVSD_SYS_ADDR_HI: c_uint = 0x004;
pub const MVSD_BLK_SIZE: c_uint = 0x008;
pub const MVSD_BLK_COUNT: c_uint = 0x00c;
pub const MVSD_ARG_LOW: c_uint = 0x010;
pub const MVSD_ARG_HI: c_uint = 0x014;
pub const MVSD_XFER_MODE: c_uint = 0x018;
pub const MVSD_CMD: c_uint = 0x01c;

pub const MVSD_RSP0: c_uint = 0x020;
pub const MVSD_RSP1: c_uint = 0x024;
pub const MVSD_RSP2: c_uint = 0x028;
pub const MVSD_RSP3: c_uint = 0x02c;
pub const MVSD_RSP4: c_uint = 0x030;
pub const MVSD_RSP5: c_uint = 0x034;
pub const MVSD_RSP6: c_uint = 0x038;
pub const MVSD_RSP7: c_uint = 0x03c;
pub const MVSD_FIFO: c_uint = 0x040;
pub const MVSD_RSP_CRC7: c_uint = 0x044;
pub const MVSD_HW_STATE: c_uint = 0x048;
pub const MVSD_HOST_CTRL: c_uint = 0x050;
pub const MVSD_BLK_GAP_CTRL: c_uint = 0x054;
pub const MVSD_CLK_CTRL: c_uint = 0x058;
pub const MVSD_SW_RESET: c_uint = 0x05c;
pub const MVSD_NOR_INTR_STATUS: c_uint = 0x060;
pub const MVSD_ERR_INTR_STATUS: c_uint = 0x064;
pub const MVSD_NOR_STATUS_EN: c_uint = 0x068;
pub const MVSD_ERR_STATUS_EN: c_uint = 0x06c;
pub const MVSD_NOR_INTR_EN: c_uint = 0x070;
pub const MVSD_ERR_INTR_EN: c_uint = 0x074;
pub const MVSD_AUTOCMD12_ERR_STATUS: c_uint = 0x078;
pub const MVSD_CURR_BYTE_LEFT: c_uint = 0x07c;
pub const MVSD_CURR_BLK_LEFT: c_uint = 0x080;
pub const MVSD_AUTOCMD12_ARG_LOW: c_uint = 0x084;
pub const MVSD_AUTOCMD12_ARG_HI: c_uint = 0x088;
pub const MVSD_AUTOCMD12_CMD: c_uint = 0x08c;

pub const MVSD_AUTO_RSP0: c_uint = 0x090;
pub const MVSD_AUTO_RSP1: c_uint = 0x094;
pub const MVSD_AUTO_RSP2: c_uint = 0x098;
pub const MVSD_CLK_DIV: c_uint = 0x128;

//
// MVSD_CMD
//

//
// MVSD_AUTOCMD12_CMD
//

//
// MVSD_XFER_MODE
//

//
// MVSD_HOST_CTRL
//

pub const MVSD_HOST_CTRL_TMOUT_MAX: c_uint = 0xf;

//
// MVSD_SW_RESET
//

//
// Normal interrupt status bits
//

//
// Error status bits
//

//
// CMD12 error status bits
//

