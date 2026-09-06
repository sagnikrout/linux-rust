//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/const.h
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
// const.h: Macros for dealing with constants.
// Some constant macros are used in both assembler and
// C code.  Therefore we cannot annotate them always with
// 'UL' and other type specifiers unilaterally.  We
// use the following macros to deal with this.
//
// Similarly, _AT() will cast an expression with a type in C, but
// leave it unchanged in asm.
//

//
// Missing asm support
//
// __BIT128() would not work in the asm code, as it shifts an
// 'unsigned __int128' data type as direct representation of
// 128 bit constants is not supported in the gcc compiler, as
// they get silently truncated.
//
// TODO: Please revisit this implementation when gcc compiler
// starts representing 128 bit constants directly like long
// and unsigned long etc. Subsequently drop the comment for
// GENMASK_U128() which would then start supporting asm code.
//

//
// Divide positive or negative dividend by positive or negative divisor
// and round to closest integer. Result is undefined for negative
// divisors if the dividend variable type is unsigned and for negative
// dividends if the divisor variable type is unsigned.
//

