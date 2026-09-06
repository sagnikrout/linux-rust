//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/entry/vdso/vdso32/fake_32bit_build.h
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
// in case of a 32 bit VDSO for a 64 bit kernel fake a 32 bit kernel
// configuration
//

pub const CONFIG_X86_32: c_int = 1;
pub const CONFIG_PGTABLE_LEVELS: c_int = 2;
pub const CONFIG_PAGE_OFFSET: c_int = 0;
pub const CONFIG_ILLEGAL_POINTER_VALUE: c_int = 0;
pub const CONFIG_NR_CPUS: c_int = 1;
// Macro flag: #define BUILD_VDSO32_64
