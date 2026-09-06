//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/tables.h
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
pub const B43_TAB_ROTOR_SIZE: c_int = 53;
pub const B43_TAB_RETARD_SIZE: c_int = 53;
pub const B43_TAB_FINEFREQA_SIZE: c_int = 256;
pub const B43_TAB_FINEFREQG_SIZE: c_int = 256;
pub const B43_TAB_NOISEA2_SIZE: c_int = 8;
pub const B43_TAB_NOISEA3_SIZE: c_int = 8;
pub const B43_TAB_NOISEG1_SIZE: c_int = 8;
pub const B43_TAB_NOISEG2_SIZE: c_int = 8;
pub const B43_TAB_NOISESCALE_SIZE: c_int = 27;
pub const B43_TAB_SIGMASQR_SIZE: c_int = 53;
pub const B43_TAB_RSSIAGC1_SIZE: c_int = 16;
pub const B43_TAB_RSSIAGC2_SIZE: c_int = 48;
