//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/mt352.h
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
// Driver for Zarlink DVB-T MT352 demodulator
//
// Written by Holger Waechtler <holger@qanu.de>
// and Daniel Mack <daniel@qanu.de>
//
// AVerMedia AVerTV DVB-T 771 support by
// Wolfram Joost <dbox2@frokaschwei.de>
//
// Support for Samsung TDTC9251DH01C(M) tuner
// Copyright (C) 2004 Antonio Mancuso <antonio.mancuso@digitaltelevision.it>
// Amauri  Celani  <acelani@essegi.net>
//
// DVICO FusionHDTV DVB-T1 and DVICO FusionHDTV DVB-T Lite support by
// Christopher Pascoe <c.pascoe@itee.uq.edu.au>
//

// the demodulator's i2c address
// frequencies in kHz
// set if no pll is connected to the secondary i2c bus
// Initialise the demodulator and PLL. Cannot be NULL

