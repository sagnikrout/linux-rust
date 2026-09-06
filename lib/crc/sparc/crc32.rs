//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crc/sparc/crc32.h
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
// CRC32c (Castagnoli), sparc64 crc32c opcode accelerated
//
// This is based largely upon arch/x86/crypto/crc32c-intel.c
//
// Copyright (C) 2008 Intel Corporation
// Authors: Austin Zhang <austin_zhang@linux.intel.com>
// Kent Liu <kent.liu@intel.com>
//

extern "C" {
    pub fn DEFINE_STATIC_KEY_FALSE(_arg: have_crc32c_opcode) -> static __ro_after_init;
}

extern "C" {
    pub fn crc32c_sparc64(crcp: *mut u32, data: *const u64, len: usize);
}
extern "C" {
    pub fn crc32c_base(_arg: crc, _arg: data, _arg: len) -> return;
}
// Data isn't 8-byte aligned.  Align it.

extern "C" {
    pub fn __volatile__(%%asr26: "rd, (cfr): %0" : "=r") -> __asm__;
}
