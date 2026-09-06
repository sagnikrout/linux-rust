//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/insn-def.h
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

pub const INSN_R_FUNC7_SHIFT: c_int = 25;
pub const INSN_R_RS2_SHIFT: c_int = 20;
pub const INSN_R_RS1_SHIFT: c_int = 15;
pub const INSN_R_FUNC3_SHIFT: c_int = 12;
pub const INSN_R_RD_SHIFT: c_int = 7;
pub const INSN_R_OPCODE_SHIFT: c_int = 0;
pub const INSN_I_SIMM12_SHIFT: c_int = 20;
pub const INSN_I_RS1_SHIFT: c_int = 15;
pub const INSN_I_FUNC3_SHIFT: c_int = 12;
pub const INSN_I_RD_SHIFT: c_int = 7;
pub const INSN_I_OPCODE_SHIFT: c_int = 0;
pub const INSN_S_SIMM7_SHIFT: c_int = 25;
pub const INSN_S_RS2_SHIFT: c_int = 20;
pub const INSN_S_RS1_SHIFT: c_int = 15;
pub const INSN_S_FUNC3_SHIFT: c_int = 12;
pub const INSN_S_SIMM5_SHIFT: c_int = 7;
pub const INSN_S_OPCODE_SHIFT: c_int = 0;

