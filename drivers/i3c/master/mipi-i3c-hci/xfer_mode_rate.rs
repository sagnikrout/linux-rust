//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/i3c/master/mipi-i3c-hci/xfer_mode_rate.h
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
// Transfer Mode/Rate Table definitions as found in extended capability
// sections 0x04 and 0x08.
// This applies starting from I3C HCI v2.0.
//
// Master Transfer Mode Table Fixed Indexes.
//
// Indexes 0x0 and 0x8 are mandatory. Availability for the rest must be
// obtained from the mode table in the extended capability area.
// Presence and definitions for indexes beyond these ones may vary.
//
pub const XFERMODE_IDX_I3C_SDR: c_uint = 0x00	/* I3C SDR Mode */;
pub const XFERMODE_IDX_I3C_HDR_DDR: c_uint = 0x01	/* I3C HDR-DDR Mode */;
pub const XFERMODE_IDX_I3C_HDR_T: c_uint = 0x02	/* I3C HDR-Ternary Mode */;
pub const XFERMODE_IDX_I3C_HDR_BT: c_uint = 0x03	/* I3C HDR-BT Mode */;
pub const XFERMODE_IDX_I2C: c_uint = 0x08	/* Legacy I2C Mode */;
//
// Transfer Mode Table Entry Bits Definitions
//

//
// Master Data Transfer Rate Selector Values.
//
// These are the values to be used in the command descriptor XFER_RATE field
// and found in the RATE_ID field below.
// The I3C_SDR0, I3C_SDR1, I3C_SDR2, I3C_SDR3, I3C_SDR4 and I2C_FM rates
// are required, everything else is optional and discoverable in the
// Data Transfer Rate Table. Indicated are typical rates. The actual
// rates may vary slightly and are also specified in the Data Transfer
// Rate Table.
//
pub const XFERRATE_I3C_SDR0: c_uint = 0x00	/* 12.5 MHz */;
pub const XFERRATE_I3C_SDR1: c_uint = 0x01	/* 8 MHz */;
pub const XFERRATE_I3C_SDR2: c_uint = 0x02	/* 6 MHz */;
pub const XFERRATE_I3C_SDR3: c_uint = 0x03	/* 4 MHz */;
pub const XFERRATE_I3C_SDR4: c_uint = 0x04	/* 2 MHz */;
pub const XFERRATE_I3C_SDR_FM_FMP: c_uint = 0x05	/* 400 KHz / 1 MHz */;
pub const XFERRATE_I3C_SDR_USER6: c_uint = 0x06	/* User Defined */;
pub const XFERRATE_I3C_SDR_USER7: c_uint = 0x07	/* User Defined */;
pub const XFERRATE_I2C_FM: c_uint = 0x00	/* 400 KHz */;
pub const XFERRATE_I2C_FMP: c_uint = 0x01	/* 1 MHz */;
pub const XFERRATE_I2C_USER2: c_uint = 0x02	/* User Defined */;
pub const XFERRATE_I2C_USER3: c_uint = 0x03	/* User Defined */;
pub const XFERRATE_I2C_USER4: c_uint = 0x04	/* User Defined */;
pub const XFERRATE_I2C_USER5: c_uint = 0x05	/* User Defined */;
pub const XFERRATE_I2C_USER6: c_uint = 0x06	/* User Defined */;
pub const XFERRATE_I2C_USER7: c_uint = 0x07	/* User Defined */;
//
// Master Data Transfer Rate Table Mode ID values.
//
pub const XFERRATE_MODE_I3C: c_uint = 0x00;
pub const XFERRATE_MODE_I2C: c_uint = 0x08;
//
// Master Data Transfer Rate Table Entry Bits Definitions
//

