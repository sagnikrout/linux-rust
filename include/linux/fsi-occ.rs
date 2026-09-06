//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fsi-occ.h
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
pub const OCC_RESP_CMD_IN_PRG: c_uint = 0xFF;
pub const OCC_RESP_SUCCESS: c_int = 0;
pub const OCC_RESP_CMD_INVAL: c_uint = 0x11;
pub const OCC_RESP_CMD_LEN_INVAL: c_uint = 0x12;
pub const OCC_RESP_DATA_INVAL: c_uint = 0x13;
pub const OCC_RESP_CHKSUM_ERR: c_uint = 0x14;
pub const OCC_RESP_INT_ERR: c_uint = 0x15;
pub const OCC_RESP_BAD_STATE: c_uint = 0x16;
pub const OCC_RESP_CRIT_EXCEPT: c_uint = 0xE0;
pub const OCC_RESP_CRIT_INIT: c_uint = 0xE1;
pub const OCC_RESP_CRIT_WATCHDOG: c_uint = 0xE2;
pub const OCC_RESP_CRIT_OCB: c_uint = 0xE3;
pub const OCC_RESP_CRIT_HW: c_uint = 0xE4;
pub const OCC_MAX_RESP_WORDS: c_int = 2048;
