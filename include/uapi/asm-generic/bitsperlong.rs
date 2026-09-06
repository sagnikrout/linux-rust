//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/asm-generic/bitsperlong.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// In order to keep safe and avoid regression, only unify uapi
// bitsperlong.h for some archs which are using newer toolchains
// that have the definitions of __CHAR_BIT__ and __SIZEOF_LONG__.
// See the following link for more info:
// https://lore.kernel.org/linux-arch/b9624545-2c80-49a1-ac3c-39264a591f7b@app.fastmail.com
//

//
// There seems to be no way of detecting this automatically from user
// space, so 64 bit architectures should override this in their
// bitsperlong.h. In particular, an architecture that supports
// both 32 and 64 bit user space must not rely on CONFIG_64BIT
// to decide it, but rather check a compiler provided macro.
//
pub const __BITS_PER_LONG: c_int = 32;

pub const __BITS_PER_LONG_LONG: c_int = 64;

