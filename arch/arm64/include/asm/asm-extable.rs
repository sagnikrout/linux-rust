//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/asm-extable.h
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

pub const EX_TYPE_NONE: c_int = 0;
pub const EX_TYPE_BPF: c_int = 1;
pub const EX_TYPE_UACCESS_ERR_ZERO: c_int = 2;
pub const EX_TYPE_KACCESS_ERR_ZERO: c_int = 3;
pub const EX_TYPE_UACCESS_CPY: c_int = 4;
pub const EX_TYPE_LOAD_UNALIGNED_ZEROPAD: c_int = 5;
// Data fields for EX_TYPE_UACCESS_ERR_ZERO
pub const EX_DATA_REG_ERR_SHIFT: c_int = 0;

pub const EX_DATA_REG_ZERO_SHIFT: c_int = 5;

// Data fields for EX_TYPE_LOAD_UNALIGNED_ZEROPAD
pub const EX_DATA_REG_DATA_SHIFT: c_int = 0;

pub const EX_DATA_REG_ADDR_SHIFT: c_int = 5;

// Data fields for EX_TYPE_UACCESS_CPY

//
// Create an exception table entry for uaccess `insn`, which will branch to `fixup`
// when an unhandled fault is taken.
//
// Create an exception table entry for `insn` if `fixup` is provided. Otherwise
// do nothing.
//

