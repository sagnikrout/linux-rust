//! Automatically rewritten from C Header to Rust Module
//! Source: tools/objtool/arch/loongarch/include/arch/cfi_regs.h
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
pub const CFI_RA: c_int = 1;
pub const CFI_SP: c_int = 3;
pub const CFI_A0: c_int = 4;
pub const CFI_FP: c_int = 22;
pub const CFI_S0: c_int = 23;
pub const CFI_S1: c_int = 24;
pub const CFI_S2: c_int = 25;
pub const CFI_S3: c_int = 26;
pub const CFI_S4: c_int = 27;
pub const CFI_S5: c_int = 28;
pub const CFI_S6: c_int = 29;
pub const CFI_S7: c_int = 30;
pub const CFI_S8: c_int = 31;
pub const CFI_NUM_REGS: c_int = 32;

