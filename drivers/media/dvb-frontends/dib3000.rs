//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/dib3000.h
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
// public header file of the frontend drivers for mobile DVB-T demodulators
// DiBcom 3000M-B and DiBcom 3000P/M-C (http://www.dibcom.fr/)
//
// Copyright (C) 2004-5 Patrick Boettcher (patrick.boettcher@posteo.de)
//
// based on GPL code from DibCom, which has
//
// Copyright (C) 2004 Amaury Demol for DiBcom
//
// Acknowledgements
//
// Amaury Demol from DiBcom for providing specs and driver
// sources, on which this driver (and the dvb-dibusb) are based.
//
// see Documentation/driver-api/media/drivers/dvb-usb.rst for more information
//

// the demodulator's i2c address
// pid and transfer handling is done in the demodulator

