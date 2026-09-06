//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/comedi/drivers/addi_tcw.h
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
//
// Following are the generic definitions for the ADDI-DATA timer/counter
// watchdog (TCW) registers and bits. Some of the registers are not used
// depending on the use of the TCW.
//
pub const ADDI_TCW_VAL_REG: c_uint = 0x00;
pub const ADDI_TCW_SYNC_REG: c_uint = 0x00;

pub const ADDI_TCW_RELOAD_REG: c_uint = 0x04;
pub const ADDI_TCW_TIMEBASE_REG: c_uint = 0x08;
pub const ADDI_TCW_CTRL_REG: c_uint = 0x0c;

pub const ADDI_TCW_STATUS_REG: c_uint = 0x10;

pub const ADDI_TCW_IRQ_REG: c_uint = 0x14;

pub const ADDI_TCW_WARN_TIMEVAL_REG: c_uint = 0x18;
pub const ADDI_TCW_WARN_TIMEBASE_REG: c_uint = 0x1c;
