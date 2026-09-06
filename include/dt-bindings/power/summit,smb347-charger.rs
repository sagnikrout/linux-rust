//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/summit,smb347-charger.h
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


// SPDX-License-Identifier: (GPL-2.0-or-later OR MIT)
//
// Author: David Heidelberg <david@ixit.cz>
//
// Charging compensation method
pub const SMB3XX_SOFT_TEMP_COMPENSATE_NONE: c_int = 0;
pub const SMB3XX_SOFT_TEMP_COMPENSATE_CURRENT: c_int = 1;
pub const SMB3XX_SOFT_TEMP_COMPENSATE_VOLTAGE: c_int = 2;
// Charging enable control
pub const SMB3XX_CHG_ENABLE_SW: c_int = 0;
pub const SMB3XX_CHG_ENABLE_PIN_ACTIVE_LOW: c_int = 1;
pub const SMB3XX_CHG_ENABLE_PIN_ACTIVE_HIGH: c_int = 2;
// Polarity of INOK signal
pub const SMB3XX_SYSOK_INOK_ACTIVE_LOW: c_int = 0;
pub const SMB3XX_SYSOK_INOK_ACTIVE_HIGH: c_int = 1;
