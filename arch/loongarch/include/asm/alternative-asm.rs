//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/alternative-asm.h
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
// Issue one struct alt_instr descriptor entry (need to put it into
// the section .altinstructions, see below). This entry contains
// enough information for the alternatives patching code to patch an
// instruction. See apply_alternatives().
//
// Define an alternative between two instructions. If @feature is
// present, early code in apply_alternatives() replaces @oldinstr with
// @newinstr. ".fill" directive takes care of proper instruction padding
// in case @newinstr is longer than @oldinstr.
//

//
// Same as ALTERNATIVE macro above but for two alternatives. If CPU
// has @feature1, it replaces @oldinstr with @newinstr1. If CPU has
// @feature2, it replaces @oldinstr with @feature2.
//

