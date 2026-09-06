//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kasan-checks.h
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
// The annotations present in this file are only relevant for the software
// KASAN modes that rely on compiler instrumentation, and will be optimized
// away for the hardware tag-based KASAN mode. Use kasan_check_byte() instead.
//
// __kasan_check_*: Always available when KASAN is enabled. This may be used
// even in compilation units that selectively disable KASAN, but must use KASAN
// to validate access to an address.   Never use these in header files!
//

extern "C" {
    pub fn __kasan_check_read(p: *const volatile void, size: c_uint) -> bool;
}
extern "C" {
    pub fn __kasan_check_write(p: *const volatile void, size: c_uint) -> bool;
}

//
// kasan_check_*: Only available when the particular compilation unit has KASAN
// instrumentation enabled. May be used in header files.
//

