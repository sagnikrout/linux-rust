//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/errata_list.h
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
// Copyright (C) 2021 Sifive.
//

//
// _val is marked as "will be overwritten", so need to set it to 0
// in the default case.
//
pub const ALT_SVPBMT_SHIFT: c_int = 61;
pub const ALT_THEAD_MAE_SHIFT: c_int = 59;

//
// IO/NOCACHE memory types are handled together with svpbmt,
// so on T-Head chips, check if no other memory type is set,
// and set the non-0 PMA type if applicable.
//

// Macro flag: #define ALT_THEAD_PMA(_val)

pub const THEAD_C9XX_RV_IRQ_PMU: c_int = 17;
pub const THEAD_C9XX_CSR_SCOUNTEROF: c_uint = 0x5c5;

