//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/kgdb.h
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
// Copyright (C) 2001-2004 Amit S. Kale
// Copyright (C) 2008 Wind River Systems, Inc.
//

//
// BUFMAX defines the maximum number of characters in inbound/outbound
// buffers at least NUMREGBYTES*2 are needed for register packets
// Longer buffer is needed to list all threads
//
pub const BUFMAX: c_int = 1024;
//
// Note that this register image is in a different order than
// the register image that Linux produces at interrupt time.
//
// Linux's register image is defined by struct pt_regs in ptrace.h.
// Just why GDB uses a different order is a historical mystery.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum regnames {
    GDB_AX,			/* 0 */
    GDB_CX,			/* 1 */
    GDB_DX,			/* 2 */
    GDB_BX,			/* 3 */
    GDB_SP,			/* 4 */
    GDB_BP,			/* 5 */
    GDB_SI,			/* 6 */
    GDB_DI,			/* 7 */
    GDB_PC,			/* 8 also known as eip */
    GDB_PS,			/* 9 also known as eflags */
    GDB_CS,			/* 10 */
    GDB_SS,			/* 11 */
    GDB_DS,			/* 12 */
    GDB_ES,			/* 13 */
    GDB_FS,			/* 14 */
    GDB_GS,			/* 15 */
}

pub const GDB_ORIG_AX: c_int = 41;
pub const DBG_MAX_REG_NUM: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum regnames {
    GDB_AX,			/* 0 */
    GDB_BX,			/* 1 */
    GDB_CX,			/* 2 */
    GDB_DX,			/* 3 */
    GDB_SI,			/* 4 */
    GDB_DI,			/* 5 */
    GDB_BP,			/* 6 */
    GDB_SP,			/* 7 */
    GDB_R8,			/* 8 */
    GDB_R9,			/* 9 */
    GDB_R10,		/* 10 */
    GDB_R11,		/* 11 */
    GDB_R12,		/* 12 */
    GDB_R13,		/* 13 */
    GDB_R14,		/* 14 */
    GDB_R15,		/* 15 */
    GDB_PC,			/* 16 */
    GDB_PS,			/* 17 */
    GDB_CS,			/* 18 */
    GDB_SS,			/* 19 */
    GDB_DS,			/* 20 */
    GDB_ES,			/* 21 */
    GDB_FS,			/* 22 */
    GDB_GS,			/* 23 */
}

pub const GDB_ORIG_AX: c_int = 57;
pub const DBG_MAX_REG_NUM: c_int = 24;
// 17 64 bit regs and 5 32 bit regs

pub const BREAK_INSTR_SIZE: c_int = 1;
pub const CACHE_FLUSH_IS_SAFE: c_int = 1;
// Macro flag: #define GDB_ADJUSTS_BREAK_OFFSET
