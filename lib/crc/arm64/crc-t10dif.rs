//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crc/arm64/crc-t10dif.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Accelerated CRC-T10DIF using arm64 NEON and Crypto Extensions instructions
//
// Copyright (C) 2016 - 2017 Linaro Ltd <ard.biesheuvel@linaro.org>
//

extern "C" {
    pub fn DEFINE_STATIC_KEY_FALSE(_arg: have_asimd) -> static __ro_after_init;
}
extern "C" {
    pub fn DEFINE_STATIC_KEY_FALSE(_arg: have_pmull) -> static __ro_after_init;
}

extern "C" {
    pub fn crc_t10dif_pmull_p64(init_crc: u16, buf: *const u8, len: usize) -> asmlinkage u16;
}
extern "C" {
    pub fn crc_t10dif_pmull_p64(_arg: crc, _arg: data, _arg: length) -> return;
}
extern "C" {
    pub fn crc_t10dif_generic(_arg: 0, _arg: buf, _arg: sizeof(buf)) -> return;
}
extern "C" {
    pub fn crc_t10dif_generic(_arg: crc, _arg: data, _arg: length) -> return;
}
