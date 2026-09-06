//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/pkru.h
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

pub const PKRU_AD_BIT: c_uint = 0x1u;
pub const PKRU_WD_BIT: c_uint = 0x2u;
pub const PKRU_BITS_PER_PKEY: c_int = 2;

pub const init_pkru_value: c_int = 0;
pub const pkru_get_init_value(): c_int = 0;

//
// Access-disable disables writes too so we need to check
// both bits here.
//
extern "C" {
    pub fn rdpkru() -> return;
}
//
// WRPKRU is relatively expensive compared to RDPKRU.
// Avoid WRPKRU when it would not change the value.
//
