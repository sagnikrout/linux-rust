//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/atp870u.h
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

// I/O Port
pub const MAX_CDB: c_int = 12;
pub const MAX_SENSE: c_int = 14;
pub const qcnt: c_int = 32;
pub const ATP870U_SCATTER: c_int = 128;
pub const MAX_ADAPTER: c_int = 8;
pub const MAX_SCSI_ID: c_int = 16;
pub const ATP870U_MAX_SECTORS: c_int = 128;
pub const ATP885_DEVID: c_uint = 0x808A;
pub const ATP880_DEVID1: c_uint = 0x8080;
pub const ATP880_DEVID2: c_uint = 0x8081;
// #define ED_DBGP
