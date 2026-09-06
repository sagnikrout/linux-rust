//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/uapi/asm/reg.h
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
// Various register offset definitions for debuggers, core file
// examiners and whatnot.
//
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//
pub const LOONGARCH_EF_R0: c_int = 0;
pub const LOONGARCH_EF_R1: c_int = 1;
pub const LOONGARCH_EF_R2: c_int = 2;
pub const LOONGARCH_EF_R3: c_int = 3;
pub const LOONGARCH_EF_R4: c_int = 4;
pub const LOONGARCH_EF_R5: c_int = 5;
pub const LOONGARCH_EF_R6: c_int = 6;
pub const LOONGARCH_EF_R7: c_int = 7;
pub const LOONGARCH_EF_R8: c_int = 8;
pub const LOONGARCH_EF_R9: c_int = 9;
pub const LOONGARCH_EF_R10: c_int = 10;
pub const LOONGARCH_EF_R11: c_int = 11;
pub const LOONGARCH_EF_R12: c_int = 12;
pub const LOONGARCH_EF_R13: c_int = 13;
pub const LOONGARCH_EF_R14: c_int = 14;
pub const LOONGARCH_EF_R15: c_int = 15;
pub const LOONGARCH_EF_R16: c_int = 16;
pub const LOONGARCH_EF_R17: c_int = 17;
pub const LOONGARCH_EF_R18: c_int = 18;
pub const LOONGARCH_EF_R19: c_int = 19;
pub const LOONGARCH_EF_R20: c_int = 20;
pub const LOONGARCH_EF_R21: c_int = 21;
pub const LOONGARCH_EF_R22: c_int = 22;
pub const LOONGARCH_EF_R23: c_int = 23;
pub const LOONGARCH_EF_R24: c_int = 24;
pub const LOONGARCH_EF_R25: c_int = 25;
pub const LOONGARCH_EF_R26: c_int = 26;
pub const LOONGARCH_EF_R27: c_int = 27;
pub const LOONGARCH_EF_R28: c_int = 28;
pub const LOONGARCH_EF_R29: c_int = 29;
pub const LOONGARCH_EF_R30: c_int = 30;
pub const LOONGARCH_EF_R31: c_int = 31;
//
// Saved special registers
//
pub const LOONGARCH_EF_ORIG_A0: c_int = 32;
pub const LOONGARCH_EF_CSR_ERA: c_int = 33;
pub const LOONGARCH_EF_CSR_BADV: c_int = 34;
pub const LOONGARCH_EF_CSR_CRMD: c_int = 35;
pub const LOONGARCH_EF_CSR_PRMD: c_int = 36;
pub const LOONGARCH_EF_CSR_EUEN: c_int = 37;
pub const LOONGARCH_EF_CSR_ECFG: c_int = 38;
pub const LOONGARCH_EF_CSR_ESTAT: c_int = 39;

