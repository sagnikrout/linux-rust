//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/lgdt330x.h
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
// Support for LGDT3302 and LGDT3303 - VSB/QAM
//
// Copyright (C) 2005 Wilson Michaels <wilsonmichaels@earthlink.net>
//

//
// struct lgdt330x_config - contains lgdt330x configuration
//
// @demod_chip:		LG demodulator chip LGDT3302 or LGDT3303
// @serial_mpeg:	MPEG hardware interface - 0:parallel 1:serial
// @pll_rf_set:		Callback function to set PLL interface
// @set_ts_params:	Callback function to set device param for start_dma
// @clock_polarity_flip:
// Flip the polarity of the mpeg data transfer clock using alternate
// init data.
// This option applies ONLY to LGDT3303 - 0:disabled (default) 1:enabled
// @get_dvb_frontend:
// returns the frontend associated with this I2C client.
// Filled by the driver.
//

