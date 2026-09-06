//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/cs4271.h
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
// Definitions for CS4271 ASoC codec driver
//
// Copyright (c) 2010 Alexander Sverdlin <subaparts@yandex.ru>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs4271_platform_data {
    pub /: *mut *mut bool amutec_eq_bmutec; / flag to enable AMUTEC=BMUTEC,
//
// The CS4271 requires its LRCLK and MCLK to be stable before its RESET
// line is de-asserted. That also means that clocks cannot be changed
// without putting the chip back into hardware reset, which also requires
// a complete re-initialization of all registers.
//
// One (undocumented) workaround is to assert and de-assert the PDN bit
// in the MODE2 register. This workaround can be enabled with the
// following flag.
//
// Note that this is not needed in case the clocks are stable
// throughout the entire runtime of the codec.
//
    pub enable_soft_reset: bool,
}
