//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/acopcode.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Name: acopcode.h - AML opcode information for the AML parser and interpreter
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
pub const MAX_EXTENDED_OPCODE: c_uint = 0x88;

// Macro flag: #define MAX_INTERNAL_OPCODE

// Used for non-assigned opcodes
pub const _UNK: c_uint = 0x6B;
//
// Reserved ASCII characters. Do not use any of these for
// internal opcodes, since they are used to differentiate
// name strings from AML opcodes
//
pub const _ASC: c_uint = 0x6C;
pub const _NAM: c_uint = 0x6C;
pub const _PFX: c_uint = 0x6D;
//
// All AML opcodes and the parse-time arguments for each. Used by the AML
// parser  Each list is compressed into a 32-bit number and stored in the
// master opcode table (in psopcode.c).
//

//
// All AML opcodes and the runtime arguments for each. Used by the AML
// interpreter  Each list is compressed into a 32-bit number and stored
// in the master opcode table (in psopcode.c).
//
// (Used by prep_operands procedure and the ASL Compiler)
//

