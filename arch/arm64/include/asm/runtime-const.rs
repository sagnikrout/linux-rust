//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/runtime-const.h
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

// Sigh. You can still run arm64 in BE mode

// 16-bit immediate for wide move (movz and movk) in bits 5..20
// Immediate value is 6 bits starting at bit #16
//
// XXX: Current implementation only supports patching masks of
// form GENMASK(n, 0) (n >= 0) using a single UBFX instruction
// to improve performance, density, and covers all the current
// use-cases.
//
// When the need arises to support any generic mask, and this
// BUG_ON() is tripped, consider using a:
//
// movz %w0, #imm16
// movk %w0, #imm16, lsl #16
//
// sequence to load the 32bit const mask, and perform a logical
// and outside the asm block before returning the result. Fixup
// can simply reuse the existing __runtime_fixup_16() to patch
// the individual mov instructions.
//
// The width of the mask is encoded as (width - 1) in imms
// which is 6 bits starting at bit #10.
//
