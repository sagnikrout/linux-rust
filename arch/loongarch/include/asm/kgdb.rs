//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/kgdb.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2023 Loongson Technology Corporation Limited
//

// gdb remote procotol expects the following register layout.
//
// General purpose registers:
// r0-r31: 64 bit
// orig_a0: 64 bit
// pc : 64 bit
// csr_badvaddr: 64 bit
//
pub const DBG_PT_REGS_BASE: c_int = 0;
pub const DBG_PT_REGS_NUM: c_int = 35;

//
// Floating point registers:
// f0-f31: 64 bit
//

pub const DBG_FPR_NUM: c_int = 32;

//
// Condition Flag registers:
// fcc0-fcc8: 8 bit
//

pub const DBG_FCC_NUM: c_int = 8;

//
// Floating-point Control and Status registers:
// fcsr: 32 bit
//
pub const DBG_FCSR_NUM: c_int = 1;

//
// Size of I/O buffer for gdb packet.
// considering to hold all register contents, size is set
//
pub const BUFMAX: c_int = 2048;
//
// Number of bytes required for gdb_regs buffer.
// PT_REGS and FPR: 8 bytes; FCSR: 4 bytes; FCC: 1 bytes.
// GDB fails to connect for size beyond this with error
// "'g' packet reply is too long"
//

pub const BREAK_INSTR_SIZE: c_int = 4;
pub const CACHE_FLUSH_IS_SAFE: c_int = 0;
// Register numbers of various important registers.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dbg_loongarch_regnum {
    DBG_LOONGARCH_ZERO = 0,
    DBG_LOONGARCH_RA,
    DBG_LOONGARCH_TP,
    DBG_LOONGARCH_SP,
    DBG_LOONGARCH_A0,
    DBG_LOONGARCH_FP = 22,
    DBG_LOONGARCH_S0,
    DBG_LOONGARCH_S1,
    DBG_LOONGARCH_S2,
    DBG_LOONGARCH_S3,
    DBG_LOONGARCH_S4,
    DBG_LOONGARCH_S5,
    DBG_LOONGARCH_S6,
    DBG_LOONGARCH_S7,
    DBG_LOONGARCH_S8,
    DBG_LOONGARCH_ORIG_A0,
    DBG_LOONGARCH_PC,
    DBG_LOONGARCH_BADV
}

extern "C" {
    pub fn kgdb_breakinst();
}
extern "C" {
    pub fn arch_kgdb_breakpoint();
}

extern "C" {
    pub fn kgdb_breakpoint_handler(regs: *mut pt_regs) -> bool;
}

