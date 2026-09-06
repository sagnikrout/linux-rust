//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/retu.h
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


//
// Retu/Tahvo MFD driver interface
//
// This file is subject to the terms and conditions of the GNU General
// Public License. See the file "COPYING" in the main directory of this
// archive for more details.
//
extern "C" {
    pub fn retu_read(: *mut retu_dev, _arg: u8) -> c_int;
}
extern "C" {
    pub fn retu_write(: *mut retu_dev, _arg: u8, _arg: u16) -> c_int;
}
// Registers
pub const RETU_REG_WATCHDOG: c_uint = 0x17		/* Watchdog */;
pub const RETU_REG_CC1: c_uint = 0x0d		/* Common control register 1 */;
pub const RETU_REG_STATUS: c_uint = 0x16		/* Status register */;
// Interrupt sources

// Interrupt status

