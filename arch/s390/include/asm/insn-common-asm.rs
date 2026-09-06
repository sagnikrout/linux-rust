//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/insn-common-asm.h
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
// Assembler helper macros to generate .byte/.word code for instructions
// that are unknown to older binutils versions.
//

//
// GR_NUM - Retrieve general-purpose register number
//
// @opd:	Operand to store register number
// @gr:		String designation register in the format "%rN"
//
// VX_NUM - Retrieve vector register number
//
// @opd:	Operand to store register number
// @vxr:	String designation register in the format "%vN"
//
// The vector register number is used for as input number to the
// instruction and, as well as, to compute the RXB field of the
// instruction.
//

