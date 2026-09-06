//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/sigp.h
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
// SIGP order codes
pub const SIGP_SENSE: c_int = 1;
pub const SIGP_EXTERNAL_CALL: c_int = 2;
pub const SIGP_EMERGENCY_SIGNAL: c_int = 3;
pub const SIGP_START: c_int = 4;
pub const SIGP_STOP: c_int = 5;
pub const SIGP_RESTART: c_int = 6;
pub const SIGP_STOP_AND_STORE_STATUS: c_int = 9;
pub const SIGP_INITIAL_CPU_RESET: c_int = 11;
pub const SIGP_CPU_RESET: c_int = 12;
pub const SIGP_SET_PREFIX: c_int = 13;
pub const SIGP_STORE_STATUS_AT_ADDRESS: c_int = 14;
pub const SIGP_SET_ARCHITECTURE: c_int = 18;
pub const SIGP_COND_EMERGENCY_SIGNAL: c_int = 19;
pub const SIGP_SENSE_RUNNING: c_int = 21;
pub const SIGP_SET_MULTI_THREADING: c_int = 22;
pub const SIGP_STORE_ADDITIONAL_STATUS: c_int = 23;
// SIGP condition codes
pub const SIGP_CC_ORDER_CODE_ACCEPTED: c_int = 0;
pub const SIGP_CC_STATUS_STORED: c_int = 1;
pub const SIGP_CC_BUSY: c_int = 2;
pub const SIGP_CC_NOT_OPERATIONAL: c_int = 3;
// SIGP cpu status bits
pub const SIGP_STATUS_INVALID_ORDER: c_uint = 0x00000002UL;
pub const SIGP_STATUS_CHECK_STOP: c_uint = 0x00000010UL;
pub const SIGP_STATUS_STOPPED: c_uint = 0x00000040UL;
pub const SIGP_STATUS_EXT_CALL_PENDING: c_uint = 0x00000080UL;
pub const SIGP_STATUS_INVALID_PARAMETER: c_uint = 0x00000100UL;
pub const SIGP_STATUS_INCORRECT_STATE: c_uint = 0x00000200UL;
pub const SIGP_STATUS_NOT_RUNNING: c_uint = 0x00000400UL;

// status = r1.even;
extern "C" {
    pub fn CC_TRANSFORM(_arg: cc) -> return;
}
// status = _status;

