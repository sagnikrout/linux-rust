//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/stv0299.h
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

pub const STV0299_LOCKOUTPUT_0: c_int = 0;
pub const STV0299_LOCKOUTPUT_1: c_int = 1;
pub const STV0299_LOCKOUTPUT_CF: c_int = 2;
pub const STV0299_LOCKOUTPUT_LK: c_int = 3;
pub const STV0299_VOLT13_OP0: c_int = 0;
pub const STV0299_VOLT13_OP1: c_int = 1;
// the demodulator's i2c address
// inittab - array of pairs of values.
// First of each pair is the register, second is the value.
// List should be terminated with an 0xff, 0xff pair.
//
// master clock to use
// does the inversion require inversion?
// Skip reinitialisation?
// LOCK OUTPUT setting
// Is 13v controlled by OP0 or OP1?
// Turn-off OP0?
// minimum delay before retuning
// Set the symbol rate
// Set device param to start dma

