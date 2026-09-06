//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sizes.h
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
// include/linux/sizes.h
//

pub const SZ_1: c_uint = 0x00000001;
pub const SZ_2: c_uint = 0x00000002;
pub const SZ_4: c_uint = 0x00000004;
pub const SZ_8: c_uint = 0x00000008;
pub const SZ_16: c_uint = 0x00000010;
pub const SZ_32: c_uint = 0x00000020;
pub const SZ_64: c_uint = 0x00000040;
pub const SZ_128: c_uint = 0x00000080;
pub const SZ_256: c_uint = 0x00000100;
pub const SZ_512: c_uint = 0x00000200;
pub const SZ_1K: c_uint = 0x00000400;
pub const SZ_2K: c_uint = 0x00000800;
pub const SZ_4K: c_uint = 0x00001000;
pub const SZ_8K: c_uint = 0x00002000;
pub const SZ_16K: c_uint = 0x00004000;
pub const SZ_24K: c_uint = 0x00006000;
pub const SZ_32K: c_uint = 0x00008000;
pub const SZ_64K: c_uint = 0x00010000;
pub const SZ_128K: c_uint = 0x00020000;
pub const SZ_192K: c_uint = 0x00030000;
pub const SZ_256K: c_uint = 0x00040000;
pub const SZ_384K: c_uint = 0x00060000;
pub const SZ_512K: c_uint = 0x00080000;
pub const SZ_1M: c_uint = 0x00100000;
pub const SZ_2M: c_uint = 0x00200000;
pub const SZ_3M: c_uint = 0x00300000;
pub const SZ_4M: c_uint = 0x00400000;
pub const SZ_6M: c_uint = 0x00600000;
pub const SZ_8M: c_uint = 0x00800000;
pub const SZ_12M: c_uint = 0x00c00000;
pub const SZ_16M: c_uint = 0x01000000;
pub const SZ_18M: c_uint = 0x01200000;
pub const SZ_24M: c_uint = 0x01800000;
pub const SZ_32M: c_uint = 0x02000000;
pub const SZ_64M: c_uint = 0x04000000;
pub const SZ_128M: c_uint = 0x08000000;
pub const SZ_256M: c_uint = 0x10000000;
pub const SZ_512M: c_uint = 0x20000000;
pub const SZ_1G: c_uint = 0x40000000;
pub const SZ_2G: c_uint = 0x80000000;

