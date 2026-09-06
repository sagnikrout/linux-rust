//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/kgdb.h
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

pub const DBG_MAX_REG_NUM: c_int = 36;

pub const CACHE_FLUSH_IS_SAFE: c_int = 1;
pub const BUFMAX: c_int = 2048;

pub const BREAK_INSTR_SIZE: c_int = 2;

pub const BREAK_INSTR_SIZE: c_int = 4;

extern "C" {
    pub fn arch_kgdb_breakpoint();
}

pub const DBG_REG_ZERO_OFF: c_int = 0;
pub const DBG_REG_RA_OFF: c_int = 1;
pub const DBG_REG_SP_OFF: c_int = 2;
pub const DBG_REG_GP_OFF: c_int = 3;
pub const DBG_REG_TP_OFF: c_int = 4;
pub const DBG_REG_T0_OFF: c_int = 5;
pub const DBG_REG_T1_OFF: c_int = 6;
pub const DBG_REG_T2_OFF: c_int = 7;
pub const DBG_REG_FP_OFF: c_int = 8;
pub const DBG_REG_S1_OFF: c_int = 9;
pub const DBG_REG_A0_OFF: c_int = 10;
pub const DBG_REG_A1_OFF: c_int = 11;
pub const DBG_REG_A2_OFF: c_int = 12;
pub const DBG_REG_A3_OFF: c_int = 13;
pub const DBG_REG_A4_OFF: c_int = 14;
pub const DBG_REG_A5_OFF: c_int = 15;
pub const DBG_REG_A6_OFF: c_int = 16;
pub const DBG_REG_A7_OFF: c_int = 17;
pub const DBG_REG_S2_OFF: c_int = 18;
pub const DBG_REG_S3_OFF: c_int = 19;
pub const DBG_REG_S4_OFF: c_int = 20;
pub const DBG_REG_S5_OFF: c_int = 21;
pub const DBG_REG_S6_OFF: c_int = 22;
pub const DBG_REG_S7_OFF: c_int = 23;
pub const DBG_REG_S8_OFF: c_int = 24;
pub const DBG_REG_S9_OFF: c_int = 25;
pub const DBG_REG_S10_OFF: c_int = 26;
pub const DBG_REG_S11_OFF: c_int = 27;
pub const DBG_REG_T3_OFF: c_int = 28;
pub const DBG_REG_T4_OFF: c_int = 29;
pub const DBG_REG_T5_OFF: c_int = 30;
pub const DBG_REG_T6_OFF: c_int = 31;
pub const DBG_REG_EPC_OFF: c_int = 32;
pub const DBG_REG_STATUS_OFF: c_int = 33;
pub const DBG_REG_BADADDR_OFF: c_int = 34;
pub const DBG_REG_CAUSE_OFF: c_int = 35;
// NOTE: increase DBG_MAX_REG_NUM if you add more values here.

