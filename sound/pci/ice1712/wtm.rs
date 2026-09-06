//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ice1712/wtm.h
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


// SPDX-License-Identifier: GPL-2.0
// ID

pub const VT1724_SUBDEVICE_WTM: c_uint = 0x36495345	/* WT192M ver1.0 */;
//
// chip addresses on I2C bus
//
pub const AK4114_ADDR: c_uint = 0x20	/*S/PDIF receiver*/;
pub const STAC9460_I2C_ADDR: c_uint = 0x54	/* ADC*2 | DAC*6 */;
pub const STAC9460_2_I2C_ADDR: c_uint = 0x56	/* ADC|DAC *2 */;
