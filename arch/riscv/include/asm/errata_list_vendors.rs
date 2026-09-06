//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/errata_list_vendors.h
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

pub const ERRATA_ANDES_NO_IOCP: c_int = 0;
pub const ERRATA_ANDES_NUMBER: c_int = 1;

pub const ERRATA_SIFIVE_CIP_453: c_int = 0;
pub const ERRATA_SIFIVE_CIP_1200: c_int = 1;
pub const ERRATA_SIFIVE_NUMBER: c_int = 2;

pub const ERRATA_THEAD_MAE: c_int = 0;
pub const ERRATA_THEAD_PMU: c_int = 1;
pub const ERRATA_THEAD_GHOSTWRITE: c_int = 2;
pub const ERRATA_THEAD_NUMBER: c_int = 3;

pub const ERRATA_MIPS_P8700_PAUSE_OPCODE: c_int = 0;
pub const ERRATA_MIPS_NUMBER: c_int = 1;

