//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/sound/cs35l32.h
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
pub const CS35L32_BOOST_MGR_AUTO: c_int = 0;
pub const CS35L32_BOOST_MGR_AUTO_AUDIO: c_int = 1;
pub const CS35L32_BOOST_MGR_BYPASS: c_int = 2;
pub const CS35L32_BOOST_MGR_FIXED: c_int = 3;
pub const CS35L32_DATA_CFG_LR_VP: c_int = 0;
pub const CS35L32_DATA_CFG_LR_STAT: c_int = 1;
pub const CS35L32_DATA_CFG_LR: c_int = 2;
pub const CS35L32_DATA_CFG_LR_VPSTAT: c_int = 3;
pub const CS35L32_BATT_THRESH_3_1V: c_int = 0;
pub const CS35L32_BATT_THRESH_3_2V: c_int = 1;
pub const CS35L32_BATT_THRESH_3_3V: c_int = 2;
pub const CS35L32_BATT_THRESH_3_4V: c_int = 3;
pub const CS35L32_BATT_RECOV_3_1V: c_int = 0;
pub const CS35L32_BATT_RECOV_3_2V: c_int = 1;
pub const CS35L32_BATT_RECOV_3_3V: c_int = 2;
pub const CS35L32_BATT_RECOV_3_4V: c_int = 3;
pub const CS35L32_BATT_RECOV_3_5V: c_int = 4;
pub const CS35L32_BATT_RECOV_3_6V: c_int = 5;
