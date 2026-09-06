//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl1251/spi.h
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
// This file is part of wl1251
//
// Copyright (c) 1998-2007 Texas Instruments Incorporated
// Copyright (C) 2008 Nokia Corporation
//

pub const WSPI_CMD_READ: c_uint = 0x40000000;
pub const WSPI_CMD_WRITE: c_uint = 0x00000000;
pub const WSPI_CMD_FIXED: c_uint = 0x20000000;
pub const WSPI_CMD_BYTE_LENGTH: c_uint = 0x1FFE0000;
pub const WSPI_CMD_BYTE_LENGTH_OFFSET: c_int = 17;
pub const WSPI_CMD_BYTE_ADDR: c_uint = 0x0001FFFF;
pub const WSPI_INIT_CMD_CRC_LEN: c_int = 5;
pub const WSPI_INIT_CMD_START: c_uint = 0x00;
pub const WSPI_INIT_CMD_TX: c_uint = 0x40;
// the extra bypass bit is sampled by the TNET as '1'
pub const WSPI_INIT_CMD_BYPASS_BIT: c_uint = 0x80;
pub const WSPI_INIT_CMD_FIXEDBUSY_LEN: c_uint = 0x07;
pub const WSPI_INIT_CMD_EN_FIXEDBUSY: c_uint = 0x80;
pub const WSPI_INIT_CMD_DIS_FIXEDBUSY: c_uint = 0x00;
pub const WSPI_INIT_CMD_IOD: c_uint = 0x40;
pub const WSPI_INIT_CMD_IP: c_uint = 0x20;
pub const WSPI_INIT_CMD_CS: c_uint = 0x10;
pub const WSPI_INIT_CMD_WS: c_uint = 0x08;
pub const WSPI_INIT_CMD_WSPI: c_uint = 0x01;
pub const WSPI_INIT_CMD_END: c_uint = 0x01;
pub const WSPI_INIT_CMD_LEN: c_int = 8;

pub const HW_ACCESS_WSPI_INIT_CMD_MASK: c_int = 0;
