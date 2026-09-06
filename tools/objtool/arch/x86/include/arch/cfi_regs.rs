//! Automatically rewritten from C Header to Rust Module
//! Source: tools/objtool/arch/x86/include/arch/cfi_regs.h
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
pub const CFI_AX: c_int = 0;
pub const CFI_CX: c_int = 1;
pub const CFI_DX: c_int = 2;
pub const CFI_BX: c_int = 3;
pub const CFI_SP: c_int = 4;
pub const CFI_BP: c_int = 5;
pub const CFI_SI: c_int = 6;
pub const CFI_DI: c_int = 7;
pub const CFI_R8: c_int = 8;
pub const CFI_R9: c_int = 9;
pub const CFI_R10: c_int = 10;
pub const CFI_R11: c_int = 11;
pub const CFI_R12: c_int = 12;
pub const CFI_R13: c_int = 13;
pub const CFI_R14: c_int = 14;
pub const CFI_R15: c_int = 15;
pub const CFI_RA: c_int = 16;
pub const CFI_NUM_REGS: c_int = 17;
