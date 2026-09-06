//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/i3c/master/mipi-i3c-hci/cmd.h
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


// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (c) 2020, MIPI Alliance, Inc.
//
// Author: Nicolas Pitre <npitre@baylibre.com>
//
// Common command/response related stuff
//
// Those bits are common to all descriptor formats and
// may be manipulated by the core code.
//

//
// Response Descriptor Structure
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hci_resp_err {
    RESP_SUCCESS			= 0x0,
    RESP_ERR_CRC			= 0x1,
    RESP_ERR_PARITY			= 0x2,
    RESP_ERR_FRAME			= 0x3,
    RESP_ERR_ADDR_HEADER		= 0x4,
    RESP_ERR_BCAST_NACK_7E		= 0x4,
    RESP_ERR_NACK			= 0x5,
    RESP_ERR_OVL			= 0x6,
    RESP_ERR_I3C_SHORT_READ		= 0x7,
    RESP_ERR_HC_TERMINATED		= 0x8,
    RESP_ERR_I2C_WR_DATA_NACK	= 0x9,
    RESP_ERR_BUS_XFER_ABORTED	= 0x9,
    RESP_ERR_NOT_SUPPORTED		= 0xa,
    RESP_ERR_ABORTED_WITH_CRC	= 0xb,
// 0xc to 0xf are reserved for transfer specific errors
}

// TID generation (4 bits wide in all cases)

// This abstracts operations with our command descriptor formats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cmd_ops {
    pub raw): u8 ccc_addr, u8 ccc_cmd, bool,
    pub xfer): *mut hci_xfer,
    pub xfer): *mut hci_xfer,
    pub hci): *mut *mut int (perform_daa)(struct i3c_hci,
}

// Our various instances
// response &= ~RESP_ERR_FIELD;
// response |= FIELD_PREP(RESP_ERR_FIELD, resp_err);
