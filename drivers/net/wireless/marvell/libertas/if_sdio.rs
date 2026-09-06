//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/libertas/if_sdio.h
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
// linux/drivers/net/wireless/libertas/if_sdio.h
//
// Copyright 2007 Pierre Ossman
//
pub const IF_SDIO_IOPORT: c_uint = 0x00;
pub const IF_SDIO_H_INT_MASK: c_uint = 0x04;
pub const IF_SDIO_H_INT_OFLOW: c_uint = 0x08;
pub const IF_SDIO_H_INT_UFLOW: c_uint = 0x04;
pub const IF_SDIO_H_INT_DNLD: c_uint = 0x02;
pub const IF_SDIO_H_INT_UPLD: c_uint = 0x01;
pub const IF_SDIO_H_INT_STATUS: c_uint = 0x05;
pub const IF_SDIO_H_INT_RSR: c_uint = 0x06;
pub const IF_SDIO_H_INT_STATUS2: c_uint = 0x07;
pub const IF_SDIO_RD_BASE: c_uint = 0x10;
pub const IF_SDIO_STATUS: c_uint = 0x20;
pub const IF_SDIO_IO_RDY: c_uint = 0x08;
pub const IF_SDIO_CIS_RDY: c_uint = 0x04;
pub const IF_SDIO_UL_RDY: c_uint = 0x02;
pub const IF_SDIO_DL_RDY: c_uint = 0x01;
pub const IF_SDIO_C_INT_MASK: c_uint = 0x24;
pub const IF_SDIO_C_INT_STATUS: c_uint = 0x28;
pub const IF_SDIO_C_INT_RSR: c_uint = 0x2C;
pub const IF_SDIO_SCRATCH: c_uint = 0x34;
pub const IF_SDIO_SCRATCH_OLD: c_uint = 0x80fe;
pub const IF_SDIO_FW_STATUS: c_uint = 0x40;
pub const IF_SDIO_FIRMWARE_OK: c_uint = 0xfedc;
pub const IF_SDIO_RX_LEN: c_uint = 0x42;
pub const IF_SDIO_RX_UNIT: c_uint = 0x43;
pub const IF_SDIO_EVENT: c_uint = 0x80fc;
pub const IF_SDIO_BLOCK_SIZE: c_int = 256;
pub const CONFIGURATION_REG: c_uint = 0x03;

