//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/framer/pef2256.h
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
// PEF2256 consumer API
//
// Copyright 2023 CS GROUP France
//
// Author: Herve Codina <herve.codina@bootlin.com>
//

// Retrieve the PEF2256 regmap
// PEF2256 hardware versions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pef2256_version {
    PEF2256_VERSION_UNKNOWN,
    PEF2256_VERSION_1_2,
    PEF2256_VERSION_2_1,
    PEF2256_VERSION_2_2,
}

// Get the PEF2256 hardware version
extern "C" {
    pub fn pef2256_get_version(pef2256: *mut pef2256) -> pef2256_version;
}
