//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/vendor_extensions/mips.h
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
// Copyright (C) 2025 MIPS.
//

pub const RISCV_ISA_VENDOR_EXT_XMIPSEXECTL: c_int = 0;

// Extension specific instructions
//
// All of the xmipsexectl extension instructions are
// ‘hint’ encodings of the SLLI instruction,
// with rd = 0, rs1 = 0 and imm = 1 for IHB, imm = 3 for EHB,
// and imm = 5 for PAUSE.
// MIPS.PAUSE is an alternative opcode which is implemented to have the
// same behavior as PAUSE on some MIPS RISCV cores.
// MIPS.EHB clears all execution hazards before allowing
// any subsequent instructions to execute.
// MIPS.IHB clears all instruction hazards before
// allowing any subsequent instructions to fetch.
//

