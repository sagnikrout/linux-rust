//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crypto/riscv/sha512.h
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
// SHA-512 and SHA-384 using the RISC-V vector crypto extensions
//
// Copyright (C) 2023 VRULL GmbH
// Author: Heiko Stuebner <heiko.stuebner@vrull.eu>
//
// Copyright (C) 2023 SiFive, Inc.
// Author: Jerry Shih <jerry.shih@sifive.com>
//

extern "C" {
    pub fn DEFINE_STATIC_KEY_FALSE(_arg: have_extensions) -> static __ro_after_init;
}
